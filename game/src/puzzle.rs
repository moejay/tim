use crate::state::*;

pub fn load_mvp_puzzle() -> GameState {
    let parts = vec![
        Part {
            kind: PartKind::Cannon {
                angle_deg: -30.0,
                power: 600.0,
            },
            x: 32.0,
            y: 160.0,
            flipped: false,
            fixed: true,
        },
        Part {
            kind: PartKind::Basket,
            x: 384.0,
            y: 224.0,
            flipped: false,
            fixed: true,
        },
    ];

    let bin_items = vec![
        BinItem {
            kind: PartKind::Ramp,
            count: 1,
        },
        BinItem {
            kind: PartKind::Wall {
                width: 64.0,
                height: 32.0,
            },
            count: 2,
        },
    ];

    GameState {
        parts,
        ball: SimBall::default(),
        mode: Mode::Normal,
        cursor: (256.0, 160.0),
        undo_stack: Vec::new(),
        won: false,
        frame: 0,
        elapsed: 0.0,
        bin_items,
        show_help: false,
    }
}
