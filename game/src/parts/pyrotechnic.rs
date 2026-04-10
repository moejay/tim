use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PyrotechnicType {
    MagnifyingGlass,
    Flashlight,
    LavaLamp,
    Candle,
    Dynamite,
    DynamitePlunger,
    Cannon,
    Rocket,
    Fireworks,
    RemoteControlBomb,
    MatchOnSpring,
    Fuse,
}

impl PartDef for PyrotechnicType {
    fn name(&self) -> &'static str {
        match self {
            PyrotechnicType::MagnifyingGlass => "Magnifying Glass",
            PyrotechnicType::Flashlight => "Flashlight",
            PyrotechnicType::LavaLamp => "Lava Lamp",
            PyrotechnicType::Candle => "Candle",
            PyrotechnicType::Dynamite => "Dynamite",
            PyrotechnicType::DynamitePlunger => "Dynamite Plunger",
            PyrotechnicType::Cannon => "Cannon",
            PyrotechnicType::Rocket => "Rocket",
            PyrotechnicType::Fireworks => "Fireworks",
            PyrotechnicType::RemoteControlBomb => "Remote Control Bomb",
            PyrotechnicType::MatchOnSpring => "Match on Spring",
            PyrotechnicType::Fuse => "Fuse",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            PyrotechnicType::MagnifyingGlass => "Untouchable; focuses light into ~100px beam",
            PyrotechnicType::Flashlight => "Object-activated; ~200px beam range",
            PyrotechnicType::LavaLamp => "Drawstring-activated; omnidirectional ~100px light",
            PyrotechnicType::Candle => "Dynamic; mass 0.05; lit/extinguished; ignites fuses",
            PyrotechnicType::Dynamite => "Dynamic; 1.5s fuse; destroys brick/wood within ~60px",
            PyrotechnicType::DynamitePlunger => "Static; instant detonation on press; no chain-react",
            PyrotechnicType::Cannon => "Static; ~1.0s fuse; fires cannonball at ~1500 px/s",
            PyrotechnicType::Rocket => "Dynamic; ~0.8s fuse; launches at ~2000 px/s; trail ignites",
            PyrotechnicType::Fireworks => "Dynamic; ~1.5s; cosmetic colored explosion at apogee",
            PyrotechnicType::RemoteControlBomb => "Dynamic; trigger signal; same as dynamite",
            PyrotechnicType::MatchOnSpring => "Match head on coiled spring",
            PyrotechnicType::Fuse => "16-200px braided wire; burns at ~60px/s from ignition point",
        }
    }

    fn category(&self) -> &'static str { "Pyrotechnic" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            PyrotechnicType::MagnifyingGlass => (16.0, 24.0),
            PyrotechnicType::Flashlight => (32.0, 12.0),
            PyrotechnicType::LavaLamp => (16.0, 32.0),
            PyrotechnicType::Candle => (8.0, 16.0),
            PyrotechnicType::Dynamite => (12.0, 20.0),
            PyrotechnicType::DynamitePlunger => (20.0, 28.0),
            PyrotechnicType::Cannon => (48.0, 24.0),
            PyrotechnicType::Rocket => (12.0, 28.0),
            PyrotechnicType::Fireworks => (10.0, 24.0),
            PyrotechnicType::RemoteControlBomb => (16.0, 16.0),
            PyrotechnicType::MatchOnSpring => (12.0, 20.0),
            PyrotechnicType::Fuse => (64.0, 4.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            PyrotechnicType::MagnifyingGlass => '\u{25CE}',
            PyrotechnicType::Flashlight => '\u{25BA}',
            PyrotechnicType::LavaLamp | PyrotechnicType::Candle => '\u{2565}',
            PyrotechnicType::Dynamite => '\u{256B}',
            PyrotechnicType::DynamitePlunger => '\u{2564}',
            PyrotechnicType::Cannon => '\u{2550}',
            PyrotechnicType::Rocket | PyrotechnicType::Fireworks => '\u{25B2}',
            PyrotechnicType::RemoteControlBomb => '\u{2731}',
            PyrotechnicType::MatchOnSpring => '\u{2191}',
            PyrotechnicType::Fuse => '\u{2500}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            PyrotechnicType::MagnifyingGlass => WHITE,
            PyrotechnicType::Flashlight | PyrotechnicType::Candle => YELLOW,
            PyrotechnicType::LavaLamp => PURPLE,
            PyrotechnicType::Dynamite | PyrotechnicType::DynamitePlunger |
            PyrotechnicType::Rocket | PyrotechnicType::RemoteControlBomb |
            PyrotechnicType::MatchOnSpring => RED,
            PyrotechnicType::Cannon => GRAY,
            PyrotechnicType::Fireworks => MAGENTA,
            PyrotechnicType::Fuse => BROWN,
        }
    }

    fn physics(&self) -> PhysicsProps {
        match self {
            PyrotechnicType::Candle => PhysicsProps {
                mass: 0.05, elasticity: 0.1, density: 0.8, friction: 0.4,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            PyrotechnicType::Dynamite => PhysicsProps {
                mass: 0.3, elasticity: 0.1, density: 1.5, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            PyrotechnicType::Rocket | PyrotechnicType::Fireworks => PhysicsProps {
                mass: 0.2, elasticity: 0.1, density: 1.2, friction: 0.3,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            PyrotechnicType::RemoteControlBomb => PhysicsProps {
                mass: 0.4, elasticity: 0.1, density: 1.5, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: false,
            },
            PyrotechnicType::Cannon | PyrotechnicType::DynamitePlunger => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.1, density: 100.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            _ => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.2, density: 100.0, friction: 0.4,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            PyrotechnicType::Candle => vec![
                StateDef { name: "Unlit", description: "No flame" },
                StateDef { name: "Lit", description: "Burning — provides light; ignites fuses on contact" },
                StateDef { name: "Extinguished", description: "Was lit, now out" },
            ],
            PyrotechnicType::Fuse => vec![
                StateDef { name: "Unburnt", description: "Static braided wire" },
                StateDef { name: "Burning", description: "Spark traveling at ~60px/s from ignition point" },
                StateDef { name: "Spent", description: "Fully burned — dark/charred appearance" },
            ],
            PyrotechnicType::Dynamite | PyrotechnicType::RemoteControlBomb => vec![
                StateDef { name: "Idle", description: "Inert — waiting for ignition" },
                StateDef { name: "FuseLit", description: "Fuse burning — 1.5s to detonation" },
                StateDef { name: "Exploded", description: "Destroyed walls within ~60px radius" },
            ],
            PyrotechnicType::Cannon => vec![
                StateDef { name: "Ready", description: "Loaded — waiting for fuse" },
                StateDef { name: "FuseLit", description: "Fuse burning — ~1.0s to fire" },
                StateDef { name: "Fired", description: "Cannonball launched at ~1500 px/s + recoil" },
            ],
            PyrotechnicType::Rocket => vec![
                StateDef { name: "Idle", description: "Grounded — waiting for fuse" },
                StateDef { name: "FuseLit", description: "Fuse burning — ~0.8s" },
                StateDef { name: "Launched", description: "Flying at ~2000 px/s; trail ignites objects" },
            ],
            PyrotechnicType::Fireworks => vec![
                StateDef { name: "Idle", description: "Waiting for ignition" },
                StateDef { name: "FuseLit", description: "~1.5s fuse" },
                StateDef { name: "Launched", description: "Rising" },
                StateDef { name: "Exploded", description: "Cosmetic colored burst at apogee" },
            ],
            PyrotechnicType::Flashlight => vec![
                StateDef { name: "Off", description: "Switch not struck" },
                StateDef { name: "On", description: "Object struck switch — ~200px beam active" },
            ],
            PyrotechnicType::LavaLamp => vec![
                StateDef { name: "Off", description: "Drawstring not pulled" },
                StateDef { name: "On", description: "Drawstring pulled — omnidirectional ~100px light" },
            ],
            PyrotechnicType::DynamitePlunger => vec![
                StateDef { name: "Ready", description: "Handle up" },
                StateDef { name: "Pressed", description: "Plunger pressed — immediate detonation" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Default state" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let (w, _h) = (props.width, props.height);
        let ix = x as i32;
        let iy = y as i32;

        match self {
            PyrotechnicType::Candle => {
                // Wax body
                fill_rect(img, ix + 2, iy + 6, 4, 10, [240, 230, 200, 255]);
                // Wick
                draw_line(img, ix + 4, iy + 6, ix + 4, iy + 4, [60, 50, 40, 255]);
                // State 0=Unlit: no flame, 1=Lit: flame+glow, 2=Extinguished: charred wick
                match props.current_state {
                    1 => {
                        draw_flame(img, x + 4.0, y + 4.0, 6.0, frame);
                        draw_glow(img, x + 4.0, y + 2.0, 10.0, [255, 200, 80]);
                    }
                    2 => {
                        // Charred wick, smoke wisp
                        draw_line(img, ix + 4, iy + 4, ix + 4, iy + 2, [40, 35, 30, 255]);
                        let smoke_y = iy - 2 - ((frame as f32 * 0.1) as i32 % 6);
                        blend_pixel(img, ix + 4, smoke_y, [140, 140, 150, 100]);
                        blend_pixel(img, ix + 5, smoke_y - 1, [140, 140, 150, 60]);
                    }
                    _ => {} // Unlit: just the candle body
                }
            }
            PyrotechnicType::Cannon => {
                // State 0=Ready, 1=FuseLit, 2=Fired
                let bx = if props.flipped { ix } else { ix + 12 };
                fill_rect_gradient_v(img, bx, iy + 4, 32, 12, [120, 120, 130], [80, 80, 90]);
                let opening_x = if props.flipped { ix } else { ix + 44 };
                fill_rect(img, opening_x, iy + 5, 4, 10, [40, 40, 45, 255]);
                fill_circle(img, x + 14.0, y + 20.0, 5.0, [100, 70, 40, 255]);
                fill_circle(img, x + 34.0, y + 20.0, 5.0, [100, 70, 40, 255]);

                match props.current_state {
                    1 => {
                        // Fuse burning — animated spark
                        let fuse_x = if props.flipped { x + w - 8.0 } else { x + 8.0 };
                        draw_flame(img, fuse_x, y + 2.0, 5.0, frame);
                        draw_glow(img, fuse_x, y + 4.0, 6.0, [255, 200, 50]);
                    }
                    2 => {
                        // Fired — smoke cloud at barrel, recoil offset
                        let smoke_x = if props.flipped { x - 4.0 } else { x + w + 2.0 };
                        for i in 0..5 {
                            let sx = smoke_x + ((frame as f32 * 0.3 + i as f32).sin() * 6.0);
                            let sy = y + 8.0 + ((frame as f32 * 0.2 + i as f32 * 1.5).cos() * 4.0);
                            fill_circle(img, sx, sy, 4.0 - i as f32 * 0.5, [180, 180, 180, (160 - i * 30) as u8]);
                        }
                    }
                    _ => {
                        // Ready — subtle fuse glow
                        if frame % 6 < 3 {
                            let fuse_x = if props.flipped { x + w - 8.0 } else { x + 8.0 };
                            draw_glow(img, fuse_x, y + 4.0, 3.0, [200, 160, 40]);
                        }
                    }
                }
            }
            PyrotechnicType::Rocket => {
                // State 0=Idle, 1=FuseLit, 2=Launched
                match props.current_state {
                    2 => {
                        // Launched — rocket ascending with trail
                        let fly_y = y - (frame as f32 * 3.0).min(40.0);
                        fill_rect(img, ix + 2, fly_y as i32 + 8, 8, 16, [200, 40, 40, 255]);
                        fill_triangle(img, (x + 6.0, fly_y), (x + 1.0, fly_y + 8.0), (x + 11.0, fly_y + 8.0), [200, 40, 40, 255]);
                        // Exhaust trail
                        for i in 0..8 {
                            let ty = fly_y + 28.0 + i as f32 * 4.0;
                            let alpha = (220 - i * 25) as u8;
                            let spread = 1 + i / 2;
                            fill_circle(img, x + 6.0 + ((frame as f32 * 0.3 + i as f32).sin() * 2.0), ty, spread as f32, [255, 180, 50, alpha]);
                        }
                    }
                    _ => {
                        // Grounded rocket
                        fill_rect(img, ix + 2, iy + 8, 8, 16, [200, 40, 40, 255]);
                        fill_triangle(img, (x + 6.0, y), (x + 1.0, y + 8.0), (x + 11.0, y + 8.0), [200, 40, 40, 255]);
                        fill_triangle(img, (x, y + 24.0), (x + 2.0, y + 18.0), (x + 2.0, y + 24.0), [180, 30, 30, 255]);
                        fill_triangle(img, (x + 12.0, y + 24.0), (x + 10.0, y + 18.0), (x + 10.0, y + 24.0), [180, 30, 30, 255]);
                        fill_rect(img, ix + 3, iy + 24, 6, 4, [80, 80, 90, 255]);
                        if props.current_state == 1 {
                            draw_flame(img, x + 6.0, y + 26.0, 4.0, frame);
                        }
                    }
                }
            }
            PyrotechnicType::Dynamite => {
                // State 0=Idle, 1=FuseLit, 2=Exploded
                match props.current_state {
                    2 => {
                        // Exploded — debris and flash
                        let phase = (frame as f32 * 0.2) as i32;
                        for i in 0..8 {
                            let angle = i as f32 * std::f32::consts::TAU / 8.0 + frame as f32 * 0.1;
                            let dist = 6.0 + (phase as f32 * 0.5).min(20.0);
                            let dx = x + 6.0 + angle.cos() * dist;
                            let dy = y + 12.0 + angle.sin() * dist;
                            fill_circle(img, dx, dy, 2.0, [200, 100, 30, (200u8).saturating_sub((phase * 5) as u8)]);
                        }
                        draw_glow(img, x + 6.0, y + 12.0, 14.0, [255, 200, 50]);
                    }
                    _ => {
                        // Intact stick
                        fill_rect(img, ix + 2, iy + 4, 8, 16, [210, 40, 30, 255]);
                        fill_rect(img, ix + 2, iy + 10, 8, 3, [240, 220, 180, 255]);
                        draw_line(img, ix + 6, iy + 4, ix + 8, iy, [120, 80, 40, 255]);
                        if props.current_state == 1 {
                            // Fuse burning
                            draw_flame(img, x + 8.0, y, 4.0, frame);
                        }
                    }
                }
            }
            PyrotechnicType::DynamitePlunger => {
                // State 0=Ready, 1=Pressed
                fill_rect(img, ix + 2, iy + 14, 16, 14, [120, 80, 40, 255]);
                let handle_y = if props.current_state == 1 { iy + 8 } else { iy };
                draw_line(img, ix + 10, handle_y, ix + 10, iy + 14, [80, 80, 90, 255]);
                fill_rect(img, ix + 4, handle_y, 12, 3, [80, 80, 90, 255]);
                if props.current_state == 1 {
                    // Spark at contact
                    draw_glow(img, x + 10.0, y + 14.0, 5.0, [255, 255, 100]);
                }
            }
            PyrotechnicType::Flashlight => {
                // State 0=Off, 1=On
                let bx = if props.flipped { ix + 8 } else { ix };
                fill_rect_gradient_v(img, bx, iy + 2, 20, 8, [200, 200, 210], [160, 160, 170]);
                let lens_x = if props.flipped { ix } else { ix + 20 };
                let lens_color = if props.current_state == 1 { [255, 255, 220, 255] } else { [200, 200, 180, 255] };
                fill_rect(img, lens_x, iy, 8, 12, lens_color);
                if props.current_state == 1 {
                    let flip_dir = if props.flipped { -1 } else { 1 };
                    let beam_x = if props.flipped { ix - 16 } else { ix + 28 };
                    let beam_w = 16;
                    for bw in 0..beam_w {
                        let alpha = (180.0 * (1.0 - bw as f32 / beam_w as f32)) as u8;
                        let spread = bw / 3;
                        for s in -(spread as i32)..=(spread as i32) {
                            blend_pixel(img, beam_x + bw * flip_dir, iy + 6 + s, [255, 255, 200, alpha]);
                        }
                    }
                    draw_glow(img, (lens_x + 4) as f32, y + 6.0, 6.0, [255, 255, 180]);
                }
            }
            PyrotechnicType::LavaLamp => {
                // State 0=Off, 1=On
                fill_rect(img, ix + 2, iy + 24, 12, 8, [60, 60, 70, 255]);
                fill_rect(img, ix + 4, iy + 2, 8, 3, [100, 100, 110, 255]);
                if props.current_state == 1 {
                    // On — glowing with moving blobs
                    fill_rect(img, ix + 3, iy + 4, 10, 20, [100, 50, 150, 200]);
                    let blob_y = iy + 12 + ((frame as f32 * 0.05).sin() * 6.0) as i32;
                    fill_circle(img, x + 8.0, blob_y as f32, 3.0, [220, 110, 220, 220]);
                    let blob_y2 = iy + 18 + ((frame as f32 * 0.03 + 2.0).sin() * 4.0) as i32;
                    fill_circle(img, x + 8.0, blob_y2 as f32, 2.0, [240, 130, 240, 200]);
                    draw_glow(img, x + 8.0, y + 14.0, 8.0, [180, 80, 180]);
                } else {
                    // Off — dark glass, no blobs
                    fill_rect(img, ix + 3, iy + 4, 10, 20, [50, 25, 70, 150]);
                }
            }
            PyrotechnicType::Fireworks => {
                // State 0=Idle, 1=FuseLit, 2=Launched, 3=Exploded
                let c1r = props.values.get("color1").copied().unwrap_or(255.0) as u8;
                let c2g = props.values.get("color2").copied().unwrap_or(100.0) as u8;
                let c3b = props.values.get("color3").copied().unwrap_or(200.0) as u8;
                match props.current_state {
                    3 => {
                        // Exploded — colorful burst
                        let burst_r = (frame as f32 * 0.8).min(18.0);
                        for i in 0..12 {
                            let a = i as f32 * std::f32::consts::TAU / 12.0 + frame as f32 * 0.05;
                            let bx = x + 5.0 + a.cos() * burst_r;
                            let by = y + 4.0 + a.sin() * burst_r;
                            let alpha = (255.0 - burst_r * 10.0).max(0.0) as u8;
                            fill_circle(img, bx, by, 2.0, [c1r, c2g, c3b, alpha]);
                        }
                        draw_glow(img, x + 5.0, y + 4.0, burst_r * 0.8, [c1r, c2g, c3b]);
                    }
                    2 => {
                        // Launched — rising with trail
                        let fly_y = y - (frame as f32 * 2.0).min(30.0);
                        fill_rect(img, ix + 3, fly_y as i32, 4, 8, [180, 180, 180, 255]);
                        fill_triangle(img, (x + 5.0, fly_y - 4.0), (x + 2.0, fly_y), (x + 8.0, fly_y), [c1r, c2g, c3b, 255]);
                        // Trail
                        for i in 0..5 {
                            let ty = fly_y + 10.0 + i as f32 * 4.0;
                            fill_circle(img, x + 5.0, ty, 1.5, [255, 200, 80, (180 - i * 35) as u8]);
                        }
                    }
                    _ => {
                        // Grounded
                        fill_rect(img, ix + 3, iy + 8, 4, 16, [180, 180, 180, 255]);
                        fill_triangle(img, (x + 5.0, y), (x + 1.0, y + 8.0), (x + 9.0, y + 8.0), [c1r, c2g, c3b, 255]);
                        if props.current_state == 1 {
                            draw_flame(img, x + 5.0, y + 22.0, 3.0, frame);
                        }
                    }
                }
            }
            PyrotechnicType::RemoteControlBomb => {
                // State 0=Idle, 1=FuseLit, 2=Exploded
                match props.current_state {
                    2 => {
                        // Same explosion as dynamite
                        for i in 0..8 {
                            let angle = i as f32 * std::f32::consts::TAU / 8.0 + frame as f32 * 0.1;
                            let dist = 6.0 + (frame as f32 * 0.5).min(20.0);
                            let dx = x + 8.0 + angle.cos() * dist;
                            let dy = y + 10.0 + angle.sin() * dist;
                            fill_circle(img, dx, dy, 2.0, [200, 100, 30, 180]);
                        }
                        draw_glow(img, x + 8.0, y + 10.0, 14.0, [255, 200, 50]);
                    }
                    _ => {
                        fill_rect(img, ix + 2, iy + 4, 12, 12, [180, 40, 40, 255]);
                        draw_line(img, ix + 8, iy + 4, ix + 10, iy, [80, 80, 90, 255]);
                        blend_pixel(img, ix + 10, iy, [200, 200, 200, 255]);
                        // Blink faster when fuse lit
                        let blink_rate = if props.current_state == 1 { 6 } else { 20 };
                        if frame % blink_rate < blink_rate / 2 {
                            blend_pixel(img, ix + 5, iy + 6, [255, 50, 50, 255]);
                            draw_glow(img, x + 5.0, y + 6.0, 3.0, [255, 50, 50]);
                        }
                    }
                }
            }
            PyrotechnicType::MagnifyingGlass => {
                // Flippable: handle direction and focus direction mirror
                let lens_cx = if props.flipped { x + 8.0 } else { x + 8.0 };
                fill_circle(img, lens_cx, y + 8.0, 7.0, [200, 220, 240, 120]);
                let r = 7.0_f32;
                for t in 0..36 {
                    let a = t as f32 * std::f32::consts::TAU / 36.0;
                    blend_pixel(img, (lens_cx + a.cos() * r) as i32, (y + 8.0 + a.sin() * r) as i32, [140, 140, 150, 255]);
                }
                // Handle — flips side
                let (hx1, hx2) = if props.flipped {
                    (ix + 2, ix)
                } else {
                    (ix + 12, ix + 14)
                };
                draw_line(img, hx1, iy + 14, hx2, iy + 22, [120, 80, 40, 255]);
                draw_line(img, hx1 + 1, iy + 14, hx2 + 1, iy + 22, [120, 80, 40, 255]);
            }
            PyrotechnicType::MatchOnSpring => {
                for sy in 0..5 {
                    let wobble = (sy as f32 * 1.5).sin() * 2.0;
                    blend_pixel(img, ix + 6 + wobble as i32, iy + 10 + sy * 2, [160, 160, 170, 255]);
                    blend_pixel(img, ix + 6 + wobble as i32 + 1, iy + 10 + sy * 2, [160, 160, 170, 255]);
                }
                fill_circle(img, x + 6.0, y + 6.0, 3.0, [200, 50, 30, 255]);
                draw_line(img, ix + 6, iy + 6, ix + 6, iy + 10, [180, 150, 100, 255]);
            }
            PyrotechnicType::Fuse => {
                // State 0=Unburnt, 1=Burning, 2=Spent
                let fuse_len = props.values.get("length").copied().unwrap_or(64.0) as i32;
                let burn_pos = if props.current_state == 1 {
                    ((frame as f32 * 1.0) as i32 % fuse_len).max(0) // ~60px/s at 60fps
                } else if props.current_state == 2 {
                    fuse_len
                } else {
                    0
                };

                for fx in 0..fuse_len {
                    let wobble = ((fx as f32) * 0.3).sin() * 1.0;
                    let py = iy + 2 + wobble as i32;
                    if props.current_state >= 1 && fx < burn_pos {
                        // Spent portion — dark charred
                        blend_pixel(img, ix + fx, py, [40, 35, 30, 255]);
                    } else {
                        // Unburnt portion
                        blend_pixel(img, ix + fx, py, [120, 80, 40, 255]);
                    }
                }
                // Spark at burn front
                if props.current_state == 1 && burn_pos < fuse_len {
                    let spark_wobble = ((burn_pos as f32) * 0.3).sin() * 1.0;
                    draw_glow(img, (ix + burn_pos) as f32, (iy + 2) as f32 + spark_wobble, 4.0, [255, 220, 80]);
                    draw_flame(img, (ix + burn_pos) as f32, (iy + 1) as f32 + spark_wobble, 3.0, frame);
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
            PyrotechnicType::Fireworks => vec![
                PropertyDef { name: "color1".into(), min: 0.0, max: 255.0, step: 25.0, default: 255.0, label: "Color R".into() },
                PropertyDef { name: "color2".into(), min: 0.0, max: 255.0, step: 25.0, default: 100.0, label: "Color G".into() },
                PropertyDef { name: "color3".into(), min: 0.0, max: 255.0, step: 25.0, default: 200.0, label: "Color B".into() },
            ],
            PyrotechnicType::Fuse => vec![
                PropertyDef { name: "length".into(), min: 16.0, max: 200.0, step: 8.0, default: 64.0, label: "Length (px)".into() },
            ],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self,
            PyrotechnicType::Candle | PyrotechnicType::Cannon | PyrotechnicType::LavaLamp |
            PyrotechnicType::RemoteControlBomb | PyrotechnicType::Flashlight
        )
    }

    fn is_flippable(&self) -> bool {
        matches!(self,
            PyrotechnicType::MagnifyingGlass | PyrotechnicType::Flashlight | PyrotechnicType::Cannon
        )
    }
}
