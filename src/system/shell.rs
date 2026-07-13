use std::env;

pub fn get_shell() -> String {
    // Linux, macOS, WSL, Git Bash all set this
    if let Ok(shell) = env::var("SHELL") {
        return shell;
    }

    // powershell sets this env var
    if env::var("PSModulePath").is_ok() {
        return "PowerShell".to_string();
    }

    // cmd.exe usually sets this
    if let Ok(comspec) = env::var("ComSpec") {
        return comspec;
    }

    "Unknown".to_string()
}
