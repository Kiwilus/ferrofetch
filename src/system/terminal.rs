use sysinfo::{Pid, System};

/// Terminal name matches — extend as needed
const KNOWN_TERMINALS: &[&str] = &[
    "kitty",
    "alacritty",
    "wezterm",
    "wezterm-gui",
    "windowsterminal",
    "wt",
    "conhost",
    "gnome-terminal-server",
    "konsole",
    "xterm",
    "urxvt",
    "foot",
    "st",
    "terminator",
    "tilix",
    "hyper",
    "code",
];

pub fn detect_terminal() -> String {
    // fast env-var checks
    if std::env::var("WT_SESSION").is_ok() {
        return "Windows Terminal".to_string();
    }
    if std::env::var("KITTY_WINDOW_ID").is_ok() {
        return "kitty".to_string();
    }
    if std::env::var("ALACRITTY_LOG").is_ok() || std::env::var("ALACRITTY_SOCKET").is_ok() {
        return "Alacritty".to_string();
    }
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
        return term_program;
    }

    // walk the process tree looking for a known terminal name
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut pid = Pid::from_u32(std::process::id());

    while let Some(process) = sys.process(pid) {
        let name = process.name().to_string_lossy().to_lowercase();

        if KNOWN_TERMINALS.iter().any(|t| name.contains(t)) {
            return process.name().to_string_lossy().into_owned();
        }

        match process.parent() {
            Some(parent_pid) => pid = parent_pid,
            None => break,
        }
    }

    "Unknown".to_string()
}
