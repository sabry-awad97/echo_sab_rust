use structopt::StructOpt;

use crate::enums::{EscapeStyle, QuoteStyle};

#[derive(Debug, StructOpt)]
#[structopt(
    name = "echo",
    about = "Print text to the console",
    author = "Sabry <dr.sabry@gmail.com>",
    version = "0.1.0",
    about = "Rust echo"
)]
struct EchoOptions {
    #[structopt(short = "n", long, help = "Suppress the trailing newline character")]
    no_newline: bool,

    #[structopt(short = "e", long, possible_values = &["none", "basic", "extended"], help = "Enable interpretation of backslash escapes")]
    escape_style: Option<EscapeStyle>,

    #[structopt(short = "s", long, help = "Print a string without any whitespace")]
    no_whitespace: bool,

    #[structopt(short = "q", long, possible_values = &["none", "single", "double"], help = "Quote the output using the specified style")]
    quote_style: Option<QuoteStyle>,

    #[structopt(short = "o", long, help = "The file to write the output to")]
    output_file: Option<String>,

    #[structopt(
        short = "r",
        long,
        help = "Print the text multiple times, with an optional delay between each iteration"
    )]
    repeat: Option<u32>,

    #[structopt(
        short = "d",
        long,
        help = "The delay, in milliseconds, between each iteration of the text"
    )]
    delay: Option<u64>,

    #[structopt(short = "f", long, help = "The font size of the text")]
    font_size: Option<u32>,

    #[structopt(help = "The text to print")]
    text: String,
}

pub struct Config {
    no_newline: bool,
    escape_style: EscapeStyle,
    no_whitespace: bool,
    quote_style: QuoteStyle,
    output: String,
    output_file: Option<String>,
    font_size: u32,
    repeat: u32,
}

impl Config {
    pub fn new() -> Self {
        let options = EchoOptions::from_args();

        Config {
            no_newline: options.no_newline,
            escape_style: options.escape_style.unwrap_or_default(),
            no_whitespace: options.no_whitespace,
            quote_style: options.quote_style.unwrap_or_default(),
            output: options.text,
            output_file: options.output_file,
            font_size: options.font_size.unwrap_or(12),
            repeat: options.repeat.unwrap_or(1),
        }
    }

    pub fn no_newline(self: &Config) -> bool {
        self.no_newline
    }

    pub fn escape_style(self: &Config) -> EscapeStyle {
        self.escape_style
    }

    pub fn no_whitespace(self: &Config) -> bool {
        self.no_whitespace
    }

    pub fn quote_style(self: &Config) -> QuoteStyle {
        self.quote_style
    }

    pub fn output(self: &Config) -> &str {
        &self.output
    }

    pub fn output_file(self: &Config) -> &Option<String> {
        &self.output_file
    }

    pub fn font_size(self: &Config) -> &u32 {
        &self.font_size
    }

    pub fn repeat(self: &Config) -> u32 {
        self.repeat
    }
}
