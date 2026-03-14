use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WallType {
    BrickWall,
    YellowBrickWall,
    CinderBlockWall,
    GrecoRomanWall,
    WoodenWall,
    LogWall,
    CautionWall,
    SandWall,
    PipeWall,
    CurvedPipeWall,
    GrassFloor,
    ScaffoldBarrier,
    WoodenBarrier,
    LatticeArchway,
    MarbleArchway,
}

impl WallType {
    fn base_color(&self) -> [u8; 3] {
        match self {
            WallType::BrickWall => RED_BROWN,
            WallType::YellowBrickWall => YELLOW,
            WallType::CinderBlockWall => GRAY,
            WallType::GrecoRomanWall => CREAM,
            WallType::WoodenWall => BROWN,
            WallType::LogWall => DARK_BROWN,
            WallType::CautionWall => YELLOW,
            WallType::SandWall => TAN,
            WallType::PipeWall => STEEL,
            WallType::CurvedPipeWall => STEEL,
            WallType::GrassFloor => GREEN,
            WallType::ScaffoldBarrier => GRAY,
            WallType::WoodenBarrier => BROWN,
            WallType::LatticeArchway => WHITE,
            WallType::MarbleArchway => CREAM,
        }
    }

    fn wall_friction(&self) -> f32 {
        match self {
            WallType::CautionWall => 0.9,
            WallType::PipeWall | WallType::CurvedPipeWall => 0.2,
            WallType::WoodenWall | WallType::LogWall => 0.5,
            _ => 0.4,
        }
    }
}

