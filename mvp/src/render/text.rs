use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    buffer::Buffer,
    Terminal,
};
use std::io::Stdout;
use crate::state::*;
use crate::parts::{ball, ramp, wall, basket, cannon};
use crate::hud;
use super::Renderer;

pub struct TextRenderer {
    terminal: Terminal<CrosstermBackend<Stdout>>,
}

fn part_color(kind: &PartKind) -> Color {
    match kind {
        PartKind::Ball => Color::Rgb(255, 107, 53),
        PartKind::Ramp => Color::Rgb(196, 149, 106),
        PartKind::Wall { .. } => Color::Rgb(90, 122, 138),
        PartKind::Basket => Color::Rgb(212, 160, 74),
        PartKind::Cannon { .. } => Color::Rgb(100, 100, 110),
    }
}

fn mode_color(mode: &Mode) -> Color {
    match mode {
        Mode::Normal => Color::Cyan,
        Mode::Place { .. } => Color::Yellow,
        Mode::Edit { .. } => Color::Green,
        Mode::Run => Color::Red,
    }
}

impl TextRenderer {
    pub fn new() -> Result<Self> {
        let backend = CrosstermBackend::new(std::io::stdout());
        let terminal = Terminal::new(backend)?;
        Ok(Self { terminal })
    }
}

