use std::io;
use std::time::{Duration, Instant};

use anyhow::Result;
use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{self, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use image::RgbaImage;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph};
use ratatui::Terminal;

use tim2::constants::*;
use tim2::parts::{catalog, Category, PartDef, PartProps};
use tim2::render::braille;
use tim2::render::{detect_mode, RenderMode};

struct ExplorerState {
    categories: Vec<Category>,
    current_cat: usize,
    current_part: usize,
    selected_prop: usize,
    selected_state: usize,
    props: PartProps,
    frame: u64,
}

impl ExplorerState {
    fn new() -> Self {
        let categories = catalog();
        let mut state = Self {
            categories,
            current_cat: 0,
            current_part: 0,
            selected_prop: 0,
            selected_state: 0,
            props: PartProps::default(),
            frame: 0,
        };
        state.reset_props();
        state
    }

    fn current_part_def(&self) -> &dyn PartDef {
        self.categories[self.current_cat].parts[self.current_part].part_def()
    }

    fn reset_props(&mut self) {
        let cat = &self.categories[self.current_cat];
        let def = cat.parts[self.current_part].part_def();
        let (w, h) = def.default_size();
        let prop_defs = def.properties();
        let mut props = PartProps {
            flipped: false,
            width: w,
            height: h,
            values: std::collections::HashMap::new(),
            current_state: 0,
        };
        for prop in prop_defs {
            props.values.insert(prop.name.clone(), prop.default);
        }
        self.props = props;
        self.selected_prop = 0;
        self.selected_state = 0;
    }

    fn next_category(&mut self) {
        self.current_cat = (self.current_cat + 1) % self.categories.len();
        self.current_part = 0;
        self.reset_props();
    }

    fn prev_category(&mut self) {
        self.current_cat = self.current_cat.checked_sub(1).unwrap_or(self.categories.len() - 1);
        self.current_part = 0;
        self.reset_props();
    }

    fn next_part(&mut self) {
        let count = self.categories[self.current_cat].parts.len();
        self.current_part = (self.current_part + 1) % count;
        self.reset_props();
    }

    fn prev_part(&mut self) {
        let count = self.categories[self.current_cat].parts.len();
        self.current_part = self.current_part.checked_sub(1).unwrap_or(count - 1);
        self.reset_props();
    }

    fn adjust_prop(&mut self, delta: f32) {
        let props_list = self.current_part_def().properties();
        if props_list.is_empty() { return; }
        let idx = self.selected_prop % props_list.len();
        let prop_def = &props_list[idx];
        let val = self.props.values.entry(prop_def.name.clone()).or_insert(prop_def.default);
        *val = (*val + delta * prop_def.step).clamp(prop_def.min, prop_def.max);
        if prop_def.name == "width" { self.props.width = *val; }
        else if prop_def.name == "height" { self.props.height = *val; }
    }

    fn toggle_flip(&mut self) {
        if self.current_part_def().is_flippable() {
            self.props.flipped = !self.props.flipped;
        }
    }

    fn cycle_state(&mut self) {
        let states = self.current_part_def().states();
        if !states.is_empty() {
            self.selected_state = (self.selected_state + 1) % states.len();
            self.props.current_state = self.selected_state;
        }
    }
}

