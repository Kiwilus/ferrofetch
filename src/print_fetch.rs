use owo_colors::OwoColorize;

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

    for i in 0..max {
        let ascii_line = ascii.get(i).unwrap_or(&"");
        let info_line  = infos.get(i).unwrap_or(&empty_info);

        // fill the line with spaces until it reaches banner_width
        // now the infos are always at the same position
        let padding = banner_width - ascii_line.chars().count();
        let padded_line = format!("{}{}", ascii_line, " ".repeat(padding));

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