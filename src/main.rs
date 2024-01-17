use clap::Parser;
use failure::Error;
use std::io;
use std::fs;
use std::path::Path;
mod converter;

#[derive(Parser)]
#[command(
    author = "Madman’s Nest",
    version = "42.0",
    about = "Various Qazaq alphabets converter",
    long_about = ""
)]

struct Options {
    /// An optional file to read input from, defaults to stdin
    input_file: Option<String>,
    /// An optional file to write output to, defaults to stdout
    #[arg(short, long)]
    output_file: Option<String>,
}

fn main() -> Result<(), Error> {
    let options = Options::parse();
    let text = read_from_file_or_stdin(&options.input_file).unwrap();
    let output = converter::convert(&text);
    write_to_file_or_stdout(&output.into_owned(), &options.output_file)?;
    Ok(())
}

fn read_from_file_or_stdin(filename: &Option<String>) -> Result<String, io::Error> {
    let input = match filename {
        Some(s) => fs::read_to_string(s)?,
        None => io::read_to_string(io::stdin())?,
    };
    Ok(input)
}

fn write_to_file_or_stdout(s: &str, filename: &Option<String>) -> io::Result<()> {
    match filename {
        Some(f) => {
            let path = Path::new(&f);
            fs::write(path, s)?;
        }
        None => { print!("{}", s); }
    };
    Ok(())
}

