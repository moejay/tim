use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GadgetType {
    SuperPhazer,
    EggTimer,
    EyeHook,
    BoatCleat,
    Gun,
    AntiGravityPad,
    SantaLamp,
    LaundryBasket,
    Bucket,
    LeakyBucket,
    Balloon,
    HotAirBalloon,
}

impl PartDef for GadgetType {
    fn name(&self) -> &'static str {
        match self {
            GadgetType::SuperPhazer => "Captain Z Super Phazer",
            GadgetType::EggTimer => "Egg Timer",
            GadgetType::EyeHook => "Eye Hook",
            GadgetType::BoatCleat => "Boat Cleat",
            GadgetType::Gun => "Gun (Revolver)",
            GadgetType::AntiGravityPad => "Anti-Gravity Pad",
            GadgetType::SantaLamp => "Santa Lamp",
            GadgetType::LaundryBasket => "Laundry Basket",
            GadgetType::Bucket => "Bucket",
            GadgetType::LeakyBucket => "Leaky Bucket",
            GadgetType::Balloon => "Balloon",
            GadgetType::HotAirBalloon => "Hot Air Balloon",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            GadgetType::SuperPhazer => "1-5 programmable blasts at ~1800 px/s; rope attachment",
            GadgetType::EggTimer => "Programmable delay (1-10s); spring-loaded arm ~400 px/s",
            GadgetType::EyeHook => "Fixed rope attachment/anchor point",
            GadgetType::BoatCleat => "Immovable rope anchor",
            GadgetType::Gun => "Rope-triggered; hitscan bullet; single shot; body is ramp",
            GadgetType::AntiGravityPad => "Reverses gravity in ~16px zone above surface",
            GadgetType::SantaLamp => "Decorative; provides ambient light",
            GadgetType::LaundryBasket => "Mass 1.2; dynamic; traps animals when dropped; rope point",
            GadgetType::Bucket => "Mass 0.5; dynamic; catches objects (mass increases); rope point",
            GadgetType::LeakyBucket => "Mass 1.5→0.3; leak rates: slow/med/fast; for pulley puzzles",
            GadgetType::Balloon => "Buoyant; poppable by gear/scissors/tack/dynamite/candle/laser/gun",
            GadgetType::HotAirBalloon => "Rises when heated; carries objects",
        }
    }

    fn category(&self) -> &'static str { "Gadgets" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            GadgetType::SuperPhazer => (32.0, 16.0),
            GadgetType::EggTimer => (16.0, 20.0),
            GadgetType::EyeHook => (8.0, 12.0),
            GadgetType::BoatCleat => (12.0, 8.0),
            GadgetType::Gun => (28.0, 20.0),
            GadgetType::AntiGravityPad => (32.0, 8.0),
            GadgetType::SantaLamp => (20.0, 28.0),
            GadgetType::LaundryBasket => (28.0, 32.0),
            GadgetType::Bucket => (20.0, 20.0),
            GadgetType::LeakyBucket => (20.0, 20.0),
            GadgetType::Balloon => (20.0, 28.0),
            GadgetType::HotAirBalloon => (32.0, 40.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            GadgetType::SuperPhazer | GadgetType::Gun => '\u{25BA}',
            GadgetType::EggTimer => '\u{231B}',
            GadgetType::EyeHook => '\u{2310}',
            GadgetType::BoatCleat => '\u{2229}',
            GadgetType::AntiGravityPad => '\u{2261}',
            GadgetType::SantaLamp => '\u{2666}',
            GadgetType::LaundryBasket => '\u{2554}',
            GadgetType::Bucket | GadgetType::LeakyBucket => 'U',
            GadgetType::Balloon => '\u{25CB}',
            GadgetType::HotAirBalloon => '\u{25EF}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            GadgetType::SuperPhazer => CYAN,
            GadgetType::EggTimer => TAN,
            GadgetType::EyeHook | GadgetType::BoatCleat | GadgetType::Bucket => SILVER,
            GadgetType::Gun => GRAY,
            GadgetType::AntiGravityPad => PURPLE,
            GadgetType::SantaLamp | GadgetType::HotAirBalloon => RED,
            GadgetType::LaundryBasket => BROWN,
            GadgetType::LeakyBucket => GRAY,
            GadgetType::Balloon => BLUE,
        }
    }

    fn physics(&self) -> PhysicsProps {
        match self {
            GadgetType::LaundryBasket => PhysicsProps {
                mass: 1.2, elasticity: 0.1, density: 0.8, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            GadgetType::Bucket => PhysicsProps {
                mass: 0.5, elasticity: 0.1, density: 1.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            GadgetType::LeakyBucket => PhysicsProps {
                mass: 1.5, elasticity: 0.1, density: 1.5, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            GadgetType::Balloon => PhysicsProps {
                mass: 0.01, elasticity: 0.3, density: 0.01, friction: 0.1,
                gravity_response: GravityResponse::Buoyant, is_static: false,
            },
            GadgetType::HotAirBalloon => PhysicsProps {
                mass: 0.5, elasticity: 0.1, density: 0.1, friction: 0.1,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            _ => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.2, density: 100.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            GadgetType::SuperPhazer => vec![
                StateDef { name: "Ready", description: "Loaded with shots" },
                StateDef { name: "Firing", description: "Emitting blast at ~1800 px/s" },
                StateDef { name: "Empty", description: "All shots expended" },
            ],
            GadgetType::EggTimer => vec![
                StateDef { name: "Ready", description: "Timer not started" },
                StateDef { name: "Counting", description: "Countdown in progress" },
                StateDef { name: "Triggered", description: "Spring arm deployed at ~400 px/s" },
            ],
            GadgetType::Gun => vec![
                StateDef { name: "Loaded", description: "Ready to fire" },
                StateDef { name: "Fired", description: "Rope-triggered; hitscan bullet" },
            ],
            GadgetType::Balloon => vec![
                StateDef { name: "Inflated", description: "Rising due to buoyancy" },
                StateDef { name: "Popped", description: "Destroyed by gear/scissors/tack/dynamite/candle/laser/gun" },
            ],
            GadgetType::HotAirBalloon => vec![
                StateDef { name: "Cold", description: "Stationary on ground" },
                StateDef { name: "Heating", description: "Heat source applied — beginning to rise" },
                StateDef { name: "Rising", description: "Airborne — carrying objects" },
            ],
            GadgetType::LeakyBucket => vec![
                StateDef { name: "Full", description: "Mass 1.5 — leaking" },
                StateDef { name: "Draining", description: "Losing mass at configured rate" },
                StateDef { name: "Empty", description: "Mass 0.3 — no more drip" },
            ],
            GadgetType::LaundryBasket => vec![
                StateDef { name: "Open", description: "Ready to trap" },
                StateDef { name: "Trapping", description: "Animal caught inside" },
            ],
            GadgetType::Bucket => vec![
                StateDef { name: "Empty", description: "No contents" },
                StateDef { name: "Filled", description: "Object(s) caught — mass increased" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Default state" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let (w, h) = self.default_size();
        let ix = x as i32;
        let iy = y as i32;
        let ic = self.icon_color();

        match self {
            GadgetType::SuperPhazer => {
                // State 0=Ready, 1=Firing, 2=Empty
                fill_rect_gradient_v(img, ix + 4, iy + 4, 20, 8, [0, 180, 200], [0, 120, 140]);
                let barrel_x = if props.flipped { ix } else { ix + 24 };
                let barrel_color = match props.current_state {
                    2 => [0, 100, 110, 255], // dim when empty
                    _ => [0, 220, 240, 255],
                };
                fill_rect(img, barrel_x, iy + 6, 8, 4, barrel_color);
                fill_rect(img, ix + 8, iy + 12, 8, 4, [0, 140, 160, 255]);
                if props.current_state == 1 {
                    // Firing — beam
                    let dir = if props.flipped { -1 } else { 1 };
                    let beam_start = if props.flipped { ix - 2 } else { ix + 32 };
                    for i in 0..20 {
                        let bx = beam_start + i * dir * 2;
                        let alpha = (240 - i * 10) as u8;
                        blend_pixel(img, bx, iy + 7, [0, 255, 255, alpha]);
                        blend_pixel(img, bx, iy + 8, [0, 255, 255, alpha]);
                    }
                    draw_glow(img, (barrel_x + 4) as f32, y + 8.0, 6.0, [0, 255, 255]);
                }
            }
            GadgetType::EggTimer => {
                // State 0=Ready, 1=Counting, 2=Triggered
                fill_triangle(img, (x + 8.0, y + 10.0), (x + 2.0, y), (x + 14.0, y), [220, 200, 160, 200]);
                fill_triangle(img, (x + 8.0, y + 10.0), (x + 2.0, y + 20.0), (x + 14.0, y + 20.0), [220, 200, 160, 200]);
                draw_line(img, ix + 2, iy, ix + 14, iy, [120, 120, 130, 255]);
                draw_line(img, ix + 2, iy + 20, ix + 14, iy + 20, [120, 120, 130, 255]);
                match props.current_state {
                    1 => {
                        // Sand flowing
                        let sand_level = ((frame as f32 * 0.04) % 1.0 * 8.0) as i32;
                        // Top sand shrinking
                        let top_r = (8 - sand_level).max(0) as f32;
                        fill_triangle(img, (x + 8.0, y + 4.0), (x + 8.0 - top_r, y), (x + 8.0 + top_r, y), [200, 180, 120, 255]);
                        // Bottom sand growing
                        fill_triangle(img, (x + 8.0, y + 10.0), (x + 4.0 + sand_level as f32, y + 20.0 - sand_level as f32), (x + 12.0 - sand_level as f32, y + 20.0 - sand_level as f32), [200, 180, 120, 255]);
                        // Falling grain
                        blend_pixel(img, ix + 8, iy + 10 + (frame as i32 % 4), [200, 180, 120, 255]);
                    }
                    2 => {
                        // Triggered — spring arm deployed
                        draw_line(img, ix + 14, iy + 10, ix + 22, iy + 6, [120, 120, 130, 255]);
                        fill_circle(img, x + 22.0, y + 6.0, 2.0, [200, 200, 210, 255]);
                        // All sand at bottom
                        fill_triangle(img, (x + 8.0, y + 14.0), (x + 4.0, y + 20.0), (x + 12.0, y + 20.0), [200, 180, 120, 255]);
                    }
                    _ => {
                        // Ready — all sand at top
                        fill_triangle(img, (x + 8.0, y + 6.0), (x + 4.0, y), (x + 12.0, y), [200, 180, 120, 255]);
                    }
                }
            }
            GadgetType::Gun => {
                // State 0=Loaded, 1=Fired
                fill_rect(img, ix + 4, iy + 4, 20, 8, [120, 120, 130, 255]);
                let barrel_x = if props.flipped { ix } else { ix + 24 };
                fill_rect(img, barrel_x, iy + 5, 4, 5, [100, 100, 110, 255]);
                fill_rect(img, ix + 8, iy + 12, 6, 8, [100, 70, 40, 255]);
                fill_circle(img, x + 14.0, y + 8.0, 4.0, [110, 110, 120, 255]);
                draw_line(img, ix + 13, iy + 12, ix + 12, iy + 15, [80, 80, 90, 255]);
                if props.current_state == 1 {
                    // Muzzle flash + smoke
                    let flash_x = if props.flipped { ix - 4 } else { ix + 28 };
                    draw_glow(img, flash_x as f32, y + 7.0, 6.0, [255, 255, 150]);
                    for i in 0..3 {
                        let sx = flash_x + (if props.flipped { -1 } else { 1 }) * (4 + i * 3);
                        fill_circle(img, sx as f32, y + 6.0 - i as f32, 2.0, [180, 180, 180, (120 - i * 35) as u8]);
                    }
                }
            }
            GadgetType::AntiGravityPad => {
                // Always active — pulsing glow
                fill_rect(img, ix, iy, w as i32, h as i32, [100, 50, 150, 255]);
                let glow_intensity = ((frame as f32 * 0.1).sin() * 0.3 + 0.7) as f32;
                let alpha = (glow_intensity * 140.0) as u8;
                // Anti-gravity field lines rising
                let field_phase = (frame as i32 * 2) % 16;
                for gx in (0..w as i32).step_by(4) {
                    for gy in (field_phase..16).step_by(8) {
                        blend_pixel(img, ix + gx, iy - 1 - gy, [180, 100, 240, alpha / (1 + gy as u8 / 3)]);
                    }
                }
                for gx in 0..w as i32 {
                    blend_pixel(img, ix + gx, iy - 1, [160, 80, 220, alpha]);
                }
            }
            GadgetType::Bucket | GadgetType::LeakyBucket => {
                for row in 0..h as i32 {
                    let t = row as f32 / h;
                    let width = (w * 0.6 + w * 0.4 * t) as i32;
                    let offset = ((w as i32 - width) / 2) as i32;
                    fill_rect(img, ix + offset, iy + row, width, 1, [ic[0], ic[1], ic[2], 255]);
                }
                for t in 0..10 {
                    let a = t as f32 * std::f32::consts::PI / 9.0;
                    let hx = x + w / 2.0 + a.cos() * w * 0.35;
                    let hy = y - 2.0 - a.sin() * 6.0;
                    blend_pixel(img, hx as i32, hy as i32, [120, 120, 130, 255]);
                }
                if matches!(self, GadgetType::LeakyBucket) {
                    let drip_y = ((frame as f32 * 0.15) % 8.0) as i32;
                    blend_pixel(img, ix + w as i32 / 2, iy + h as i32 + drip_y, [100, 150, 220, 200]);
                    blend_pixel(img, ix + w as i32 / 2, iy + h as i32 + drip_y + 1, [100, 150, 220, 140]);
                }
            }
            GadgetType::Balloon => {
                // State 0=Inflated, 1=Popped
                let design = props.values.get("design").copied().unwrap_or(1.0) as u8;
                let balloon_color = match design {
                    1 => [220, 50, 50], 2 => [50, 50, 220], 3 => [50, 200, 50], _ => [220, 200, 50],
                };
                if props.current_state == 1 {
                    // Popped — fragments
                    for i in 0..6 {
                        let angle = i as f32 * std::f32::consts::TAU / 6.0 + frame as f32 * 0.05;
                        let dist = 4.0 + (frame as f32 * 0.3).min(12.0);
                        let fx = x + 10.0 + angle.cos() * dist;
                        let fy = y + 10.0 + angle.sin() * dist;
                        blend_pixel(img, fx as i32, fy as i32, [balloon_color[0], balloon_color[1], balloon_color[2], (200u8).saturating_sub((frame as u8).saturating_mul(3))]);
                    }
                    // Dangling string
                    for sy in 0..8 {
                        blend_pixel(img, ix + 10, iy + 12 + sy, [180, 180, 180, 140]);
                    }
                } else {
                    // Inflated — bobbing gently
                    let bob = ((frame as f32 * 0.06).sin() * 2.0) as f32;
                    fill_circle_gradient(img, x + 10.0, y + 10.0 + bob, 9.0, balloon_color, [balloon_color[0].saturating_sub(40), balloon_color[1].saturating_sub(40), balloon_color[2].saturating_sub(40)]);
                    // Specular
                    fill_circle(img, x + 7.0, y + 7.0 + bob, 2.5, [255, 255, 255, 80]);
                    blend_pixel(img, ix + 10, iy + 19 + bob as i32, [balloon_color[0].saturating_sub(60), balloon_color[1].saturating_sub(60), balloon_color[2].saturating_sub(60), 255]);
                    for sy in 0..8 {
                        let wobble = ((sy as f32 * 0.5).sin() * 1.0) as i32;
                        blend_pixel(img, ix + 10 + wobble, iy + 20 + bob as i32 + sy, [180, 180, 180, 200]);
                    }
                }
            }
            GadgetType::HotAirBalloon => {
                // State 0=Cold, 1=Heating, 2=Rising
                let rise_offset = if props.current_state == 2 { -((frame as f32 * 0.5).min(20.0)) } else { 0.0 };
                let by = y + rise_offset;
                let biy = by as i32;
                fill_circle_gradient(img, x + 16.0, by + 14.0, 14.0, [220, 60, 40], [180, 40, 30]);
                for stripe in [-6, 0, 6] {
                    draw_line(img, ix + 16 + stripe, biy + 2, ix + 16 + stripe, biy + 28, [240, 220, 60, 150]);
                }
                draw_line(img, ix + 8, biy + 26, ix + 10, biy + 32, [120, 80, 40, 255]);
                draw_line(img, ix + 24, biy + 26, ix + 22, biy + 32, [120, 80, 40, 255]);
                fill_rect(img, ix + 10, biy + 32, 12, 8, [140, 100, 50, 255]);
                if props.current_state >= 1 {
                    // Flame/heat source in basket
                    draw_flame(img, x + 16.0, by + 30.0, 5.0, frame);
                }
            }
            GadgetType::LaundryBasket => {
                // State 0=Open, 1=Trapping
                draw_line(img, ix, iy, ix, iy + h as i32, [ic[0], ic[1], ic[2], 255]);
                draw_line(img, ix + w as i32 - 1, iy, ix + w as i32 - 1, iy + h as i32, [ic[0], ic[1], ic[2], 255]);
                for by in (0..h as i32).step_by(6) {
                    draw_line(img, ix, iy + by, ix + w as i32, iy + by, [ic[0], ic[1], ic[2], 180]);
                }
                if props.current_state == 1 {
                    // Trapped animal — movement inside
                    let shake = ((frame as f32 * 0.3).sin() * 2.0) as i32;
                    fill_circle(img, x + w / 2.0 + shake as f32, y + h * 0.6, 4.0, [200, 160, 80, 120]);
                    // Rattling effect
                    if frame % 4 < 2 {
                        draw_line(img, ix - 1, iy + shake, ix - 1, iy + 4 + shake, [ic[0], ic[1], ic[2], 100]);
                        draw_line(img, ix + w as i32, iy - shake, ix + w as i32, iy + 4 - shake, [ic[0], ic[1], ic[2], 100]);
                    }
                }
            }
            GadgetType::SantaLamp => {
                fill_triangle(img, (x + 10.0, y), (x + 2.0, y + 14.0), (x + 18.0, y + 14.0), [200, 50, 40, 255]);
                draw_line(img, ix + 10, iy + 14, ix + 10, iy + 24, [80, 80, 90, 255]);
                fill_rect(img, ix + 6, iy + 24, 8, 4, [80, 80, 90, 255]);
                draw_glow(img, x + 10.0, y + 13.0, 4.0, [255, 255, 200]);
            }
            _ => {
                fill_rect(img, ix, iy, w as i32, h as i32, [ic[0], ic[1], ic[2], 200]);
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
            GadgetType::SuperPhazer => vec![
                PropertyDef { name: "shots".into(), min: 1.0, max: 5.0, step: 1.0, default: 3.0, label: "Shots".into() },
            ],
            GadgetType::EggTimer => vec![
                PropertyDef { name: "delay".into(), min: 1.0, max: 10.0, step: 0.5, default: 3.0, label: "Delay (s)".into() },
            ],
            GadgetType::LeakyBucket => vec![
                PropertyDef { name: "leak_rate".into(), min: 0.0, max: 2.0, step: 1.0, default: 1.0, label: "Leak (0=slow/1=med/2=fast)".into() },
            ],
            GadgetType::Balloon => vec![
                PropertyDef { name: "design".into(), min: 1.0, max: 4.0, step: 1.0, default: 1.0, label: "Design (1-4)".into() },
            ],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self, GadgetType::EggTimer | GadgetType::AntiGravityPad | GadgetType::LeakyBucket)
    }

    fn is_flippable(&self) -> bool {
        matches!(self, GadgetType::SuperPhazer | GadgetType::Gun)
    }

    fn has_rope_point(&self) -> bool {
        matches!(self, GadgetType::EyeHook | GadgetType::BoatCleat | GadgetType::LaundryBasket | GadgetType::Bucket | GadgetType::LeakyBucket | GadgetType::SuperPhazer | GadgetType::Gun)
    }

    fn can_be_ramp(&self) -> bool {
        matches!(self, GadgetType::Gun)
    }
}
