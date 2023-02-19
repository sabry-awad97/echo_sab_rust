use std::io::{self, Write};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo", about = "Print text to the console")]
struct Options {
    #[structopt(short = "n", long, help = "Suppress the trailing newline character")]
    no_newline: bool,

    #[structopt(short = "e", long, help = "Enable interpretation of backslash escapes")]
    enable_escapes: bool,

    #[structopt(
        short = "E",
        long,
        help = "Disable interpretation of backslash escapes"
    )]
    disable_escapes: bool,

    #[structopt(short = "s", long, help = "Print a string without any whitespace")]
    no_whitespace: bool,

    #[structopt(help = "The text to print")]
    text: String,
}

fn main() -> io::Result<()> {
    let options = Options::from_args();

    let mut stdout = io::stdout();
    let mut output = options.text;

    if options.enable_escapes {
        output = output.replace("\\n", "\n");
        output = output.replace("\\t", "\t");
        // Add more escape sequences here if needed
    }

    if options.disable_escapes {
        output = output.replace("\\", "");
    }

    if options.no_whitespace {
        output = output.replace(" ", "");
    }

    write!(stdout, "{}", output)?;

    if !options.no_newline {
        writeln!(stdout)?;
    }

    Ok(())
}
