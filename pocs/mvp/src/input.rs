use crossterm::event::{self, Event, KeyCode};
use log::info;
use std::time::Duration;
use crate::state::*;
use crate::parts::cannon;

pub fn handle_input(state: &mut GameState) -> anyhow::Result<bool> {
    // Non-blocking poll
    while event::poll(Duration::ZERO)? {
        if let Event::Key(key) = event::read()? {
            // Skip release events
            if key.kind != event::KeyEventKind::Press {
                continue;
            }

            // Handle won state input (overrides mode)
            if state.won {
                match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char(' ') => {
                        state.won = false;
                        state.mode = Mode::Normal;
                        state.ball = SimBall::new();
                        state.pop_undo();
                    }
                    _ => {}
                }
                continue;
            }

            match state.mode.clone() {
                Mode::Normal => match key.code {
                    KeyCode::Char('q') => return Ok(false),
                    KeyCode::Char('h') => state.cursor.0 = (state.cursor.0 - 4.0).max(0.0),
                    KeyCode::Char('l') => state.cursor.0 = (state.cursor.0 + 4.0).min(PLAYFIELD_W),
                    KeyCode::Char('j') => state.cursor.1 = (state.cursor.1 + 4.0).min(PLAYFIELD_H),
                    KeyCode::Char('k') => state.cursor.1 = (state.cursor.1 - 4.0).max(0.0),
                    KeyCode::Char('H') => state.cursor.0 = (state.cursor.0 - 16.0).max(0.0),
                    KeyCode::Char('L') => state.cursor.0 = (state.cursor.0 + 16.0).min(PLAYFIELD_W),
                    KeyCode::Char('J') => state.cursor.1 = (state.cursor.1 + 16.0).min(PLAYFIELD_H),
                    KeyCode::Char('K') => state.cursor.1 = (state.cursor.1 - 16.0).max(0.0),
                    KeyCode::Char('p') => {
                        if let Some(idx) = state.bin_items.iter().position(|b| b.count > 0) {
                            info!("Mode: NORMAL -> PLACE (bin_idx={})", idx);
                            state.mode = Mode::Place { bin_idx: idx };
                        }
                    }
                    KeyCode::Char('e') => {
                        if let Some(idx) = state.part_under_cursor() {
                            if !state.parts[idx].fixed {
                                info!("Mode: NORMAL -> EDIT (part_idx={}, kind={})", idx, state.parts[idx].kind.label());
                                state.push_undo();
                                state.mode = Mode::Edit { part_idx: idx };
                            }
                        }
                    }
                    KeyCode::Char(' ') => {
                        info!("Mode: NORMAL -> RUN");
                        state.push_undo();
                        state.mode = Mode::Run;
                        state.won = false;
                        state.ball = SimBall::new();
                        for part in &state.parts {
                            if let PartKind::Cannon { .. } = &part.kind {
                                state.ball = cannon::fire_cannon(part);
                                info!("Cannon fired: pos=({:.0},{:.0}) vel=({:.0},{:.0})",
                                    state.ball.pos.0, state.ball.pos.1,
                                    state.ball.vel.0, state.ball.vel.1);
                                break;
                            }
                        }
                    }
                    KeyCode::Char('x') => {
                        if let Some(idx) = state.part_under_cursor() {
                            if !state.parts[idx].fixed {
                                state.push_undo();
                                // Return to bin
                                let removed = state.parts.remove(idx);
                                for item in &mut state.bin_items {
                                    if std::mem::discriminant(&item.kind)
                                        == std::mem::discriminant(&removed.kind)
                                    {
                                        item.count += 1;
                                        break;
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('f') => {
                        if let Some(idx) = state.part_under_cursor() {
                            if !state.parts[idx].fixed {
                                state.push_undo();
                                state.parts[idx].flipped = !state.parts[idx].flipped;
                            }
                        }
                    }
                    KeyCode::Char('u') => state.pop_undo(),
                    KeyCode::Char('?') => state.show_help = !state.show_help,
                    _ => {}
                },
                Mode::Place { bin_idx } => match key.code {
                    KeyCode::Char('h') => state.cursor.0 = (state.cursor.0 - 4.0).max(0.0),
                    KeyCode::Char('l') => {
                        state.cursor.0 = (state.cursor.0 + 4.0).min(PLAYFIELD_W)
                    }
                    KeyCode::Char('j') => {
                        state.cursor.1 = (state.cursor.1 + 4.0).min(PLAYFIELD_H)
                    }
                    KeyCode::Char('k') => state.cursor.1 = (state.cursor.1 - 4.0).max(0.0),
                    KeyCode::Char('H') => state.cursor.0 = (state.cursor.0 - 16.0).max(0.0),
                    KeyCode::Char('L') => {
                        state.cursor.0 = (state.cursor.0 + 16.0).min(PLAYFIELD_W)
                    }
                    KeyCode::Char('J') => {
                        // Scroll bin down
                        let next = (bin_idx + 1) % state.bin_items.len();
                        state.mode = Mode::Place { bin_idx: next };
                    }
                    KeyCode::Char('K') => {
                        // Scroll bin up
                        let next = if bin_idx == 0 {
                            state.bin_items.len() - 1
                        } else {
                            bin_idx - 1
                        };
                        state.mode = Mode::Place { bin_idx: next };
                    }
                    KeyCode::Char(c @ '1'..='5') => {
                        let idx = (c as usize) - ('1' as usize);
                        if idx < state.bin_items.len() {
                            state.mode = Mode::Place { bin_idx: idx };
                        }
                    }
                    KeyCode::Enter => {
                        if bin_idx < state.bin_items.len() && state.bin_items[bin_idx].count > 0 {
                            let kind = state.bin_items[bin_idx].kind.clone();
                            let w = kind.width(false);
                            let h = kind.height(false);
                            let px = state.cursor.0 - w / 2.0;
                            let py = state.cursor.1 - h / 2.0;
                            let px = px.max(0.0).min(PLAYFIELD_W - w);
                            let py = py.max(0.0).min(PLAYFIELD_H - h);
                            if !state.parts_overlap_at(&kind, px, py, false, None) {
                                info!("Placed {} at ({:.0},{:.0})", kind.label(), px, py);
                                state.push_undo();
                                state.parts.push(Part {
                                    kind,
                                    x: px,
                                    y: py,
                                    flipped: false,
                                    fixed: false,
                                });
                                state.bin_items[bin_idx].count -= 1;
                                state.mode = Mode::Normal;
                            } else {
                                info!("Placement rejected (overlap) at ({:.0},{:.0})", px, py);
                            }
                        }
                    }
                    KeyCode::Esc => {
                        info!("Mode: PLACE -> NORMAL (cancelled)");
                        state.mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Edit { part_idx } => {
                    let idx = part_idx;
                    match key.code {
                        KeyCode::Char('h') => {
                            let new_x = (state.parts[idx].x - 4.0).max(0.0);
                            let kind = state.parts[idx].kind.clone();
                            let flipped = state.parts[idx].flipped;
                            let y = state.parts[idx].y;
                            if !state.parts_overlap_at(&kind, new_x, y, flipped, Some(idx)) {
                                state.parts[idx].x = new_x;
                            }
                        }
                        KeyCode::Char('l') => {
                            let flipped = state.parts[idx].flipped;
                            let w = state.parts[idx].kind.width(flipped);
                            let new_x = (state.parts[idx].x + 4.0).min(PLAYFIELD_W - w);
                            let kind = state.parts[idx].kind.clone();
                            let y = state.parts[idx].y;
                            if !state.parts_overlap_at(&kind, new_x, y, flipped, Some(idx)) {
                                state.parts[idx].x = new_x;
                            }
                        }
                        KeyCode::Char('j') => {
                            let flipped = state.parts[idx].flipped;
                            let h = state.parts[idx].kind.height(flipped);
                            let new_y = (state.parts[idx].y + 4.0).min(PLAYFIELD_H - h);
                            let kind = state.parts[idx].kind.clone();
                            let x = state.parts[idx].x;
                            if !state.parts_overlap_at(&kind, x, new_y, flipped, Some(idx)) {
                                state.parts[idx].y = new_y;
                            }
                        }
                        KeyCode::Char('k') => {
                            let new_y = (state.parts[idx].y - 4.0).max(0.0);
                            let kind = state.parts[idx].kind.clone();
                            let flipped = state.parts[idx].flipped;
                            let x = state.parts[idx].x;
                            if !state.parts_overlap_at(&kind, x, new_y, flipped, Some(idx)) {
                                state.parts[idx].y = new_y;
                            }
                        }
                        KeyCode::Char('H') => {
                            let new_x = (state.parts[idx].x - 16.0).max(0.0);
                            let kind = state.parts[idx].kind.clone();
                            let flipped = state.parts[idx].flipped;
                            let y = state.parts[idx].y;
                            if !state.parts_overlap_at(&kind, new_x, y, flipped, Some(idx)) {
                                state.parts[idx].x = new_x;
                            }
                        }
                        KeyCode::Char('L') => {
                            let flipped = state.parts[idx].flipped;
                            let w = state.parts[idx].kind.width(flipped);
                            let new_x = (state.parts[idx].x + 16.0).min(PLAYFIELD_W - w);
                            let kind = state.parts[idx].kind.clone();
                            let y = state.parts[idx].y;
                            if !state.parts_overlap_at(&kind, new_x, y, flipped, Some(idx)) {
                                state.parts[idx].x = new_x;
                            }
                        }
                        KeyCode::Char('J') => {
                            let flipped = state.parts[idx].flipped;
                            let h = state.parts[idx].kind.height(flipped);
                            let new_y = (state.parts[idx].y + 16.0).min(PLAYFIELD_H - h);
                            let kind = state.parts[idx].kind.clone();
                            let x = state.parts[idx].x;
                            if !state.parts_overlap_at(&kind, x, new_y, flipped, Some(idx)) {
                                state.parts[idx].y = new_y;
                            }
                        }
                        KeyCode::Char('K') => {
                            let new_y = (state.parts[idx].y - 16.0).max(0.0);
                            let kind = state.parts[idx].kind.clone();
                            let flipped = state.parts[idx].flipped;
                            let x = state.parts[idx].x;
                            if !state.parts_overlap_at(&kind, x, new_y, flipped, Some(idx)) {
                                state.parts[idx].y = new_y;
                            }
                        }
                        KeyCode::Char('f') => {
                            state.parts[idx].flipped = !state.parts[idx].flipped;
                        }
                        KeyCode::Char('x') => {
                            let removed = state.parts.remove(idx);
                            for item in &mut state.bin_items {
                                if std::mem::discriminant(&item.kind)
                                    == std::mem::discriminant(&removed.kind)
                                {
                                    item.count += 1;
                                    break;
                                }
                            }
                            state.mode = Mode::Normal;
                        }
                        KeyCode::Esc | KeyCode::Enter => state.mode = Mode::Normal,
                        _ => {}
                    }
                }
                Mode::Run => match key.code {
                    KeyCode::Esc | KeyCode::Char(' ') => {
                        info!("Mode: RUN -> NORMAL (stopped)");
                        state.mode = Mode::Normal;
                        state.ball = SimBall::new();
                        state.pop_undo();
                    }
                    KeyCode::Char('q') => return Ok(false),
                    _ => {}
                },
            }
        }
    }
    Ok(true)
}
