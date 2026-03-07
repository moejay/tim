use image::{Rgba, RgbaImage};

use crate::gfx;
use crate::parts::{ball, basket, cannon, ramp, wall};
use crate::state::*;

const BG_COLOR: Rgba<u8> = Rgba([10, 10, 14, 255]);
const BIN_BG: Rgba<u8> = Rgba([18, 18, 24, 255]);
const GRID_COLOR: Rgba<u8> = Rgba([30, 30, 40, 255]);
const CURSOR_CYAN: Rgba<u8> = Rgba([0, 220, 255, 200]);
const CURSOR_YELLOW: Rgba<u8> = Rgba([255, 220, 0, 200]);
const BIN_HIGHLIGHT: Rgba<u8> = Rgba([60, 60, 20, 180]);
const FIXED_MARKER: Rgba<u8> = Rgba([255, 255, 255, 40]);
const TEXT_COLOR: Rgba<u8> = Rgba([200, 200, 210, 255]);
const DIM_TEXT: Rgba<u8> = Rgba([100, 100, 120, 255]);

pub fn render(state: &GameState) -> RgbaImage {
    let mut img = RgbaImage::from_pixel(CANVAS_W, CANVAS_H, BG_COLOR);

    // Parts bin background
    gfx::fill_rect(&mut img, PLAYFIELD_W as i32, 0, CANVAS_W - PLAYFIELD_W, CANVAS_H, BIN_BG);
    for y in 0..CANVAS_H {
        gfx::blend_pixel(&mut img, PLAYFIELD_W as i32, y as i32, Rgba([50, 50, 60, 255]));
    }

    // Draw placed parts
    for part in &state.parts {
        draw_part(&mut img, part);
        if part.fixed {
            gfx::fill_rect(&mut img, part.x as i32, part.y as i32, 6, 6, FIXED_MARKER);
        }
    }

    // Edit mode highlight
    if let Mode::Edit { part_idx } = &state.mode {
        if let Some(part) = state.parts.get(*part_idx) {
            let (w, h) = part.size_px();
            let pulse = ((state.frame as f32 * 0.15).sin() * 0.3 + 0.7) as f32;
            let a = (200.0 * pulse) as u8;
            gfx::draw_rect_outline(
                &mut img,
                part.x as i32 - 1,
                part.y as i32 - 1,
                w as u32 + 2,
                h as u32 + 2,
                Rgba([80, 255, 80, a]),
            );
            gfx::draw_rect_outline(
                &mut img,
                part.x as i32 - 2,
                part.y as i32 - 2,
                w as u32 + 4,
                h as u32 + 4,
                Rgba([80, 255, 80, a / 2]),
            );
        }
    }

    // Cursor
    if !matches!(state.mode, Mode::Run | Mode::Edit { .. }) {
        draw_cursor(&mut img, state);
    }

    // Live ball
    if state.mode == Mode::Run {
        ball::draw_ball(&mut img, &state.ball);
    }

    draw_bin(&mut img, state);

    if state.won {
        draw_win_overlay(&mut img);
    }

    if state.show_help {
        draw_help_overlay(&mut img);
    }

    img
}

fn draw_part(img: &mut RgbaImage, part: &Part) {
    match &part.kind {
        PartKind::Ball => {
            let cx = part.x + 14.0;
            let cy = part.y + 14.0;
            gfx::fill_circle_shaded(img, cx, cy, ball::ball_radius(), [220, 60, 60]);
        }
        PartKind::Ramp => ramp::draw_ramp(img, part),
        PartKind::Wall { .. } => wall::draw_wall(img, part),
        PartKind::Basket => basket::draw_basket(img, part),
        PartKind::Cannon { .. } => cannon::draw_cannon(img, part),
    }
}

