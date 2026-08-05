use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use base64::Engine as _;
use base64::alphabet::{STANDARD, URL_SAFE};
use base64::engine::general_purpose::GeneralPurpose;
use base64::engine::{DecodePaddingMode, GeneralPurposeConfig};

#[derive(clap::Args)]
pub struct EncodeArgs {
    pub text: String,
    #[arg(long)]
    pub url_safe: bool,
    #[arg(long)]
    pub no_padding: bool,
}

#[derive(clap::Args)]
pub struct DecodeArgs {
    pub text: String,
    #[arg(long)]
    pub url_safe: bool,
}

pub fn run_encode(args: EncodeArgs) -> Result<()> {
    let header = "Base64 Encode";
    print_header(header);
    println!("{}", encode(&args.text, args.url_safe, !args.no_padding));
    print_footer(header.len());
    Ok(())
}

pub fn run_decode(args: DecodeArgs) -> Result<()> {
    let header = "Base64 Decode";
    let decoded = decode(&args.text, args.url_safe)?;
    print_header(header);
    println!("{}", decoded);
    print_footer(header.len());
    Ok(())
}

fn encode(input: &str, url_safe: bool, padded: bool) -> String {
    let alphabet = if url_safe { &URL_SAFE } else { &STANDARD };
    let config = GeneralPurposeConfig::new().with_encode_padding(padded);
    let engine = GeneralPurpose::new(alphabet, config);
    engine.encode(input.as_bytes())
}

fn decode(input: &str, url_safe: bool) -> Result<String> {
    let alphabet = if url_safe { &URL_SAFE } else { &STANDARD };
    let config =
        GeneralPurposeConfig::new().with_decode_padding_mode(DecodePaddingMode::Indifferent);
    let engine = GeneralPurpose::new(alphabet, config);
    let bytes = engine.decode(input.as_bytes())?;
    Ok(match String::from_utf8(bytes) {
        Ok(s) => s,
        Err(e) => hex::encode_upper(e.into_bytes()),
    })
}
