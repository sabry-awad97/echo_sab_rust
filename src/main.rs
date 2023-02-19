use formatter::Formatter;
use std::io::{self, Write};
use structopt::StructOpt;

mod formatter;
mod options;

fn main() -> io::Result<()> {
    let options = options::Options::from_args();

    let mut stdout = io::stdout();
    let formatter = Formatter::new(
        &options.text,
        options.escape_style.unwrap_or(formatter::EscapeStyle::None),
        options.no_whitespace,
        options.quote_style.unwrap_or(formatter::QuoteStyle::None),
    );
    let output = formatter.format_output();

    write!(stdout, "{}", output)?;

    if !options.no_newline {
        writeln!(stdout)?;
    }

    Ok(())
}
