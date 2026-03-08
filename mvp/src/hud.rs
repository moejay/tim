use crate::state::*;

pub fn hud_line1(state: &GameState) -> String {
    let mode_str = match &state.mode {
        Mode::Normal => "NORMAL",
        Mode::Place { .. } => "PLACE",
        Mode::Edit { .. } => "EDIT",
        Mode::Run => "RUN",
    };
    format!(
        "MODE: {} | ({:.0}, {:.0}) | parts: {} | frame: {}",
        mode_str, state.cursor.0, state.cursor.1, state.parts.len(), state.frame
    )
}

pub fn hud_line2(state: &GameState) -> String {
    match &state.mode {
        Mode::Normal => "[p]lace [e]dit [Space]run [x]del [f]lip [u]ndo [?]help [q]uit".into(),
        Mode::Place { .. } => "[hjkl]move [JK]scroll [1-5]select [Enter]place [Esc]cancel".into(),
        Mode::Edit { .. } => "[hjkl]move [f]lip [x]del [Enter/Esc]done".into(),
        Mode::Run => "[Esc/Space] stop".into(),
    }
}
