use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use argon2::password_hash::phc::Salt;
use argon2::{Argon2, PasswordHasher};

#[derive(clap::Args)]
pub struct Args {
    pub password: String,
}

pub fn run(args: Args) -> Result<()> {
    if args.password.is_empty() {
        anyhow::bail!("password is empty");
    }
    let header = "Argon2";
    print_header(header);
    let salt = Salt::generate();
    let hash = Argon2::default()
        .hash_password_with_salt(args.password.as_bytes(), &salt)?
        .to_string();
    println!("{}", hash);
    print_footer(header.len());
    Ok(())
}
