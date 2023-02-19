use std::{
    io::{self, Write},
    process,
};

use options::OutputOptions;

mod enums;
mod formatter;
mod options;

fn run(options: &OutputOptions) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let formatter = formatter::Formatter::new(
        options.output(),
        options.escape_style(),
        options.no_whitespace(),
        options.quote_style(),
    );
    let output = formatter.format_output();

    handle.write_all(output.as_bytes())?;
    if !options.no_newline() {
        handle.write_all(b"\n")?;
    }

    Ok(())
}

fn main() {
    if let Err(err) = run(&OutputOptions::new()) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
