use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct EnumVariant {
    pub ident: String,
    pub data: Option<Type>,
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
    pub ident: String,
    pub variants: Vec<EnumVariant>,

    pub generics: Option<GenericArguments>,
    pub where_clause: Option<Vec<WhereUnit>>,
}

impl Parse for Enumeration {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut variants = vec![];

        let mut generics = None;
        let mut where_clause = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::enum_variant => variants.push(EnumVariant::parse(rule)?),

                Rule::generic_args => {
                    generics = Some(GenericArguments::parse(rule)?);
                }
                Rule::where_clause => {
                    where_clause = Some(WhereUnit::parse_many(rule)?);
                }
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            variants,
            generics,
            where_clause,
        })
    }
}
