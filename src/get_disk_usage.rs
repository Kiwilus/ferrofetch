use sysinfo::Disks;
use owo_colors::OwoColorize;

fn disk_usage_info(total: u64, available: u64) -> (f64, f64, f64) {
    let total_gb     = total as f64 / 1_073_741_824.0;
    let available_gb = available as f64 / 1_073_741_824.0;
    let used_gb      = total_gb - available_gb;
    let percent      = if total > 0 { (used_gb / total_gb) * 100.0 } else { 0.0 };
    (percent, used_gb, total_gb)
}

// returns lines as Strings instead of printing directly
pub fn get_disk_usage() -> Vec<String> {
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