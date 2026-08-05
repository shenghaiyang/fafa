use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use strum::Display;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Eq, Display, clap::ValueEnum)]
enum Version {
    V4,
    V7,
}

#[derive(clap::Args)]
pub struct Args {
    #[arg(long, default_value_t = 1)]
    count: usize,
    #[arg(long, value_enum, default_value_t = Version::V7)]
    version: Version,
    #[arg(long)]
    no_hyphens: bool,
}

pub fn run(args: Args) -> Result<()> {
    let is_v4 = args.version == Version::V4;
    let header = format!("# UUID {}", args.version);
    print_header(&header);
    for _ in 0..args.count {
        let uuid = if is_v4 {
            Uuid::new_v4()
        } else {
            Uuid::now_v7()
        };
        let uuid = if args.no_hyphens {
            uuid.simple().to_string()
        } else {
            uuid.hyphenated().to_string()
        };
        println!("{}", uuid);
    }
    print_footer(header.len());
    Ok(())
}
