use std::io::{self, BufRead};

fn read_stdin() -> Result<String, io::Error> {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    return Ok(buffer);
}

mod ansi {
    pub fn color(color: &str) -> &str {
        match color {
            "white" => "37",
            "yellow" => "33",
            "red" => "31",
            _ => "",
        }
    }

    pub fn style(color: &str) -> &str {
        match color {
            "bold" => "1",
            _ => "",
        }
    }
}


fn wrap_ansi(input: String, style: &str, color: &str) -> String {
    return format!(
        "\u{001B}[{};{}m{}\u{001B}[0m",
        ansi::style(style),
        ansi::color(color),
        input
    );
}

fn main() -> io::Result<()> {
    let buffer = read_stdin();
    let args: Vec<String> = std::env::args().collect();

    let style = &args[1];
    let color = &args[2];

    print!("{}", wrap_ansi(buffer.unwrap(), style, color));

    Ok(())
}