fn draw_cursor(img: &mut RgbaImage, state: &GameState) {
    let (cx, cy) = state.cursor;

    let color = match &state.mode {
        Mode::Place { .. } => CURSOR_YELLOW,
        _ => CURSOR_CYAN,
    };

    let pulse = ((state.frame as f32 * 0.1).sin() * 0.3 + 0.7) as f32;
    let a = (color.0[3] as f32 * pulse) as u8;
    let c = Rgba([color.0[0], color.0[1], color.0[2], a]);

    // Crosshair
    let arm = 12;
    let gap = 4;
    // Horizontal arms
    gfx::draw_line(img, cx - arm as f32, cy, cx - gap as f32, cy, 2.0, c);
    gfx::draw_line(img, cx + gap as f32, cy, cx + arm as f32, cy, 2.0, c);
    // Vertical arms
    gfx::draw_line(img, cx, cy - arm as f32, cx, cy - gap as f32, 2.0, c);
    gfx::draw_line(img, cx, cy + gap as f32, cx, cy + arm as f32, 2.0, c);

    // In PLACE mode, show ghost outline of selected part
    if let Mode::Place { bin_idx } = &state.mode {
        if let Some(item) = state.bin_items.get(*bin_idx) {
            let (w, h) = item.kind.size_px();
            let ghost = Rgba([color.0[0], color.0[1], color.0[2], 60]);
            gfx::draw_rect_outline(
                img,
                (cx - w / 2.0) as i32,
                (cy - h / 2.0) as i32,
                w as u32,
                h as u32,
                ghost,
            );
        }
    }
}

fn draw_bin(img: &mut RgbaImage, state: &GameState) {
    let bin_x = PLAYFIELD_W as i32;
    let slot_h = 60u32;

    for (i, item) in state.bin_items.iter().enumerate() {
        let slot_y = i as i32 * slot_h as i32;

        if let Mode::Place { bin_idx } = &state.mode {
            if *bin_idx == i {
                gfx::fill_rect(img, bin_x + 2, slot_y + 2, 124, slot_h - 4, BIN_HIGHLIGHT);
            }
        }

        gfx::fill_rect(
            img,
            bin_x,
            slot_y + slot_h as i32 - 1,
            CANVAS_W - PLAYFIELD_W,
            1,
            GRID_COLOR,
        );

        let icon_x = bin_x as f32 + 10.0;
        let icon_y = slot_y as f32 + 18.0;
        match &item.kind {
            PartKind::Ball => ball::draw_ball_icon(img, icon_x + 10.0, icon_y + 5.0),
            PartKind::Ramp => ramp::draw_ramp_icon(img, icon_x, icon_y),
            PartKind::Wall { .. } => wall::draw_wall_icon(img, icon_x, icon_y + 5.0),
            PartKind::Basket => basket::draw_basket_icon(img, icon_x, icon_y),
            PartKind::Cannon { .. } => cannon::draw_cannon_icon(img, icon_x, icon_y),
        }

        draw_small_text(img, bin_x + 55, slot_y + 10, item.kind.name(), TEXT_COLOR);
        let count_str = format!("x{}", item.count);
        draw_small_text(img, bin_x + 55, slot_y + 28, &count_str, DIM_TEXT);
        let key_str = format!("[{}]", i + 1);
        draw_small_text(img, bin_x + 100, slot_y + 42, &key_str, DIM_TEXT);
    }

    let instr_y = state.bin_items.len() as i32 * slot_h as i32 + 8;
    draw_small_text(img, bin_x + 8, instr_y, "p: place", DIM_TEXT);
    draw_small_text(img, bin_x + 8, instr_y + 14, "Space: run", DIM_TEXT);
    draw_small_text(img, bin_x + 8, instr_y + 28, "?: help", DIM_TEXT);
}

fn draw_win_overlay(img: &mut RgbaImage) {
    gfx::fill_rect(img, 100, 120, 312, 120, Rgba([0, 0, 0, 180]));
    gfx::draw_rect_outline(img, 100, 120, 312, 120, Rgba([80, 220, 80, 255]));
    gfx::draw_rect_outline(img, 101, 121, 310, 118, Rgba([80, 220, 80, 120]));

    draw_large_text(img, 160, 145, "PUZZLE SOLVED!", Rgba([80, 255, 80, 255]));
    draw_small_text(img, 140, 190, "[Space] Try Again    [q] Quit", TEXT_COLOR);
}

fn draw_help_overlay(img: &mut RgbaImage) {
    gfx::fill_rect(img, 40, 30, 432, 300, Rgba([0, 0, 0, 210]));
    gfx::draw_rect_outline(img, 40, 30, 432, 300, Rgba([100, 100, 140, 255]));

    let x = 60;
    let mut y = 50;
    let lh = 18;
    draw_large_text(img, x, y, "CONTROLS", Rgba([200, 200, 255, 255]));
    y += 30;
    let lines = [
        "NORMAL MODE:",
        "  h/j/k/l    Move cursor",
        "  H/J/K/L    Move cursor fast",
        "  p          Enter PLACE mode",
        "  e          Edit part under cursor",
        "  Space      Start simulation",
        "  x          Delete part",
        "  f          Flip part",
        "  u          Undo",
        "  ?/q        Help / Quit",
        "",
        "EDIT MODE:",
        "  h/j/k/l    Move part",
        "  H/J/K/L    Move part fast",
        "  f          Flip part",
        "  x          Delete part",
        "  Esc/Enter  Done editing",
    ];
    for line in &lines {
        draw_small_text(img, x, y, line, TEXT_COLOR);
        y += lh;
    }
}

