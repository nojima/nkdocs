use std::{error, io::{self, Read}};

mod markdown;
mod renderer;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut buffer = String::new();
    let mut stdin = io::stdin();
    stdin.read_to_string(&mut buffer)?;

    let renderer = renderer::HtmlRenderer {};
    let html = markdown::parse(&buffer, &renderer)?;
    print!("{}", html);

    Ok(())
}
