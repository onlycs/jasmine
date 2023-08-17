use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    ident: String,
    data: Option<Type>,
}

impl Parse for EnumVariant {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut data = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::ty => data = Some(Type::parse(rule)?),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            data,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Enumeration {
    ident: String,
    variants: Vec<EnumVariant>,
}

impl Parse for Enumeration {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut variants = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::enum_variant => variants.push(EnumVariant::parse(rule)?),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            variants,
        })
    }
}
