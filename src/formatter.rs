use crate::enums::{EscapeSequence, EscapeStyle, QuoteStyle};

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
