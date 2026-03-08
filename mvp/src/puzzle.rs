use crate::state::*;

pub fn load_mvp_puzzle() -> (Vec<Part>, Vec<BinItem>) {
    let parts = vec![
        Part {
            kind: PartKind::Cannon { angle_deg: -30.0, power: 600.0 },
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
        BinItem { kind: PartKind::Ramp, count: 1 },
        BinItem { kind: PartKind::Wall { width: 64.0, height: 32.0 }, count: 2 },
    ];
    (parts, bin_items)
}
