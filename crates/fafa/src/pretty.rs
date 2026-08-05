use std::fmt::Display;

use anstream::println;
use owo_colors::OwoColorize as _;

const DIVIDER_LEN: usize = 6;
const FOOTER_BASE_LEN: usize = DIVIDER_LEN * 2 + 2;

pub fn print_header(header: impl Display) {
    let divider = "-".repeat(DIVIDER_LEN);
    println!(
        "{} {} {}",
        divider.green().bold(),
        header.green().bold(),
        divider.green().bold()
    );
}

pub fn print_footer(header_len: usize) {
    let divider = "-".repeat(FOOTER_BASE_LEN + header_len);
    println!("{}", divider.green().bold(),);
}

pub fn gen_padding(key: &str, max_length: usize) -> String {
    " ".repeat(max_length.saturating_sub(key.chars().count()))
}

pub fn println_key_value(key: &str, value: impl Display, max_length: usize) {
    let padding = gen_padding(key, max_length);
    println!("{}{padding}{}", key.cyan().bold(), value);
}

pub fn swatch(r: u8, g: u8, b: u8) -> String {
    format!("{}", "  ".on_truecolor(r, g, b))
}
