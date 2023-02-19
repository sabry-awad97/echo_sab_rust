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

        if let EscapeStyle::Basic = self.escape_style {
            result = replace_basic_escapes(result);
        } else if let EscapeStyle::Extended = self.escape_style {
            result = replace_extended_escapes(result);
        }

        if self.no_whitespace {
            result = remove_whitespace(result);
        }

        
        let quote_char = match self.quote_style {
            QuoteStyle::None => "",
            QuoteStyle::Single => &single_quote,
            QuoteStyle::Double => &double_quote,
        };

        format!("{}{}{}", quote_char, result, quote_char)
    }
}

fn replace_basic_escapes(mut output: String) -> String {
    output = output.replace("\\n", &EscapeSequence::Newline.to_string());
    output = output.replace("\\t", &EscapeSequence::Tab.to_string());
    output
}

fn replace_extended_escapes(mut output: String) -> String {
    output = output.replace("\\\\", &EscapeSequence::Backslash.to_string());
    output = output.replace("\\n", &EscapeSequence::Newline.to_string());
    output = output.replace("\\t", &EscapeSequence::Tab.to_string());
    output = output.replace("\\'", &EscapeSequence::SingleQuote.to_string());
    output = output.replace("\\\"", &EscapeSequence::DoubleQuote.to_string());
    output
}

fn remove_whitespace(mut output: String) -> String {
    output = output.replace(" ", "");
    output
}
