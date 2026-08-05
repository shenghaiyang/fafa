use crate::pretty::{print_footer, print_header, println_key_value};
use anyhow::Result;
use strum::Display;

#[derive(Clone, Copy, Display, clap::ValueEnum)]
enum Radix {
    #[value(name = "2")]
    #[strum(to_string = "Binary(2)")]
    Binary,
    #[value(name = "8")]
    #[strum(to_string = "Octal(8)")]
    Octal,
    #[value(name = "10")]
    #[strum(to_string = "Decimal(10)")]
    Decimal,
    #[value(name = "16")]
    #[strum(to_string = "Hex(16)")]
    Hex,
}

#[derive(clap::Args)]
pub struct Args {
    number: String,
    #[arg(long, value_enum, default_value_t = Radix::Decimal)]
    from: Radix,
}

const MAX_LEN: usize = 16;

pub fn run(args: Args) -> Result<()> {
    let number = compute_num(&args.number, args.from)?;
    let header = "Base Conversion";
    print_header(header);
    println_key_value(&Radix::Binary.to_string(), format!("{:b}", number), MAX_LEN);
    println_key_value(&Radix::Octal.to_string(), format!("{:o}", number), MAX_LEN);
    println_key_value(&Radix::Decimal.to_string(), format!("{}", number), MAX_LEN);
    println_key_value(&Radix::Hex.to_string(), format!("{:x}", number), MAX_LEN);
    print_footer(header.len());
    Ok(())
}

fn compute_num(input: &str, from: Radix) -> Result<i64> {
    let num: i64 = match from {
        Radix::Binary => {
            let input = input
                .strip_prefix("0b")
                .or_else(|| input.strip_prefix("0B"))
                .unwrap_or(input);
            i64::from_str_radix(input, 2)?
        }
        Radix::Octal => i64::from_str_radix(input, 8)?,
        Radix::Decimal => i64::from_str_radix(input, 10)?,
        Radix::Hex => {
            let input = input
                .strip_prefix("0x")
                .or_else(|| input.strip_prefix("0X"))
                .unwrap_or(input);
            i64::from_str_radix(input, 16)?
        }
    };
    Ok(num)
}
