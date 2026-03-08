use anyhow::Result;
use image::{DynamicImage, Rgba, RgbaImage};
use log::warn;
use std::io::Write;

use crossterm::{cursor, execute, style, terminal};

use crate::state::*;
use crate::parts::{ball, ramp, wall, basket, cannon};
use crate::hud;
use super::pixel_gfx::*;
use super::Renderer;

pub struct PixelRenderer {
    img: RgbaImage,
    viuer_config: viuer::Config,
}

impl PixelRenderer {
    pub fn new() -> Result<Self> {
        let img = RgbaImage::new(CANVAS_W, CANVAS_H);
        let viuer_config = viuer::Config {
            width: Some(80),
            height: None,
            absolute_offset: false,
            ..Default::default()
        };
        Ok(Self { img, viuer_config })
    }
}

impl Renderer for PixelRenderer {
    fn render_frame(&mut self, state: &GameState) -> Result<()> {
        let _render_start = std::time::Instant::now();

        // 1. Clear image to BG_COLOR
        for pixel in self.img.pixels_mut() {
            *pixel = Rgba(BG_COLOR);
        }

        // 2. Draw playfield border
        let border_color: [u8; 4] = [26, 26, 42, 255];
        // Left edge (x=0)
        for y in 0..CANVAS_H as i32 {
            blend_pixel(&mut self.img, 0, y, border_color);
        }
        // Right edge (x=511)
        for y in 0..CANVAS_H as i32 {
            blend_pixel(&mut self.img, 511, y, border_color);
        }
        // Top edge (y=0)
        for x in 0..512 {
            blend_pixel(&mut self.img, x, 0, border_color);
        }
        // Bottom edge (y=359)
        for x in 0..512 {
            blend_pixel(&mut self.img, x, CANVAS_H as i32 - 1, border_color);
        }

        // 3. Fill parts bin background (x: 512..640, y: 0..360)
        fill_rect(&mut self.img, 512, 0, BIN_W as i32, CANVAS_H as i32, BIN_BG_COLOR);

        // 4. Draw bin divider lines between slots (every 60px in the bin area)
        for slot in 1..6 {
            let y = slot * 60;
            draw_line(&mut self.img, 512, y, CANVAS_W as i32 - 1, y, BIN_DIVIDER_COLOR);
        }

        // 5. Draw each part
        for (i, part) in state.parts.iter().enumerate() {
            match &part.kind {
                PartKind::Ball => {
                    ball::draw_ball_pixel(&mut self.img, part.x + 14.0, part.y + 14.0);
                }
                PartKind::Ramp => {
                    ramp::draw_ramp_pixel(&mut self.img, part.x, part.y, part.flipped);
                }
                PartKind::Wall { width, height } => {
                    let (w, h) = if part.flipped {
                        (*height, *width)
                    } else {
                        (*width, *height)
                    };
                    wall::draw_wall_pixel(&mut self.img, part.x, part.y, w, h);
                }
                PartKind::Basket => {
                    let ball_near = if state.ball.active {
                        let bx = state.ball.pos.0;
                        let by = state.ball.pos.1;
                        let cx = part.x + 32.0;
                        let cy = part.y + 32.0;
                        let dx = bx - cx;
                        let dy = by - cy;
                        (dx * dx + dy * dy).sqrt() < 48.0
                    } else {
                        false
                    };
                    basket::draw_basket_pixel(
                        &mut self.img,
                        part.x,
                        part.y,
                        ball_near,
                        state.frame,
                    );
                }
                PartKind::Cannon { angle_deg, .. } => {
                    cannon::draw_cannon_pixel(
                        &mut self.img,
                        part.x,
                        part.y,
                        *angle_deg,
                        part.flipped,
                        state.frame,
                    );
                }
            }

            // If part is fixed, draw a dim overlay
            if part.fixed {
                let pw = part.kind.width(part.flipped) as i32;
                let ph = part.kind.height(part.flipped) as i32;
                let px = part.x as i32;
                let py = part.y as i32;
                fill_rect(&mut self.img, px, py, pw, ph, [0, 0, 0, 40]);
                // Small lock indicator in top-right corner
                fill_rect(&mut self.img, px + pw - 6, py + 1, 5, 5, [80, 80, 100, 180]);
            }
        }

        // 6. EDIT mode: draw pulsing green outline around edited part
        if let Mode::Edit { part_idx } = state.mode {
            if let Some(part) = state.parts.get(part_idx) {
                let alpha =
                    ((state.frame as f32 * 0.15).sin() * 0.3 + 0.5).clamp(0.0, 1.0) * 255.0;
                let color = [0, 230, 118, alpha as u8];
                let px = part.x as i32;
                let py = part.y as i32;
                let pw = part.kind.width(part.flipped) as i32;
                let ph = part.kind.height(part.flipped) as i32;

                // 2px outline: top
                fill_rect(&mut self.img, px - 2, py - 2, pw + 4, 2, color);
                // bottom
                fill_rect(&mut self.img, px - 2, py + ph, pw + 4, 2, color);
                // left
                fill_rect(&mut self.img, px - 2, py, 2, ph, color);
                // right
                fill_rect(&mut self.img, px + pw, py, 2, ph, color);
            }
        }

        // 7. PLACE mode: draw ghost outline of selected part at cursor position
        if let Mode::Place { bin_idx } = state.mode {
            if let Some(item) = state.bin_items.get(bin_idx) {
                let pw = item.kind.width(false) as i32;
                let ph = item.kind.height(false) as i32;
                let cx = state.cursor.0 as i32 - pw / 2;
                let cy = state.cursor.1 as i32 - ph / 2;
                // Ghost outline at 40% opacity
                let ghost = [200, 200, 200, 102]; // ~40% of 255
                // top
                fill_rect(&mut self.img, cx, cy, pw, 1, ghost);
                // bottom
                fill_rect(&mut self.img, cx, cy + ph - 1, pw, 1, ghost);
                // left
                fill_rect(&mut self.img, cx, cy, 1, ph, ghost);
                // right
                fill_rect(&mut self.img, cx + pw - 1, cy, 1, ph, ghost);
            }
        }

        // 8. Draw crosshair cursor (not in RUN mode)
        if state.mode != Mode::Run {
            let (cr, cg, cb) = match state.mode {
                Mode::Normal | Mode::Edit { .. } => (0u8, 229u8, 255u8),
                Mode::Place { .. } => (255u8, 214u8, 0u8),
                _ => (255, 255, 255),
            };
            let alpha =
                ((state.frame as f32 * 0.12).sin() * 0.2 + 0.8).clamp(0.0, 1.0) * 255.0;
            let color = [cr, cg, cb, alpha as u8];
            draw_crosshair(&mut self.img, state.cursor.0, state.cursor.1, color, 4.0, 8.0);
        }

        // 9. RUN mode: draw ball trail and ball
        if state.mode == Mode::Run && state.ball.active {
            ball::draw_trail_pixel(&mut self.img, &state.ball.trail);
            ball::draw_ball_pixel(&mut self.img, state.ball.pos.0, state.ball.pos.1);
        }

        // 10. Win overlay
        if state.won {
            let ox = (PLAYFIELD_W as i32 - 200) / 2;
            let oy = (PLAYFIELD_H as i32 - 120) / 2;
            // Semi-transparent black backdrop
            fill_rect(&mut self.img, ox, oy, 200, 120, [0, 0, 0, 180]);
            // "PUZZLE SOLVED!" text centered, green
            let text1 = "PUZZLE SOLVED!";
            let tw = text_width(text1, 2);
            let tx = ox + (200 - tw) / 2;
            draw_text(&mut self.img, tx, oy + 20, text1, [0, 230, 118, 255], 2);
            // "[SPACE] TRY AGAIN  [Q] QUIT" below, white
            let text2 = "[SPACE] AGAIN [Q] QUIT";
            let tw2 = text_width(text2, 1);
            let tx2 = ox + (200 - tw2) / 2;
            draw_text(&mut self.img, tx2, oy + 60, text2, [200, 200, 200, 255], 1);
        }

        // 11. Help overlay
        if state.show_help {
            let ox = 40;
            let oy = 30;
            let ow = 432;
            let oh = 300;
            fill_rect(&mut self.img, ox, oy, ow, oh, [10, 10, 18, 220]);

            let help_lines = [
                "KEYBINDINGS",
                "",
                "[1]-[5] SELECT PART FROM BIN",
                "[CLICK/ENTER] PLACE PART",
                "[E] EDIT SELECTED PART",
                "[F] FLIP PART",
                "[DEL] DELETE PART",
                "[U] UNDO",
                "[SPACE] RUN SIMULATION",
                "[R] RESET SIMULATION",
                "[H] TOGGLE HELP",
                "[Q] QUIT",
            ];
            for (i, line) in help_lines.iter().enumerate() {
                let color = if i == 0 {
                    [0, 229, 255, 255]
                } else {
                    [180, 180, 200, 255]
                };
                let scale = if i == 0 { 2 } else { 1 };
                draw_text(&mut self.img, ox + 16, oy + 16 + i as i32 * 22, line, color, scale);
            }
        }

        // 12. Draw parts bin content
        for (slot, item) in state.bin_items.iter().enumerate().take(5) {
            let slot_y = slot as i32 * 60;
            let bin_x = 512;

            // If PLACE mode and this slot selected, highlight
            if let Mode::Place { bin_idx } = state.mode {
                if bin_idx == slot {
                    fill_rect(&mut self.img, bin_x, slot_y, BIN_W as i32, 60, [255, 214, 0, 64]);
                }
            }

            // Slot key hint
            let key_str = format!("[{}]", slot + 1);
            draw_text(
                &mut self.img,
                bin_x + 4,
                slot_y + 4,
                &key_str,
                [120, 120, 150, 255],
                1,
            );

            // Icon: draw a miniature representation
            let icon_x = bin_x + 30;
            let icon_y = slot_y + 12;
            match &item.kind {
                PartKind::Ball => {
                    fill_circle_gradient(
                        &mut self.img,
                        icon_x as f32 + 10.0,
                        icon_y as f32 + 10.0,
                        8.0,
                        [255, 140, 66],
                        [179, 58, 0],
                    );
                }
                PartKind::Ramp => {
                    fill_triangle(
                        &mut self.img,
                        (icon_x as f32, (icon_y + 20) as f32),
                        ((icon_x + 20) as f32, (icon_y + 20) as f32),
                        ((icon_x + 20) as f32, icon_y as f32),
                        [222, 184, 135, 255],
                    );
                }
                PartKind::Wall { .. } => {
                    fill_rect(&mut self.img, icon_x, icon_y, 20, 16, [90, 122, 138, 255]);
                }
                PartKind::Basket => {
                    // Simple U-shape
                    fill_rect(&mut self.img, icon_x, icon_y, 2, 20, [218, 165, 32, 255]);
                    fill_rect(&mut self.img, icon_x + 18, icon_y, 2, 20, [218, 165, 32, 255]);
                    fill_rect(&mut self.img, icon_x, icon_y + 18, 20, 2, [218, 165, 32, 255]);
                }
                PartKind::Cannon { .. } => {
                    // Simple barrel shape
                    fill_rect(&mut self.img, icon_x, icon_y + 4, 20, 12, [100, 100, 120, 255]);
                    fill_rect(&mut self.img, icon_x + 16, icon_y + 2, 4, 16, [80, 80, 100, 255]);
                }
            }

            // Label text
            let label = item.kind.label().to_ascii_uppercase();
            draw_text(
                &mut self.img,
                bin_x + 60,
                slot_y + 14,
                &label,
                [180, 180, 200, 255],
                1,
            );

            // Quantity
            let qty = format!("X{}", item.count);
            draw_text(
                &mut self.img,
                bin_x + 60,
                slot_y + 30,
                &qty,
                [140, 140, 160, 255],
                1,
            );
        }

        // 13. Transmit image via viuer
        execute!(std::io::stdout(), cursor::MoveTo(0, 0))?;
        let dyn_img = DynamicImage::ImageRgba8(self.img.clone());
        viuer::print(&dyn_img, &self.viuer_config)?;

        // 14. Write HUD text via crossterm
        let mut stdout = std::io::stdout();
        let line1 = hud::hud_line1(state);
        let line2 = hud::hud_line2(state);

        execute!(
            stdout,
            style::SetForegroundColor(style::Color::Rgb { r: 0, g: 229, b: 255 }),
            style::Print(&line1),
            style::ResetColor,
            style::Print("\r\n"),
            style::SetForegroundColor(style::Color::Rgb {
                r: 160,
                g: 160,
                b: 180
            }),
            style::Print(&line2),
            style::ResetColor,
            style::Print("\r\n"),
        )?;
        stdout.flush()?;

        let render_ms = _render_start.elapsed().as_millis();
        if render_ms > 32 {
            warn!("Slow render frame {}: {}ms", state.frame, render_ms);
        }

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            cursor::Show,
            terminal::Clear(terminal::ClearType::All),
            cursor::MoveTo(0, 0),
        )?;
        Ok(())
    }
}
