use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};

use crate::parts::{GravityResponse, PartDef, PartProps, PhysicsProps, PropertyDef, StateDef};
use crate::render::pixel_gfx::*;
use crate::render::text_chars::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElectricalType {
    Generator,
    ElectricalOutlet,
    SwitchOutlet,
    SolarPanel,
    LaserActivatedPlug,
    ElectricFan,
    ElectricMotor,
    Toaster,
    CanOpener,
    ElectricMixer,
}

impl PartDef for ElectricalType {
    fn name(&self) -> &'static str {
        match self {
            ElectricalType::Generator => "Generator",
            ElectricalType::ElectricalOutlet => "Electrical Outlet",
            ElectricalType::SwitchOutlet => "Switch Outlet",
            ElectricalType::SolarPanel => "Solar Panel",
            ElectricalType::LaserActivatedPlug => "Laser-Activated Plug",
            ElectricalType::ElectricFan => "Electric Fan",
            ElectricalType::ElectricMotor => "Electric Motor",
            ElectricalType::Toaster => "Toaster",
            ElectricalType::CanOpener => "Can Opener",
            ElectricalType::ElectricMixer => "Electric Mixer",
        }
    }

    fn description(&self) -> &'static str {
        match self {
            ElectricalType::Generator => "Spinning-part driven; has outlet (2 sockets)",
            ElectricalType::ElectricalOutlet => "Always-on power; 2 sockets; no input needed",
            ElectricalType::SwitchOutlet => "Toggled by falling object; flippable orientation",
            ElectricalType::SolarPanel => "Powered by light source within ~80px line-of-sight",
            ElectricalType::LaserActivatedPlug => "Color-programmable; powered by matching laser",
            ElectricalType::ElectricFan => "Directional airflow ~120px range; requires power",
            ElectricalType::ElectricMotor => "Continuous rotation when powered; drives belts",
            ElectricalType::Toaster => "Launches 2 toast at ~600 px/s after delay",
            ElectricalType::CanOpener => "Powered mechanical opener",
            ElectricalType::ElectricMixer => "Rotating beaters; drives belts when powered",
        }
    }

    fn category(&self) -> &'static str { "Electrical" }

    fn default_size(&self) -> (f32, f32) {
        match self {
            ElectricalType::Generator => (32.0, 32.0),
            ElectricalType::ElectricalOutlet => (16.0, 24.0),
            ElectricalType::SwitchOutlet => (16.0, 32.0),
            ElectricalType::SolarPanel => (32.0, 24.0),
            ElectricalType::LaserActivatedPlug => (16.0, 24.0),
            ElectricalType::ElectricFan => (24.0, 24.0),
            ElectricalType::ElectricMotor => (20.0, 20.0),
            ElectricalType::Toaster => (24.0, 20.0),
            ElectricalType::CanOpener => (20.0, 16.0),
            ElectricalType::ElectricMixer => (20.0, 24.0),
        }
    }

    fn icon_char(&self) -> char {
        match self {
            ElectricalType::Generator => '\u{26A1}',
            ElectricalType::ElectricalOutlet | ElectricalType::SwitchOutlet => '\u{25AA}',
            ElectricalType::SolarPanel => '\u{25A6}',
            ElectricalType::LaserActivatedPlug => '\u{25C9}',
            ElectricalType::ElectricFan => '\u{274A}',
            ElectricalType::ElectricMotor => '\u{2299}',
            ElectricalType::Toaster => '\u{25AC}',
            ElectricalType::CanOpener => '\u{22A1}',
            ElectricalType::ElectricMixer => '\u{229B}',
        }
    }

    fn icon_color(&self) -> [u8; 3] {
        match self {
            ElectricalType::Generator => YELLOW,
            ElectricalType::ElectricalOutlet | ElectricalType::SwitchOutlet => WHITE,
            ElectricalType::SolarPanel => BLUE,
            ElectricalType::LaserActivatedPlug => RED,
            ElectricalType::ElectricFan => BLUE,
            ElectricalType::ElectricMotor => GRAY,
            ElectricalType::Toaster | ElectricalType::CanOpener | ElectricalType::ElectricMixer => SILVER,
        }
    }

    fn physics(&self) -> PhysicsProps {
        PhysicsProps {
            mass: f32::INFINITY,
            elasticity: 0.2,
            density: 100.0,
            friction: 0.4,
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }

    fn states(&self) -> Vec<StateDef> {
        match self {
            ElectricalType::Generator => vec![
                StateDef { name: "Unpowered", description: "Not being driven by a spinning part" },
                StateDef { name: "Generating", description: "Driven — providing power to outlet" },
            ],
            ElectricalType::ElectricalOutlet => vec![
                StateDef { name: "On", description: "Always providing power" },
            ],
            ElectricalType::SwitchOutlet => vec![
                StateDef { name: "Off", description: "Switch in original position" },
                StateDef { name: "On", description: "Toggled by falling object — providing power" },
            ],
            ElectricalType::SolarPanel => vec![
                StateDef { name: "Dark", description: "No light source within range" },
                StateDef { name: "Powered", description: "Light source within ~80px, line-of-sight" },
            ],
            ElectricalType::LaserActivatedPlug => vec![
                StateDef { name: "Inactive", description: "Not receiving correct laser color" },
                StateDef { name: "Active", description: "Matching laser hitting sensor" },
            ],
            ElectricalType::ElectricFan | ElectricalType::ElectricMotor |
            ElectricalType::ElectricMixer | ElectricalType::CanOpener => vec![
                StateDef { name: "Off", description: "Unpowered — static" },
                StateDef { name: "On", description: "Powered — operating" },
            ],
            ElectricalType::Toaster => vec![
                StateDef { name: "Off", description: "Unpowered" },
                StateDef { name: "Heating", description: "Powered — countdown to launch" },
                StateDef { name: "Popped", description: "Toast launched at ~600 px/s" },
            ],
        }
    }

    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64) {
        let (w, h) = (props.width, props.height);
        let ix = x as i32;
        let iy = y as i32;
        let ic = self.icon_color();

        match self {
            ElectricalType::Generator => {
                // State 0=Unpowered, 1=Generating
                fill_rect(img, ix + 4, iy + 8, 24, 20, [80, 80, 90, 255]);
                let cx = x + 16.0;
                let cy = y + 12.0;
                if props.current_state == 1 {
                    // Spinning wheel
                    let angle = frame as f32 * 0.15;
                    for i in 0..4 {
                        let a = angle + i as f32 * std::f32::consts::FRAC_PI_2;
                        let ex = cx + a.cos() * 8.0;
                        let ey = cy + a.sin() * 8.0;
                        draw_line(img, cx as i32, cy as i32, ex as i32, ey as i32, [ic[0], ic[1], ic[2], 255]);
                    }
                    fill_circle(img, cx, cy, 10.0, [ic[0], ic[1], ic[2], 200]);
                    draw_glow(img, cx, cy, 14.0, [255, 255, 100]);
                    // Electricity sparks
                    if frame % 4 < 2 {
                        let spark_a = frame as f32 * 0.7;
                        blend_pixel(img, (cx + spark_a.cos() * 12.0) as i32, (cy + spark_a.sin() * 12.0) as i32, [255, 255, 200, 255]);
                    }
                } else {
                    fill_circle(img, cx, cy, 10.0, [ic[0] / 2, ic[1] / 2, ic[2] / 2, 255]);
                }
                fill_circle(img, cx, cy, 4.0, [60, 60, 70, 255]);
            }
            ElectricalType::ElectricFan => {
                // State 0=Off, 1=On
                let cx = x + 12.0;
                let cy = y + 12.0;
                if props.current_state == 1 {
                    // Spinning fast — blurred blades
                    let angle = frame as f32 * 0.3;
                    for i in 0..5 {
                        let a = angle + i as f32 * std::f32::consts::TAU / 5.0;
                        let ex = cx + a.cos() * 10.0;
                        let ey = cy + a.sin() * 10.0;
                        draw_line(img, cx as i32, cy as i32, ex as i32, ey as i32, [ic[0], ic[1], ic[2], 220]);
                    }
                    // Airflow lines
                    let flow_dir: i32 = if props.flipped { -1 } else { 1 };
                    for i in 0..3 {
                        let fx = cx + flow_dir as f32 * (14.0 + i as f32 * 6.0 + ((frame as f32 * 0.2) % 6.0));
                        let alpha = (120 - i * 30) as u8;
                        draw_line(img, fx as i32, (cy - 3.0) as i32, (fx + flow_dir as f32 * 4.0) as i32, (cy - 3.0) as i32, [200, 220, 255, alpha]);
                        draw_line(img, fx as i32, cy as i32, (fx + flow_dir as f32 * 4.0) as i32, cy as i32, [200, 220, 255, alpha]);
                        draw_line(img, fx as i32, (cy + 3.0) as i32, (fx + flow_dir as f32 * 4.0) as i32, (cy + 3.0) as i32, [200, 220, 255, alpha]);
                    }
                } else {
                    // Static blades
                    for i in 0..5 {
                        let a = i as f32 * std::f32::consts::TAU / 5.0;
                        let ex = cx + a.cos() * 10.0;
                        let ey = cy + a.sin() * 10.0;
                        draw_line(img, cx as i32, cy as i32, ex as i32, ey as i32, [ic[0] / 2, ic[1] / 2, ic[2] / 2, 180]);
                    }
                }
                fill_circle(img, cx, cy, 3.0, [80, 80, 90, 255]);
            }
            ElectricalType::Toaster => {
                // State 0=Off, 1=Heating, 2=Popped
                fill_rect_gradient_v(img, ix, iy, w as i32, h as i32, [210, 210, 220], [170, 170, 180]);
                fill_rect(img, ix + w as i32 - 3, iy + 6, 3, 8, [150, 150, 160, 255]);
                match props.current_state {
                    1 => {
                        // Heating — glow from slots
                        fill_rect(img, ix + 4, iy + 2, 6, 3, [255, 120, 40, 255]);
                        fill_rect(img, ix + 14, iy + 2, 6, 3, [255, 120, 40, 255]);
                        draw_glow(img, x + 7.0, y + 3.0, 5.0, [255, 150, 50]);
                        draw_glow(img, x + 17.0, y + 3.0, 5.0, [255, 150, 50]);
                    }
                    2 => {
                        // Popped — toast flying up
                        fill_rect(img, ix + 4, iy + 2, 6, 3, [40, 40, 40, 255]);
                        fill_rect(img, ix + 14, iy + 2, 6, 3, [40, 40, 40, 255]);
                        let toast_y = iy - 8 - ((frame as i32 * 2) % 16);
                        fill_rect(img, ix + 5, toast_y, 4, 5, [220, 190, 120, 255]);
                        fill_rect(img, ix + 15, toast_y + 2, 4, 5, [220, 190, 120, 255]);
                    }
                    _ => {
                        fill_rect(img, ix + 4, iy + 2, 6, 3, [40, 40, 40, 255]);
                        fill_rect(img, ix + 14, iy + 2, 6, 3, [40, 40, 40, 255]);
                    }
                }
            }
            ElectricalType::ElectricalOutlet => {
                // State 0=On (always on)
                fill_rect(img, ix, iy, w as i32, h as i32, [230, 225, 220, 255]);
                draw_line(img, ix, iy, ix + w as i32, iy, [200, 200, 200, 255]);
                fill_rect(img, ix + 4, iy + 8, 3, 4, [40, 40, 40, 255]);
                fill_rect(img, ix + 9, iy + 8, 3, 4, [40, 40, 40, 255]);
                // Power indicator
                blend_pixel(img, ix + 8, iy + 2, [100, 255, 100, 200]);
            }
            ElectricalType::SwitchOutlet => {
                // State 0=Off, 1=On
                // Flippable: determines which side the switch is on
                fill_rect(img, ix, iy, w as i32, h as i32, [230, 225, 220, 255]);
                draw_line(img, ix, iy, ix + w as i32, iy, [200, 200, 200, 255]);
                let socket_x = if props.flipped { ix + w as i32 - 7 } else { ix + 4 };
                fill_rect(img, socket_x, iy + h as i32 / 2, 3, 4, [40, 40, 40, 255]);
                fill_rect(img, socket_x + 5, iy + h as i32 / 2, 3, 4, [40, 40, 40, 255]);
                // Toggle switch — position depends on state and flip
                let switch_x = if props.flipped { ix + w as i32 - 11 } else { ix + 5 };
                fill_rect(img, switch_x, iy + 4, 6, 10, [180, 180, 180, 255]);
                if props.current_state == 1 {
                    fill_rect(img, switch_x + 1, iy + 9, 4, 5, [220, 220, 220, 255]); // switch down (on)
                    let led_x = if props.flipped { ix + w as i32 - 8 } else { ix + 8 };
                    blend_pixel(img, led_x, iy + 2, [100, 255, 100, 200]); // green LED
                } else {
                    fill_rect(img, switch_x + 1, iy + 4, 4, 5, [220, 220, 220, 255]); // switch up (off)
                }
            }
            ElectricalType::SolarPanel => {
                // State 0=Dark, 1=Powered
                let panel_bright = if props.current_state == 1 { 220 } else { 80 };
                fill_rect(img, ix, iy, w as i32, h as i32, [40, panel_bright / 3, panel_bright, 255]);
                for gx in (0..w as i32).step_by(8) {
                    draw_line(img, ix + gx, iy, ix + gx, iy + h as i32, [60, 100, 200, 180]);
                }
                for gy in (0..h as i32).step_by(6) {
                    draw_line(img, ix, iy + gy, ix + w as i32, iy + gy, [60, 100, 200, 180]);
                }
                draw_line(img, ix, iy, ix + w as i32, iy, [120, 120, 130, 255]);
                draw_line(img, ix, iy + h as i32 - 1, ix + w as i32, iy + h as i32 - 1, [120, 120, 130, 255]);
                if props.current_state == 1 {
                    // Sun reflection
                    draw_glow(img, x + w / 2.0, y + h / 2.0, 8.0, [255, 255, 200]);
                }
            }
            ElectricalType::ElectricMotor => {
                // State 0=Off, 1=On
                // Flippable: clockwise vs counter-clockwise spin
                fill_rect(img, ix, iy, w as i32, h as i32, [ic[0], ic[1], ic[2], 200]);
                let cx = x + w / 2.0;
                let cy = y + h / 2.0;
                fill_circle(img, cx, cy, 8.0, [100, 100, 110, 255]);
                if props.current_state == 1 {
                    // Spinning axle — direction depends on flip
                    let spin_dir = if props.flipped { -1.0_f32 } else { 1.0 };
                    let a = frame as f32 * 0.2 * spin_dir;
                    draw_line(img, cx as i32, cy as i32, (cx + a.cos() * 7.0) as i32, (cy + a.sin() * 7.0) as i32, [60, 60, 70, 255]);
                    // Direction arrow indicator
                    let arrow_a = a + spin_dir * std::f32::consts::FRAC_PI_2;
                    let ax = cx + arrow_a.cos() * 6.0;
                    let ay = cy + arrow_a.sin() * 6.0;
                    blend_pixel(img, ax as i32, ay as i32, [200, 200, 255, 180]);
                    // Vibration
                    let vib = ((frame as f32 * 0.5).sin() * 1.0) as i32;
                    fill_rect(img, ix + vib, iy, 2, h as i32, [120, 120, 130, 120]);
                }
                fill_circle(img, cx, cy, 3.0, [60, 60, 70, 255]);
            }
            _ => {
                // CanOpener, ElectricMixer, LaserActivatedPlug
                // State 0=Off/Inactive, 1=On/Active
                let brightness = if props.current_state >= 1 { 255 } else { 150 };
                fill_rect(img, ix, iy, w as i32, h as i32, [ic[0] * brightness / 255, ic[1] * brightness / 255, ic[2] * brightness / 255, 200]);
                draw_line(img, ix, iy, ix + w as i32 - 1, iy, [ic[0].saturating_add(30), ic[1].saturating_add(30), ic[2].saturating_add(30), 255]);
                if props.current_state >= 1 {
                    // Active indicator
                    let spin_a = frame as f32 * 0.15;
                    let cx = x + w / 2.0;
                    let cy = y + h / 2.0;
                    blend_pixel(img, (cx + spin_a.cos() * 4.0) as i32, (cy + spin_a.sin() * 4.0) as i32, [255, 255, 200, 200]);
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
            ElectricalType::LaserActivatedPlug => vec![
                PropertyDef { name: "activation_color".into(), min: 0.0, max: 2.0, step: 1.0, default: 0.0, label: "Color (0=R/1=G/2=B)".into() },
            ],
            ElectricalType::Toaster => vec![
                PropertyDef { name: "delay".into(), min: 0.0, max: 2.0, step: 1.0, default: 1.0, label: "Delay (0=light/1=med/2=burnt)".into() },
            ],
            _ => vec![],
        }
    }

    fn has_animation(&self) -> bool {
        matches!(self, ElectricalType::Generator | ElectricalType::ElectricFan | ElectricalType::ElectricMotor | ElectricalType::ElectricMixer)
    }

    fn is_flippable(&self) -> bool {
        matches!(self, ElectricalType::ElectricFan | ElectricalType::ElectricMotor | ElectricalType::SwitchOutlet)
    }

    fn requires_power(&self) -> bool {
        matches!(self, ElectricalType::ElectricFan | ElectricalType::ElectricMotor | ElectricalType::Toaster | ElectricalType::CanOpener | ElectricalType::ElectricMixer)
    }

    fn provides_power(&self) -> bool {
        matches!(self, ElectricalType::Generator | ElectricalType::ElectricalOutlet | ElectricalType::SwitchOutlet | ElectricalType::SolarPanel | ElectricalType::LaserActivatedPlug)
    }

    fn can_be_ramp(&self) -> bool {
        matches!(self, ElectricalType::ElectricFan | ElectricalType::ElectricMotor)
    }
}
