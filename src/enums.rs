use std::{fmt, str::FromStr};

#[derive(Debug, Copy, Clone)]
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

impl Default for EscapeStyle {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug, Copy, Clone)]
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

impl Default for QuoteStyle {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Debug)]
pub enum EscapeSequence {
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
