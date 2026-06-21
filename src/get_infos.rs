use sysinfo::{System, Disks};
use local_ip_address::local_ip;
use owo_colors::OwoColorize;
use gfxinfo::active_gpu;

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

    let cpu_name = system.cpus()
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
            if total > 0 { format!("{} MB", total / 1024 / 1024) } else { "Unknown".to_string() }
        })
        .unwrap_or_else(|| "Unknown".to_string());

    let ram_total = system.total_memory() / 1024 / 1024;
    let ram_used  = system.used_memory()  / 1024 / 1024;

    let swap_total = system.total_swap() / 1024 / 1024;
    let swap_used  = system.used_swap()  / 1024 / 1024;

    let uptime_secs = System::uptime();
    let uptime = format!("{}h {}m", uptime_secs / 3600, uptime_secs % 3600 / 60);

    let ip_address = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "Unknown".to_string());


    macro_rules! info {
        () => { String::new() };
        ($label:expr, $val:expr, $color:ident) => {
            format!("{}{}", format!("{:<12}", $label).$color(), $val)
        };
    }

    // build the info lines: empty info!() = blank line, color per category
    let mut infos = vec![
        info!(),
        format!(" [{}@{}]", username.yellow(), hostname.green()),

        // system
        info!("OS:",        os_name,    green),
        info!("Kernel:",    kernel,     green),
        info!("Uptime:",    uptime,     green),
        // info!(),

        // processor
        info!("CPU:",       cpu_name,   cyan),
        info!("CPU Cores:", cpu_cores,  cyan),
        // info!(),

        // graphics
        info!("GPU:",       gpu_model,  magenta),
        info!("VRAM:",      vram_mb,    magenta),
        // info!(),

        // memory
        info!("RAM:",  format!("{ram_used} MB / {ram_total} MB"),   yellow),
        info!("SWAP:", format!("{swap_used} MB / {swap_total} MB"), yellow),
        // info!(),

        // network
        info!("Local IP:", ip_address, bright_red),
        // info!(),

        "Disk usage:".bright_black().to_string(),
    ];

    infos.extend(get_disk_usage());
    infos
}