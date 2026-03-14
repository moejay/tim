use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaserType {
    RedLaser,
    GreenLaser,
    BlueLaser,
    AngledMirror,
    LaserMixer,
    LaserDetector,
    LaserActivatedPlug,
}

impl LaserType {
    fn laser_color(&self) -> [u8; 3] {
        match self {
            LaserType::RedLaser => [255, 40, 40],
            LaserType::GreenLaser => [40, 255, 40],
            LaserType::BlueLaser => [40, 80, 255],
            _ => WHITE,
        }
    }
}

impl PartDef for LaserType {
    fn name(&self) -> &'static str {
        match self {
            LaserType::RedLaser => "Red Laser",
            LaserType::GreenLaser => "Green Laser",
            LaserType::BlueLaser => "Blue Laser",
            LaserType::AngledMirror => "Angled Mirror",
            LaserType::LaserMixer => "Laser Mixer",
            LaserType::LaserDetector => "Laser Detector",
            LaserType::LaserActivatedPlug => "Laser-Activated Plug",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            LaserType::RedLaser => "Red laser emitter; ignites fuses/candles; pops balloons",
            LaserType::GreenLaser => "Green laser emitter; ignites fuses/candles; pops balloons",
            LaserType::BlueLaser => "Blue laser emitter; ignites fuses/candles; pops balloons",
            LaserType::AngledMirror => "Reflective surface; redirects laser beams 90 degrees",
            LaserType::LaserMixer => "Prism — combines colored laser inputs",
            LaserType::LaserDetector => "Sensor — triggers on any laser beam hit",
            LaserType::LaserActivatedPlug => "Powers outlet only when correct laser color hits",
        }
    }

    fn category(&self) -> &'static str { "Lasers" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser => (24.0, 12.0),
            LaserType::AngledMirror | LaserType::LaserDetector => (16.0, 16.0),
            LaserType::LaserMixer => (24.0, 24.0),
            LaserType::LaserActivatedPlug => (16.0, 24.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser => '\u{2500}',
            LaserType::AngledMirror => '\u{2571}',
            LaserType::LaserMixer => '\u{25C7}',
            LaserType::LaserDetector | LaserType::LaserActivatedPlug => '\u{25C9}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            LaserType::RedLaser => RED,
            LaserType::GreenLaser => GREEN,
            LaserType::BlueLaser => BLUE,
            _ => WHITE,
        }
    }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: f32::INFINITY,
            elasticity: 0.1,
            density: 100.0,
            friction: 0.3,
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser => vec![
                StateDef { name: "Off", description: "Not emitting" },
                StateDef { name: "Emitting", description: "Beam active — ignites/pops on contact" },
            ],
            LaserType::LaserDetector => vec![
                StateDef { name: "Idle", description: "No beam hitting sensor" },
                StateDef { name: "Triggered", description: "Beam detected — signal active" },
            ],
            LaserType::LaserActivatedPlug => vec![
                StateDef { name: "Inactive", description: "Wrong color or no laser" },
                StateDef { name: "Active", description: "Correct laser color — providing power" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Passive component" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let ix = x as i32;
        let iy = y as i32;

        match self {
            LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser => {
                // State 0=Off, 1=Emitting
                let lc = self.laser_color();
                fill_rect(img, ix, iy, 16, 12, [80, 80, 90, 255]);
                let lens_x = ix + 16;
                if props.current_state == 1 {
                    // Emitting — bright lens + beam
                    fill_rect(img, lens_x, iy + 2, 4, 8, [lc[0], lc[1], lc[2], 255]);
                    draw_glow(img, x + 18.0, y + 6.0, 6.0, lc);
                    // Laser beam extending
                    for bx in 0..40 {
                        let alpha = (220 - bx * 4) as u8;
                        let flicker = ((frame as f32 * 0.3 + bx as f32 * 0.1).sin() * 20.0) as u8;
                        blend_pixel(img, ix + 20 + bx, iy + 6, [lc[0], lc[1], lc[2], alpha.saturating_sub(flicker)]);
                        blend_pixel(img, ix + 20 + bx, iy + 5, [lc[0], lc[1], lc[2], alpha / 3]);
                        blend_pixel(img, ix + 20 + bx, iy + 7, [lc[0], lc[1], lc[2], alpha / 3]);
                    }
                } else {
                    // Off — dim lens
                    fill_rect(img, lens_x, iy + 2, 4, 8, [lc[0] / 3, lc[1] / 3, lc[2] / 3, 200]);
                }
            }
            LaserType::AngledMirror => {
                let rotation = props.values.get("rotation").copied().unwrap_or(0.0) as i32;
                match rotation % 4 {
                    0 | 2 => draw_line_aa(img, x, y + 16.0, x + 16.0, y, [220, 220, 230]),
                    _ => draw_line_aa(img, x, y, x + 16.0, y + 16.0, [220, 220, 230]),
                }
                fill_rect(img, ix + 4, iy + 14, 8, 2, [120, 120, 130, 255]);
                let mid_x = x + 8.0;
                let mid_y = y + 8.0;
                blend_pixel(img, mid_x as i32, mid_y as i32, [255, 255, 255, 180]);
            }
            LaserType::LaserMixer => {
                fill_triangle(img, (x + 12.0, y), (x, y + 12.0), (x + 12.0, y + 24.0), [220, 220, 240, 200]);
                fill_triangle(img, (x + 12.0, y), (x + 24.0, y + 12.0), (x + 12.0, y + 24.0), [220, 220, 240, 200]);
                blend_pixel(img, ix, iy + 6, [255, 40, 40, 255]);
                blend_pixel(img, ix, iy + 12, [40, 255, 40, 255]);
                blend_pixel(img, ix, iy + 18, [40, 80, 255, 255]);
                draw_glow(img, x + 12.0, y + 12.0, 6.0, [255, 255, 255]);
            }
            LaserType::LaserDetector => {
                // State 0=Idle, 1=Triggered
                fill_rect(img, ix + 2, iy + 2, 12, 12, [80, 80, 90, 255]);
                fill_circle(img, x + 8.0, y + 8.0, 4.0, [40, 40, 50, 255]);
                if props.current_state == 1 {
                    // Triggered — solid green LED
                    fill_circle(img, x + 8.0, y + 8.0, 2.0, [100, 255, 100, 255]);
                    draw_glow(img, x + 8.0, y + 8.0, 6.0, [100, 255, 100]);
                } else {
                    // Idle — dim red blink
                    if frame % 40 < 5 {
                        fill_circle(img, x + 8.0, y + 8.0, 1.5, [200, 50, 50, 180]);
                    }
                }
            }
            LaserType::LaserActivatedPlug => {
                // State 0=Inactive, 1=Active
                fill_rect(img, ix + 2, iy + 6, 12, 18, [220, 215, 210, 255]);
                let activation = props.values.get("activation_color").copied().unwrap_or(0.0) as i32;
                let lens_color = match activation {
                    0 => [255, 40, 40], 1 => [40, 255, 40], _ => [40, 80, 255],
                };
                if props.current_state == 1 {
                    // Active — glowing lens
                    fill_circle(img, x + 8.0, y + 4.0, 3.0, [lens_color[0], lens_color[1], lens_color[2], 255]);
                    draw_glow(img, x + 8.0, y + 4.0, 5.0, lens_color);
                    // Power indicator
                    blend_pixel(img, ix + 8, iy + 10, [100, 255, 100, 200]);
                } else {
                    // Dim lens
                    fill_circle(img, x + 8.0, y + 4.0, 3.0, [lens_color[0] / 3, lens_color[1] / 3, lens_color[2] / 3, 200]);
                }
                fill_rect(img, ix + 4, iy + 14, 3, 4, [40, 40, 40, 255]);
                fill_rect(img, ix + 9, iy + 14, 3, 4, [40, 40, 40, 255]);
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
            LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser => vec![
                PropertyDef { name: "rotation".into(), min: 0.0, max: 3.0, step: 1.0, default: 0.0, label: "Direction (0-3)".into() },
            ],
            LaserType::AngledMirror => vec![
                PropertyDef { name: "rotation".into(), min: 0.0, max: 3.0, step: 1.0, default: 0.0, label: "Rotation (0-3)".into() },
            ],
            LaserType::LaserActivatedPlug => vec![
                PropertyDef { name: "activation_color".into(), min: 0.0, max: 2.0, step: 1.0, default: 0.0, label: "Color (0=R/1=G/2=B)".into() },
            ],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self, LaserType::RedLaser | LaserType::GreenLaser | LaserType::BlueLaser | LaserType::LaserDetector)
    }

    fn provides_power(&self) -> bool {
        matches!(self, LaserType::LaserActivatedPlug)
    }
}
