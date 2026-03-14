use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InclineType {
    BrickIncline,
    YellowBrickIncline,
    GraniteIncline,
}

impl InclineType {
    fn base_color(&self) -> [u8; 3] {
        match self {
            InclineType::BrickIncline => RED_BROWN,
            InclineType::YellowBrickIncline => YELLOW,
            InclineType::GraniteIncline => GRAY,
        }
    }

    /// Stretch level → (width, height) in px.
    /// VeryShort=30px/~70°, Short=60px/~45°, Medium=100px/~30°, Long=150px/~18°, VeryLong=200px/~10°
    fn size_dims(size_level: f32) -> (f32, f32) {
        match size_level as i32 {
            0 => (30.0, 28.0),
            1 => (60.0, 32.0),
            2 => (100.0, 36.0),
            3 => (150.0, 40.0),
            _ => (200.0, 44.0),
        }
    }

    fn incline_friction(&self) -> f32 {
        match self {
            InclineType::GraniteIncline => 0.3,
            _ => 0.4,
        }
    }
}

impl PartDef for InclineType {
    fn name(&self) -> &'static str {
        match self {
            InclineType::BrickIncline => "Brick Incline",
            InclineType::YellowBrickIncline => "Yellow Brick Incline",
            InclineType::GraniteIncline => "Granite Incline",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            InclineType::BrickIncline => "Brick-textured ramp, 5 stretch sizes, flippable",
            InclineType::YellowBrickIncline => "Yellow brick ramp, 5 stretch sizes, flippable",
            InclineType::GraniteIncline => "Granite-textured ramp, 5 stretch sizes, lower friction",
        }
    }

    fn category(&self) -> &'static str { "Inclines" }

    fn default_size(&self) -> (f32, f32) { (100.0, 36.0) }

    fn icon_char(&self) -> char { '\u{25E2}' }

    fn icon_color(&self) -> [u8; 3] { self.base_color() }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: f32::INFINITY,
            elasticity: 0.2,
            density: 100.0,
            friction: self.incline_friction(),
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        vec![StateDef { name: "Intact", description: "Solid incline surface" }]
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, _frame: u64) {
        // State 0=Intact (only state — indestructible by dynamite)
        let _ = props.current_state;
        let size_level = props.values.get("size").copied().unwrap_or(2.0);
        let (w, h) = InclineType::size_dims(size_level);
        let bc = self.base_color();
        let darker = [bc[0].saturating_sub(30), bc[1].saturating_sub(30), bc[2].saturating_sub(30)];

        if props.flipped {
            fill_triangle_gradient(img, (x + w, y), (x, y + h), (x + w, y + h), bc, darker);
            draw_line_aa(img, x + w, y, x, y + h, [255, 255, 255]);
        } else {
            fill_triangle_gradient(img, (x, y), (x, y + h), (x + w, y + h), bc, darker);
            draw_line_aa(img, x, y, x + w, y + h, [255, 255, 255]);
        }
    }

    fn draw_text(&self, buf: &mut Buffer, area: Rect, props: &PartProps, _frame: u64) {
        if area.width == 0 || area.height == 0 { return; }
        let c = self.icon_color();
        let color = Color::Rgb(c[0], c[1], c[2]);
        let ch = if props.flipped { '\u{25E3}' } else { '\u{25E2}' };
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        if cx < area.right() && cy < area.bottom() {
            buf[(cx, cy)].set_char(ch).set_style(Style::default().fg(color));
        }
    }

    fn properties(&self) -> Vec<PropertyDef> {
        vec![
            PropertyDef { name: "size".into(), min: 0.0, max: 4.0, step: 1.0, default: 2.0, label: "Stretch (0=VShort..4=VLong)".into() },
        ]
    }

    fn is_flippable(&self) -> bool { true }
}
