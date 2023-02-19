use structopt::StructOpt;

use crate::formatter::{EscapeStyle, QuoteStyle};

#[derive(Debug, StructOpt)]
#[structopt(name = "echo", about = "Print text to the console")]
pub struct Options {
    #[structopt(short = "n", long, help = "Suppress the trailing newline character")]
    pub no_newline: bool,

    #[structopt(short = "e", long, possible_values = &["none", "basic", "extended"], help = "Enable interpretation of backslash escapes")]
    pub escape_style: Option<EscapeStyle>,

    #[structopt(short = "s", long, help = "Print a string without any whitespace")]
    pub no_whitespace: bool,

    #[structopt(short = "q", long, possible_values = &["none", "single", "double"], help = "Quote the output using the specified style")]
    pub quote_style: Option<QuoteStyle>,

    #[structopt(help = "The text to print")]
    pub text: String,
}
