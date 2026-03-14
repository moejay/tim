use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipeType {
    StraightPipe,
    TConnector,
    CurvedPipe,
    AcceleratorTube,
}

impl PartDef for PipeType {
    fn name(&self) -> &'static str {
        match self {
            PipeType::StraightPipe => "Straight Pipe",
            PipeType::TConnector => "T-Connector",
            PipeType::CurvedPipe => "Curved Pipe",
            PipeType::AcceleratorTube => "Accelerator Tube",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            PipeType::StraightPipe => "Metal tube with open ends",
            PipeType::TConnector => "3-way junction",
            PipeType::CurvedPipe => "90-degree bend",
            PipeType::AcceleratorTube => "Glowing tube; speed multiplier 1.5x/2x/3x",
        }
    }

    fn category(&self) -> &'static str { "Pipes" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            PipeType::StraightPipe => (64.0, 24.0),
            PipeType::TConnector | PipeType::CurvedPipe => (24.0, 24.0),
            PipeType::AcceleratorTube => (32.0, 24.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            PipeType::StraightPipe => '\u{2550}',
            PipeType::TConnector => '\u{2566}',
            PipeType::CurvedPipe => '\u{256E}',
            PipeType::AcceleratorTube => '\u{21D2}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            PipeType::AcceleratorTube => CYAN,
            _ => GRAY,
        }
    }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: f32::INFINITY,
            elasticity: 0.1,
            density: 100.0,
            friction: 0.2,
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            PipeType::AcceleratorTube => vec![
                StateDef { name: "Idle", description: "No object inside" },
                StateDef { name: "Accelerating", description: "Object passing through — speed multiplied" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Static pipe segment" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let ix = x as i32;
        let iy = y as i32;
        let ic = self.icon_color();

        match self {
            PipeType::StraightPipe => {
                let length = props.values.get("length").copied().unwrap_or(64.0) as i32;
                let orientation = props.values.get("orientation").copied().unwrap_or(0.0);
                if orientation < 0.5 {
                    fill_rect_gradient_v(img, ix, iy + 4, length, 16, [180, 180, 190], [120, 120, 130]);
                    draw_line(img, ix, iy + 8, ix + length, iy + 8, [210, 210, 220, 120]);
                    fill_rect(img, ix, iy + 3, 2, 18, [140, 140, 150, 255]);
                    fill_rect(img, ix + length - 2, iy + 3, 2, 18, [140, 140, 150, 255]);
                } else {
                    fill_rect_gradient_v(img, ix + 4, iy, 16, length, [180, 180, 190], [120, 120, 130]);
                    draw_line(img, ix + 12, iy, ix + 12, iy + length, [210, 210, 220, 120]);
                    fill_rect(img, ix + 3, iy, 18, 2, [140, 140, 150, 255]);
                    fill_rect(img, ix + 3, iy + length - 2, 18, 2, [140, 140, 150, 255]);
                }
            }
            PipeType::TConnector => {
                let rotation = props.values.get("rotation").copied().unwrap_or(0.0) as i32;
                fill_rect(img, ix, iy + 4, 24, 16, [ic[0], ic[1], ic[2], 255]);
                match rotation % 4 {
                    0 => fill_rect(img, ix + 8, iy, 8, 8, [ic[0], ic[1], ic[2], 255]),
                    1 => fill_rect(img, ix + 16, iy + 8, 8, 8, [ic[0], ic[1], ic[2], 255]),
                    2 => fill_rect(img, ix + 8, iy + 16, 8, 8, [ic[0], ic[1], ic[2], 255]),
                    _ => fill_rect(img, ix, iy + 8, 8, 8, [ic[0], ic[1], ic[2], 255]),
                }
                draw_line(img, ix, iy + 8, ix + 24, iy + 8, [200, 200, 210, 100]);
            }
            PipeType::CurvedPipe => {
                let rotation = props.values.get("rotation").copied().unwrap_or(0.0) as i32;
                for t in 0..90 {
                    let angle = (t as f32 + rotation as f32 * 90.0) * std::f32::consts::PI / 180.0;
                    for thickness in 0..8 {
                        let r = 8.0 + thickness as f32;
                        let px = x + 12.0 + angle.cos() * r;
                        let py = y + 12.0 + angle.sin() * r;
                        let shade = if thickness < 3 { 180 } else { 140 };
                        blend_pixel(img, px as i32, py as i32, [shade, shade, shade + 10, 255]);
                    }
                }
            }
            PipeType::AcceleratorTube => {
                // State 0=Idle, 1=Accelerating
                let speed_mult = props.values.get("speed_mult").copied().unwrap_or(2.0);
                let base_bright = if props.current_state == 1 { 220 } else { 140 };
                fill_rect_gradient_v(img, ix, iy + 4, 32, 16, [0, base_bright * 8 / 10, base_bright], [0, base_bright * 5 / 10, base_bright * 6 / 10]);
                if props.current_state == 1 {
                    // Active — bright glow + fast arrows
                    let glow = ((frame as f32 * 0.2).sin() * 0.2 + 0.8) as f32;
                    let alpha = (glow * 140.0) as u8;
                    fill_rect(img, ix + 2, iy + 6, 28, 12, [0, 240, 255, alpha]);
                    // Fast scrolling arrows proportional to speed_mult
                    let arrow_speed = (speed_mult * 3.0) as i32;
                    let arrow_phase = (frame as i32 * arrow_speed) % 16;
                    for ax in (arrow_phase..32).step_by(8) {
                        if ax > 0 && ax < 28 {
                            draw_line(img, ix + ax - 3, iy + 12, ix + ax, iy + 9, [255, 255, 255, 220]);
                            draw_line(img, ix + ax - 3, iy + 12, ix + ax, iy + 15, [255, 255, 255, 220]);
                        }
                    }
                    // Object streak
                    let obj_x = ix + (frame as i32 * 4) % 32;
                    fill_circle(img, obj_x as f32, y + 12.0, 3.0, [255, 255, 200, 150]);
                } else {
                    // Idle — dim, static arrows
                    fill_rect(img, ix + 2, iy + 6, 28, 12, [0, 160, 180, 60]);
                    for ax in (8..32).step_by(16) {
                        draw_line(img, ix + ax - 3, iy + 12, ix + ax, iy + 9, [200, 200, 200, 100]);
                        draw_line(img, ix + ax - 3, iy + 12, ix + ax, iy + 15, [200, 200, 200, 100]);
                    }
                }
            }
        }
    }

    fn draw_text(&self, buf: &mut Buffer, area: Rect, _props: &PartProps, _frame: u64) {
        if area.width == 0 || area.height == 0 { return; }
        let c = self.icon_color();
        let color = Color::Rgb(c[0], c[1], c[2]);
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        if cx < area.right() && cy < area.bottom() {
            buf[(cx, cy)].set_char(self.icon_char()).set_style(Style::default().fg(color));
        }
    }

    fn properties(&self) -> Vec<PropertyDef> {
        match self {
            PipeType::StraightPipe => vec![
                PropertyDef { name: "length".into(), min: 32.0, max: 192.0, step: 32.0, default: 64.0, label: "Length".into() },
                PropertyDef { name: "orientation".into(), min: 0.0, max: 1.0, step: 1.0, default: 0.0, label: "Orient (0=H/1=V)".into() },
            ],
            PipeType::TConnector | PipeType::CurvedPipe => vec![
                PropertyDef { name: "rotation".into(), min: 0.0, max: 3.0, step: 1.0, default: 0.0, label: "Rotation (0-3)".into() },
            ],
            PipeType::AcceleratorTube => vec![
                PropertyDef { name: "speed_mult".into(), min: 1.5, max: 3.0, step: 0.5, default: 2.0, label: "Speed Mult".into() },
                PropertyDef { name: "direction".into(), min: 0.0, max: 3.0, step: 1.0, default: 0.0, label: "Direction (0-3)".into() },
            ],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self, PipeType::AcceleratorTube)
    }

    fn can_be_ramp(&self) -> bool { true }
}