impl Renderer for TextRenderer {
    fn render_frame(&mut self, state: &GameState) -> Result<()> {
        self.terminal.draw(|frame| {
            let size = frame.area();

            // Outer vertical split: game area (Min) | HUD (Length 3)
            let outer = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(10), Constraint::Length(3)])
                .split(size);

            // Inner horizontal split: playfield (80%) | bin (20%)
            let inner = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
                .split(outer[0]);

            let playfield_area = inner[0];
            let bin_area = inner[1];
            let hud_area = outer[1];

            // --- Playfield ---
            let pf_block = Block::default()
                .borders(Borders::ALL)
                .title(" TIM2 ")
                .border_style(Style::default().fg(Color::DarkGray));
            let pf_inner = pf_block.inner(playfield_area);
            frame.render_widget(pf_block, playfield_area);

            if pf_inner.width > 0 && pf_inner.height > 0 {
                let scale_x = pf_inner.width as f32 / PLAYFIELD_W;
                let scale_y = pf_inner.height as f32 / PLAYFIELD_H;
                let buf = frame.buffer_mut();

                // Draw parts
                for (i, part) in state.parts.iter().enumerate() {
                    let cell_x = pf_inner.x + (part.x * scale_x) as u16;
                    let cell_y = pf_inner.y + (part.y * scale_y) as u16;
                    let cell_w = (part.kind.width(part.flipped) * scale_x).max(1.0) as u16;
                    let cell_h = (part.kind.height(part.flipped) * scale_y).max(1.0) as u16;

                    let is_edited = matches!(&state.mode, Mode::Edit { part_idx } if *part_idx == i);

                    match &part.kind {
                        PartKind::Ball => {
                            ball::draw_ball_text(buf, cell_x, cell_y, pf_inner);
                        }
                        PartKind::Ramp => {
                            ramp::draw_ramp_text(
                                buf, cell_x, cell_y, cell_w, cell_h, part.flipped, pf_inner,
                            );
                        }
                        PartKind::Wall { .. } => {
                            wall::draw_wall_text(buf, cell_x, cell_y, cell_w, cell_h, pf_inner);
                        }
                        PartKind::Basket => {
                            basket::draw_basket_text(
                                buf, cell_x, cell_y, cell_w, cell_h, state.frame, pf_inner,
                            );
                        }
                        PartKind::Cannon { angle_deg, .. } => {
                            cannon::draw_cannon_text(
                                buf, cell_x, cell_y, cell_w, cell_h, *angle_deg, part.flipped,
                                pf_inner,
                            );
                        }
                    }

                    // Apply dim modifier to fixed parts
                    if part.fixed {
                        apply_modifier_to_region(buf, cell_x, cell_y, cell_w, cell_h, pf_inner, Modifier::DIM);
                    }

                    // Green background tint for edited part
                    if is_edited {
                        apply_bg_to_region(buf, cell_x, cell_y, cell_w, cell_h, pf_inner, Color::Rgb(0, 60, 0));
                    }
                }

                // PLACE mode: draw ghost of selected part at cursor
                if let Mode::Place { bin_idx } = &state.mode {
                    if *bin_idx < state.bin_items.len() {
                        let kind = &state.bin_items[*bin_idx].kind;
                        let cursor_cx = pf_inner.x + (state.cursor.0 * scale_x) as u16;
                        let cursor_cy = pf_inner.y + (state.cursor.1 * scale_y) as u16;
                        let ghost_w = (kind.width(false) * scale_x).max(1.0) as u16;
                        let ghost_h = (kind.height(false) * scale_y).max(1.0) as u16;

                        // Draw ghost part with DIM style
                        match kind {
                            PartKind::Ball => {
                                ball::draw_ball_text(buf, cursor_cx, cursor_cy, pf_inner);
                            }
                            PartKind::Ramp => {
                                ramp::draw_ramp_text(buf, cursor_cx, cursor_cy, ghost_w, ghost_h, false, pf_inner);
                            }
                            PartKind::Wall { .. } => {
                                wall::draw_wall_text(buf, cursor_cx, cursor_cy, ghost_w, ghost_h, pf_inner);
                            }
                            PartKind::Basket => {
                                basket::draw_basket_text(buf, cursor_cx, cursor_cy, ghost_w, ghost_h, state.frame, pf_inner);
                            }
                            PartKind::Cannon { angle_deg, .. } => {
                                cannon::draw_cannon_text(buf, cursor_cx, cursor_cy, ghost_w, ghost_h, *angle_deg, false, pf_inner);
                            }
                        }
                        apply_modifier_to_region(buf, cursor_cx, cursor_cy, ghost_w, ghost_h, pf_inner, Modifier::DIM);
                    }
                }

                // Draw cursor (not in RUN mode)
                if state.mode != Mode::Run {
                    let cursor_cx = pf_inner.x + (state.cursor.0 * scale_x) as u16;
                    let cursor_cy = pf_inner.y + (state.cursor.1 * scale_y) as u16;
                    if cursor_cx >= pf_inner.x
                        && cursor_cx < pf_inner.x + pf_inner.width
                        && cursor_cy >= pf_inner.y
                        && cursor_cy < pf_inner.y + pf_inner.height
                    {
                        let cursor_color = match &state.mode {
                            Mode::Normal => Color::Cyan,
                            Mode::Place { .. } => Color::Yellow,
                            Mode::Edit { .. } => Color::Green,
                            _ => Color::Cyan,
                        };
                        buf[(cursor_cx, cursor_cy)]
                            .set_char('\u{253C}') // ┼
                            .set_style(
                                Style::default()
                                    .fg(cursor_color)
                                    .add_modifier(Modifier::BOLD),
                            );
                    }
                }

                // Draw ball and trail in RUN mode
                if state.mode == Mode::Run && state.ball.active {
                    // Convert trail positions to cell coords
                    let trail_cells: Vec<(u16, u16)> = state
                        .ball
                        .trail
                        .iter()
                        .map(|&(tx, ty)| {
                            (
                                pf_inner.x + (tx * scale_x) as u16,
                                pf_inner.y + (ty * scale_y) as u16,
                            )
                        })
                        .collect();
                    ball::draw_trail_text(buf, &trail_cells, pf_inner);

                    // Draw ball
                    let ball_cx = pf_inner.x + (state.ball.pos.0 * scale_x) as u16;
                    let ball_cy = pf_inner.y + (state.ball.pos.1 * scale_y) as u16;
                    ball::draw_ball_text(buf, ball_cx, ball_cy, pf_inner);
                }
            }

            // --- Won overlay ---
            if state.won {
                let popup_w: u16 = 25;
                let popup_h: u16 = 6;
                let popup_x = playfield_area.x + (playfield_area.width.saturating_sub(popup_w)) / 2;
                let popup_y = playfield_area.y + (playfield_area.height.saturating_sub(popup_h)) / 2;
                let popup_area = Rect::new(popup_x, popup_y, popup_w, popup_h);

                frame.render_widget(Clear, popup_area);

                let popup_block = Block::default()
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Green));
                let popup_inner = popup_block.inner(popup_area);
                frame.render_widget(popup_block, popup_area);