fn draw_small_text(img: &mut RgbaImage, x: i32, y: i32, text: &str, color: Rgba<u8>) {
    let mut cx = x;
    for ch in text.chars() {
        draw_char(img, cx, y, ch, color, 1);
        cx += 6;
    }
}

fn draw_large_text(img: &mut RgbaImage, x: i32, y: i32, text: &str, color: Rgba<u8>) {
    let mut cx = x;
    for ch in text.chars() {
        draw_char(img, cx, y, ch, color, 2);
        cx += 12;
    }
}

fn draw_char(img: &mut RgbaImage, x: i32, y: i32, ch: char, color: Rgba<u8>, scale: i32) {
    let bitmap = char_bitmap(ch);
    for (row, bits) in bitmap.iter().enumerate() {
        for col in 0..5 {
            if bits & (1 << (4 - col)) != 0 {
                for sy in 0..scale {
                    for sx in 0..scale {
                        gfx::blend_pixel(
                            img,
                            x + col * scale + sx,
                            y + row as i32 * scale + sy,
                            color,
                        );
                    }
                }
            }
        }
    }
}

fn char_bitmap(ch: char) -> [u8; 7] {
    match ch.to_ascii_uppercase() {
        'A' => [0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'B' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110],
        'C' => [0b01110, 0b10001, 0b10000, 0b10000, 0b10000, 0b10001, 0b01110],
        'D' => [0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110],
        'E' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111],
        'F' => [0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000],
        'G' => [0b01110, 0b10001, 0b10000, 0b10111, 0b10001, 0b10001, 0b01110],
        'H' => [0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001],
        'I' => [0b01110, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        'J' => [0b00111, 0b00010, 0b00010, 0b00010, 0b00010, 0b10010, 0b01100],
        'K' => [0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001],
        'L' => [0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111],
        'M' => [0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001],
        'N' => [0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001],
        'O' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'P' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000],
        'Q' => [0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101],
        'R' => [0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001],
        'S' => [0b01110, 0b10001, 0b10000, 0b01110, 0b00001, 0b10001, 0b01110],
        'T' => [0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        'U' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110],
        'V' => [0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100],
        'W' => [0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b11011, 0b10001],
        'X' => [0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001],
        'Y' => [0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100],
        'Z' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111],
        '0' => [0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110],
        '1' => [0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110],
        '2' => [0b01110, 0b10001, 0b00001, 0b00110, 0b01000, 0b10000, 0b11111],
        '3' => [0b01110, 0b10001, 0b00001, 0b00110, 0b00001, 0b10001, 0b01110],
        '4' => [0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010],
        '5' => [0b11111, 0b10000, 0b11110, 0b00001, 0b00001, 0b10001, 0b01110],
        '6' => [0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110],
        '7' => [0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000],
        '8' => [0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110],
        '9' => [0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110],
        ':' => [0b00000, 0b00100, 0b00100, 0b00000, 0b00100, 0b00100, 0b00000],
        '(' => [0b00010, 0b00100, 0b01000, 0b01000, 0b01000, 0b00100, 0b00010],
        ')' => [0b01000, 0b00100, 0b00010, 0b00010, 0b00010, 0b00100, 0b01000],
        '[' => [0b01110, 0b01000, 0b01000, 0b01000, 0b01000, 0b01000, 0b01110],
        ']' => [0b01110, 0b00010, 0b00010, 0b00010, 0b00010, 0b00010, 0b01110],
        '/' => [0b00001, 0b00010, 0b00010, 0b00100, 0b01000, 0b01000, 0b10000],
        '|' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100],
        '?' => [0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b00000, 0b00100],
        '!' => [0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00000, 0b00100],
        '.' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100],
        ',' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00100, 0b01000],
        '-' => [0b00000, 0b00000, 0b00000, 0b11111, 0b00000, 0b00000, 0b00000],
        '+' => [0b00000, 0b00100, 0b00100, 0b11111, 0b00100, 0b00100, 0b00000],
        ' ' => [0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000, 0b00000],
        _ => [0b11111, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11111],
    }
}
