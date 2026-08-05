mod cmd;
mod pretty;

use crate::cmd::hash::{
    Algorithm::{Blake3, Md5, Sha1, Sha3_256, Sha3_512, Sha256, Sha512},
    Args,
};
use anyhow::Result;
use clap::{Parser, Subcommand};
use cmd::{argon2, base, base64, bcrypt, color, hash, qrcode, text_case, timestamp, url, uuid};

#[derive(Parser)]
#[command(
    name = "fafa",
    version,
    about = "A developer toolbox command-line tool"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Compute hash digests (md5/sha1/sha256/sha512/sha3/blake3)")]
    Hash(Args),
    #[command(about = "Compute an MD5 digest")]
    Md5(Args),
    #[command(about = "Compute a SHA-1 digest")]
    Sha1(Args),
    #[command(about = "Compute a SHA-256 digest")]
    Sha256(Args),
    #[command(about = "Compute a SHA-512 digest")]
    Sha512(Args),
    #[command(about = "Compute a SHA3-256 digest")]
    Sha3_256(Args),
    #[command(about = "Compute a SHA3-512 digest")]
    Sha3_512(Args),
    #[command(about = "Compute a BLAKE3 digest")]
    Blake3(Args),
    #[command(about = "Generate an Argon2 password hash")]
    Argon2(argon2::Args),
    #[command(about = "Generate a bcrypt password hash")]
    Bcrypt(bcrypt::Args),
    #[command(about = "Generate a QR code as SVG")]
    Qrcode(qrcode::Args),
    #[command(about = "Convert between timestamps and dates")]
    Timestamp(timestamp::Args),
    #[command(about = "Percent-encode a URL")]
    UrlEncode(url::EncodeArgs),
    #[command(about = "Percent-decode a URL")]
    UrlDecode(url::DecodeArgs),
    #[command(about = "Convert numbers between bases (2/8/10/16)")]
    Base(base::Args),
    #[command(about = "Generate UUIDs (v4/v7)")]
    Uuid(uuid::Args),
    #[command(about = "Base64 encode")]
    Base64Encode(base64::EncodeArgs),
    #[command(about = "Base64 decode")]
    Base64Decode(base64::DecodeArgs),
    #[command(about = "Convert colors between hex/rgba/hsla")]
    Color(color::Args),
    #[command(about = "Convert text between 8 naming styles")]
    TextCase(text_case::Args),
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        // Hash
        Command::Hash(args) => hash::run(None, args),
        Command::Md5(args) => hash::run(Some(Md5), args),
        Command::Sha1(args) => hash::run(Some(Sha1), args),
        Command::Sha256(args) => hash::run(Some(Sha256), args),
        Command::Sha512(args) => hash::run(Some(Sha512), args),
        Command::Sha3_256(args) => hash::run(Some(Sha3_256), args),
        Command::Sha3_512(args) => hash::run(Some(Sha3_512), args),
        Command::Blake3(args) => hash::run(Some(Blake3), args),
        // Password Hash
        Command::Argon2(args) => argon2::run(args),
        Command::Bcrypt(args) => bcrypt::run(args),
        // QRCode
        Command::Qrcode(args) => qrcode::run(args),
        Command::Timestamp(args) => timestamp::run(args),
        Command::UrlEncode(args) => url::run_encode(args),
        Command::UrlDecode(args) => url::run_decode(args),
        Command::Base(args) => base::run(args),
        Command::Uuid(args) => uuid::run(args),
        Command::Base64Encode(args) => base64::run_encode(args),
        Command::Base64Decode(args) => base64::run_decode(args),
        Command::Color(args) => color::run(args),
        Command::TextCase(args) => text_case::run(args),
    }
}
