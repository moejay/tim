use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BallType {
    BowlingBall,
    Cannonball,
    Basketball,
    SoccerBall,
    Baseball,
    TennisBall,
    SuperBall,
    Pinball,
    PoolBall,
    ProgrammableBall,
}

impl BallType {
    fn radius(&self) -> f32 {
        match self {
            BallType::BowlingBall => 16.0,
            BallType::Cannonball => 12.0,
            BallType::Basketball => 14.0,
            BallType::SoccerBall => 13.0,
            BallType::Baseball => 6.0,
            BallType::TennisBall => 5.0,
            BallType::SuperBall => 7.0,
            BallType::Pinball => 5.0,
            BallType::PoolBall => 6.0,
            BallType::ProgrammableBall => 8.0,
        }
    }

    fn center_color(&self) -> [u8; 3] {
        match self {
            BallType::BowlingBall => [60, 60, 70],
            BallType::Cannonball => [40, 40, 45],
            BallType::Basketball => [240, 150, 40],
            BallType::SoccerBall => [250, 250, 250],
            BallType::Baseball => [250, 245, 240],
            BallType::TennisBall => [200, 230, 60],
            BallType::SuperBall => [220, 60, 220],
            BallType::Pinball => [210, 210, 220],
            BallType::PoolBall => [200, 40, 40],
            BallType::ProgrammableBall => [0, 190, 190],
        }
    }

    fn edge_color(&self) -> [u8; 3] {
        match self {
            BallType::BowlingBall => [30, 30, 35],
            BallType::Cannonball => [20, 20, 22],
            BallType::Basketball => [180, 90, 20],
            BallType::SoccerBall => [200, 200, 200],
            BallType::Baseball => [220, 210, 200],
            BallType::TennisBall => [140, 180, 30],
            BallType::SuperBall => [140, 20, 140],
            BallType::Pinball => [150, 150, 160],
            BallType::PoolBall => [120, 20, 20],
            BallType::ProgrammableBall => [0, 120, 120],
        }
    }
}

