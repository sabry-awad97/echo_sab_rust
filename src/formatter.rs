use std::{str::FromStr, fmt};


#[derive(Debug)]
pub enum EscapeStyle {
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
pub enum QuoteStyle {
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

#[derive(Debug)]
enum EscapeSequence {
    Tab,
    Newline,
    Backslash,
    SingleQuote,
    DoubleQuote,
}

impl fmt::Display for EscapeSequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let printable = match self {
            EscapeSequence::Backslash => "\\",
            EscapeSequence::Newline => "\n",
            EscapeSequence::Tab => "\t",
            EscapeSequence::SingleQuote => "'",
            EscapeSequence::DoubleQuote => "\"",
        };
        write!(f, "{}", printable)
    }
}

pub struct Formatter<'a> {
    output: &'a str,
    escape_style: EscapeStyle,
    no_whitespace: bool,
    quote_style: QuoteStyle,
}

impl<'a> Formatter<'a> {
    pub fn new(
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

    pub fn format_output(&self) -> String {
        let mut result = self.output.to_string();
        let single_quote = EscapeSequence::SingleQuote.to_string();
        let double_quote = EscapeSequence::DoubleQuote.to_string();

        match self.escape_style {
            EscapeStyle::None => (),
            EscapeStyle::Basic => {
                result = result.replace("\\n", &EscapeSequence::Newline.to_string());
                result = result.replace("\\t", &EscapeSequence::Tab.to_string());
            }
            EscapeStyle::Extended => {
                result = result.replace("\\\\", &EscapeSequence::Backslash.to_string());
                result = result.replace("\\n", &EscapeSequence::Newline.to_string());
                result = result.replace("\\t", &EscapeSequence::Tab.to_string());
                result = result.replace("\\'", &single_quote);
                result = result.replace("\\\"", &double_quote);
            }
        }

        if self.no_whitespace {
            result = result.replace(" ", "");
        }

        let quote_char = match self.quote_style {
            QuoteStyle::None => "",
            QuoteStyle::Single => &single_quote,
            QuoteStyle::Double => &double_quote,
        };

        format!("{}{}{}", quote_char, result, quote_char)
    }
}
