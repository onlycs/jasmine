use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Escape {
    Newline,
    Tab,
    CarriageReturn,
    Backslash,
    SingleQuote,
    DoubleQuote,
    NullByte,
    Unicode(String),
}

impl Parse for Escape {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        if let Some(rule) = pair.into_inner().next() {
            let rule_str = rule.as_str();

            match rule.as_rule() {
                Rule::escape_predefined if rule_str == "n" => Some(Escape::Newline),
                Rule::escape_predefined if rule_str == "t" => Some(Escape::Tab),
                Rule::escape_predefined if rule_str == "r" => Some(Escape::CarriageReturn),
                Rule::escape_predefined if rule_str == "\\" => Some(Escape::Backslash),
                Rule::escape_predefined if rule_str == "'" => Some(Escape::SingleQuote),
                Rule::escape_predefined if rule_str == "\"" => Some(Escape::DoubleQuote),
                Rule::escape_predefined if rule_str == "0" => Some(Escape::NullByte),
                Rule::unicode_escape => {
                    // in format u{XXXX} (4 digits)
                    let mut chars = rule_str.chars();

                    // remove first 2
                    chars.next();
                    chars.next();

                    // remove last 1
                    chars.next_back();

                    let unicode_digits = chars.as_str().to_string();

                    Some(Escape::Unicode(unicode_digits))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

impl Escape {
    pub fn rewrite(&self) -> String {
        match self {
            Escape::Newline => "\\n".to_string(),
            Escape::Tab => "\\t".to_string(),
            Escape::CarriageReturn => "\\r".to_string(),
            Escape::Backslash => "\\\\".to_string(),
            Escape::SingleQuote => "\\'".to_string(),
            Escape::DoubleQuote => "\\\"".to_string(),
            Escape::NullByte => "\\0".to_string(),
            Escape::Unicode(digits) => format!("\\u{}", digits),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum CharDecl {
    RawChar(char),
    EscapeChar(Escape),
}

impl Parse for CharDecl {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::raw_char => Some(CharDecl::RawChar(pair.as_str().chars().next()?)),
            Rule::escape => Some(CharDecl::EscapeChar(Escape::parse(pair)?)),
            _ => None,
        }
    }
}

impl ParseMany for CharDecl {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut chars = vec![];

        for char_decl in pair.into_inner() {
            match char_decl.as_rule() {
                Rule::raw_char | Rule::escape => chars.push(CharDecl::parse(char_decl)?),
                _ => {}
            }
        }

        Some(chars)
    }
}

impl CharDecl {
    pub fn rewrite(&self) -> String {
        match self {
            CharDecl::RawChar(ch) => ch.to_string(),
            CharDecl::EscapeChar(esc) => esc.rewrite(),
        }
    }

    pub fn rewrite_many(all: Vec<CharDecl>, sep: &'static str) -> String {
        all.iter().map(|c| c.rewrite()).join(sep)
    }
}
