use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnimalType {
    PokeyCat,
    MortMouse,
    Cheese,
    EdisonAlligator,
    MelSchlemming,
    MelsHouse,
    BillsFishTank,
    MouseHole,
    Leprechaun,
}

impl PartDef for AnimalType {
    fn name(&self) -> &'static str {
        match self {
            AnimalType::PokeyCat => "Pokey the Cat",
            AnimalType::MortMouse => "Mort the Mouse",
            AnimalType::Cheese => "Cheese",
            AnimalType::EdisonAlligator => "Edison Alligator",
            AnimalType::MelSchlemming => "Mel Schlemming",
            AnimalType::MelsHouse => "Mel's House",
            AnimalType::BillsFishTank => "Bill's Fish Tank",
            AnimalType::MouseHole => "Mouse Hole",
            AnimalType::Leprechaun => "Leprechaun",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            AnimalType::PokeyCat => "Mass 0.8; chases mouse (200px LOS, 80 px/s); attracted to broken fishbowl; NOT eaten by gator",
            AnimalType::MortMouse => "Mass 0.02; flees cat (70 px/s); seeks cheese (150px, 60 px/s); activates wheel (30px); eaten by gator",
            AnimalType::Cheese => "Mass 0.1; dynamic; attracts Mort (150px); NOT consumed",
            AnimalType::EdisonAlligator => "Static; tail bounces ~500 px/s @ 2Hz; jaw snaps ~600 px/s; eats mouse/Mel, NOT cat",
            AnimalType::MelSchlemming => "Mass 0.5; walk 50/run 100 px/s; dies: fall >150px or >200 px/s impact or eaten",
            AnimalType::MelsHouse => "Static; suburban/cabin; Mel enters → lights on, chimney smoke",
            AnimalType::BillsFishTank => "Static; breaks at >100 px/s impact; fish dies 3s after; broken bowl attracts cat",
            AnimalType::MouseHole => "Static; Mort enters → safe",
            AnimalType::Leprechaun => "Green-clad character",
        }
    }

    fn category(&self) -> &'static str { "Animals" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            AnimalType::PokeyCat => (24.0, 20.0),
            AnimalType::MortMouse => (10.0, 8.0),
            AnimalType::Cheese => (12.0, 10.0),
            AnimalType::EdisonAlligator => (64.0, 32.0),
            AnimalType::MelSchlemming => (16.0, 24.0),
            AnimalType::MelsHouse => (48.0, 48.0),
            AnimalType::BillsFishTank => (20.0, 24.0),
            AnimalType::MouseHole => (16.0, 16.0),
            AnimalType::Leprechaun => (24.0, 32.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            AnimalType::PokeyCat => 'C',
            AnimalType::MortMouse => 'm',
            AnimalType::Cheese => '\u{25B2}',
            AnimalType::EdisonAlligator => 'A',
            AnimalType::MelSchlemming => 'M',
            AnimalType::MelsHouse => '\u{2302}',
            AnimalType::BillsFishTank => '\u{25CB}',
            AnimalType::MouseHole => '\u{25E0}',
            AnimalType::Leprechaun => '\u{2663}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            AnimalType::PokeyCat => ORANGE,
            AnimalType::MortMouse => GRAY,
            AnimalType::Cheese => YELLOW,
            AnimalType::EdisonAlligator => GREEN,
            AnimalType::MelSchlemming => BLUE,
            AnimalType::MelsHouse => BROWN,
            AnimalType::BillsFishTank => CYAN,
            AnimalType::MouseHole => DARK_GRAY,
            AnimalType::Leprechaun => GREEN,
        }
    }

    fn physics(&self) -> PhysicsProps {
        match self {
            AnimalType::PokeyCat => PhysicsProps {
                mass: 0.8, elasticity: 0.1, density: 1.0, friction: 0.7,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            AnimalType::MortMouse => PhysicsProps {
                mass: 0.02, elasticity: 0.1, density: 0.5, friction: 0.6,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            AnimalType::Cheese => PhysicsProps {
                mass: 0.1, elasticity: 0.2, density: 0.8, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            AnimalType::MelSchlemming => PhysicsProps {
                mass: 0.5, elasticity: 0.1, density: 1.0, friction: 0.8,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            AnimalType::EdisonAlligator => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.5, density: 100.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            _ => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.2, density: 100.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            AnimalType::PokeyCat => vec![
                StateDef { name: "Idle", description: "Sitting still; occasional meow" },
                StateDef { name: "Walking", description: "Moving at 50-80 px/s toward target" },
                StateDef { name: "Chasing", description: "Pursuing mouse or broken fishbowl (200px LOS)" },
                StateDef { name: "Startled", description: "Shriek + fur standing up (1.0s)" },
            ],
            AnimalType::MortMouse => vec![
                StateDef { name: "Idle", description: "Sitting still" },
                StateDef { name: "Fleeing", description: "Running from cat at 70 px/s" },
                StateDef { name: "SeekingCheese", description: "Running toward cheese (150px, 60 px/s)" },
                StateDef { name: "Eaten", description: "Consumed by alligator" },
                StateDef { name: "Safe", description: "Entered mouse hole" },
            ],
            AnimalType::EdisonAlligator => vec![
                StateDef { name: "Idle", description: "Tail bouncing @ ~2Hz; jaw ready" },
                StateDef { name: "Snapping", description: "Jaw closing on target (0.3s animation)" },
                StateDef { name: "Laughing", description: "Laugh animation after eating (1.5s)" },
            ],
            AnimalType::MelSchlemming => vec![
                StateDef { name: "Walking", description: "Moving at 50 px/s" },
                StateDef { name: "Running", description: "Moving at 100 px/s" },
                StateDef { name: "Stationary", description: "Standing still" },
                StateDef { name: "EnteringHouse", description: "Walking into Mel's House" },
                StateDef { name: "Dead", description: "Fatal fall/impact/eaten" },
            ],
            AnimalType::MelsHouse => vec![
                StateDef { name: "Empty", description: "No occupant" },
                StateDef { name: "Occupied", description: "Mel inside — lights on, chimney smoke" },
            ],
            AnimalType::BillsFishTank => vec![
                StateDef { name: "Intact", description: "Glass bowl with live fish" },
                StateDef { name: "Broken", description: "Shattered — fish dies after 3s; attracts cat" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Default state" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let ix = x as i32;
        let iy = y as i32;

        match self {
            AnimalType::PokeyCat => {
                // State 0=Idle, 1=Walking, 2=Chasing, 3=Startled
                let fur_color = if props.current_state == 3 {
                    [255, 200, 80, 255] // Brighter when startled
                } else {
                    [230, 160, 60, 255]
                };
                fill_rect(img, ix + 4, iy + 8, 16, 10, fur_color);
                fill_circle(img, x + 8.0, y + 6.0, 6.0, fur_color);
                // Ears — puffed up when startled
                let ear_w = if props.current_state == 3 { 3.0 } else { 2.0 };
                fill_triangle(img, (x + 4.0, y - if props.current_state == 3 { 2.0 } else { 0.0 }), (x + 4.0 - ear_w, y + 4.0), (x + 4.0 + ear_w, y + 4.0), fur_color);
                fill_triangle(img, (x + 12.0, y - if props.current_state == 3 { 2.0 } else { 0.0 }), (x + 12.0 - ear_w, y + 4.0), (x + 12.0 + ear_w, y + 4.0), fur_color);
                // Eyes — wide when startled or chasing
                let eye_size = if props.current_state >= 2 { 2 } else { 1 };
                for dx in 0..eye_size { for dy in 0..eye_size {
                    blend_pixel(img, ix + 6 + dx, iy + 5 + dy, [30, 30, 30, 255]);
                    blend_pixel(img, ix + 10 + dx, iy + 5 + dy, [30, 30, 30, 255]);
                }}
                // Tail — speed varies by state
                let tail_speed = match props.current_state { 2 => 0.25, 3 => 0.4, _ => 0.1 };
                let tail_wave = ((frame as f32 * tail_speed).sin() * 3.0) as i32;
                draw_line(img, ix + 20, iy + 10, ix + 22, iy + 6 + tail_wave, [210, 140, 40, 255]);
                // Legs — animate when walking/chasing
                let leg_speed = match props.current_state { 1 => 0.15, 2 => 0.3, _ => 0.0 };
                let leg_anim = ((frame as f32 * leg_speed).sin() * 2.0) as i32;
                draw_line(img, ix + 6, iy + 18, ix + 6 + leg_anim, iy + 20, [210, 140, 40, 255]);
                draw_line(img, ix + 18, iy + 18, ix + 18 - leg_anim, iy + 20, [210, 140, 40, 255]);
                // Startled stars
                if props.current_state == 3 && frame % 6 < 3 {
                    blend_pixel(img, ix - 2, iy - 2, [255, 255, 100, 255]);
                    blend_pixel(img, ix + 16, iy - 3, [255, 255, 100, 255]);
                    blend_pixel(img, ix + 20, iy + 2, [255, 255, 100, 255]);
                }
            }
            AnimalType::MortMouse => {
                // State 0=Idle, 1=Fleeing, 2=SeekingCheese, 3=Eaten, 4=Safe
                match props.current_state {
                    3 => {
                        // Eaten — just a poof
                        if frame % 8 < 4 {
                            blend_pixel(img, ix + 5, iy + 4, [160, 160, 160, 80]);
                        }
                    }
                    4 => {
                        // Safe — tiny in mouse hole, just eyes peeking
                        blend_pixel(img, ix + 4, iy + 6, [200, 200, 200, 255]);
                        blend_pixel(img, ix + 6, iy + 6, [200, 200, 200, 255]);
                    }
                    _ => {
                        let leg_speed = match props.current_state { 1 => 0.35, 2 => 0.2, _ => 0.0 };
                        fill_circle(img, x + 5.0, y + 5.0, 4.0, [160, 160, 160, 255]);
                        fill_circle(img, x + 2.0, y + 2.0, 2.0, [180, 150, 150, 255]);
                        blend_pixel(img, ix + 3, iy + 4, [30, 30, 30, 255]);
                        let tail_wave = ((frame as f32 * (0.2 + leg_speed)).sin() * 2.0) as i32;
                        draw_line(img, ix + 8, iy + 5, ix + 10, iy + 3 + tail_wave, [140, 140, 140, 255]);
                        // Legs animated when fleeing/seeking
                        if leg_speed > 0.0 {
                            let leg = ((frame as f32 * leg_speed).sin() * 2.0) as i32;
                            blend_pixel(img, ix + 3 + leg, iy + 8, [140, 140, 140, 255]);
                            blend_pixel(img, ix + 7 - leg, iy + 8, [140, 140, 140, 255]);
                        }
                    }
                }
            }
            AnimalType::Cheese => {
                fill_triangle(img, (x + 6.0, y), (x, y + 10.0), (x + 12.0, y + 10.0), [240, 210, 60, 255]);
                fill_circle(img, x + 5.0, y + 6.0, 1.5, [200, 170, 40, 255]);
                fill_circle(img, x + 8.0, y + 8.0, 1.0, [200, 170, 40, 255]);
            }
            AnimalType::EdisonAlligator => {
                // State 0=Idle (tail bouncing), 1=Snapping, 2=Laughing
                // Body
                fill_rect(img, ix + 8, iy + 12, 48, 14, [50, 160, 50, 255]);
                // Head — jaw gap depends on state
                let jaw_open = match props.current_state {
                    1 => 6, // snapping — wide open then closing
                    _ => 2,
                };
                fill_rect(img, ix, iy + 10, 12, 8, [60, 180, 60, 255]);
                fill_rect(img, ix, iy + 18 + jaw_open, 12, 4, [50, 150, 50, 255]);
                // Teeth
                for t in (0..12).step_by(3) {
                    blend_pixel(img, ix + t, iy + 17, [255, 255, 255, 255]);
                    blend_pixel(img, ix + t, iy + 18 + jaw_open, [255, 255, 255, 255]);
                }
                // Eye
                fill_circle(img, x + 8.0, y + 10.0, 2.0, [255, 255, 50, 255]);
                blend_pixel(img, ix + 8, iy + 10, [30, 30, 30, 255]);
                // Tail — always bouncing
                let tail_y = ((frame as f32 * 0.12).sin() * 4.0) as i32;
                draw_line(img, ix + 56, iy + 16, ix + 64, iy + 12 + tail_y, [40, 140, 40, 255]);
                // Legs
                for lx in [16, 28, 40, 50] {
                    draw_line(img, ix + lx, iy + 26, ix + lx, iy + 30, [40, 130, 40, 255]);
                }
                // Scales
                for sx in (12..52).step_by(6) {
                    blend_pixel(img, ix + sx, iy + 12, [40, 140, 40, 255]);
                }
                // Laughing — belly shaking, open mouth
                if props.current_state == 2 {
                    let shake = ((frame as f32 * 0.4).sin() * 2.0) as i32;
                    fill_rect(img, ix + 20 + shake, iy + 14, 20, 2, [70, 200, 70, 200]);
                    // Ha ha text bubbles
                    if frame % 10 < 5 {
                        blend_pixel(img, ix - 4, iy + 6, [255, 255, 255, 200]);
                        blend_pixel(img, ix - 2, iy + 4, [255, 255, 255, 200]);
                    }
                }
            }
            AnimalType::MelSchlemming => {
                // State 0=Walking, 1=Running, 2=Stationary, 3=EnteringHouse, 4=Dead
                match props.current_state {
                    4 => {
                        // Dead — fallen figure
                        fill_circle(img, x + 8.0, y + 20.0, 4.0, [180, 150, 120, 200]);
                        fill_rect(img, ix + 2, iy + 16, 12, 4, [50, 80, 160, 200]);
                        // X eyes
                        blend_pixel(img, ix + 6, iy + 19, [60, 30, 30, 255]);
                        blend_pixel(img, ix + 10, iy + 19, [60, 30, 30, 255]);
                    }
                    3 => {
                        // Entering house — partially visible, walking into door
                        let visible = (8 - (frame as i32 % 8)).max(0) as i32;
                        if visible > 0 {
                            fill_circle(img, x + 8.0, y + 5.0, 4.0, [220, 180, 150, (visible * 30) as u8]);
                            fill_rect(img, ix + 4, iy + 9, visible.min(8), 10, [60, 100, 200, (visible * 30) as u8]);
                        }
                    }
                    _ => {
                        let speed = match props.current_state {
                            0 => 1.0_f32, // walking
                            1 => 2.0,     // running
                            _ => 0.0,     // stationary
                        };
                        fill_circle(img, x + 8.0, y + 5.0, 4.0, [220, 180, 150, 255]);
                        fill_rect(img, ix + 4, iy + 9, 8, 10, [60, 100, 200, 255]);
                        let step = if speed > 0.0 { ((frame as f32 * speed * 0.1).sin() * 3.0) as i32 } else { 0 };
                        draw_line(img, ix + 6, iy + 19, ix + 4 + step, iy + 24, [60, 60, 80, 255]);
                        draw_line(img, ix + 10, iy + 19, ix + 12 - step, iy + 24, [60, 60, 80, 255]);
                        draw_line(img, ix + 4, iy + 11, ix + 2, iy + 16, [220, 180, 150, 255]);
                        draw_line(img, ix + 12, iy + 11, ix + 14, iy + 16, [220, 180, 150, 255]);
                    }
                }
            }
            AnimalType::MelsHouse => {
                // State 0=Empty, 1=Occupied (lights, chimney smoke, Mel visible)
                let style = props.values.get("style").copied().unwrap_or(0.0);
                if style < 0.5 {
                    fill_rect(img, ix + 4, iy + 20, 40, 28, [200, 180, 160, 255]);
                    fill_triangle(img, (x + 24.0, y + 8.0), (x + 2.0, y + 20.0), (x + 46.0, y + 20.0), [160, 60, 40, 255]);
                    fill_rect(img, ix + 18, iy + 32, 10, 16, [120, 80, 40, 255]);
                    let window_color = if props.current_state == 1 { [255, 240, 150, 255] } else { [180, 220, 240, 255] };
                    fill_rect(img, ix + 8, iy + 24, 8, 6, window_color);
                    fill_rect(img, ix + 32, iy + 24, 8, 6, window_color);
                } else {
                    fill_rect(img, ix + 4, iy + 16, 40, 32, [120, 70, 30, 255]);
                    for ly in (16..48).step_by(6) {
                        draw_line(img, ix + 4, iy + ly, ix + 44, iy + ly, [100, 55, 20, 255]);
                    }
                    fill_triangle(img, (x + 24.0, y + 4.0), (x + 2.0, y + 16.0), (x + 46.0, y + 16.0), [100, 55, 20, 255]);
                    fill_rect(img, ix + 18, iy + 30, 10, 18, [80, 45, 15, 255]);
                }
                // Occupied — chimney smoke + window glow + Mel silhouette
                if props.current_state == 1 {
                    let chimney_x = ix + 36;
                    let chimney_top = if style < 0.5 { iy + 8 } else { iy + 4 };
                    fill_rect(img, chimney_x, chimney_top, 4, 8, [140, 100, 60, 255]);
                    // Smoke puffs
                    for i in 0..3 {
                        let sy = chimney_top - 4 - i * 5 - ((frame as i32 / 4) % 4);
                        let sx = chimney_x + 2 + ((frame as f32 * 0.1 + i as f32).sin() * 3.0) as i32;
                        fill_circle(img, sx as f32, sy as f32, 2.5 + i as f32 * 0.5, [180, 180, 190, (120 - i * 30) as u8]);
                    }
                    // Mel silhouette in window
                    let win_x = if style < 0.5 { ix + 10 } else { ix + 10 };
                    let win_y = if style < 0.5 { iy + 25 } else { iy + 25 };
                    blend_pixel(img, win_x, win_y, [60, 40, 30, 180]);
                    blend_pixel(img, win_x, win_y + 1, [60, 40, 30, 180]);
                    blend_pixel(img, win_x + 1, win_y, [60, 40, 30, 180]);
                }
            }
            AnimalType::BillsFishTank => {
                // State 0=Intact, 1=Broken
                if props.current_state == 1 {
                    // Broken — shards, water puddle, dead fish
                    // Water puddle
                    fill_circle(img, x + 10.0, y + 20.0, 8.0, [80, 150, 200, 80]);
                    // Glass shards
                    for i in 0..5 {
                        let sx = x + 4.0 + i as f32 * 3.5;
                        let sy = y + 16.0 + (i as f32 * 1.7).sin() * 4.0;
                        draw_line(img, sx as i32, sy as i32, (sx + 2.0) as i32, (sy + 3.0) as i32, [200, 220, 240, 180]);
                    }
                    // Dead fish (upside down)
                    fill_circle(img, x + 10.0, y + 18.0, 2.0, [200, 100, 30, 200]);
                    blend_pixel(img, ix + 13, iy + 17, [200, 100, 30, 200]);
                } else {
                    // Intact bowl
                    fill_circle(img, x + 10.0, y + 12.0, 9.0, [180, 220, 240, 120]);
                    fill_circle(img, x + 10.0, y + 14.0, 7.0, [100, 180, 220, 100]);
                    let fish_x = x + 8.0 + ((frame as f32 * 0.05).sin() * 3.0);
                    fill_circle(img, fish_x, y + 14.0, 2.0, [255, 140, 40, 255]);
                    blend_pixel(img, fish_x as i32 - 3, iy + 13, [255, 140, 40, 255]);
                    blend_pixel(img, fish_x as i32 - 3, iy + 15, [255, 140, 40, 255]);
                }
            }
            AnimalType::MouseHole => {
                for dy in 0..16 {
                    for dx in 0..16 {
                        let dist = ((dx as f32 - 8.0).powi(2) + (dy as f32 - 16.0).powi(2)).sqrt();
                        if dist < 8.0 && dy < 16 {
                            blend_pixel(img, ix + dx, iy + dy, [30, 30, 35, 255]);
                        }
                    }
                }
                for t in 0..18 {
                    let a = t as f32 * std::f32::consts::PI / 17.0;
                    let px = x + 8.0 + a.cos() * 8.0;
                    let py = y + 16.0 - a.sin() * 8.0;
                    blend_pixel(img, px as i32, py as i32, [80, 60, 40, 255]);
                }
            }
            AnimalType::Leprechaun => {
                fill_rect(img, ix + 6, iy + 12, 12, 14, [40, 160, 40, 255]);
                fill_circle(img, x + 12.0, y + 8.0, 5.0, [220, 180, 150, 255]);
                fill_rect(img, ix + 6, iy, 12, 4, [30, 120, 30, 255]);
                fill_rect(img, ix + 4, iy + 4, 16, 2, [30, 120, 30, 255]);
                draw_line(img, ix + 9, iy + 26, ix + 7, iy + 32, [30, 120, 30, 255]);
                draw_line(img, ix + 15, iy + 26, ix + 17, iy + 32, [30, 120, 30, 255]);
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
            AnimalType::MelSchlemming => vec![
                PropertyDef { name: "speed".into(), min: 0.0, max: 2.0, step: 1.0, default: 1.0, label: "Speed (0=stop/1=walk/2=run)".into() },
            ],
            AnimalType::MelsHouse => vec![
                PropertyDef { name: "style".into(), min: 0.0, max: 1.0, step: 1.0, default: 0.0, label: "Style (0=suburban/1=cabin)".into() },
            ],
            AnimalType::EdisonAlligator => vec![],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self,
            AnimalType::PokeyCat | AnimalType::MortMouse | AnimalType::EdisonAlligator |
            AnimalType::MelSchlemming | AnimalType::BillsFishTank
        )
    }

    fn is_flippable(&self) -> bool {
        matches!(self, AnimalType::EdisonAlligator)
    }
}
