pub mod balls;
pub mod walls;
pub mod inclines;
pub mod mechanical;
pub mod electrical;
pub mod pyrotechnic;
pub mod animals;
pub mod gadgets;
pub mod pipes;
pub mod lasers;
pub mod ropes;

use image::RgbaImage;
use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use std::collections::HashMap;

// ── Physics types ──────────────────────────────────────────────

/// How a part responds to gravity.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GravityResponse {
    /// Falls regardless of gravity slider (bowling ball, cannonball, pinball)
    AlwaysFalls,
    /// Standard gravity behavior
    Normal,
    /// Rises under normal/weak gravity, falls under strong
    Buoyant,
    /// Floats until struck, momentum-based (pool ball)
    ZeroGravity,
    /// User-defined gravity factor (programmable ball)
    Custom(f32),
}

/// Intrinsic physics properties for a part type (from spec).
#[derive(Debug, Clone, Copy)]
pub struct PhysicsProps {
    pub mass: f32,
    pub elasticity: f32,
    pub density: f32,
    pub friction: f32,
    pub gravity_response: GravityResponse,
    pub is_static: bool,
}

impl Default for PhysicsProps {
    fn default() -> Self {
        Self {
            mass: 1.0,
            elasticity: 0.5,
            density: 1.0,
            friction: 0.5,
            gravity_response: GravityResponse::Normal,
            is_static: true,
        }
    }
}

// ── State system ───────────────────────────────────────────────

/// Named state that a part can be in during simulation.
#[derive(Debug, Clone, PartialEq)]
pub struct StateDef {
    pub name: &'static str,
    pub description: &'static str,
}

// ── Runtime properties ─────────────────────────────────────────

/// Runtime-editable properties for a part instance.
#[derive(Debug, Clone)]
pub struct PartProps {
    pub flipped: bool,
    pub width: f32,
    pub height: f32,
    pub values: HashMap<String, f32>,
    /// Current state index (into the states vec from PartDef)
    pub current_state: usize,
}

impl Default for PartProps {
    fn default() -> Self {
        Self {
            flipped: false,
            width: 32.0,
            height: 32.0,
            values: HashMap::new(),
            current_state: 0,
        }
    }
}

/// Definition of an editable property.
#[derive(Debug, Clone)]
pub struct PropertyDef {
    pub name: String,
    pub min: f32,
    pub max: f32,
    pub step: f32,
    pub default: f32,
    pub label: String,
}

// ── Part definition trait ──────────────────────────────────────

/// Trait implemented by all part types for metadata + dual rendering.
pub trait PartDef {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;
    fn category(&self) -> &'static str;
    fn default_size(&self) -> (f32, f32);
    fn icon_char(&self) -> char;
    fn icon_color(&self) -> [u8; 3];

    /// Intrinsic physics properties from the TIM2 spec.
    fn physics(&self) -> PhysicsProps { PhysicsProps::default() }

    /// Possible states this part can be in during simulation.
    /// Every part MUST define at least one state. No default.
    fn states(&self) -> Vec<StateDef>;

    /// Draw into a pixel buffer (for pixel mode AND braille conversion).
    fn draw_pixel(&self, img: &mut RgbaImage, x: f32, y: f32, props: &PartProps, frame: u64);

    /// Draw into a ratatui buffer (used only if braille is disabled).
    fn draw_text(&self, buf: &mut Buffer, area: Rect, props: &PartProps, frame: u64);

    /// Editable property definitions.
    fn properties(&self) -> Vec<PropertyDef> { vec![] }
    fn has_animation(&self) -> bool { false }
    fn is_resizable(&self) -> bool { false }
    fn is_flippable(&self) -> bool { false }

    /// Whether this part is destructible by dynamite.
    fn destructible_by_dynamite(&self) -> bool { false }

    /// Whether this part can serve as a ramp surface.
    fn can_be_ramp(&self) -> bool { false }

    /// Whether rope can attach to this part.
    fn has_rope_point(&self) -> bool { false }

    /// Whether this part requires electrical power.
    fn requires_power(&self) -> bool { false }

    /// Whether this part provides electrical power.
    fn provides_power(&self) -> bool { false }
}

// ── Part ID enum ───────────────────────────────────────────────

/// Top-level part identifier spanning all categories.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PartId {
    Ball(balls::BallType),
    Wall(walls::WallType),
    Incline(inclines::InclineType),
    Mechanical(mechanical::MechanicalType),
    Electrical(electrical::ElectricalType),
    Pyrotechnic(pyrotechnic::PyrotechnicType),
    Animal(animals::AnimalType),
    Gadget(gadgets::GadgetType),
    Pipe(pipes::PipeType),
    Laser(lasers::LaserType),
    Rope(ropes::RopeType),
}

impl PartId {
    pub fn part_def(&self) -> &dyn PartDef {
        match self {
            PartId::Ball(t) => t,
            PartId::Wall(t) => t,
            PartId::Incline(t) => t,
            PartId::Mechanical(t) => t,
            PartId::Electrical(t) => t,
            PartId::Pyrotechnic(t) => t,
            PartId::Animal(t) => t,
            PartId::Gadget(t) => t,
            PartId::Pipe(t) => t,
            PartId::Laser(t) => t,
            PartId::Rope(t) => t,
        }
    }
}

/// A named category of parts.
pub struct Category {
    pub name: &'static str,
    pub parts: Vec<PartId>,
}

