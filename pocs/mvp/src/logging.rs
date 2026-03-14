use log::{info, error, LevelFilter};
use std::io::Write;
use std::fs::{self, OpenOptions};
use std::path::PathBuf;

use crate::state::RenderMode;

/// Returns the log directory, creating it if needed.
fn log_dir() -> PathBuf {
    let dir = PathBuf::from("logs");
    let _ = fs::create_dir_all(&dir);
    dir
}

/// Returns a timestamped log filename.
fn log_filename() -> String {
    let now = chrono::Local::now();
    now.format("tim2_%Y%m%d_%H%M%S.log").to_string()
}

/// Initialize file-based logger and crash dump panic hook.
pub fn init() {
    let dir = log_dir();
    let filename = log_filename();
    let log_path = dir.join(&filename);

    // Also create/update a "latest" symlink for convenience
    let latest = dir.join("latest.log");
    let _ = fs::remove_file(&latest);
    #[cfg(unix)]
    let _ = std::os::unix::fs::symlink(&filename, &latest);
    #[cfg(not(unix))]
    let _ = fs::copy(&log_path, &latest);

    // Init env_logger writing to file
    let target = Box::new(
        OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .expect("Failed to open log file"),
    );

    env_logger::Builder::new()
        .target(env_logger::Target::Pipe(target))
        .filter_level(LevelFilter::Info)
        .format(|buf, record| {
            let ts = chrono::Local::now().format("%H:%M:%S%.3f");
            writeln!(
                buf, "[{} {:5} {}] {}",
                ts,
                record.level(),
                record.module_path().unwrap_or(""),
                record.args()
            )
        })
        .init();

    // Install panic hook that writes crash dump
    let crash_dir = dir.clone();
    let prev_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        // Try to restore terminal before printing crash info
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = crossterm::execute!(
            std::io::stdout(),
            crossterm::cursor::Show,
            crossterm::terminal::LeaveAlternateScreen
        );

        // Build crash dump
        let mut dump = String::new();
        dump.push_str("=== TIM2 CRASH DUMP ===\n");
        dump.push_str(&format!("Time: {}\n", chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f")));
        dump.push_str(&format!("Panic: {}\n", panic_info));

        if let Some(loc) = panic_info.location() {
            dump.push_str(&format!("Location: {}:{}:{}\n", loc.file(), loc.line(), loc.column()));
        }

        dump.push_str("\n--- Terminal Environment ---\n");
        for var in &[
            "TERM", "TERM_PROGRAM", "TERM_PROGRAM_VERSION",
            "COLORTERM", "KITTY_WINDOW_ID", "KITTY_PID",
            "WEZTERM_PANE", "WEZTERM_UNIX_SOCKET",
            "GHOSTTY_RESOURCES_DIR",
            "ITERM_SESSION_ID", "ITERM_PROFILE",
            "TMUX", "STY",
            "SSH_CONNECTION", "SSH_TTY",
            "DISPLAY", "WAYLAND_DISPLAY",
            "XDG_SESSION_TYPE",
            "LANG", "LC_ALL",
            "COLUMNS", "LINES",
        ] {
            if let Ok(val) = std::env::var(var) {
                dump.push_str(&format!("  {}={}\n", var, val));
            }
        }

        if let Ok((cols, rows)) = crossterm::terminal::size() {
            dump.push_str(&format!("  terminal::size()={}x{}\n", cols, rows));
        }

        dump.push_str("\n--- Backtrace ---\n");
        dump.push_str(&format!("{}", std::backtrace::Backtrace::force_capture()));

        // Write crash dump to file
        let crash_name = format!("crash_{}.txt",
            chrono::Local::now().format("%Y%m%d_%H%M%S"));
        let crash_path = crash_dir.join(&crash_name);
        if let Ok(mut f) = std::fs::File::create(&crash_path) {
            let _ = f.write_all(dump.as_bytes());
        }

        // Also log it
        error!("PANIC: {}", panic_info);

        // Print to stderr so user sees it
        eprintln!("\n{}", dump);
        eprintln!("Crash dump written to: {}", crash_path.display());

        // Call previous hook (default behavior)
        prev_hook(panic_info);
    }));

    info!("Logging initialized: {}", log_path.display());
}

/// Log terminal environment and startup info.
pub fn log_startup_info(args: &[String], mode: RenderMode) {
    info!("=== TIM2 Session Start ===");
    info!("Args: {:?}", args);
    info!("Render mode: {:?} ({})", mode, match mode {
        RenderMode::Pixel => "pixel",
        RenderMode::Text => "text",
    });

    // Terminal size
    if let Ok((cols, rows)) = crossterm::terminal::size() {
        info!("Terminal size: {}x{} (cols x rows)", cols, rows);
    }

    // All relevant terminal env vars
    let vars = [
        "TERM", "TERM_PROGRAM", "TERM_PROGRAM_VERSION",
        "COLORTERM",
        "KITTY_WINDOW_ID", "KITTY_PID",
        "WEZTERM_PANE", "WEZTERM_UNIX_SOCKET",
        "GHOSTTY_RESOURCES_DIR",
        "ITERM_SESSION_ID", "ITERM_PROFILE",
        "TMUX", "STY",
        "SSH_CONNECTION", "SSH_TTY",
        "DISPLAY", "WAYLAND_DISPLAY",
        "XDG_SESSION_TYPE",
        "LANG", "LC_ALL", "LC_CTYPE",
        "COLUMNS", "LINES",
        "SHELL",
    ];

    info!("--- Terminal Environment ---");
    for var in &vars {
        match std::env::var(var) {
            Ok(val) => info!("  {}={}", var, val),
            Err(_) => {} // only log vars that are set
        }
    }

    // OS info
    info!("--- System ---");
    if let Ok(contents) = std::fs::read_to_string("/etc/os-release") {
        for line in contents.lines().take(4) {
            info!("  {}", line);
        }
    }
    info!("  arch={}", std::env::consts::ARCH);
    info!("  os={}", std::env::consts::OS);

    info!("=== End Startup Info ===");
}
