use anyhow::{Result, bail};
use clap::Subcommand;
use jiff::civil::DateTime;
use jiff::fmt::strtime;
use jiff::tz::{Offset, TimeZone};
use jiff::{Timestamp, Zoned};

use crate::pretty::{print_footer, print_header};
use anstream::println;

#[derive(Subcommand)]
pub enum Command {
    Now {
        #[arg(long)]
        millis: bool,
        #[arg(long, default_value_t = 8)]
        offset: i8,
    },
    From {
        epoch: i64,
        #[arg(long)]
        millis: bool,
        #[arg(long, default_value_t = 8)]
        offset: i8,
    },
    To {
        datetime: String,
        #[arg(long, default_value_t = 8)]
        offset: i8,
    },
}

#[derive(clap::Args)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}

pub fn run(args: Args) -> Result<()> {
    let (input, mode, millis, offset) = match args.command {
        Command::Now { millis, offset } => (String::new(), "now", millis, offset),
        Command::From {
            epoch,
            millis,
            offset,
        } => (epoch.to_string(), "from", millis, offset),
        Command::To { datetime, offset } => (datetime, "to", false, offset),
    };
    let header = match mode {
        "now" => "Timestamp (now)",
        "from" => "Timestamp to date",
        _ => "Date to timestamp",
    };
    print_header(header);
    let tz = fixed_tz(offset)?;
    match mode {
        "now" => {
            let ts = Timestamp::now();
            println!(
                "{}",
                if millis {
                    ts.as_millisecond().to_string()
                } else {
                    ts.as_second().to_string()
                }
            );
        }
        "from" => {
            let epoch: i64 = input
                .trim()
                .parse()
                .map_err(|_| anyhow::anyhow!("invalid epoch: {input}"))?;
            let (secs, subsec) = if millis {
                (
                    epoch.div_euclid(1000),
                    (epoch.rem_euclid(1000) as i32) * 1_000_000,
                )
            } else {
                (epoch, 0)
            };
            let ts = Timestamp::new(secs, subsec)?;
            let zdt = Zoned::new(ts, tz);
            let utc = Zoned::new(ts, TimeZone::UTC);
            println!("{} {}", format_zoned(&zdt)?, format_offset(offset));
            println!("{} UTC", format_zoned(&utc)?);
        }
        "to" => {
            let (base, frac_ms) = split_frac(&input);
            let dt = DateTime::strptime("%Y-%m-%d %H:%M:%S", base)?;
            let zdt = dt.to_zoned(tz)?;
            let seconds = zdt.timestamp().as_second();
            println!("{seconds}");
            println!("{}", seconds * 1000 + frac_ms);
        }
        _ => bail!("unknown mode: {mode}"),
    }
    print_footer(header.len());
    Ok(())
}

fn fixed_tz(offset: i8) -> Result<TimeZone> {
    if !(-12..=14).contains(&offset) {
        bail!("offset must be in -12..=14");
    }
    Ok(TimeZone::fixed(Offset::from_seconds(
        i32::from(offset) * 3600,
    )?))
}

fn format_zoned(zdt: &Zoned) -> Result<String> {
    let mut s = strtime::format("%Y-%m-%d %H:%M:%S", zdt)?;
    if zdt.subsec_nanosecond() != 0 {
        s.push_str(&format!(".{:03}", zdt.subsec_nanosecond() / 1_000_000));
    }
    Ok(s)
}

fn format_offset(offset: i8) -> String {
    if offset == 0 {
        "UTC".to_string()
    } else {
        format!("UTC{offset:+}")
    }
}

fn split_frac(input: &str) -> (&str, i64) {
    match input.rsplit_once('.') {
        Some((base, frac)) if !frac.is_empty() && frac.chars().all(|c| c.is_ascii_digit()) => {
            let mut digits = frac.chars().take(3).collect::<String>();
            while digits.len() < 3 {
                digits.push('0');
            }
            (base, digits.parse::<i64>().unwrap_or(0))
        }
        _ => (input, 0),
    }
}