                let won_text = Paragraph::new(vec![
                    Line::from(Span::styled(
                        "   PUZZLE SOLVED!",
                        Style::default()
                            .fg(Color::Green)
                            .add_modifier(Modifier::BOLD),
                    )),
                    Line::from(""),
                    Line::from(" [Space] Try Again"),
                    Line::from(" [q] Quit"),
                ]);
                frame.render_widget(won_text, popup_inner);
            }

            // --- Help overlay ---
            if state.show_help {
                let help_w: u16 = 40;
                let help_h: u16 = 16;
                let help_x = playfield_area.x + (playfield_area.width.saturating_sub(help_w)) / 2;
                let help_y = playfield_area.y + (playfield_area.height.saturating_sub(help_h)) / 2;
                let help_area = Rect::new(help_x, help_y, help_w, help_h);

                frame.render_widget(Clear, help_area);

                let help_block = Block::default()
                    .borders(Borders::ALL)
                    .title(" Help ")
                    .border_style(Style::default().fg(Color::Cyan));
                let help_inner = help_block.inner(help_area);
                frame.render_widget(help_block, help_area);

                let help_text = Paragraph::new(vec![
                    Line::from(Span::styled("  Keybindings", Style::default().add_modifier(Modifier::BOLD))),
                    Line::from(""),
                    Line::from("  hjkl    Move cursor (small)"),
                    Line::from("  HJKL    Move cursor (large)"),
                    Line::from("  p       Place mode"),
                    Line::from("  e       Edit part under cursor"),
                    Line::from("  x       Delete part"),
                    Line::from("  f       Flip part"),
                    Line::from("  u       Undo"),
                    Line::from("  Space   Run simulation"),
                    Line::from("  Esc     Cancel / stop"),
                    Line::from("  ?       Toggle help"),
                    Line::from("  q       Quit"),
                ]);
                frame.render_widget(help_text, help_inner);
            }

            // --- Parts Bin ---
            let bin_block = Block::default()
                .borders(Borders::ALL)
                .title(" Parts ")
                .border_style(Style::default().fg(Color::DarkGray));
            let bin_inner = bin_block.inner(bin_area);
            frame.render_widget(bin_block, bin_area);

            let selected_bin_idx = if let Mode::Place { bin_idx } = &state.mode {
                Some(*bin_idx)
            } else {
                None
            };

            let items: Vec<ListItem> = state
                .bin_items
                .iter()
                .enumerate()
                .map(|(i, item)| {
                    let icon = format!(" {} ", item.kind.icon_char());
                    let label = format!("{:<10}", item.kind.label());
                    let count = format!("\u{00d7}{}", item.count);

                    let is_selected = selected_bin_idx == Some(i);
                    let is_empty = item.count == 0;

                    let mut icon_style = Style::default().fg(part_color(&item.kind));
                    let mut label_style = Style::default().fg(Color::White);
                    let mut count_style = Style::default().fg(Color::DarkGray);

                    if is_empty {
                        icon_style = icon_style.add_modifier(Modifier::DIM);
                        label_style = label_style.add_modifier(Modifier::DIM);
                        count_style = count_style.add_modifier(Modifier::DIM);
                    }

                    if is_selected {
                        icon_style = icon_style.add_modifier(Modifier::REVERSED);
                        label_style = label_style.add_modifier(Modifier::REVERSED);
                        count_style = count_style.add_modifier(Modifier::REVERSED);
                    }

                    ListItem::new(Line::from(vec![
                        Span::styled(icon, icon_style),
                        Span::styled(label, label_style),
                        Span::styled(count, count_style),
                    ]))
                })
                .collect();

            let bin_list = List::new(items);
            frame.render_widget(bin_list, bin_inner);

            // --- HUD ---
            let hud_block = Block::default()
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::DarkGray));
            let hud_inner = hud_block.inner(hud_area);
            frame.render_widget(hud_block, hud_area);

            let line1 = hud::hud_line1(state);
            let line2 = hud::hud_line2(state);

            // Color the mode word in line1
            let mc = mode_color(&state.mode);
            let mode_word = match &state.mode {
                Mode::Normal => "NORMAL",
                Mode::Place { .. } => "PLACE",
                Mode::Edit { .. } => "EDIT",
                Mode::Run => "RUN",
            };
            let rest = &line1["MODE: ".len() + mode_word.len()..];
            let hud_paragraph = Paragraph::new(vec![
                Line::from(vec![
                    Span::raw("MODE: "),
                    Span::styled(mode_word, Style::default().fg(mc).add_modifier(Modifier::BOLD)),
                    Span::raw(rest.to_string()),
                ]),
                Line::from(Span::styled(line2, Style::default().fg(Color::DarkGray))),
            ]);
            frame.render_widget(hud_paragraph, hud_inner);
        })?;

        Ok(())
    }

    fn cleanup(&mut self) -> Result<()> {
        Ok(())
    }
}

/// Apply a modifier to all cells in a rectangular region, clipped to the given area.
fn apply_modifier_to_region(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    area: Rect,
    modifier: Modifier,
) {
    for row in y..y + h {
        for col in x..x + w {
            if col >= area.x && col < area.x + area.width && row >= area.y && row < area.y + area.height {
                let existing = buf[(col, row)].style();
                let new_style = existing.add_modifier(modifier);
                buf[(col, row)].set_style(new_style);
            }
        }
    }
}

/// Apply a background color to all cells in a rectangular region, clipped to the given area.
fn apply_bg_to_region(
    buf: &mut Buffer,
    x: u16,
    y: u16,
    w: u16,
    h: u16,
    area: Rect,
    bg: Color,
) {
    for row in y..y + h {
        for col in x..x + w {
            if col >= area.x && col < area.x + area.width && row >= area.y && row < area.y + area.height {
                let existing = buf[(col, row)].style();
                let new_style = existing.bg(bg);
                buf[(col, row)].set_style(new_style);
            }
        }
    }
}
