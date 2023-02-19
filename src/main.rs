use std::io::{self, Write};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "echo", about = "Print text to the console")]
struct Options {
    #[structopt(short = "n", long, help = "Suppress the trailing newline character")]
    no_newline: bool,

    #[structopt(short = "e", long, possible_values = &["none", "basic", "extended"], help = "Enable interpretation of backslash escapes")]
    escape_style: Option<EscapeStyle>,

    #[structopt(short = "s", long, help = "Print a string without any whitespace")]
    no_whitespace: bool,

    #[structopt(short = "q", long, possible_values = &["none", "single", "double"], help = "Quote the output using the specified style")]
    quote_style: Option<QuoteStyle>,

    #[structopt(help = "The text to print")]
    text: String,
}

#[derive(Debug)]
enum EscapeStyle {
    None,
    Basic,
    Extended,
}

impl FromStr for EscapeStyle {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "none" => Ok(EscapeStyle::None),
            "basic" => Ok(EscapeStyle::Basic),
            "extended" => Ok(EscapeStyle::Extended),
            _ => Err(format!("Invalid escape style: {}", s)),
        }
    }
}

#[derive(Debug)]
enum QuoteStyle {
    None,
    Single,
    Double,
}

impl FromStr for QuoteStyle {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "single" => Ok(Self::Single),
            "double" => Ok(Self::Double),
            "none" => Ok(Self::None),
            _ => Err(format!("Invalid quote style: {}", s)),
        }
    }
}

struct Formatter<'a> {
    output: &'a str,
    escape_style: EscapeStyle,
    no_whitespace: bool,
    quote_style: QuoteStyle,
}

impl<'a> Formatter<'a> {
    fn new(
        output: &'a str,
        escape_style: EscapeStyle,
        no_whitespace: bool,
        quote_style: QuoteStyle,
    ) -> Self {
        Formatter {
            output,
            escape_style,
            no_whitespace,
            quote_style,
        }
    }

    fn format_output(&self) -> String {
        let mut result = self.output.to_string();

        match self.escape_style {
            EscapeStyle::None => (),
            EscapeStyle::Basic => {
                result = result.replace("\\n", "\n");
                result = result.replace("\\t", "\t");
            }
            EscapeStyle::Extended => {
                result = result.replace("\\\\", "\\");
                result = result.replace("\\'", "'");
                result = result.replace("\\\"", "\"");
                result = result.replace("\\n", "\n");
                result = result.replace("\\t", "\t");
            }
        }

        if self.no_whitespace {
            result = result.replace(" ", "");
        }

        let quote_char = match self.quote_style {
            QuoteStyle::None => "",
            QuoteStyle::Single => "'",
            QuoteStyle::Double => "\"",
        };

        format!("{}{}{}", quote_char, result, quote_char)
    }
}

fn main() -> io::Result<()> {
    let options = Options::from_args();

    let mut stdout = io::stdout();
    let formatter = Formatter::new(
        &options.text,
        options.escape_style.unwrap_or(EscapeStyle::None),
        options.no_whitespace,
        options.quote_style.unwrap_or(QuoteStyle::None),
    );
    let output = formatter.format_output();

    write!(stdout, "{}", output)?;

    if !options.no_newline {
        writeln!(stdout)?;
    }

    Ok(())
}