impl PartDef for BallType {
    fn name(&self) -> &'static str {
        match self {
            BallType::BowlingBall => "Bowling Ball",
            BallType::Cannonball => "Cannonball",
            BallType::Basketball => "Basketball",
            BallType::SoccerBall => "Soccer Ball",
            BallType::Baseball => "Baseball",
            BallType::TennisBall => "Tennis Ball",
            BallType::SuperBall => "Super Ball",
            BallType::Pinball => "Pinball",
            BallType::PoolBall => "Pool Ball",
            BallType::ProgrammableBall => "Programmable Ball",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            BallType::BowlingBall => "Heavy ball with 3 finger holes",
            BallType::Cannonball => "Dense iron ball",
            BallType::Basketball => "Orange ball with seam lines",
            BallType::SoccerBall => "White ball with pentagon pattern",
            BallType::Baseball => "White ball with red stitching",
            BallType::TennisBall => "Yellow-green fuzzy ball",
            BallType::SuperBall => "Highly elastic bouncy ball (gains energy)",
            BallType::Pinball => "Small metallic silver ball",
            BallType::PoolBall => "Colored ball with number, zero-gravity float",
            BallType::ProgrammableBall => "Ball with adjustable physics properties",
        }
    }

    fn category(&self) -> &'static str { "Balls" }

    fn default_size(&self) -> (f32, f32) {
        let d = self.radius() * 2.0;
        (d, d)
    }

    fn icon_char(&self) -> char {
        match self {
            BallType::BowlingBall => '\u{2B24}', // ⬤
            _ => '\u{25CF}', // ●
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            BallType::BowlingBall => DARK_GRAY,
            BallType::Cannonball => DARK_GRAY,
            BallType::Basketball => ORANGE,
            BallType::SoccerBall => WHITE,
            BallType::Baseball => WHITE,
            BallType::TennisBall => YELLOW_GREEN,
            BallType::SuperBall => MAGENTA,
            BallType::Pinball => SILVER,
            BallType::PoolBall => RED,
            BallType::ProgrammableBall => TEAL,
        }
    }

    fn physics(&self) -> PhysicsProps {
        match self {
            BallType::BowlingBall => PhysicsProps {
                mass: 7.0, elasticity: 0.1, density: 8.0, friction: 0.4,
                gravity_response: GravityResponse::AlwaysFalls, is_static: false,
            },
            BallType::Cannonball => PhysicsProps {
                mass: 6.0, elasticity: 0.0, density: 10.0, friction: 0.3,
                gravity_response: GravityResponse::AlwaysFalls, is_static: false,
            },
            BallType::Basketball => PhysicsProps {
                mass: 0.6, elasticity: 0.75, density: 0.5, friction: 0.6,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            BallType::SoccerBall => PhysicsProps {
                mass: 0.45, elasticity: 0.6, density: 0.4, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            BallType::Baseball => PhysicsProps {
                mass: 0.15, elasticity: 0.3, density: 1.5, friction: 0.4,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            BallType::TennisBall => PhysicsProps {
                mass: 0.06, elasticity: 0.8, density: 0.3, friction: 0.7,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            BallType::SuperBall => PhysicsProps {
                mass: 0.1, elasticity: 1.1, density: 0.8, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            BallType::Pinball => PhysicsProps {
                mass: 0.08, elasticity: 0.15, density: 8.0, friction: 0.2,
                gravity_response: GravityResponse::AlwaysFalls, is_static: false,
            },
            BallType::PoolBall => PhysicsProps {
                mass: 0.17, elasticity: 0.9, density: 2.0, friction: 0.3,
                gravity_response: GravityResponse::ZeroGravity, is_static: false,
            },
            BallType::ProgrammableBall => PhysicsProps {
                mass: 1.0, elasticity: 0.8, density: 1.0, friction: 0.5,
                gravity_response: GravityResponse::Custom(1.0), is_static: false,
            },
        }
    }

    fn states(&self) -> Vec<StateDef> {
        vec![
            StateDef { name: "Idle", description: "At rest or placed" },
            StateDef { name: "Moving", description: "In motion during simulation" },
            StateDef { name: "AtRest", description: "Velocity < 0.5 px/s for 30 ticks" },
        ]
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let r = self.radius();
        let cx = x + r;
        let cy = y + r;

        // State 2 = AtRest: draw dimmer with "zzz"
        let at_rest = props.current_state == 2;
        // State 1 = Moving: draw motion blur trail
        let moving = props.current_state == 1;

        if moving {
            // Motion blur trail behind the ball
            for i in 1..=4 {
                let trail_x = cx - i as f32 * 4.0;
                let alpha = (120 - i * 25) as u8;
                fill_circle(img, trail_x, cy, r * (1.0 - i as f32 * 0.1), [self.center_color()[0], self.center_color()[1], self.center_color()[2], alpha]);
            }
        }

        // Drop shadow
        fill_circle(img, cx + 2.0, cy + 2.0, r, [0, 0, 0, 80]);

        // Main sphere with gradient
        if at_rest {
            // Dimmer when at rest
            let cc = self.center_color();
            let ec = self.edge_color();
            let dim_c = [cc[0] / 2, cc[1] / 2, cc[2] / 2];
            let dim_e = [ec[0] / 2, ec[1] / 2, ec[2] / 2];
            fill_circle_gradient(img, cx, cy, r, dim_c, dim_e);
        } else {
            fill_circle_gradient(img, cx, cy, r, self.center_color(), self.edge_color());
        }

        // Specular highlight
        fill_circle(img, cx - r * 0.3, cy - r * 0.3, r * 0.25, [255, 255, 255, 120]);

        // "zzz" when at rest
        if at_rest {
            let zc = [180, 180, 220, 200];
            for (i, zx_off) in [4.0, 8.0, 12.0].iter().enumerate() {
                blend_pixel(img, (cx + r + zx_off) as i32, (cy - r + i as f32 * 3.0 - 4.0) as i32, zc);
            }
            let phase = (frame as f32 * 0.05).sin();
            let _ = phase; // subtle idle bob could go here
        }

        // Type-specific details
        match self {
            BallType::BowlingBall => {
                let hole_r = r * 0.12;
                fill_circle(img, cx - r * 0.15, cy - r * 0.25, hole_r, [20, 20, 25, 200]);
                fill_circle(img, cx + r * 0.15, cy - r * 0.25, hole_r, [20, 20, 25, 200]);
                fill_circle(img, cx, cy - r * 0.5, hole_r, [20, 20, 25, 200]);
            }
            BallType::Basketball => {
                let c = [60, 30, 10, 180];
                draw_line(img, (cx - r * 0.9) as i32, cy as i32, (cx + r * 0.9) as i32, cy as i32, c);
                draw_line(img, cx as i32, (cy - r * 0.9) as i32, cx as i32, (cy + r * 0.9) as i32, c);
            }
            BallType::SoccerBall => {
                let c = [40, 40, 40, 160];
                for i in 0..5 {
                    let angle = i as f32 * std::f32::consts::TAU / 5.0 - std::f32::consts::FRAC_PI_2;
                    let px = cx + angle.cos() * r * 0.5;
                    let py = cy + angle.sin() * r * 0.5;
                    fill_circle(img, px, py, r * 0.15, c);
                }
            }
            BallType::Baseball => {
                let c = [200, 30, 30, 200];
                for i in 0..8 {
                    let t = i as f32 / 7.0;
                    let angle = t * std::f32::consts::PI - std::f32::consts::FRAC_PI_2;
                    let px = cx + angle.cos() * r * 0.6;
                    let py = cy + angle.sin() * r * 0.6;
                    blend_pixel(img, px as i32, py as i32, c);
                }
            }
            BallType::TennisBall => {
                draw_line_aa(img, cx - r * 0.7, cy, cx + r * 0.7, cy, [255, 255, 255]);
            }
            BallType::SuperBall => {
                draw_glow(img, cx, cy, r * 1.3, [255, 100, 255]);
            }
            BallType::PoolBall => {
                fill_circle(img, cx, cy, r * 0.45, [255, 255, 255, 220]);
                let num = props.values.get("surface_number").copied().unwrap_or(8.0) as u32;
                let s = format!("{}", num % 10);
                let tw = text_width(&s, 1);
                draw_text(img, cx as i32 - tw / 2, cy as i32 - 3, &s, [0, 0, 0, 255], 1);
            }
            BallType::ProgrammableBall => {
                let c = [0, 100, 100, 180];
                for i in 0..6 {
                    let angle = i as f32 * std::f32::consts::TAU / 6.0;
                    let px = cx + angle.cos() * r * 0.5;
                    let py = cy + angle.sin() * r * 0.5;
                    blend_pixel(img, px as i32, py as i32, c);
                }
            }
            _ => {}
        }
    }

    fn draw_text(&self, buf: &mut Buffer, area: Rect, _props: &PartProps, _frame: u64) {
        if area.width == 0 || area.height == 0 { return; }
        let c = self.icon_color();
        let color = Color::Rgb(c[0], c[1], c[2]);
        let x = area.x + area.width / 2;
        let y = area.y + area.height / 2;
        if x < area.right() && y < area.bottom() {
            buf[(x, y)].set_char(self.icon_char()).set_style(Style::default().fg(color));
        }
    }

    fn properties(&self) -> Vec<PropertyDef> {
        match self {
            BallType::PoolBall => vec![
                PropertyDef {
                    name: "surface_number".into(),
                    min: 0.0, max: 15.0, step: 1.0, default: 8.0,
                    label: "Number".into(),
                },
            ],
            BallType::ProgrammableBall => vec![
                PropertyDef { name: "mass".into(), min: 0.01, max: 10.0, step: 0.1, default: 1.0, label: "Mass".into() },
                PropertyDef { name: "elasticity".into(), min: 0.0, max: 2.0, step: 0.05, default: 0.8, label: "Elasticity".into() },
                PropertyDef { name: "density".into(), min: 0.1, max: 10.0, step: 0.1, default: 1.0, label: "Density".into() },
                PropertyDef { name: "friction".into(), min: 0.0, max: 2.0, step: 0.05, default: 0.5, label: "Friction".into() },
                PropertyDef { name: "gravity_factor".into(), min: 0.0, max: 2.0, step: 0.1, default: 1.0, label: "Gravity Factor".into() },
            ],
            _ => vec![],
        }
    }
}
