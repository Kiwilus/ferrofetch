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
    banner_path: Option<String>
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

    // CLI Flag > config.toml > Hardcoded Fallback
    let banner = args.banner
        .or(cfg.banner)
        .unwrap_or_else(|| "batman".to_string());

    let color = args.color
        .or(cfg.color)
        .unwrap_or_else(|| "white".to_string());

    let banner_path = args.banner_path.or(cfg.banner_path);

    let ascii: Vec<String>;

    // check banner path and use the banner file
    if let Some(path) = banner_path {
        ascii = banners::get_banner_from_path(&path);
    } else {
        // logic for build in banners
        let vec_static = banners::get_banners(&banner);
        ascii = vec_static.iter().map(|&s| s.to_string()).collect();
    }

    let infos = get_infos::get_infos();
    print_fetch::print_fetch(&ascii.iter().map(|s| s.as_str()).collect::<Vec<&str>>(), &infos, &color);
}