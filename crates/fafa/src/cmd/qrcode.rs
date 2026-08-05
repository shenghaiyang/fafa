use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use qrcode::render::unicode::Dense1x2;
use qrcode::{EcLevel, QrCode};
use strum::Display;

#[derive(Clone, Copy, Display, clap::ValueEnum)]
enum Level {
    #[strum(to_string = "L 7%")]
    L,
    #[strum(to_string = "M 15%")]
    M,
    #[strum(to_string = "Q 25%")]
    Q,
    #[strum(to_string = "H 30%")]
    H,
}

impl From<Level> for EcLevel {
    fn from(value: Level) -> Self {
        match value {
            Level::L => EcLevel::L,
            Level::M => EcLevel::M,
            Level::Q => EcLevel::Q,
            Level::H => EcLevel::H,
        }
    }
}

#[derive(clap::Args)]
pub struct Args {
    text: String,
    #[arg(long, value_enum, default_value_t = Level::M)]
    level: Level,
}

pub fn run(args: Args) -> Result<()> {
    if args.text.is_empty() {
        anyhow::bail!("text is empty");
    }
    let ascii = compute_ascii(&args.text, args.level.into())?;
    let header = format!("QR Code {}", args.level);
    print_header(&header);
    for line in ascii.lines() {
        println!("  {line}");
    }
    print_footer(header.len());
    Ok(())
}

pub fn compute_ascii(text: &str, level: EcLevel) -> Result<String> {
    let code = QrCode::with_error_correction_level(text.as_bytes(), level)?;
    Ok(code.render::<Dense1x2>().quiet_zone(true).build())
}
