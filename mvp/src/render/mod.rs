pub mod pixel;
pub mod pixel_gfx;
pub mod text;

use anyhow::Result;
use crate::state::GameState;

pub trait Renderer {
    fn render_frame(&mut self, state: &GameState) -> Result<()>;
    fn cleanup(&mut self) -> Result<()>;
}
