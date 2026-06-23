/*
MAIN FILE
*/

use clap::Parser;

mod banners;
mod config;
mod get_infos;
mod print_fetch;

// cli arguments e.g. ferrofetch --banner green
#[derive(Parser)]
#[command(name = "ferrofetch", about = "A system fetch tool written in Rust")]
struct Args {
    #[arg(short, long)]
    banner: Option<String>,

    #[arg(short, long)]
    color: Option<String>,

    #[arg(short, long)]
    list: bool,

    #[arg(long)]
    banner_path: Option<String>,

    #[arg(short, long, default_value_t = false)]
    no_ascii: bool,
}

// main entry point
fn main() {
    let args = Args::parse();

    if args.list {
        println!("Available banners:");
        for name in banners::list_banners() {
            println!("  - {}", name);
        }
        return;
    }

    // load config from config.toml
    let cfg = config::load_config();

    let no_ascii = args.no_ascii || cfg.no_ascii.unwrap_or(false);

    // CLI Flag > config.toml > Hardcoded Fallback
    let banner = args
        .banner
        .or(cfg.banner)
        .unwrap_or_else(|| "batman".to_string());

    let color = args
        .color
        .or(cfg.color)
        .unwrap_or_else(|| "white".to_string());

    let banner_path = args.banner_path.or(cfg.banner_path);

    let _ascii: Vec<String>;

    let infos = get_infos::get_infos();

    if no_ascii {
        // show only infos, without banner
        for line in &infos {
            println!("{}", line);
        }
    } else {
        // show banner + infos
        let ascii: Vec<String> = if let Some(path) = banner_path {
            banners::get_banner_from_path(&path)
        } else {
            let vec_static = banners::get_banners(&banner);
            vec_static.iter().map(|&s| s.to_string()).collect()
        };

        print_fetch::print_fetch(
            &ascii.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
            &infos,
            &color,
        );
    }
}
