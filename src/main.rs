use std::{
    io::{self, Write},
    process,
};
use structopt::StructOpt;

mod enums;
mod formatter;
mod options;

fn run(options: options::Options) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let formatter = formatter::Formatter::new(
        &options.text,
        options.escape_style.unwrap_or_default(),
        options.no_whitespace,
        options.quote_style.unwrap_or_default(),
    );
    let output = formatter.format_output();

    handle.write_all(output.as_bytes())?;
    if !options.no_newline {
        handle.write_all(b"\n")?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run(options::Options::from_args()) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
