use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RopeType {
    Rope,
    SteelCable,
}

impl PartDef for RopeType {
    fn name(&self) -> &'static str {
        match self {
            RopeType::Rope => "Rope",
            RopeType::SteelCable => "Steel Cable",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            RopeType::Rope => "Max 600px total; transmits force when taut; max 8 pulleys; cut by scissors/hedge trimmers/tin snips",
            RopeType::SteelCable => "Rigid line; ONLY cut by tin snips",
        }
    }

    fn category(&self) -> &'static str { "Ropes" }

    fn default_size(&self) -> (f32, f32) { (64.0, 32.0) }

    fn icon_char(&self) -> char { '\u{2502}' }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            RopeType::Rope => BROWN,
            RopeType::SteelCable => SILVER,
        }
    }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: 0.01,
            elasticity: 0.0,
            density: 0.5,
            friction: 0.3,
            gravity_response: GravityResponse::Normal,
            is_static: false,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        vec![
            StateDef { name: "Slack", description: "Not under tension — no force transmitted" },
            StateDef { name: "Taut", description: "Under tension — transmitting force" },
            StateDef { name: "Cut", description: "Severed — connected parts lose tension" },
        ]
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        // State 0=Slack, 1=Taut, 2=Cut
        let length = props.values.get("length").copied().unwrap_or(64.0);
        let ic = self.icon_color();

        if props.current_state == 2 {
            // Cut — two dangling halves
            let half = length / 2.0;
            let cut_color = [ic[0] / 2, ic[1] / 2, ic[2] / 2];
            // Left half drooping down
            for i in 0..(half as i32) {
                let t = i as f32 / half;
                let px = x + i as f32;
                let sag = t * t * 20.0; // gravity droop
                blend_pixel(img, px as i32, (y + sag) as i32, [cut_color[0], cut_color[1], cut_color[2], 200]);
            }
            // Right half drooping down
            for i in 0..(half as i32) {
                let t = i as f32 / half;
                let px = x + half + i as f32;
                let sag = (1.0 - t) * (1.0 - t) * 20.0;
                blend_pixel(img, px as i32, (y + sag) as i32, [cut_color[0], cut_color[1], cut_color[2], 200]);
            }
            // Frayed ends at cut point
            for dy in 0..3 {
                blend_pixel(img, (x + half) as i32 - 1, (y + 2.0) as i32 + dy, [ic[0], ic[1], ic[2], 120]);
                blend_pixel(img, (x + half) as i32 + 1, (y + 2.0) as i32 + dy, [ic[0], ic[1], ic[2], 120]);
            }
            return;
        }

        let sag_factor = match props.current_state {
            1 => 0.02, // Taut — nearly straight
            _ => match self { // Slack
                RopeType::Rope => 0.15,
                RopeType::SteelCable => 0.05,
            },
        };

        let segments = length as i32;
        for i in 0..segments {
            let t = i as f32 / segments as f32;
            let px = x + t * length;
            let sag = (t * std::f32::consts::PI).sin() * length * sag_factor;
            // Slight vibration when taut
            let vib = if props.current_state == 1 {
                ((frame as f32 * 0.5 + i as f32 * 0.3).sin() * 0.5) as f32
            } else { 0.0 };
            let py = y + sag + vib;
            blend_pixel(img, px as i32, py as i32, [ic[0], ic[1], ic[2], 255]);
            if matches!(self, RopeType::Rope) {
                blend_pixel(img, px as i32, py as i32 + 1, [ic[0].saturating_sub(20), ic[1].saturating_sub(20), ic[2].saturating_sub(20), 200]);
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
        vec![
            PropertyDef { name: "length".into(), min: 16.0, max: 600.0, step: 16.0, default: 64.0, label: "Length (max 600px)".into() },
        ]
    }
}
