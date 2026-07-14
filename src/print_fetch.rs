use owo_colors::{OwoColorize, Style};


// Rainbow color function
fn rainbow_style(x: usize, y: usize) -> Style {
    // Hue verschiebt sich diagonal
    let hue = ((x as f32 * 10.0) + (y as f32 * 8.0)) % 360.0;
    
    let (r, g, b) = hsl_to_rgb(hue, 1.0, 0.65);
    Style::new().truecolor(r, g, b)
}

// convert HSL to RGB
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match h as i32 {
        0..=59   => (c, x, 0.0),
        60..=119 => (x, c, 0.0),
        120..=179=> (0.0, c, x),
        180..=239=> (0.0, x, c),
        240..=299=> (x, 0.0, c),
        _        => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

/*
FUNCTION TO PRINT THE ASCII BANNER AND SYSTEM INFORMATIONS SIDE BY SIDE
*/

pub fn print_fetch(ascii: &[&str], infos: &[String], color: &str) {
    let empty_info = String::new();

    // find the longest line in the banner
    let banner_width = ascii.iter()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(0);

    let max = ascii.len().max(infos.len());

    for (y, i) in (0..max).enumerate() {
        let ascii_line = ascii.get(i).unwrap_or(&"");
        let info_line = infos.get(i).unwrap_or(&empty_info);

        // fill the line with spaces until it reaches banner_width
        // now the infos are always at the same position
        let padding = banner_width.saturating_sub(ascii_line.chars().count());
        let padded_line = format!("{}{}", ascii_line, " ".repeat(padding));

        if color == "rainbow" {
            let mut colored = String::new();
            for (x, ch) in padded_line.chars().enumerate() {
                let style = rainbow_style(x, y);
                colored.push_str(&format!("{}", ch.style(style)));
            }
            println!("{}    {}", colored, info_line);
        } else {
            // Normale Farben
            let colored_line = match color {
                "red"            => padded_line.red().to_string(),
                "green"          => padded_line.green().to_string(),
                "yellow"         => padded_line.yellow().to_string(),
                "blue"           => padded_line.blue().to_string(),
                "magenta"        => padded_line.magenta().to_string(),
                "cyan"           => padded_line.cyan().to_string(),
                "white"          => padded_line.white().to_string(),
                "black"          => padded_line.black().to_string(),
                "bright_red"     => padded_line.bright_red().to_string(),
                "bright_green"   => padded_line.bright_green().to_string(),
                "bright_yellow"  => padded_line.bright_yellow().to_string(),
                "bright_blue"    => padded_line.bright_blue().to_string(),
                "bright_magenta" => padded_line.bright_magenta().to_string(),
                "bright_cyan"    => padded_line.bright_cyan().to_string(),
                "bright_white"   => padded_line.bright_white().to_string(),
                _                => padded_line.white().to_string(),
            };
            println!("{}    {}", colored_line, info_line);
        }
    }
}