/// Handle vim-like input. Returns true if should quit.
fn handle_input(state: &mut ExplorerState) -> Result<bool> {
    if !event::poll(Duration::from_millis(1))? { return Ok(false); }
    if let Event::Key(key) = event::read()? {
        match key.code {
            // Quit
            KeyCode::Char('q') => return Ok(true),

            // Category navigation: Tab / Shift+Tab or gt / gT style
            KeyCode::Tab => {
                if key.modifiers.contains(KeyModifiers::SHIFT) {
                    state.prev_category();
                } else {
                    state.next_category();
                }
            }
            KeyCode::Char('L') => state.next_category(),
            KeyCode::Char('H') => state.prev_category(),

            // Part navigation: h/l or Left/Right
            KeyCode::Char('l') | KeyCode::Right => state.next_part(),
            KeyCode::Char('h') | KeyCode::Left => state.prev_part(),

            // Property selection: j/k or Up/Down
            KeyCode::Char('j') | KeyCode::Down => {
                let count = state.current_part_def().properties().len();
                if count > 0 {
                    state.selected_prop = (state.selected_prop + 1) % count;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                let count = state.current_part_def().properties().len();
                if count > 0 {
                    state.selected_prop = state.selected_prop.checked_sub(1).unwrap_or(count - 1);
                }
            }

            // Adjust property value: +/- or =/[
            KeyCode::Char('+') | KeyCode::Char('=') | KeyCode::Char(']') => state.adjust_prop(1.0),
            KeyCode::Char('-') | KeyCode::Char('[') => state.adjust_prop(-1.0),

            // Flip: f
            KeyCode::Char('f') => state.toggle_flip(),

            // Cycle state: s
            KeyCode::Char('s') => state.cycle_state(),

            _ => {}
        }
    }
    Ok(false)
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let render_mode = if args.iter().any(|a| a == "--pixel") {
        RenderMode::Pixel
    } else if args.iter().any(|a| a == "--text") {
        RenderMode::Text
    } else {
        detect_mode()
    };

    match render_mode {
        RenderMode::Pixel => run_pixel_mode()?,
        RenderMode::Text => run_text_mode()?,
    }
    Ok(())
}

// ── Pixel mode ─────────────────────────────────────────────────

fn run_pixel_mode() -> Result<()> {
    let mut state = ExplorerState::new();
    let frame_dur = Duration::from_millis(1000 / TARGET_FPS as u64);

    terminal::enable_raw_mode()?;
    execute!(io::stdout(), EnterAlternateScreen, cursor::Hide)?;

    loop {
        let frame_start = Instant::now();
        if handle_input(&mut state)? { break; }
        render_pixel_frame(&state)?;
        state.frame += 1;
        let elapsed = frame_start.elapsed();
        if elapsed < frame_dur { std::thread::sleep(frame_dur - elapsed); }
    }

    execute!(io::stdout(), cursor::Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn render_pixel_frame(state: &ExplorerState) -> Result<()> {
    let mut img = RgbaImage::from_pixel(CANVAS_W, CANVAS_H, image::Rgba(BG_COLOR));

    // Grid
    for gx in (0..CANVAS_W).step_by(GRID_SIZE as usize) {
        for gy in 0..CANVAS_H {
            tim2::render::pixel_gfx::blend_pixel(&mut img, gx as i32, gy as i32, GRID_COLOR);
        }
    }
    for gy in (0..CANVAS_H).step_by(GRID_SIZE as usize) {
        for gx in 0..CANVAS_W {
            tim2::render::pixel_gfx::blend_pixel(&mut img, gx as i32, gy as i32, GRID_COLOR);
        }
    }

    let preview_w = 460;
    let def = state.current_part_def();
    let (pw, ph) = def.default_size();
    let part_x = (preview_w as f32 - pw) / 2.0;
    let part_y = (CANVAS_H as f32 - ph) / 2.0 - 30.0;
    def.draw_pixel(&mut img, part_x, part_y, &state.props, state.frame);

    // ── Top-left HUD overlay: current state + selected prop ──
    let hud_x = 8;
    let mut hud_y = 8;

    // State badge
    let states = def.states();
    if !states.is_empty() {
        let st = &states[state.selected_state % states.len()];
        let state_label = format!("[S] {}", st.name);
        // Dark background for readability
        let slw = tim2::render::pixel_gfx::text_width(&state_label, 1);
        tim2::render::pixel_gfx::fill_rect(&mut img, hud_x - 2, hud_y - 1, slw + 4, 9, [0, 0, 0, 160]);
        tim2::render::pixel_gfx::draw_text(&mut img, hud_x, hud_y, &state_label, [100, 255, 100, 255], 1);
        hud_y += 11;

        // State description
        let desc_w = tim2::render::pixel_gfx::text_width(st.description, 1);
        tim2::render::pixel_gfx::fill_rect(&mut img, hud_x - 2, hud_y - 1, desc_w + 4, 9, [0, 0, 0, 140]);
        tim2::render::pixel_gfx::draw_text(&mut img, hud_x, hud_y, st.description, [80, 180, 80, 220], 1);
        hud_y += 13;
    }

    // Selected property
    let prop_defs = def.properties();
    if !prop_defs.is_empty() {
        let idx = state.selected_prop % prop_defs.len();
        let pd = &prop_defs[idx];
        let val = state.props.values.get(&pd.name).copied().unwrap_or(pd.default);
        let prop_label = format!("[P] {}: {:.1}", pd.label, val);
        let plw = tim2::render::pixel_gfx::text_width(&prop_label, 1);
        tim2::render::pixel_gfx::fill_rect(&mut img, hud_x - 2, hud_y - 1, plw + 4, 9, [0, 0, 0, 160]);
        tim2::render::pixel_gfx::draw_text(&mut img, hud_x, hud_y, &prop_label, [255, 220, 80, 255], 1);
        hud_y += 11;
    }

    // Flip indicator
    if def.is_flippable() {
        let flip_label = if state.props.flipped { "[F] FLIPPED" } else { "[F] NORMAL" };
        let flw = tim2::render::pixel_gfx::text_width(flip_label, 1);
        tim2::render::pixel_gfx::fill_rect(&mut img, hud_x - 2, hud_y - 1, flw + 4, 9, [0, 0, 0, 140]);
        tim2::render::pixel_gfx::draw_text(&mut img, hud_x, hud_y, flip_label, [180, 180, 220, 220], 1);
    }

    // Part name centered at bottom
    let name = def.name();
    let tw = tim2::render::pixel_gfx::text_width(name, 2);
    tim2::render::pixel_gfx::draw_text(&mut img, preview_w / 2 - tw / 2, CANVAS_H as i32 - 70, name, [255, 255, 255, 255], 2);

    // Size + physics summary
    let phys = def.physics();
    let size_str = format!("{}X{}  M:{:.2} E:{:.2} F:{:.2}", pw as i32, ph as i32, phys.mass.min(99.0), phys.elasticity, phys.friction);
    let stw = tim2::render::pixel_gfx::text_width(&size_str, 1);
    tim2::render::pixel_gfx::draw_text(&mut img, preview_w / 2 - stw / 2, CANVAS_H as i32 - 50, &size_str, [180, 180, 180, 255], 1);

    // Right panel
    let panel_x = 470;
    let cat = &state.categories[state.current_cat];
    tim2::render::pixel_gfx::draw_text(&mut img, panel_x, 10, cat.name, [255, 220, 80, 255], 2);

    for (i, part_id) in cat.parts.iter().enumerate() {
        let pd = part_id.part_def();
        let color = if i == state.current_part { [255, 255, 255, 255] } else { [140, 140, 140, 255] };
        let prefix = if i == state.current_part { "> " } else { "  " };
        let label = format!("{}{}", prefix, pd.name());
        tim2::render::pixel_gfx::draw_text(&mut img, panel_x, 35 + i as i32 * 12, &label, color, 1);
    }

    // Properties
    let props_y = 35 + cat.parts.len() as i32 * 12 + 15;
    tim2::render::pixel_gfx::draw_text(&mut img, panel_x, props_y, "PROPS:", [200, 200, 200, 255], 1);

    let prop_defs = def.properties();
    if prop_defs.is_empty() {
        tim2::render::pixel_gfx::draw_text(&mut img, panel_x, props_y + 12, "(NONE)", [120, 120, 120, 255], 1);
    } else {
        for (i, pd) in prop_defs.iter().enumerate() {
            let val = state.props.values.get(&pd.name).copied().unwrap_or(pd.default);
            let label = format!("{}: {:.1}", pd.label, val);
            let color = if i == state.selected_prop % prop_defs.len() { [100, 255, 100, 255] } else { [180, 180, 180, 255] };
            tim2::render::pixel_gfx::draw_text(&mut img, panel_x, props_y + 12 + i as i32 * 10, &label, color, 1);
        }
    }

    // Flip/state
    let extra_y = props_y + 12 + (prop_defs.len() as i32 + 1) * 10;
    if def.is_flippable() {
        let s = if state.props.flipped { "FLIPPED: YES" } else { "FLIPPED: NO" };
        tim2::render::pixel_gfx::draw_text(&mut img, panel_x, extra_y, s, [180, 180, 180, 255], 1);
    }

    // Help
    let help = "H/L:CAT  H/L:PART  J/K:PROP  +/-:VAL  F:FLIP  S:STATE  Q:QUIT";
    tim2::render::pixel_gfx::draw_text(&mut img, 10, CANVAS_H as i32 - 14, help, [120, 120, 140, 255], 1);

    let counter = format!("{} ({}/{})", cat.name, state.current_part + 1, cat.parts.len());
    let cw = tim2::render::pixel_gfx::text_width(&counter, 1);
    tim2::render::pixel_gfx::draw_text(&mut img, CANVAS_W as i32 - cw - 10, CANVAS_H as i32 - 14, &counter, [120, 120, 140, 255], 1);

    let dimg = image::DynamicImage::ImageRgba8(img);
    execute!(io::stdout(), cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;
    let conf = viuer::Config {
        width: Some(CANVAS_W / 4),
        height: Some(CANVAS_H / 8),
        absolute_offset: false,
        ..Default::default()
    };
    viuer::print(&dimg, &conf)?;
    Ok(())
}

// ── Text mode (braille) ───────────────────────────────────────

fn run_text_mode() -> Result<()> {
    let mut state = ExplorerState::new();
    let frame_dur = Duration::from_millis(1000 / TARGET_FPS as u64);

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, cursor::Hide)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        let frame_start = Instant::now();
        if handle_input(&mut state)? { break; }
        terminal.draw(|f| render_text_frame(f, &state))?;
        state.frame += 1;
        let elapsed = frame_start.elapsed();
        if elapsed < frame_dur { std::thread::sleep(frame_dur - elapsed); }
    }

    execute!(terminal.backend_mut(), cursor::Show, LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;
    Ok(())
}

fn render_text_frame(f: &mut ratatui::Frame, state: &ExplorerState) {
    let size = f.area();

    // Main layout: content + help bar
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(3), Constraint::Length(2)])
        .split(size);

    // Content: preview (left) + sidebar (right)
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[0]);

    let cat = &state.categories[state.current_cat];
    let def = state.current_part_def();
    let (pw, ph) = def.default_size();
    let phys = def.physics();

    // ── Preview area with braille rendering ──
    let title = format!(" {} ", cat.name);
    let preview_block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));
    let preview_inner = preview_block.inner(content_chunks[0]);
    f.render_widget(preview_block, content_chunks[0]);

    // Reserve rows: top 2-3 for HUD overlay, bottom 4 for name/info
    let hud_rows = 3_u16;
    let info_rows = 4_u16;
    let braille_area_h = preview_inner.height.saturating_sub(info_rows + hud_rows);

    // ── Top-left HUD overlay (state + prop + flip) ──
    let states = def.states();
    let prop_defs = def.properties();
    {
        let mut hud_line = 0_u16;
        // State
        if !states.is_empty() {
            let st = &states[state.selected_state % states.len()];
            let state_spans = vec![
                Span::styled("[s] ", Style::default().fg(Color::DarkGray)),
                Span::styled(st.name, Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::styled(format!(" {}", st.description), Style::default().fg(Color::DarkGray)),
            ];
            let hy = preview_inner.y + hud_line;
            if hy < preview_inner.bottom() {
                f.render_widget(
                    Paragraph::new(Line::from(state_spans)),
                    Rect::new(preview_inner.x, hy, preview_inner.width, 1),
                );
            }
            hud_line += 1;
        }
        // Selected property
        if !prop_defs.is_empty() {
            let idx = state.selected_prop % prop_defs.len();
            let pd = &prop_defs[idx];
            let val = state.props.values.get(&pd.name).copied().unwrap_or(pd.default);
            let prop_spans = vec![
                Span::styled("[+/-] ", Style::default().fg(Color::DarkGray)),
                Span::styled(format!("{}: {:.1}", pd.label, val), Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)),
            ];
            let hy = preview_inner.y + hud_line;
            if hy < preview_inner.bottom() {
                f.render_widget(
                    Paragraph::new(Line::from(prop_spans)),
                    Rect::new(preview_inner.x, hy, preview_inner.width, 1),
                );
            }
            hud_line += 1;
        }
        // Flip
        if def.is_flippable() {
            let flip_label = if state.props.flipped { "FLIPPED" } else { "NORMAL" };
            let flip_spans = vec![
                Span::styled("[f] ", Style::default().fg(Color::DarkGray)),
                Span::styled(flip_label, Style::default().fg(Color::Rgb(160, 160, 220))),
            ];
            let hy = preview_inner.y + hud_line;
            if hy < preview_inner.bottom() {
                f.render_widget(
                    Paragraph::new(Line::from(flip_spans)),
                    Rect::new(preview_inner.x, hy, preview_inner.width, 1),
                );
            }
        }
    }

    // ── Braille part preview ──
    if braille_area_h > 0 && preview_inner.width > 0 {
        let braille_area = Rect::new(preview_inner.x, preview_inner.y + hud_rows, preview_inner.width, braille_area_h);

        let buf_w = braille_area.width as u32 * 2;
        let buf_h = braille_area.height as u32 * 4;
        let mut pixel_buf = RgbaImage::from_pixel(buf_w, buf_h, image::Rgba([20, 20, 28, 255]));

        let part_x = ((buf_w as f32 - pw) / 2.0).max(0.0);
        let part_y = ((buf_h as f32 - ph) / 2.0).max(0.0);
        def.draw_pixel(&mut pixel_buf, part_x, part_y, &state.props, state.frame);

        braille::render_braille(&pixel_buf, f.buffer_mut(), braille_area);
    }

    // ── Bottom info area ──
    let info_y = preview_inner.y + hud_rows + braille_area_h;
    if info_y < preview_inner.bottom() {
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(
                def.name(),
                Style::default().fg(Color::White).add_modifier(Modifier::BOLD),
            ))).alignment(ratatui::layout::Alignment::Center),
            Rect::new(preview_inner.x, info_y, preview_inner.width, 1),
        );
    }

    let size_y = info_y + 1;
    if size_y < preview_inner.bottom() {
        let info_str = format!(
            "{}x{} px  mass:{:.2}  elast:{:.2}  fric:{:.2}",
            pw as i32, ph as i32,
            phys.mass.min(99.0), phys.elasticity, phys.friction,
        );
        f.render_widget(
            Paragraph::new(Line::from(Span::styled(info_str, Style::default().fg(Color::DarkGray))))
                .alignment(ratatui::layout::Alignment::Center),
            Rect::new(preview_inner.x, size_y, preview_inner.width, 1),
        );
    }

    // ── Sidebar ──
    let sidebar_block = Block::default()
        .title(" Properties ")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));
    let sidebar_inner = sidebar_block.inner(content_chunks[1]);
    f.render_widget(sidebar_block, content_chunks[1]);

    let mut lines: Vec<Line> = Vec::new();

    // Category header
    lines.push(Line::from(Span::styled(
        format!("  {} ({}/{})", cat.name, state.current_part + 1, cat.parts.len()),
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD),
    )));
    lines.push(Line::from(""));

    // Part list
    for (i, part_id) in cat.parts.iter().enumerate() {
        let pd = part_id.part_def();
        let ic = pd.icon_color();
        let (style, prefix) = if i == state.current_part {
            (Style::default().fg(Color::White).add_modifier(Modifier::BOLD), "> ")
        } else {
            (Style::default().fg(Color::DarkGray), "  ")
        };
        let icon_style = Style::default().fg(Color::Rgb(ic[0], ic[1], ic[2]));
        lines.push(Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{} ", pd.icon_char()), icon_style),
            Span::styled(pd.name(), style),
        ]));
    }

    // Properties section
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Properties:", Style::default().fg(Color::Cyan))));

    let prop_defs = def.properties();
    if prop_defs.is_empty() {
        lines.push(Line::from(Span::styled("  (none)", Style::default().fg(Color::DarkGray))));
    } else {
        for (i, pd) in prop_defs.iter().enumerate() {
            let val = state.props.values.get(&pd.name).copied().unwrap_or(pd.default);
            let is_sel = i == state.selected_prop % prop_defs.len();
            let style = if is_sel {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            let prefix = if is_sel { "  > " } else { "    " };
            lines.push(Line::from(Span::styled(
                format!("{}{}: {:.1}", prefix, pd.label, val), style,
            )));
        }
    }

    // Flip
    if def.is_flippable() {
        let s = if state.props.flipped { "  Flip: Yes" } else { "  Flip: No" };
        lines.push(Line::from(Span::styled(s, Style::default().fg(Color::Gray))));
    }

    // States section
    if !states.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled("  States:", Style::default().fg(Color::Cyan))));
        for (i, st) in states.iter().enumerate() {
            let is_active = i == state.selected_state % states.len();
            let style = if is_active {
                Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            let marker = if is_active { " * " } else { "   " };
            lines.push(Line::from(Span::styled(
                format!("{}{}", marker, st.name), style,
            )));
        }
    }

    // Physics summary
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled("  Physics:", Style::default().fg(Color::Cyan))));
    lines.push(Line::from(Span::styled(
        format!("    mass: {:.2}", phys.mass.min(999.0)), Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        format!("    elast: {:.2}", phys.elasticity), Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        format!("    fric: {:.2}", phys.friction), Style::default().fg(Color::DarkGray),
    )));
    lines.push(Line::from(Span::styled(
        format!("    static: {}", phys.is_static), Style::default().fg(Color::DarkGray),
    )));
    let grav_str = match phys.gravity_response {
        tim2::parts::GravityResponse::AlwaysFalls => "AlwaysFalls",
        tim2::parts::GravityResponse::Normal => "Normal",
        tim2::parts::GravityResponse::Buoyant => "Buoyant",
        tim2::parts::GravityResponse::ZeroGravity => "ZeroGravity",
        tim2::parts::GravityResponse::Custom(_) => "Custom",
    };
    lines.push(Line::from(Span::styled(
        format!("    gravity: {}", grav_str), Style::default().fg(Color::DarkGray),
    )));

    let sidebar_para = Paragraph::new(lines);
    f.render_widget(sidebar_para, sidebar_inner);

    // ── Help bar (vim-like) ──
    let help_line = Line::from(vec![
        Span::styled(" H/L", Style::default().fg(Color::Yellow)),
        Span::styled(" cat  ", Style::default().fg(Color::DarkGray)),
        Span::styled("h/l", Style::default().fg(Color::Yellow)),
        Span::styled(" part  ", Style::default().fg(Color::DarkGray)),
        Span::styled("j/k", Style::default().fg(Color::Yellow)),
        Span::styled(" prop  ", Style::default().fg(Color::DarkGray)),
        Span::styled("+/-", Style::default().fg(Color::Yellow)),
        Span::styled(" val  ", Style::default().fg(Color::DarkGray)),
        Span::styled("f", Style::default().fg(Color::Yellow)),
        Span::styled("lip  ", Style::default().fg(Color::DarkGray)),
        Span::styled("s", Style::default().fg(Color::Yellow)),
        Span::styled("tate  ", Style::default().fg(Color::DarkGray)),
        Span::styled("q", Style::default().fg(Color::Yellow)),
        Span::styled("uit", Style::default().fg(Color::DarkGray)),
    ]);
    f.render_widget(Paragraph::new(help_line), main_chunks[1]);
}
