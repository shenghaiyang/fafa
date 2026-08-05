use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use percent_encoding::{NON_ALPHANUMERIC, percent_decode, utf8_percent_encode};

#[derive(clap::Args)]
pub struct EncodeArgs {
    pub text: String,
}

#[derive(clap::Args)]
pub struct DecodeArgs {
    pub text: String,
}

pub fn run_encode(args: EncodeArgs) -> Result<()> {
    let header = "URL Encode";
    print_header(header);
    println!("{}", encode(&args.text));
    print_footer(header.len());
    Ok(())
}

pub fn run_decode(args: DecodeArgs) -> Result<()> {
    let header = "URL Decode";
    print_header(header);
    println!("{}", decode(&args.text));
    print_footer(header.len());
    Ok(())
}

fn encode(input: &str) -> String {
    utf8_percent_encode(input, NON_ALPHANUMERIC).to_string()
}

fn decode(input: &str) -> String {
    percent_decode(input.as_bytes())
        .decode_utf8()
        .map(|c| c.into_owned())
        .unwrap_or_else(|_| input.to_string())
}
