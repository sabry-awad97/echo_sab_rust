use std::{
    io::{self, Write},
    process,
};

use options::Config;

mod enums;
mod formatter;
mod options;

fn run(options: &Config) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let formatter = formatter::Formatter::new(
        options.output(),
        options.escape_style(),
        options.no_whitespace(),
        options.quote_style(),
    );
    let output = formatter.format_output();

    let font_size = options.font_size();
    print!("\x1b[{}m", font_size);

    for _ in 0..options.repeat() {
        handle.write_all(output.as_bytes())?;
        if !options.no_newline() {
            handle.write_all(b"\n")?;
        }

        std::thread::sleep(std::time::Duration::from_millis(options.delay()));
    }

    if let Some(file_name) = options.output_file() {
        use std::fs::File;
        let mut file = File::create(file_name)?;
        file.write_all(output.as_bytes())?;
        if !options.no_newline() {
            file.write_all(b"\n")?;
        }
    }

    print!("\x1b[0m"); // Reset the font size
    Ok(())
}

fn main() {
    if let Err(err) = run(&Config::new()) {
        eprintln!("Error: {}", err);
        process::exit(1);
    }
}
