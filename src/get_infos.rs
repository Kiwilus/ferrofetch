use sysinfo::{System, Disks};
use local_ip_address::local_ip;
use owo_colors::OwoColorize;
use std::process::Command;


/*
FUNCTION TO GET GPU NAME
*/
fn get_gpu() -> String {
    // Run "lspci" and filter for VGA (works on Linux)
    let output = Command::new("sh")
        .arg("-c")
        .arg("lspci | grep -i vga")
        .output();

    match output {
        Ok(out) => {
            let text = String::from_utf8_lossy(&out.stdout).to_string();
            // Nimm nur den Teil nach dem ":" 
            text.split(':')
                .last()
                .unwrap_or("Unknown")
                .trim()
                .to_string()
        }
        Err(_) => "Unknown".to_string()
    }
}


/*
2 FUNCTIONS TO GET THE DISK USAGE
*/

// calculates disk usage
fn disk_usage_info(total: u64, available: u64) -> (f64, f64, f64) {
    let total_gb     = total as f64 / 1_073_741_824.0;
    let available_gb = available as f64 / 1_073_741_824.0;
    let used_gb      = total_gb - available_gb;
    let percent      = if total > 0 { (used_gb / total_gb) * 100.0 } else { 0.0 };
    (percent, used_gb, total_gb)
}

// returns lines as Strings instead of printing directly
fn get_disk_usage() -> Vec<String> {
    let disks = Disks::new_with_refreshed_list();
    let mut lines = Vec::new();

    for disk in &disks {
        let total     = disk.total_space();
        let available = disk.available_space();
        let (percent, used_gb, total_gb) = disk_usage_info(total, available);

        let mount = disk.mount_point().display().to_string();
        let kind  = format!("{:?}", disk.kind());

        lines.push(format!("* {}", 
            format!("{mount} [{kind}] -> {used_gb:.1} GB / {total_gb:.1} GB ({percent:.1}%)")
            .to_string()
        ));

        let bar_width       = 40;
        let occupied_blocks = ((percent / 100.0) * bar_width as f64).round() as usize;
        let filled          = "#".repeat(occupied_blocks);
        let empty           = "#".repeat(bar_width - occupied_blocks);

        let bar = if percent > 90.0 {
            filled.red().to_string()
        } else if percent > 75.0 {
            filled.yellow().to_string()
        } else {
            filled.green().to_string()
        };

        lines.push(format!("   [{}{}]", bar, empty));
        lines.push(format!(""));
    }

    lines
}


/* 
'MAIN' FUNCTION IN THIS FILE
*/

pub fn get_infos() -> Vec<String> {
    let mut system = System::new();
    system.refresh_all();

    let username  = whoami::realname().unwrap_or_else(|_| "Unknown".to_string());
    let hostname  = whoami::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let os_name   = System::name().unwrap_or("Unknown".to_string());
    let kernel    = System::kernel_version().unwrap_or("Unknown".to_string());
    let cpu_name  = system.cpus()[0].brand().to_string();
    let gpu_name = get_gpu();

    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used  = system.used_memory()  / 1024 / 1024;

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    let ip_adress =  local_ip();

    // Return all infos as a list
    let mut infos = vec![
        format!(""),
        format!(" [{}@{}]", username.yellow(), hostname.green()),
        format!("OS:         {}", os_name),
        format!("Kernel:     {}", kernel),
        format!("Uptime:     {}", uptime),
        format!("CPU:        {}", cpu_name),
        format!("GPU:        {}", gpu_name),
        format!("RAM:        {} MB / {} MB", ram_used, ram_total),
        format!("Local IP:   {:?}", ip_adress),
        format!("Disk usage: "),
    ];

    // Add disk lines under infos
    infos.extend(get_disk_usage());
    infos

}