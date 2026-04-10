use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MechanicalType {
    Gear,
    Pulley,
    Belt,
    TeeterTotter,
    ConveyorBelt,
    Trampoline,
    JackInTheBox,
    Windmill,
    MandrillMotor,
    MouseExerciseWheel,
    TransRotoMatic,
    RotoTransConverter,
    TipsyTrailer,
    Scissors,
    HedgeTrimmers,
    TinSnips,
    BoxingGlove,
    VacuumCleaner,
    PinballBumper,
    Tack,
    Bellows,
}

impl PartDef for MechanicalType {
    fn name(&self) -> &'static str {
        match self {
            MechanicalType::Gear => "Gear",
            MechanicalType::Pulley => "Pulley",
            MechanicalType::Belt => "Belt",
            MechanicalType::TeeterTotter => "Teeter-Totter",
            MechanicalType::ConveyorBelt => "Conveyor Belt",
            MechanicalType::Trampoline => "Trampoline",
            MechanicalType::JackInTheBox => "Jack-in-the-Box",
            MechanicalType::Windmill => "Windmill",
            MechanicalType::MandrillMotor => "Mandrill Motor",
            MechanicalType::MouseExerciseWheel => "Mouse Exercise Wheel",
            MechanicalType::TransRotoMatic => "Trans-Roto-Matic",
            MechanicalType::RotoTransConverter => "Roto-Trans-Converter",
            MechanicalType::TipsyTrailer => "Tipsy Trailer",
            MechanicalType::Scissors => "Scissors",
            MechanicalType::HedgeTrimmers => "Hedge Trimmers",
            MechanicalType::TinSnips => "Tin Snips",
            MechanicalType::BoxingGlove => "Boxing Glove",
            MechanicalType::VacuumCleaner => "Vacuum Cleaner",
            MechanicalType::PinballBumper => "Pinball Bumper",
            MechanicalType::Tack => "Tack",
            MechanicalType::Bellows => "Bike Pump (Bellows)",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            MechanicalType::Gear => "Interlocking cog; driven by belts/adjacent gears; pops balloons",
            MechanicalType::Pulley => "Frictionless rope redirection (max 8 per rope)",
            MechanicalType::Belt => "Connects two rotating parts; max 200px stretch; 1:1 speed",
            MechanicalType::TeeterTotter => "Seesaw: torque = force x distance from pivot",
            MechanicalType::ConveyorBelt => "Belt-driven surface; high friction (1.0)",
            MechanicalType::Trampoline => "Elasticity 1.2; bounces higher than fall; max 1500 px/s",
            MechanicalType::JackInTheBox => "Opens after ~3 rotations; catapult ~500 px/s",
            MechanicalType::Windmill => "Spins from air (fan/bellows/steam); drives belts",
            MechanicalType::MandrillMotor => "Monkey pedals when shade open; bonk = 2s stun",
            MechanicalType::MouseExerciseWheel => "Activates on nearby impact; mouse direction = spin",
            MechanicalType::TransRotoMatic => "Converts translational to rotational motion",
            MechanicalType::RotoTransConverter => "Converts rotational to translational motion",
            MechanicalType::TipsyTrailer => "Tilting platform on wheels",
            MechanicalType::Scissors => "Opens/closes on pressure; cuts rope, pops balloons",
            MechanicalType::HedgeTrimmers => "Always active; cuts rope on contact (NOT steel cable)",
            MechanicalType::TinSnips => "ONLY tool that cuts BOTH rope AND steel cable",
            MechanicalType::BoxingGlove => "Triggered punch at ~1200 px/s",
            MechanicalType::VacuumCleaner => "Requires power; sucks objects in ~60px radius",
            MechanicalType::PinballBumper => "Bounces objects away at ~800 px/s",
            MechanicalType::Tack => "Pops balloons on contact; forms walking surfaces",
            MechanicalType::Bellows => "Air burst when compressed; blows objects/spins windmills; flippable",
        }
    }

    fn category(&self) -> &'static str { "Mechanical" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            MechanicalType::Gear => (40.0, 40.0),
            MechanicalType::Pulley => (24.0, 24.0),
            MechanicalType::Belt => (64.0, 8.0),
            MechanicalType::TeeterTotter => (80.0, 16.0),
            MechanicalType::ConveyorBelt => (64.0, 16.0),
            MechanicalType::Trampoline => (48.0, 24.0),
            MechanicalType::JackInTheBox => (32.0, 32.0),
            MechanicalType::Windmill => (48.0, 48.0),
            MechanicalType::MandrillMotor => (48.0, 40.0),
            MechanicalType::MouseExerciseWheel => (40.0, 40.0),
            MechanicalType::TransRotoMatic => (24.0, 24.0),
            MechanicalType::RotoTransConverter => (24.0, 24.0),
            MechanicalType::TipsyTrailer => (48.0, 24.0),
            MechanicalType::Scissors => (24.0, 16.0),
            MechanicalType::HedgeTrimmers => (32.0, 12.0),
            MechanicalType::TinSnips => (24.0, 12.0),
            MechanicalType::BoxingGlove => (24.0, 32.0),
            MechanicalType::VacuumCleaner => (40.0, 32.0),
            MechanicalType::PinballBumper => (24.0, 24.0),
            MechanicalType::Tack => (8.0, 8.0),
            MechanicalType::Bellows => (32.0, 24.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            MechanicalType::Gear => '\u{2699}',
            MechanicalType::Pulley | MechanicalType::MouseExerciseWheel => '\u{25CE}',
            MechanicalType::Belt => '\u{2500}',
            MechanicalType::TeeterTotter | MechanicalType::TipsyTrailer => '\u{2550}',
            MechanicalType::ConveyorBelt => '\u{25AC}',
            MechanicalType::Trampoline => '\u{255A}',
            MechanicalType::JackInTheBox => '\u{2554}',
            MechanicalType::Windmill => '\u{2731}',
            MechanicalType::MandrillMotor => 'M',
            MechanicalType::TransRotoMatic => '\u{229E}',
            MechanicalType::RotoTransConverter => '\u{229F}',
            MechanicalType::Scissors => '\u{2702}',
            MechanicalType::HedgeTrimmers | MechanicalType::TinSnips => '\u{2704}',
            MechanicalType::BoxingGlove => 'B',
            MechanicalType::VacuumCleaner => 'V',
            MechanicalType::PinballBumper => '\u{25C9}',
            MechanicalType::Tack => '\u{25B4}',
            MechanicalType::Bellows => '\u{25C4}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            MechanicalType::Gear | MechanicalType::Scissors | MechanicalType::TinSnips | MechanicalType::Tack => SILVER,
            MechanicalType::Pulley | MechanicalType::MouseExerciseWheel | MechanicalType::ConveyorBelt
            | MechanicalType::Belt | MechanicalType::TransRotoMatic | MechanicalType::RotoTransConverter => GRAY,
            MechanicalType::TeeterTotter | MechanicalType::Windmill | MechanicalType::MandrillMotor | MechanicalType::TipsyTrailer => BROWN,
            MechanicalType::Trampoline => BLUE,
            MechanicalType::JackInTheBox => MAGENTA,
            MechanicalType::HedgeTrimmers => GREEN,
            MechanicalType::BoxingGlove => RED,
            MechanicalType::VacuumCleaner => BLUE,
            MechanicalType::PinballBumper => YELLOW,
            MechanicalType::Bellows => BROWN,
        }
    }

    fn physics(&self) -> PhysicsProps {
        match self {
            MechanicalType::Trampoline => PhysicsProps {
                mass: f32::INFINITY, elasticity: 1.2, density: 100.0, friction: 0.3,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            MechanicalType::PinballBumper => PhysicsProps {
                mass: f32::INFINITY, elasticity: 1.5, density: 100.0, friction: 0.1,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            MechanicalType::Tack => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.0, density: 100.0, friction: 0.8,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            MechanicalType::ConveyorBelt => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.1, density: 100.0, friction: 1.0,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
            _ => PhysicsProps {
                mass: f32::INFINITY, elasticity: 0.3, density: 100.0, friction: 0.5,
                gravity_response: GravityResponse::Normal, is_static: true,
            },
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            MechanicalType::Gear | MechanicalType::Windmill | MechanicalType::MouseExerciseWheel => vec![
                StateDef { name: "Idle", description: "Not spinning" },
                StateDef { name: "Spinning", description: "Rotating continuously" },
            ],
            MechanicalType::MandrillMotor => vec![
                StateDef { name: "ShadeClosed", description: "Shade down, monkey not pedaling" },
                StateDef { name: "Pedaling", description: "Shade open, monkey pedaling" },
                StateDef { name: "Stunned", description: "Bonked on head, stars for 2s" },
            ],
            MechanicalType::Scissors => vec![
                StateDef { name: "Open", description: "Blades apart" },
                StateDef { name: "Closed", description: "Blades shut — cuts rope/pops balloons" },
            ],
            MechanicalType::JackInTheBox => vec![
                StateDef { name: "Closed", description: "Lid sealed, counting rotations" },
                StateDef { name: "WindingUp", description: "Receiving rotation input" },
                StateDef { name: "Open", description: "Lid sprung open — catapult active" },
            ],
            MechanicalType::BoxingGlove => vec![
                StateDef { name: "Retracted", description: "Glove at rest" },
                StateDef { name: "Punching", description: "Spring extending — ~1200 px/s force" },
            ],
            MechanicalType::VacuumCleaner => vec![
                StateDef { name: "Off", description: "Unpowered" },
                StateDef { name: "On", description: "Powered — sucking objects in ~60px radius" },
            ],
            MechanicalType::TeeterTotter => vec![
                StateDef { name: "Level", description: "Balanced at center" },
                StateDef { name: "TiltedLeft", description: "Left side down" },
                StateDef { name: "TiltedRight", description: "Right side down" },
            ],
            MechanicalType::ConveyorBelt => vec![
                StateDef { name: "Idle", description: "Not belt-driven" },
                StateDef { name: "Running", description: "Belt-driven, moving objects" },
            ],
            MechanicalType::Bellows => vec![
                StateDef { name: "Open", description: "Handle up — ready to compress" },
                StateDef { name: "Compressed", description: "Handle pushed down — emitting air burst" },
                StateDef { name: "Spent", description: "Fully compressed — no more air" },
            ],
            MechanicalType::Trampoline => vec![
                StateDef { name: "Idle", description: "Surface at rest" },
                StateDef { name: "Compressed", description: "Object landing — storing energy" },
                StateDef { name: "Releasing", description: "Springing back — launching object" },
            ],
            _ => vec![
                StateDef { name: "Idle", description: "Default state" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let (w, h) = (props.width, props.height);
        let ix = x as i32;
        let iy = y as i32;
        let ic = self.icon_color();

        match self {
            MechanicalType::Gear => {
                // State 0=Idle, 1=Spinning
                let r = props.values.get("radius").copied().unwrap_or(20.0);
                let cx = x + r;
                let cy = y + r;
                fill_circle(img, cx, cy, r, [ic[0], ic[1], ic[2], 255]);
                fill_circle(img, cx, cy, r * 0.6, [ic[0].saturating_sub(30), ic[1].saturating_sub(30), ic[2].saturating_sub(30), 255]);
                let teeth = 8;
                let angle_offset = if props.current_state == 1 { frame as f32 * 0.08 } else { 0.0 };
                for i in 0..teeth {
                    let angle = angle_offset + i as f32 * std::f32::consts::TAU / teeth as f32;
                    let tx = cx + angle.cos() * r;
                    let ty = cy + angle.sin() * r;
                    fill_rect(img, tx as i32 - 2, ty as i32 - 2, 5, 5, [ic[0], ic[1], ic[2], 255]);
                }
                fill_circle(img, cx, cy, 3.0, [80, 80, 90, 255]);
            }
            MechanicalType::TeeterTotter => {
                // State 0=Level, 1=TiltedLeft, 2=TiltedRight
                let length = props.values.get("length").copied().unwrap_or(80.0) as i32;
                let mid = ix + length / 2;
                fill_triangle(img, (mid as f32, iy as f32 + h - 4.0), ((mid - 8) as f32, iy as f32 + h), ((mid + 8) as f32, iy as f32 + h), [ic[0], ic[1], ic[2], 255]);
                let tilt = match props.current_state {
                    1 => -6_i32,  // tilted left
                    2 => 6,       // tilted right
                    _ => 0,       // level
                };
                draw_line(img, ix, iy + (h as i32) / 2 - tilt, ix + length, iy + (h as i32) / 2 + tilt, [ic[0], ic[1], ic[2], 255]);
                draw_line(img, ix, iy + (h as i32) / 2 - tilt + 1, ix + length, iy + (h as i32) / 2 + tilt + 1, [ic[0], ic[1], ic[2], 255]);
            }
            MechanicalType::Trampoline => {
                // State 0=Idle, 1=Compressed, 2=Releasing
                let c = [120, 120, 130, 255];
                draw_line(img, ix + 4, iy + h as i32, ix + 8, iy + h as i32 / 2, c);
                draw_line(img, ix + w as i32 - 4, iy + h as i32, ix + w as i32 - 8, iy + h as i32 / 2, c);
                for sx in (8..w as i32 - 8).step_by(6) {
                    let spring_compress = if props.current_state == 1 { 2 } else { 0 };
                    draw_line(img, ix + sx, iy + h as i32 / 2 + spring_compress, ix + sx + 2, iy + h as i32 / 2 - 4 + spring_compress, c);
                }
                let surface_y = match props.current_state {
                    1 => iy + h as i32 / 2 + 2,  // compressed down
                    2 => iy + h as i32 / 2 - 8,  // spring up high
                    _ => iy + h as i32 / 2 - 4,  // resting
                };
                let fabric_color = match props.current_state {
                    1 => [60, 100, 200, 255],  // stretched
                    2 => [120, 160, 255, 255],  // bright releasing
                    _ => [80, 120, 220, 255],
                };
                draw_line(img, ix + 4, surface_y, ix + w as i32 - 4, surface_y, fabric_color);
                draw_line(img, ix + 4, surface_y + 1, ix + w as i32 - 4, surface_y + 1, fabric_color);
                if props.current_state == 2 {
                    // Energy release lines
                    for i in 0..3 {
                        let ly = surface_y - 3 - i * 2;
                        let alpha = (180 - i * 50) as u8;
                        draw_line(img, ix + 10 + i * 4, ly, ix + w as i32 - 10 - i * 4, ly, [200, 220, 255, alpha]);
                    }
                }
            }
            MechanicalType::Windmill => {
                // State 0=Idle, 1=Spinning
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                let blade_len = w / 2.0 - 4.0;
                let spin_dir = if props.flipped { -1.0_f32 } else { 1.0 };
                let angle = if props.current_state == 1 { frame as f32 * 0.08 * spin_dir } else { 0.4 }; // static angle when idle
                for i in 0..4 {
                    let a = angle + i as f32 * std::f32::consts::FRAC_PI_2;
                    let ex = cx + a.cos() * blade_len;
                    let ey = cy + a.sin() * blade_len;
                    draw_line(img, cx as i32, cy as i32, ex as i32, ey as i32, [ic[0], ic[1], ic[2], 255]);
                    let perp_a = a + std::f32::consts::FRAC_PI_2;
                    let pw = 3.0;
                    draw_line(img,
                        (cx + a.cos() * blade_len * 0.3 + perp_a.cos() * pw) as i32,
                        (cy + a.sin() * blade_len * 0.3 + perp_a.sin() * pw) as i32,
                        (ex + perp_a.cos() * pw) as i32,
                        (ey + perp_a.sin() * pw) as i32,
                        [ic[0], ic[1], ic[2], 200],
                    );
                }
                fill_circle(img, cx, cy, 4.0, [80, 80, 90, 255]);
            }
            MechanicalType::PinballBumper => {
                // Always animated — flash on hit is part of state
                let cx = x + 12.0;
                let cy = y + 12.0;
                fill_circle_gradient(img, cx, cy, 12.0, [255, 255, 200], [200, 180, 50]);
                // Blink pattern
                let blink = (frame % 10 < 3) || props.current_state == 0;
                if blink {
                    draw_glow(img, cx, cy, 16.0, [255, 255, 100]);
                }
            }
            MechanicalType::Tack => {
                fill_triangle(img, (x + 4.0, y), (x + 1.0, y + 6.0), (x + 7.0, y + 6.0), [200, 200, 210, 255]);
                fill_rect(img, ix + 1, iy + 6, 6, 2, [180, 180, 190, 255]);
            }
            MechanicalType::ConveyorBelt => {
                // State 0=Idle, 1=Running
                let cw = props.values.get("length").copied().unwrap_or(64.0) as i32;
                fill_rect(img, ix, iy + 4, cw, 8, [ic[0], ic[1], ic[2], 255]);
                let rib_offset = if props.current_state == 1 { (frame as i32 * 2) % 8 } else { 0 };
                for rx in (rib_offset..cw).step_by(8) {
                    draw_line(img, ix + rx, iy + 4, ix + rx, iy + 12, [ic[0].saturating_sub(40), ic[1].saturating_sub(40), ic[2].saturating_sub(40), 200]);
                }
                // Animated arrows when running
                if props.current_state == 1 {
                    let arrow_phase = (frame as i32 * 2) % 16;
                    for ax in (arrow_phase..cw).step_by(16) {
                        let arrow_x = ix + ax;
                        let arrow_y = iy + 8;
                        draw_line(img, arrow_x - 3, arrow_y, arrow_x + 3, arrow_y, [255, 255, 255, 200]);
                        draw_line(img, arrow_x + 1, arrow_y - 2, arrow_x + 3, arrow_y, [255, 255, 255, 200]);
                        draw_line(img, arrow_x + 1, arrow_y + 2, arrow_x + 3, arrow_y, [255, 255, 255, 200]);
                    }
                } else {
                    let arrow_x = ix + cw / 2;
                    let arrow_y = iy + 8;
                    draw_line(img, arrow_x - 4, arrow_y, arrow_x + 4, arrow_y, [180, 180, 180, 120]);
                    draw_line(img, arrow_x + 2, arrow_y - 2, arrow_x + 4, arrow_y, [180, 180, 180, 120]);
                }
                // End wheels — spin when running
                let wheel_angle = if props.current_state == 1 { frame as f32 * 0.2 } else { 0.0 };
                fill_circle(img, x + 6.0, y + 8.0, 5.0, [100, 100, 110, 255]);
                fill_circle(img, x + cw as f32 - 6.0, y + 8.0, 5.0, [100, 100, 110, 255]);
                if props.current_state == 1 {
                    // Wheel spokes
                    for wc in [x + 6.0, x + cw as f32 - 6.0] {
                        for i in 0..4 {
                            let a = wheel_angle + i as f32 * std::f32::consts::FRAC_PI_2;
                            blend_pixel(img, (wc + a.cos() * 3.0) as i32, (y + 8.0 + a.sin() * 3.0) as i32, [60, 60, 70, 255]);
                        }
                    }
                }
            }
            MechanicalType::BoxingGlove => {
                // State 0=Retracted, 1=Punching
                // Punches horizontally in facing direction (flippable)
                let dir: i32 = if props.flipped { -1 } else { 1 };
                let punch_extend = if props.current_state == 1 { 12.0 } else { 0.0 };
                let base_x = if props.flipped { x + w - 8.0 } else { x };
                let glove_x = base_x + dir as f32 * punch_extend;
                let arm_cy = y + h / 2.0;
                // Spring arm — horizontal coils
                let coils = if props.current_state == 1 { 3 } else { 6 };
                let arm_start = base_x + dir as f32 * 10.0;
                let arm_end = if props.flipped { x + w } else { x + 8.0 };
                for sy in 0..coils {
                    let t = sy as f32 / coils as f32;
                    let arm_px = arm_start as i32 + ((arm_end - arm_start) * t) as i32;
                    let wobble = (sy as f32 * 1.5).sin() * 3.0;
                    blend_pixel(img, arm_px, (arm_cy + wobble) as i32, [160, 160, 170, 255]);
                    blend_pixel(img, arm_px, (arm_cy + wobble) as i32 + 1, [160, 160, 170, 255]);
                }
                // Glove
                fill_circle_gradient(img, glove_x + dir as f32 * 2.0, arm_cy, 10.0, [230, 50, 50], [180, 30, 30]);
                // Mount box on opposite side
                let mount_x = if props.flipped { ix + w as i32 - 8 } else { ix };
                fill_rect(img, mount_x, iy + 4, 8, h as i32 - 8, [100, 100, 110, 255]);
                if props.current_state == 1 {
                    // Motion blur / impact lines
                    for i in 1..4 {
                        let lx = glove_x + dir as f32 * (10.0 + i as f32 * 4.0);
                        draw_line(img, lx as i32, (arm_cy - 6.0) as i32, lx as i32, (arm_cy + 6.0) as i32, [255, 200, 200, (100 - i * 25) as u8]);
                    }
                }
            }
            MechanicalType::Scissors => {
                // Two blades
                let open_angle = if props.current_state == 1 { 0.0_f32 } else { 0.3 };
                let cx = x + 12.0;
                let cy = y + 8.0;
                draw_line(img, cx as i32, cy as i32, (cx + 12.0 * (open_angle).cos()) as i32, (cy - 12.0 * (open_angle).sin()) as i32, [200, 200, 210, 255]);
                draw_line(img, cx as i32, cy as i32, (cx + 12.0 * (open_angle).cos()) as i32, (cy + 12.0 * (open_angle).sin()) as i32, [200, 200, 210, 255]);
                // Pivot
                fill_circle(img, cx, cy, 2.0, [150, 150, 160, 255]);
                // Handles
                draw_line(img, cx as i32, cy as i32, (cx - 8.0) as i32, (cy - 4.0) as i32, [180, 60, 60, 255]);
                draw_line(img, cx as i32, cy as i32, (cx - 8.0) as i32, (cy + 4.0) as i32, [180, 60, 60, 255]);
            }
            MechanicalType::VacuumCleaner => {
                // State 0=Off, 1=On
                fill_rect(img, ix, iy, w as i32, h as i32, [ic[0], ic[1], ic[2], 200]);
                fill_rect(img, ix + 4, iy + 4, 12, 20, [60, 90, 180, 255]);
                // Hose
                draw_line(img, ix + 16, iy + 8, ix + w as i32 - 4, iy + 4, [80, 80, 90, 255]);
                if props.current_state == 1 {
                    // Suction lines
                    for i in 0..4 {
                        let sx = ix + w as i32 + 2 + ((frame as i32 + i * 3) % 12);
                        let alpha = (180 - (frame as i32 + i * 3) % 12 * 12) as u8;
                        draw_line(img, sx, iy + 2, sx, iy + h as i32 - 2, [200, 220, 255, alpha]);
                    }
                    // Vibration
                    let vib = ((frame as f32 * 0.5).sin() * 1.0) as i32;
                    fill_rect(img, ix + vib, iy + h as i32 - 4, w as i32, 2, [ic[0], ic[1], ic[2], 150]);
                }
            }
            MechanicalType::JackInTheBox => {
                // State 0=Closed, 1=WindingUp, 2=Open
                let box_c = [180, 50, 180, 255];
                fill_rect(img, ix, iy + 12, w as i32, h as i32 - 12, box_c);
                draw_line(img, ix, iy + 12, ix + w as i32, iy + 12, [200, 80, 200, 255]);
                let dir: i32 = if props.flipped { -1 } else { 1 };
                let crank_side = if props.flipped { ix - 6 } else { ix + w as i32 + 2 };
                match props.current_state {
                    1 => {
                        // Winding — lid vibrating
                        let vib = ((frame as f32 * 0.4).sin() * 2.0) as i32;
                        fill_rect(img, ix + vib, iy + 10, w as i32, 3, [200, 80, 200, 255]);
                        // Crank handle
                        let crank_a = frame as f32 * 0.15;
                        blend_pixel(img, crank_side + (crank_a.cos() * 4.0) as i32, iy + 20 + (crank_a.sin() * 4.0) as i32, [120, 120, 130, 255]);
                    }
                    2 => {
                        // Open — spring popping out with figure, catapults in facing direction
                        let lid_x = if props.flipped { ix - w as i32 / 2 } else { ix + w as i32 / 2 };
                        fill_rect(img, lid_x, iy + 8, w as i32 / 2, 3, [200, 80, 200, 200]);
                        // Spring — angled in catapult direction
                        let fig_x = x + w / 2.0 + dir as f32 * 6.0;
                        for sy in 0..4 {
                            let wobble = (sy as f32 * 2.0).sin() * 3.0;
                            blend_pixel(img, (fig_x + wobble) as i32, iy + 8 - sy * 2, [160, 160, 170, 255]);
                        }
                        // Figure head
                        fill_circle(img, fig_x, y + 2.0, 4.0, [255, 220, 100, 255]);
                        blend_pixel(img, fig_x as i32 - 1, iy + 1, [40, 40, 40, 255]);
                        blend_pixel(img, fig_x as i32 + 1, iy + 1, [40, 40, 40, 255]);
                    }
                    _ => {
                        // Closed lid
                        fill_rect(img, ix, iy + 10, w as i32, 3, [200, 80, 200, 255]);
                    }
                }
            }
            MechanicalType::MandrillMotor => {
                // State 0=ShadeClosed, 1=Pedaling, 2=Stunned
                fill_rect(img, ix, iy + 20, w as i32, h as i32 - 20, [100, 70, 40, 255]);
                // Bicycle frame
                fill_circle(img, x + 12.0, y + 34.0, 6.0, [80, 80, 90, 255]);
                fill_circle(img, x + 36.0, y + 34.0, 6.0, [80, 80, 90, 255]);
                draw_line(img, ix + 12, iy + 28, ix + 24, iy + 22, [80, 80, 90, 255]);
                draw_line(img, ix + 24, iy + 22, ix + 36, iy + 28, [80, 80, 90, 255]);
                // Monkey
                fill_circle(img, x + 24.0, y + 14.0, 6.0, [160, 110, 60, 255]);
                fill_rect(img, ix + 20, iy + 20, 8, 8, [160, 110, 60, 255]);
                match props.current_state {
                    1 => {
                        // Pedaling — legs animated
                        let pedal_a = frame as f32 * 0.2;
                        let foot_x = x + 24.0 + pedal_a.cos() * 6.0;
                        let foot_y = y + 32.0 + pedal_a.sin() * 4.0;
                        draw_line(img, ix + 22, iy + 28, foot_x as i32, foot_y as i32, [140, 90, 40, 255]);
                        draw_line(img, ix + 26, iy + 28, (foot_x + 4.0) as i32, foot_y as i32, [140, 90, 40, 255]);
                        // Shade open (up)
                        fill_rect(img, ix + 4, iy, 16, 3, [200, 200, 200, 180]);
                    }
                    2 => {
                        // Stunned — stars above head
                        let star_phase = frame as f32 * 0.3;
                        for i in 0..3 {
                            let sa = star_phase + i as f32 * std::f32::consts::TAU / 3.0;
                            let sx = x + 24.0 + sa.cos() * 8.0;
                            let sy = y + 6.0 + sa.sin() * 3.0;
                            blend_pixel(img, sx as i32, sy as i32, [255, 255, 100, 255]);
                        }
                        // Shade closed
                        fill_rect(img, ix + 2, iy + 6, 20, 14, [200, 200, 200, 200]);
                    }
                    _ => {
                        // Shade closed — monkey idle
                        fill_rect(img, ix + 2, iy + 6, 20, 14, [200, 200, 200, 200]);
                    }
                }
            }
            MechanicalType::MouseExerciseWheel => {
                // State 0=Idle, 1=Spinning
                // Flip changes mouse direction and spin direction
                let spin_dir = if props.flipped { -1.0_f32 } else { 1.0 };
                let cx = x + 20.0;
                let cy = y + 20.0;
                let wheel_angle = if props.current_state == 1 { frame as f32 * 0.1 * spin_dir } else { 0.0 };
                // Wire wheel
                fill_circle(img, cx, cy, 16.0, [ic[0], ic[1], ic[2], 120]);
                for i in 0..8 {
                    let a = wheel_angle + i as f32 * std::f32::consts::TAU / 8.0;
                    let ex = cx + a.cos() * 15.0;
                    let ey = cy + a.sin() * 15.0;
                    draw_line(img, cx as i32, cy as i32, ex as i32, ey as i32, [ic[0], ic[1], ic[2], 180]);
                }
                fill_circle(img, cx, cy, 3.0, [80, 80, 90, 255]);
                // Mouse inside — faces left or right based on flip
                if props.current_state == 1 {
                    let mx = cx + (wheel_angle + 1.0).cos() * 10.0;
                    let my = cy + (wheel_angle + 1.0).sin() * 10.0;
                    fill_circle(img, mx, my, 3.0, [160, 160, 160, 255]);
                } else {
                    let mouse_x = if props.flipped { cx + 6.0 } else { cx - 6.0 };
                    fill_circle(img, mouse_x, cy + 8.0, 3.0, [160, 160, 160, 255]);
                }
            }
            MechanicalType::HedgeTrimmers => {
                // Always active — blades slightly open/close
                let open = ((frame as f32 * 0.15).sin() * 0.15).abs();
                let cx = x + 8.0;
                let cy = y + 6.0;
                draw_line(img, cx as i32, cy as i32, (cx + 24.0) as i32, (cy - 6.0 * open) as i32, [50, 160, 50, 255]);
                draw_line(img, cx as i32, cy as i32, (cx + 24.0) as i32, (cy + 6.0 * open) as i32, [50, 160, 50, 255]);
                fill_circle(img, cx, cy, 2.0, [80, 80, 90, 255]);
                fill_rect(img, ix, iy + 3, 8, 6, [100, 70, 40, 255]);
            }
            MechanicalType::TinSnips => {
                // Similar to scissors — always slightly animated
                let open = ((frame as f32 * 0.12).sin() * 0.2).abs();
                let cx = x + 8.0;
                let cy = y + 6.0;
                draw_line(img, cx as i32, cy as i32, (cx + 16.0) as i32, (cy - 6.0 * open) as i32, [200, 200, 210, 255]);
                draw_line(img, cx as i32, cy as i32, (cx + 16.0) as i32, (cy + 6.0 * open) as i32, [200, 200, 210, 255]);
                fill_circle(img, cx, cy, 2.0, [150, 150, 160, 255]);
                fill_rect(img, ix, iy + 3, 8, 6, [180, 180, 190, 255]);
            }
            MechanicalType::Bellows => {
                // State 0=Open, 1=Compressed, 2=Spent
                // Flippable: air blows left or right
                let dir: i32 = if props.flipped { -1 } else { 1 };
                let compress = match props.current_state {
                    1 => 8, // compressed
                    2 => 12, // fully spent
                    _ => 0, // open
                };
                // Accordion body — narrows when compressed
                let body_w = (w as i32 - 8 - compress).max(4);
                let body_x = if props.flipped { ix + w as i32 - body_w - 4 } else { ix + 4 };
                fill_rect(img, body_x, iy + 4, body_w, h as i32 - 8, [ic[0], ic[1], ic[2], 255]);
                // Accordion folds
                for fold in (0..body_w).step_by(4) {
                    draw_line(img, body_x + fold, iy + 4, body_x + fold, iy + h as i32 - 4,
                        [ic[0].saturating_sub(30), ic[1].saturating_sub(30), ic[2].saturating_sub(30), 200]);
                }
                // Handle/top plate
                let handle_x = if props.flipped { body_x + body_w } else { body_x - 4 };
                fill_rect(img, handle_x, iy + 2, 6, h as i32 - 4, [100, 70, 40, 255]);
                // Nozzle
                let nozzle_x = if props.flipped { ix } else { ix + w as i32 - 4 };
                fill_rect(img, nozzle_x, iy + 8, 4, 8, [120, 120, 130, 255]);
                // Air burst when compressed
                if props.current_state == 1 {
                    let air_x = if props.flipped { ix - 4 } else { ix + w as i32 };
                    for i in 0..4 {
                        let ax = air_x + dir * (i * 4 + ((frame as i32 * 2) % 4));
                        let alpha = (180 - i * 40) as u8;
                        draw_line(img, ax, iy + 8, ax, iy + 16, [200, 220, 255, alpha]);
                    }
                }
            }
            _ => {
                // Pulley, Belt, TransRotoMatic, RotoTransConverter, TipsyTrailer
                fill_rect(img, ix, iy, w as i32, h as i32, [ic[0], ic[1], ic[2], 200]);
                draw_line(img, ix, iy, ix + w as i32 - 1, iy, [ic[0].saturating_add(30), ic[1].saturating_add(30), ic[2].saturating_add(30), 255]);
                draw_line(img, ix, iy + h as i32 - 1, ix + w as i32 - 1, iy + h as i32 - 1, [ic[0].saturating_sub(30), ic[1].saturating_sub(30), ic[2].saturating_sub(30), 255]);
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
            MechanicalType::Gear => vec![
                PropertyDef { name: "radius".into(), min: 16.0, max: 32.0, step: 2.0, default: 20.0, label: "Radius".into() },
            ],
            MechanicalType::TeeterTotter => vec![
                PropertyDef { name: "length".into(), min: 60.0, max: 120.0, step: 10.0, default: 80.0, label: "Length".into() },
            ],
            MechanicalType::ConveyorBelt => vec![
                PropertyDef { name: "length".into(), min: 64.0, max: 256.0, step: 64.0, default: 64.0, label: "Length".into() },
            ],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self,
            MechanicalType::Gear | MechanicalType::Windmill | MechanicalType::TeeterTotter |
            MechanicalType::Trampoline | MechanicalType::PinballBumper | MechanicalType::MandrillMotor |
            MechanicalType::MouseExerciseWheel
        )
    }

    fn is_flippable(&self) -> bool {
        matches!(self, MechanicalType::Windmill | MechanicalType::BoxingGlove | MechanicalType::JackInTheBox | MechanicalType::MouseExerciseWheel | MechanicalType::Bellows)
    }

    fn requires_power(&self) -> bool {
        matches!(self, MechanicalType::VacuumCleaner)
    }

    fn has_rope_point(&self) -> bool {
        matches!(self, MechanicalType::TeeterTotter | MechanicalType::Pulley)
    }

    fn can_be_ramp(&self) -> bool {
        matches!(self, MechanicalType::TeeterTotter | MechanicalType::PinballBumper | MechanicalType::VacuumCleaner | MechanicalType::Bellows)
    }
}
