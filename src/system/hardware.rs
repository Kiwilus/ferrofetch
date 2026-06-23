use gfxinfo::active_gpu;
use local_ip_address::local_ip;
use owo_colors::OwoColorize;
use sysinfo::System;

use super::disk::get_disk_usage;

/*
'MAIN' FUNCTION IN THIS FILE
*/

pub fn get_infos() -> Vec<String> {
    let mut system = System::new();
    system.refresh_all();

    let username = whoami::realname().unwrap_or_else(|_| "Unknown".to_string());
    let hostname = whoami::hostname().unwrap_or_else(|_| "Unknown".to_string());
    let os_name = System::name().unwrap_or("Unknown".to_string());
    let kernel = System::kernel_version().unwrap_or("Unknown".to_string());

    let cpu_name = system
        .cpus()
        .first()
        .map(|c| c.brand().to_string())
        .unwrap_or_else(|| "Unknown".to_string());

    let cpu_cores = system.cpus().len();

    let binding = active_gpu().ok();
    let gpu_model = binding
        .as_ref()
        .map(|g| g.model())
        .unwrap_or_else(|| "Unknown");

    let vram_mb = active_gpu()
        .ok()
        .map(|g| {
            let total = g.info().total_vram();
            if total > 0 {
                format!("{} MB", total / 1024 / 1024)
            } else {
                "Unknown".to_string()
            }
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used = system.used_memory() / 1024 / 1024;

    let swap_total = system.total_swap() / 1024 / 1024;
    let swap_used = system.used_swap() / 1024 / 1024;

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    let ip_address = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "Unknown".to_string());

    macro_rules! info {
        () => {
            String::new()
        };
        ($label:expr, $val:expr, $color:ident) => {
            format!("{}{}", format!("{:<12}", $label).$color(), $val)
        };
    }

    // build the info lines: empty info!() = blank line, color per category
    let mut infos = vec![
        info!(),
        format!(" [{}@{}]", username.yellow(), hostname.green()),
        /*system*/
        //format!("[System Info]").green().to_string(),
        info!("OS:", os_name, green),
        info!("Kernel:", kernel, green),
        info!("Uptime:", uptime, green),
        //info!(),

        /*processor*/
        //format!("[CPU Info]").cyan().to_string(),
        info!("CPU:", cpu_name, cyan),
        info!("CPU Cores:", cpu_cores, cyan),
        //info!(),

        /*graphics*/
        //format!("[Graphics Info]").magenta().to_string(),
        info!("GPU:", gpu_model, magenta),
        info!("VRAM:", vram_mb, magenta),
        //info!(),

        /*memory*/
        //format!("[Memory Info]").yellow().to_string(),
        info!("RAM:", format!("{ram_used} MB / {ram_total} MB"), yellow),
        info!("SWAP:", format!("{swap_used} MB / {swap_total} MB"), yellow),
        //info!(),

        /*network*/
        //format!("[Network Info]").bright_red().to_string(),
        info!("Local IP:", ip_address, bright_red),
        //info!(),
        "Disk usage:".bright_black().to_string(),
    ];

    infos.extend(get_disk_usage());
    infos
}