/// Return all parts grouped by category.
pub fn catalog() -> Vec<Category> {
    use balls::BallType::*;
    use walls::WallType::*;
    use inclines::InclineType::*;
    use mechanical::MechanicalType::*;
    use electrical::ElectricalType::*;
    use pyrotechnic::PyrotechnicType::*;
    use animals::AnimalType::*;
    use gadgets::GadgetType::*;
    use pipes::PipeType::*;
    use lasers::LaserType::*;
    use ropes::RopeType::*;

    vec![
        Category {
            name: "Balls",
            parts: vec![
                PartId::Ball(BowlingBall), PartId::Ball(Cannonball), PartId::Ball(Basketball),
                PartId::Ball(SoccerBall), PartId::Ball(Baseball), PartId::Ball(TennisBall),
                PartId::Ball(SuperBall), PartId::Ball(Pinball), PartId::Ball(PoolBall),
                PartId::Ball(ProgrammableBall),
            ],
        },
        Category {
            name: "Walls",
            parts: vec![
                PartId::Wall(BrickWall), PartId::Wall(YellowBrickWall), PartId::Wall(CinderBlockWall),
                PartId::Wall(GrecoRomanWall), PartId::Wall(WoodenWall), PartId::Wall(LogWall),
                PartId::Wall(CautionWall), PartId::Wall(SandWall), PartId::Wall(PipeWall),
                PartId::Wall(CurvedPipeWall), PartId::Wall(GrassFloor), PartId::Wall(ScaffoldBarrier),
                PartId::Wall(WoodenBarrier), PartId::Wall(LatticeArchway), PartId::Wall(MarbleArchway),
            ],
        },
        Category {
            name: "Inclines",
            parts: vec![
                PartId::Incline(BrickIncline), PartId::Incline(YellowBrickIncline),
                PartId::Incline(GraniteIncline),
            ],
        },
        Category {
            name: "Mechanical",
            parts: vec![
                PartId::Mechanical(Gear), PartId::Mechanical(Pulley), PartId::Mechanical(Belt),
                PartId::Mechanical(TeeterTotter), PartId::Mechanical(ConveyorBelt),
                PartId::Mechanical(Trampoline), PartId::Mechanical(JackInTheBox),
                PartId::Mechanical(Windmill), PartId::Mechanical(MandrillMotor),
                PartId::Mechanical(MouseExerciseWheel), PartId::Mechanical(TransRotoMatic),
                PartId::Mechanical(RotoTransConverter), PartId::Mechanical(TipsyTrailer),
                PartId::Mechanical(Scissors), PartId::Mechanical(HedgeTrimmers),
                PartId::Mechanical(TinSnips), PartId::Mechanical(BoxingGlove),
                PartId::Mechanical(VacuumCleaner), PartId::Mechanical(PinballBumper),
                PartId::Mechanical(Tack),
            ],
        },
        Category {
            name: "Electrical",
            parts: vec![
                PartId::Electrical(Generator), PartId::Electrical(ElectricalOutlet),
                PartId::Electrical(SwitchOutlet), PartId::Electrical(SolarPanel),
                PartId::Electrical(electrical::ElectricalType::LaserActivatedPlug), PartId::Electrical(ElectricFan),
                PartId::Electrical(ElectricMotor), PartId::Electrical(Toaster),
                PartId::Electrical(CanOpener), PartId::Electrical(ElectricMixer),
            ],
        },
        Category {
            name: "Pyrotechnic",
            parts: vec![
                PartId::Pyrotechnic(MagnifyingGlass), PartId::Pyrotechnic(Flashlight),
                PartId::Pyrotechnic(LavaLamp), PartId::Pyrotechnic(Candle),
                PartId::Pyrotechnic(Dynamite), PartId::Pyrotechnic(DynamitePlunger),
                PartId::Pyrotechnic(Cannon), PartId::Pyrotechnic(Rocket),
                PartId::Pyrotechnic(Fireworks), PartId::Pyrotechnic(RemoteControlBomb),
                PartId::Pyrotechnic(MatchOnSpring), PartId::Pyrotechnic(Fuse),
            ],
        },
        Category {
            name: "Animals",
            parts: vec![
                PartId::Animal(PokeyCat), PartId::Animal(MortMouse), PartId::Animal(Cheese),
                PartId::Animal(EdisonAlligator), PartId::Animal(MelSchlemming),
                PartId::Animal(MelsHouse), PartId::Animal(BillsFishTank),
                PartId::Animal(MouseHole), PartId::Animal(Leprechaun),
            ],
        },
        Category {
            name: "Gadgets",
            parts: vec![
                PartId::Gadget(SuperPhazer), PartId::Gadget(EggTimer), PartId::Gadget(EyeHook),
                PartId::Gadget(BoatCleat), PartId::Gadget(Gun), PartId::Gadget(AntiGravityPad),
                PartId::Gadget(SantaLamp), PartId::Gadget(LaundryBasket), PartId::Gadget(Bucket),
                PartId::Gadget(LeakyBucket), PartId::Gadget(Balloon), PartId::Gadget(HotAirBalloon),
            ],
        },
        Category {
            name: "Pipes",
            parts: vec![
                PartId::Pipe(StraightPipe), PartId::Pipe(TConnector),
                PartId::Pipe(CurvedPipe), PartId::Pipe(AcceleratorTube),
            ],
        },
        Category {
            name: "Lasers",
            parts: vec![
                PartId::Laser(RedLaser), PartId::Laser(GreenLaser), PartId::Laser(BlueLaser),
                PartId::Laser(AngledMirror), PartId::Laser(LaserMixer),
                PartId::Laser(LaserDetector), PartId::Laser(lasers::LaserType::LaserActivatedPlug),
            ],
        },
        Category {
            name: "Ropes",
            parts: vec![
                PartId::Rope(Rope), PartId::Rope(SteelCable),
            ],
        },
    ]
}
