use crate::pretty::{print_footer, print_header, println_key_value};
use anyhow::Result;
use heck::{
    ToKebabCase, ToLowerCamelCase, ToShoutyKebabCase, ToShoutySnakeCase, ToSnakeCase, ToTitleCase,
    ToTrainCase, ToUpperCamelCase,
};

#[derive(clap::Args)]
pub struct Args {
    text: String,
}

const MAX_LEN: usize = 20;

pub fn run(args: Args) -> Result<()> {
    let input = args.text;
    if input.is_empty() {
        return Ok(());
    }
    let header = "Text Case";
    print_header(header);
    println_key_value("lowerCamelCase", input.to_lower_camel_case(), MAX_LEN);
    println_key_value("UpperCamelCase", input.to_upper_camel_case(), MAX_LEN);
    println_key_value("snake_case", input.to_snake_case(), MAX_LEN);
    println_key_value("SHOUTY_SNAKE_CASE", input.to_shouty_snake_case(), MAX_LEN);
    println_key_value("kebab-case", input.to_kebab_case(), MAX_LEN);
    println_key_value("SHOUTY-KEBAB-CASE", input.to_shouty_kebab_case(), MAX_LEN);
    println_key_value("Train-Case", input.to_train_case(), MAX_LEN);
    println_key_value("Title Case", input.to_title_case(), MAX_LEN);
    print_footer(header.len());
    Ok(())
}
