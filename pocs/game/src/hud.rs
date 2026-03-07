use std::io::{self, Write};

use crossterm::{cursor, execute, style, terminal};

use crate::state::*;

pub fn write_hud(state: &GameState) -> io::Result<()> {
    let mut stdout = io::stdout();

    let (_, term_h) = terminal::size()?;
    let hud_row = term_h.saturating_sub(2);

    execute!(stdout, cursor::MoveTo(0, hud_row))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;

    let mode_str = match &state.mode {
        Mode::Normal => "NORMAL",
        Mode::Place { .. } => "PLACE",
        Mode::Edit { .. } => "EDIT",
        Mode::Run => "RUN",
    };
    let status = format!(
        " MODE: {} | cursor: ({:.0},{:.0}) | parts: {} | frame: {}",
        mode_str,
        state.cursor.0,
        state.cursor.1,
        state.parts.len(),
        state.frame,
    );
    execute!(stdout, style::Print(&status))?;

    execute!(stdout, cursor::MoveTo(0, hud_row + 1))?;
    execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine))?;

    let hints = match &state.mode {
        Mode::Normal => {
            if state.won {
                " [Space] Try Again  [q] Quit"
            } else {
                " h/j/k/l:move  H/J/K/L:fast  p:place  e:edit  Space:run  x:del  f:flip  u:undo  ?:help  q:quit"
            }
        }
        Mode::Place { .. } => {
            " h/j/k/l:move  H/L:fast  J/K:bin scroll  1-5:select  Enter:place  Esc:cancel"
        }
        Mode::Edit { .. } => {
            " h/j/k/l:move part  H/J/K/L:fast  f:flip  x:delete  Esc/Enter:done"
        }
        Mode::Run => " Esc/Space: stop simulation",
    };
    execute!(stdout, style::Print(hints))?;

    stdout.flush()?;
    Ok(())
}
