use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum WhereType {
    Extends,
    Implements,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WhereUnit {
    pub kind: WhereType,
    pub generic: String,
    pub constraint: String,
}

impl Parse for WhereUnit {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut generic = None;
        let mut constraint = None;
        let mut kind = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if generic.is_none() => generic = Some(rule.as_str().to_string()),
                Rule::ident if generic.is_some() => constraint = Some(rule.as_str().to_string()),
                Rule::assign => kind = Some(WhereType::Extends),
                Rule::colon => kind = Some(WhereType::Implements),
                _ => {}
            }
        }

        Some(Self {
            kind: kind?,
            generic: generic?,
            constraint: constraint?,
        })
    }
}

impl ParseMany for WhereUnit {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut units = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::where_unit => {
                    units.push(WhereUnit::parse(rule)?);
                }
                _ => {}
            }
        }

        Some(units)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct GenericArguments {
    pub args: Vec<String>,
}

impl Parse for GenericArguments {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut args = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => args.push(rule.as_str().to_string()),
                _ => {}
            }
        }

        Some(Self { args })
    }
}

impl GenericArguments {
    pub fn rewrite(&self, where_clause: Option<&Vec<WhereUnit>>) -> String {
        let mut rewritten = "".to_string();
        rewritten.push('<');

        for arg in self.clone().args {
            rewritten.push_str(&arg);

            if let Some(where_unit) = where_clause
                .as_ref()
                .map(|n| n.iter().find(|n| n.generic == arg))
                .flatten()
                .cloned()
            {
                match where_unit.kind {
                    WhereType::Extends => rewritten.push_str(" extends "),
                    WhereType::Implements => rewritten.push_str(" extends "),
                }

                rewritten.push_str(&where_unit.constraint);
            }

            rewritten.push(',')
        }

        rewritten.pop();

        rewritten.push('>');

        rewritten
    }
}
