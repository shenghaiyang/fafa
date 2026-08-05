use anyhow::Result;

use anstream::println;
use md5::{Digest as _, Md5};
use owo_colors::OwoColorize as _;
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sha3::{Sha3_256, Sha3_512};
use strum::{Display, EnumIter, IntoEnumIterator};

use crate::pretty::{gen_padding, print_footer, print_header};

#[derive(Clone, Copy, PartialEq, Eq, Display, EnumIter)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE")]
pub enum Algorithm {
    Md5,
    Sha1,
    Sha256,
    Sha512,
    Sha3_256,
    Sha3_512,
    Blake3,
}

#[derive(clap::Args)]
pub struct Args {
    input: String,
}

pub fn run(algorithm: Option<Algorithm>, args: Args) -> Result<()> {
    let input = args.input;
    if input.is_empty() {
        anyhow::bail!("input is empty");
    }
    match algorithm {
        Some(algo) => {
            let header = algo.to_string();
            print_header(&header);
            println!("{}", compute(algo, &input));
            print_footer(header.len());
        }
        None => {
            let header = "🔐 Hash";
            print_header(header);
            Algorithm::iter().for_each(|algo| {
                let pad = gen_padding(&algo.to_string(), 12);
                println!("{}{pad}{}", algo.cyan().bold(), &compute(algo, &input));
            });
            print_footer(header.len());
        }
    }
    Ok(())
}

fn compute(algo: Algorithm, input: &str) -> String {
    match algo {
        Algorithm::Md5 => hex::encode(Md5::digest(input.as_bytes())),
        Algorithm::Sha1 => hex::encode(Sha1::digest(input.as_bytes())),
        Algorithm::Sha256 => hex::encode(Sha256::digest(input.as_bytes())),
        Algorithm::Sha512 => hex::encode(Sha512::digest(input.as_bytes())),
        Algorithm::Sha3_256 => {
            let mut h = Sha3_256::new();
            h.update(input.as_bytes());
            hex::encode(h.finalize())
        }
        Algorithm::Sha3_512 => {
            let mut h = Sha3_512::new();
            h.update(input.as_bytes());
            hex::encode(h.finalize())
        }
        Algorithm::Blake3 => blake3::hash(input.as_bytes()).to_hex().to_string(),
    }
}
