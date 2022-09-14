use std::io::{self, BufRead};

use clap::Parser;

fn read_stdin() -> Result<String, io::Error> {
    let mut buffer: String = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();

    handle.read_line(&mut buffer)?;

    return Ok(buffer);
}

mod ansi {
    pub fn color(color: &str) -> i32 {
        // 3-bit and 4-bit mode. Super simple. https://en.wikipedia.org/wiki/ANSI_escape_code#3-bit_and_4-bit
        match color {
            "brightwhite" => 97,
            "brightcyan" => 96,
            "brightmagenta" => 95,
            "brightblue" => 94,
            "brightyellow" => 93,
            "brightgreen" => 92,
            "brightred" => 91,
            "brightblack" => 90,
            "white" => 37,
            "cyen" => 36,
            "magenta" => 35,
            "blue" => 34,
            "yellow" => 33,
            "green" => 32,
            "red" => 31,
            "black" => 30,
            _ => 37,
        }
    }

    pub fn bg_color(c: &str) -> i32 {
        return color(c) + 10;
    }

    pub fn style(color: &str) -> i32 {
        match color {
            "bold" => 1,
            _ => 0,
        }
    }
}

fn wrap_ansi(input: String, style: &str, color: &str, is_bg: bool) -> String {
    let s = ansi::style(style);
    let c = ansi::color(color);

    return format!("\u{001B}[{};{}m{}\u{001B}[0m", s, c, input);
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    color: String,

    #[clap(short = 's', long = "--style")]
    style: String,

    #[clap(short = 'b', long = "--bg", default_value_t = false)]
    is_bg: bool,
}

fn main() -> io::Result<()> {
    let buffer = read_stdin();
    let args = Args::parse();

    print!(
        "{}",
        wrap_ansi(
            buffer.unwrap(),
            args.style.as_str(),
            args.color.as_str(),
            args.is_bg
        )
    );

    Ok(())
}
