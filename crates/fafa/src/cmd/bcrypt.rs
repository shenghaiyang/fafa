use crate::pretty::{print_footer, print_header};
use anstream::println;
use anyhow::Result;
use bcrypt::{DEFAULT_COST, hash as bcrypt_hash};

#[derive(clap::Args)]
pub struct Args {
    pub password: String,
}

pub fn run(args: Args) -> Result<()> {
    if args.password.is_empty() {
        anyhow::bail!("password is empty");
    }
    let header = "Bcrypt";
    print_header(header);
    let hash = bcrypt_hash(&args.password, DEFAULT_COST)?;
    println!("{}", hash);
    print_footer(header.len());
    Ok(())
}