impl PartDef for WallType {
    fn name(&self) -> &'static str {
        match self {
            WallType::BrickWall => "Brick Wall",
            WallType::YellowBrickWall => "Yellow Brick Wall",
            WallType::CinderBlockWall => "Cinder Block Wall",
            WallType::GrecoRomanWall => "Greco-Roman Wall",
            WallType::WoodenWall => "Wooden Wall",
            WallType::LogWall => "Log Wall",
            WallType::CautionWall => "Caution Wall",
            WallType::SandWall => "Sand Wall",
            WallType::PipeWall => "Pipe Wall",
            WallType::CurvedPipeWall => "Curved Pipe Wall",
            WallType::GrassFloor => "Grass Floor",
            WallType::ScaffoldBarrier => "Scaffold Barrier",
            WallType::WoodenBarrier => "Wooden Barrier",
            WallType::LatticeArchway => "Lattice Archway",
            WallType::MarbleArchway => "Marble Archway",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            WallType::BrickWall => "Red-brown bricks with mortar lines",
            WallType::YellowBrickWall => "Yellow bricks pattern",
            WallType::CinderBlockWall => "Gray blocks, heavy texture",
            WallType::GrecoRomanWall => "Marble texture with column lines",
            WallType::WoodenWall => "Wood grain, horizontal lines",
            WallType::LogWall => "Rounded log cross-sections",
            WallType::CautionWall => "Yellow/black diagonal stripes, high friction",
            WallType::SandWall => "Sandy texture, speckled",
            WallType::PipeWall => "Steel grey with rivet lines, indestructible",
            WallType::CurvedPipeWall => "Quarter-circle metal pipe, 4 rotations",
            WallType::GrassFloor => "Green grass top, brown dirt below",
            WallType::ScaffoldBarrier => "Metal scaffold frame",
            WallType::WoodenBarrier => "Small wooden cross-planks",
            WallType::LatticeArchway => "Criss-cross lattice pattern",
            WallType::MarbleArchway => "Marble columns with arch top",
        }
    }

    fn category(&self) -> &'static str { "Walls" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            WallType::CurvedPipeWall => (32.0, 32.0),
            WallType::GrassFloor => (64.0, 16.0),
            WallType::ScaffoldBarrier => (32.0, 48.0),
            WallType::WoodenBarrier => (24.0, 24.0),
            WallType::LatticeArchway | WallType::MarbleArchway => (48.0, 64.0),
            _ => (64.0, 32.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            WallType::CurvedPipeWall => '\u{256E}',
            WallType::GrassFloor => '\u{2594}',
            WallType::ScaffoldBarrier => '\u{256C}',
            WallType::WoodenBarrier => '\u{2573}',
            WallType::LatticeArchway => '\u{256C}',
            WallType::MarbleArchway => '\u{03A0}',
            _ => '\u{2588}',
        }
    }

    fn icon_color(&self) -> [u8; 3] { self.base_color() }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: f32::INFINITY,
            elasticity: 0.3,
            density: 100.0,
            friction: self.wall_friction(),
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        let mut s = vec![StateDef { name: "Intact", description: "Normal solid state" }];
        if self.destructible_by_dynamite() {
            s.push(StateDef { name: "Destroyed", description: "Demolished by dynamite" });
        }
        s
    }

    fn destructible_by_dynamite(&self) -> bool {
        matches!(self,
            WallType::BrickWall | WallType::YellowBrickWall |
            WallType::CinderBlockWall | WallType::WoodenWall
        )
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        // State 1 = Destroyed (for destructible walls)
        if props.current_state == 1 && self.destructible_by_dynamite() {
            let ix = x as i32;
            let iy = y as i32;
            let w = props.width as i32;
            let h = props.height as i32;
            let bc = self.base_color();
            // Rubble: scattered debris chunks
            for i in 0..12 {
                let rx = ix + ((i * 37 + 13) % w);
                let ry = iy + h - ((i * 23 + 7) % (h / 2)) - 4;
                let rw = 3 + (i % 4) as i32;
                let rh = 2 + (i % 3) as i32;
                let dim = [bc[0] / 2, bc[1] / 2, bc[2] / 2, (180 - i * 10) as u8];
                fill_rect(img, rx, ry, rw, rh, dim);
            }
            // Dust cloud
            let dust_a = (80.0 + (frame as f32 * 0.1).sin() * 30.0) as u8;
            fill_circle(img, x + w as f32 / 2.0, y + h as f32 * 0.6, w as f32 * 0.3, [180, 170, 150, dust_a]);
            return;
        }

        let w = props.width as i32;
        let h = props.height as i32;
        let ix = x as i32;
        let iy = y as i32;
        let bc = self.base_color();

        match self {
            WallType::BrickWall | WallType::YellowBrickWall => {
                fill_rect(img, ix, iy, w, h, [bc[0], bc[1], bc[2], 255]);
                let mortar = [bc[0].saturating_sub(40), bc[1].saturating_sub(30), bc[2].saturating_sub(30), 255];
                let brick_h = 8;
                let brick_w = 16;
                for row in 0..(h / brick_h) {
                    let py = iy + row * brick_h;
                    draw_line(img, ix, py, ix + w, py, mortar);
                    let offset = if row % 2 == 0 { 0 } else { brick_w / 2 };
                    for col in 0..=(w / brick_w) {
                        let px = ix + col * brick_w + offset;
                        if px < ix + w {
                            draw_line(img, px, py, px, py + brick_h, mortar);
                        }
                    }
                }
            }
            WallType::CinderBlockWall => {
                fill_rect(img, ix, iy, w, h, [bc[0], bc[1], bc[2], 255]);
                let mortar = [120, 120, 120, 255];
                let block_h = 16;
                let block_w = 32;
                for row in 0..(h / block_h + 1) {
                    let py = iy + row * block_h;
                    draw_line(img, ix, py, ix + w, py, mortar);
                    let offset = if row % 2 == 0 { 0 } else { block_w / 2 };
                    for col in 0..=(w / block_w) {
                        let px = ix + col * block_w + offset;
                        draw_line(img, px, py, px, py + block_h, mortar);
                    }
                }
            }
            WallType::CautionWall => {
                fill_rect(img, ix, iy, w, h, [230, 200, 50, 255]);
                let stripe_w = 12;
                for s in (-h..w).step_by(stripe_w as usize * 2) {
                    for d in 0..stripe_w {
                        let x0 = ix + s + d;
                        draw_line(img, x0, iy, x0 + h, iy + h, [30, 30, 30, 200]);
                    }
                }
            }
            WallType::GrassFloor => {
                fill_rect(img, ix, iy + h / 3, w, h - h / 3, [120, 80, 40, 255]);
                fill_rect(img, ix, iy, w, h / 3, [50, 160, 50, 255]);
                for gx in (0..w).step_by(3) {
                    let gh = (gx * 7 % 5 + 2) as i32;
                    blend_pixel(img, ix + gx, iy - gh.min(1), [30, 180, 30, 200]);
                }
            }
            WallType::WoodenWall => {
                fill_rect_gradient_v(img, ix, iy, w, h, [160, 110, 60], [120, 70, 30]);
                for row in (0..h).step_by(4) {
                    let c = [bc[0].saturating_sub(20), bc[1].saturating_sub(15), bc[2].saturating_sub(10), 80];
                    draw_line(img, ix, iy + row, ix + w, iy + row, c);
                }
            }
            WallType::LogWall => {
                fill_rect(img, ix, iy, w, h, [bc[0], bc[1], bc[2], 255]);
                let log_d = 16;
                for col in (0..w).step_by(log_d as usize) {
                    for row in (0..h).step_by(log_d as usize) {
                        let lcx = ix + col + log_d / 2;
                        let lcy = iy + row + log_d / 2;
                        fill_circle(img, lcx as f32, lcy as f32, 6.0, [80, 50, 25, 180]);
                        fill_circle(img, lcx as f32, lcy as f32, 3.0, [60, 35, 15, 200]);
                    }
                }
            }
            WallType::CurvedPipeWall => {
                let rotation = props.values.get("rotation").copied().unwrap_or(0.0) as i32;
                let r = w.min(h);
                let (corner_x, corner_y) = match rotation % 4 {
                    0 => (ix, iy),
                    1 => (ix + w, iy),
                    2 => (ix + w, iy + h),
                    _ => (ix, iy + h),
                };
                for t in 0..90 {
                    let angle = t as f32 * std::f32::consts::PI / 180.0 + (rotation as f32 * std::f32::consts::FRAC_PI_2);
                    for thickness in 0..4 {
                        let rad = r as f32 - thickness as f32;
                        let px = corner_x as f32 + angle.cos() * rad;
                        let py = corner_y as f32 + angle.sin() * rad;
                        blend_pixel(img, px as i32, py as i32, [bc[0], bc[1], bc[2], 255]);
                    }
                }
            }
            _ => {
                fill_rect(img, ix, iy, w, h, [bc[0], bc[1], bc[2], 255]);
                draw_line(img, ix, iy, ix + w - 1, iy, [bc[0].saturating_add(40), bc[1].saturating_add(40), bc[2].saturating_add(40), 255]);
                draw_line(img, ix, iy + h - 1, ix + w - 1, iy + h - 1, [bc[0].saturating_sub(40), bc[1].saturating_sub(40), bc[2].saturating_sub(40), 255]);
            }
        }
    }

    fn draw_text(&self, buf: &mut Buffer, area: Rect, _props: &PartProps, _frame: u64) {
        if area.width == 0 || area.height == 0 { return; }
        let c = self.icon_color();
        let color = Color::Rgb(c[0], c[1], c[2]);
        let ch = self.icon_char();
        let cx = area.x + area.width / 2;
        let cy = area.y + area.height / 2;
        if cx < area.right() && cy < area.bottom() {
            buf[(cx, cy)].set_char(ch).set_style(Style::default().fg(color));
        }
    }

    fn properties(&self) -> Vec<PropertyDef> {
        match self {
            WallType::CurvedPipeWall => vec![
                PropertyDef { name: "rotation".into(), min: 0.0, max: 3.0, step: 1.0, default: 0.0, label: "Rotation".into() },
            ],
            WallType::GrassFloor | WallType::ScaffoldBarrier | WallType::WoodenBarrier
            | WallType::LatticeArchway | WallType::MarbleArchway => vec![],
            _ => vec![
                PropertyDef { name: "width".into(), min: 16.0, max: 256.0, step: 16.0, default: 64.0, label: "Width".into() },
                PropertyDef { name: "height".into(), min: 16.0, max: 256.0, step: 16.0, default: 32.0, label: "Height".into() },
            ],
        }
    }

    fn is_resizable(&self) -> bool {
        matches!(self,
            WallType::BrickWall | WallType::YellowBrickWall | WallType::CinderBlockWall |
            WallType::GrecoRomanWall | WallType::WoodenWall | WallType::LogWall |
            WallType::CautionWall | WallType::SandWall | WallType::PipeWall
        )
    }
}
