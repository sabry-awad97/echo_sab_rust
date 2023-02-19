use structopt::StructOpt;

use crate::enums::{EscapeStyle, QuoteStyle};

#[derive(Debug, StructOpt)]
#[structopt(name = "echo", about = "Print text to the console")]
struct EchoOptions {
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

pub struct OutputOptions {
    no_newline: bool,
    escape_style: EscapeStyle,
    no_whitespace: bool,
    quote_style: QuoteStyle,
    output: String,
}

impl OutputOptions {
    pub fn new() -> Self {
        let options = EchoOptions::from_args();

        OutputOptions {
            no_newline: options.no_newline,
            escape_style: options.escape_style.unwrap_or_default(),
            no_whitespace: options.no_whitespace,
            quote_style: options.quote_style.unwrap_or_default(),
            output: options.text,
        }
    }

    pub fn no_newline(self: &OutputOptions) -> bool {
        self.no_newline
    }

    pub fn escape_style(self: &OutputOptions) -> EscapeStyle {
        self.escape_style
    }

    pub fn no_whitespace(self: &OutputOptions) -> bool {
        self.no_whitespace
    }

    pub fn quote_style(self: &OutputOptions) -> QuoteStyle {
        self.quote_style
    }

    pub fn output(self: &OutputOptions) -> &str {
        &self.output
    }
}
