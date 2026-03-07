mod gfx;
mod hud;
mod parts;
mod physics;
mod puzzle;
mod render;
mod state;

use std::io::{self, Write};
use std::time::{Duration, Instant};

use crossterm::{cursor, event, execute, terminal};

use state::*;

fn main() -> io::Result<()> {
    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let protocol = detect_protocol();
    if protocol == "half-block" {
        execute!(stdout, cursor::MoveTo(0, 0))?;
        execute!(
            stdout,
            crossterm::style::Print(
                "WARNING: Your terminal does not support Kitty/Sixel/iTerm2 graphics.\n\r\
                 Rendering will use Unicode half-block fallback (lower fidelity).\n\r\
                 For the best experience, use WezTerm, kitty, or Ghostty.\n\r\
                 \n\r\
                 Press any key to continue, or 'q' to quit.\n\r"
            )
        )?;
        stdout.flush()?;
        if let Ok(event::Event::Key(key)) = event::read() {
            if key.code == event::KeyCode::Char('q') {
                cleanup()?;
                return Ok(());
            }
        }
    }

    let mut state = puzzle::load_mvp_puzzle();
    let frame_dur = Duration::from_secs_f64(1.0 / 60.0);
    let mut last_frame = Instant::now();

    loop {
        let frame_start = Instant::now();
        let dt = last_frame.elapsed().as_secs_f32().min(0.05);
        last_frame = frame_start;

        if !handle_input(&mut state)? {
            break;
        }

        if state.mode == Mode::Run && !state.won {
            physics::update_physics(&mut state, dt);
        }

        state.frame += 1;
        state.elapsed += dt;

        let img = render::render(&state);

        let (term_w, term_h) = terminal::size()?;
        execute!(stdout, cursor::MoveTo(0, 0))?;
        let conf = viuer::Config {
            x: 0,
            y: 0,
            width: Some(term_w as u32),
            height: Some(term_h.saturating_sub(2) as u32),
            ..Default::default()
        };
        viuer::print(&image::DynamicImage::ImageRgba8(img), &conf)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        hud::write_hud(&state)?;

        let elapsed = frame_start.elapsed();
        if elapsed < frame_dur {
            std::thread::sleep(frame_dur - elapsed);
        }
    }

    cleanup()?;
    Ok(())
}

fn move_cursor(state: &mut GameState, dx: f32, dy: f32) {
    let max_x = PLAYFIELD_W as f32 - 1.0;
    let max_y = CANVAS_H as f32 - 1.0;
    state.cursor.0 = (state.cursor.0 + dx).clamp(0.0, max_x);
    state.cursor.1 = (state.cursor.1 + dy).clamp(0.0, max_y);
}

