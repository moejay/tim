mod state;
mod parts;
mod render;
mod physics;
mod input;
mod hud;
mod puzzle;
mod logging;

use anyhow::Result;
use crossterm::{cursor, execute, terminal};
use log::{info, error, warn};
use std::io::stdout;
use std::time::Instant;
use state::*;
use render::Renderer;

fn detect_mode(args: &[String]) -> RenderMode {
    for arg in args {
        match arg.as_str() {
            "--pixel" => return RenderMode::Pixel,
            "--text" => return RenderMode::Text,
            _ => {}
        }
    }
    if std::env::var("TERM")
        .map(|t| t.contains("kitty"))
        .unwrap_or(false)
        || std::env::var("KITTY_WINDOW_ID").is_ok()
        || std::env::var("TERM_PROGRAM")
            .map(|t| t == "WezTerm" || t == "ghostty" || t == "iTerm.app")
            .unwrap_or(false)
    {
        RenderMode::Pixel
    } else {
        RenderMode::Text
    }
}

fn main() -> Result<()> {
    // Initialize file logger + panic hook before anything else
    logging::init();

    let args: Vec<String> = std::env::args().collect();
    let mode = detect_mode(&args);

    // Log startup environment
    logging::log_startup_info(&args, mode);

    let mode_name = match mode {
        RenderMode::Pixel => "Pixel (viuer)",
        RenderMode::Text => "Text (ratatui)",
    };
    println!("TIM2 Terminal \u{2014} Renderer: {}", mode_name);
    std::thread::sleep(std::time::Duration::from_millis(500));

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
    info!("Terminal raw mode enabled, alternate screen entered");

    // Load puzzle
    let (parts, bin_items) = puzzle::load_mvp_puzzle();
    let mut state = GameState::new(parts, bin_items);
    info!("Puzzle loaded: {} fixed parts, {} bin items", state.parts.len(), state.bin_items.len());

    // Create renderer
    let mut renderer: Box<dyn Renderer> = match mode {
        RenderMode::Pixel => {
            info!("Creating PixelRenderer");
            Box::new(render::pixel::PixelRenderer::new()?)
        }
        RenderMode::Text => {
            info!("Creating TextRenderer");
            Box::new(render::text::TextRenderer::new()?)
        }
    };

    // Game loop
    let frame_dur = std::time::Duration::from_secs_f64(1.0 / 60.0);
    let mut last_frame = Instant::now();
    let mut frame_count: u64 = 0;
    let mut slow_frame_count: u64 = 0;
    let loop_start = Instant::now();

    info!("Entering game loop");

    let result: Result<()> = (|| {
        loop {
            let frame_start = Instant::now();
            let dt = last_frame.elapsed().as_secs_f32();
            last_frame = frame_start;

            // Log slow frames
            if dt > 0.05 && frame_count > 1 {
                slow_frame_count += 1;
                if slow_frame_count <= 20 || slow_frame_count % 100 == 0 {
                    warn!("Slow frame #{}: dt={:.3}s ({:.0} fps), frame {}",
                        slow_frame_count, dt, 1.0/dt, frame_count);
                }
            }

            // Input
            if !input::handle_input(&mut state)? {
                info!("Quit requested at frame {}", frame_count);
                break;
            }

            // Physics
            if state.mode == Mode::Run && !state.won {
                physics::update_physics(&mut state, dt);
            }

            // Log mode transitions
            if frame_count == 0 {
                info!("First frame rendered");
            }

            // Render
            state.frame += 1;
            state.elapsed += dt;
            renderer.render_frame(&state)?;

            frame_count += 1;

            // Frame rate cap
            let elapsed = frame_start.elapsed();
            if elapsed < frame_dur {
                std::thread::sleep(frame_dur - elapsed);
            }
        }
        Ok(())
    })();

    // Log session summary
    let session_secs = loop_start.elapsed().as_secs_f32();
    info!("Session ended: {} frames in {:.1}s ({:.1} avg fps), {} slow frames",
        frame_count, session_secs,
        if session_secs > 0.0 { frame_count as f32 / session_secs } else { 0.0 },
        slow_frame_count);

    // Cleanup terminal regardless of error
    let cleanup_result = renderer.cleanup();
    let _ = execute!(stdout, cursor::Show, terminal::LeaveAlternateScreen);
    let _ = terminal::disable_raw_mode();

    if let Err(e) = &cleanup_result {
        error!("Renderer cleanup error: {:#}", e);
    }

    if let Err(e) = &result {
        error!("Game loop error: {:#}", e);
    }

    result
}