fn handle_input(state: &mut GameState) -> io::Result<bool> {
    while event::poll(Duration::ZERO)? {
        if let event::Event::Key(key) = event::read()? {
            use event::KeyCode;

            let slow = CURSOR_STEP;
            let fast = CURSOR_STEP_FAST;

            match state.mode.clone() {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char('h') => move_cursor(state, -slow, 0.0),
                    KeyCode::Char('l') => move_cursor(state, slow, 0.0),
                    KeyCode::Char('j') => move_cursor(state, 0.0, slow),
                    KeyCode::Char('k') => move_cursor(state, 0.0, -slow),
                    KeyCode::Char('H') => move_cursor(state, -fast, 0.0),
                    KeyCode::Char('L') => move_cursor(state, fast, 0.0),
                    KeyCode::Char('J') => move_cursor(state, 0.0, fast),
                    KeyCode::Char('K') => move_cursor(state, 0.0, -fast),
                    KeyCode::Char('p') => {
                        if !state.bin_items.is_empty() {
                            state.mode = Mode::Place { bin_idx: 0 };
                        }
                    }
                    KeyCode::Char('e') => {
                        enter_edit_mode(state);
                    }
                    KeyCode::Char(' ') => {
                        if state.won {
                            *state = puzzle::load_mvp_puzzle();
                        } else {
                            start_simulation(state);
                        }
                    }
                    KeyCode::Char('x') => {
                        delete_part_at_cursor(state);
                    }
                    KeyCode::Char('f') => {
                        flip_part_at_cursor(state);
                    }
                    KeyCode::Char('u') => {
                        state.pop_undo();
                    }
                    KeyCode::Char('?') => {
                        state.show_help = !state.show_help;
                    }
                    _ => {}
                },
                Mode::Place { bin_idx } => match key.code {
                    KeyCode::Esc => {
                        state.mode = Mode::Normal;
                    }
                    KeyCode::Char('h') => move_cursor(state, -slow, 0.0),
                    KeyCode::Char('l') => move_cursor(state, slow, 0.0),
                    KeyCode::Char('j') => move_cursor(state, 0.0, slow),
                    KeyCode::Char('k') => move_cursor(state, 0.0, -slow),
                    KeyCode::Char('H') => move_cursor(state, -fast, 0.0),
                    KeyCode::Char('L') => move_cursor(state, fast, 0.0),
                    // J/K are bin scroll in PLACE mode
                    KeyCode::Char('J') => {
                        let new_idx = (bin_idx + 1).min(state.bin_items.len() - 1);
                        state.mode = Mode::Place { bin_idx: new_idx };
                    }
                    KeyCode::Char('K') => {
                        let new_idx = bin_idx.saturating_sub(1);
                        state.mode = Mode::Place { bin_idx: new_idx };
                    }
                    KeyCode::Char(c @ '1'..='5') => {
                        let idx = (c as usize) - ('1' as usize);
                        if idx < state.bin_items.len() {
                            state.mode = Mode::Place { bin_idx: idx };
                        }
                    }
                    KeyCode::Enter => {
                        place_part(state, bin_idx);
                    }
                    _ => {}
                },
                Mode::Edit { part_idx } => match key.code {
                    KeyCode::Esc | KeyCode::Enter => {
                        state.mode = Mode::Normal;
                    }
                    KeyCode::Char('h') => move_edited_part(state, part_idx, -slow, 0.0),
                    KeyCode::Char('l') => move_edited_part(state, part_idx, slow, 0.0),
                    KeyCode::Char('j') => move_edited_part(state, part_idx, 0.0, slow),
                    KeyCode::Char('k') => move_edited_part(state, part_idx, 0.0, -slow),
                    KeyCode::Char('H') => move_edited_part(state, part_idx, -fast, 0.0),
                    KeyCode::Char('L') => move_edited_part(state, part_idx, fast, 0.0),
                    KeyCode::Char('J') => move_edited_part(state, part_idx, 0.0, fast),
                    KeyCode::Char('K') => move_edited_part(state, part_idx, 0.0, -fast),
                    KeyCode::Char('f') => {
                        match &mut state.parts[part_idx].kind {
                            PartKind::Wall {
                                ref mut width,
                                ref mut height,
                            } => std::mem::swap(width, height),
                            _ => state.parts[part_idx].flipped = !state.parts[part_idx].flipped,
                        }
                    }
                    KeyCode::Char('x') => {
                        let removed = state.parts.remove(part_idx);
                        if let Some(bin_item) = state.bin_items.iter_mut().find(|b| {
                            std::mem::discriminant(&b.kind)
                                == std::mem::discriminant(&removed.kind)
                        }) {
                            bin_item.count += 1;
                        } else {
                            state.bin_items.push(BinItem {
                                kind: removed.kind,
                                count: 1,
                            });
                        }
                        state.mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Run => match key.code {
                    KeyCode::Esc | KeyCode::Char(' ') => {
                        stop_simulation(state);
                    }
                    _ => {}
                },
            }
        }
    }
    Ok(true)
}

fn enter_edit_mode(state: &mut GameState) {
    let (cx, cy) = state.cursor;
    if let Some(idx) = state
        .parts
        .iter()
        .position(|p| !p.fixed && p.contains(cx, cy))
    {
        state.push_undo();
        state.mode = Mode::Edit { part_idx: idx };
    }
}

fn move_edited_part(state: &mut GameState, part_idx: usize, dx: f32, dy: f32) {
    let part = &mut state.parts[part_idx];
    let new_x = (part.x + dx).clamp(0.0, PLAYFIELD_W as f32 - part.size_px().0);
    let new_y = (part.y + dy).clamp(0.0, CANVAS_H as f32 - part.size_px().1);

    let old_x = part.x;
    let old_y = part.y;
    part.x = new_x;
    part.y = new_y;

    // Check overlap with other parts
    let overlaps = has_overlap_at(state, part_idx);
    if overlaps {
        state.parts[part_idx].x = old_x;
        state.parts[part_idx].y = old_y;
    }
}

fn has_overlap_at(state: &GameState, check_idx: usize) -> bool {
    let part = &state.parts[check_idx];
    for (i, other) in state.parts.iter().enumerate() {
        if i != check_idx && part.overlaps(other) {
            return true;
        }
    }
    false
}

fn start_simulation(state: &mut GameState) {
    state.mode = Mode::Run;
    state.won = false;
    state.ball = SimBall::default();

    for part in &state.parts {
        if let PartKind::Cannon { .. } = &part.kind {
            state.ball = parts::cannon::spawn_ball(part);
            break;
        }
    }

    if !state.ball.active {
        for part in &state.parts {
            if let PartKind::Ball = &part.kind {
                state.ball = SimBall {
                    pos: (part.x + 14.0, part.y + 14.0),
                    vel: (0.0, 0.0),
                    active: true,
                };
                break;
            }
        }
    }
}

fn stop_simulation(state: &mut GameState) {
    state.mode = Mode::Normal;
    state.ball = SimBall::default();
}

fn place_part(state: &mut GameState, bin_idx: usize) {
    if bin_idx >= state.bin_items.len() {
        return;
    }
    if state.bin_items[bin_idx].count == 0 {
        state.mode = Mode::Normal;
        return;
    }

    let item = &state.bin_items[bin_idx];
    let (w, h) = item.kind.size_px();
    let candidate = Part {
        kind: item.kind.clone(),
        x: state.cursor.0 - w / 2.0,
        y: state.cursor.1 - h / 2.0,
        flipped: false,
        fixed: false,
    };

    // Check overlap
    for existing in &state.parts {
        if candidate.overlaps(existing) {
            return; // reject placement silently
        }
    }

    state.push_undo();
    state.parts.push(candidate);
    state.bin_items[bin_idx].count -= 1;

    if state.bin_items[bin_idx].count == 0 {
        state.bin_items.remove(bin_idx);
    }

    state.mode = Mode::Normal;
}

fn delete_part_at_cursor(state: &mut GameState) {
    let (cx, cy) = state.cursor;
    if let Some(idx) = state
        .parts
        .iter()
        .position(|p| !p.fixed && p.contains(cx, cy))
    {
        state.push_undo();
        let removed = state.parts.remove(idx);

        if let Some(bin_item) = state
            .bin_items
            .iter_mut()
            .find(|b| std::mem::discriminant(&b.kind) == std::mem::discriminant(&removed.kind))
        {
            bin_item.count += 1;
        } else {
            state.bin_items.push(BinItem {
                kind: removed.kind,
                count: 1,
            });
        }
    }
}

fn flip_part_at_cursor(state: &mut GameState) {
    let (cx, cy) = state.cursor;
    let idx = state
        .parts
        .iter()
        .position(|p| !p.fixed && p.contains(cx, cy));
    if let Some(idx) = idx {
        state.push_undo();
        match &mut state.parts[idx].kind {
            PartKind::Wall {
                ref mut width,
                ref mut height,
            } => {
                std::mem::swap(width, height);
            }
            _ => {
                state.parts[idx].flipped = !state.parts[idx].flipped;
            }
        }
    }
}

fn detect_protocol() -> &'static str {
    if std::env::var("TERM_PROGRAM").as_deref() == Ok("WezTerm")
        || std::env::var("TERM").as_deref() == Ok("xterm-kitty")
        || std::env::var("KITTY_WINDOW_ID").is_ok()
    {
        return "kitty";
    }
    if std::env::var("TERM_PROGRAM").as_deref() == Ok("iTerm.app") {
        return "iterm2";
    }
    "half-block"
}

fn cleanup() -> io::Result<()> {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}
