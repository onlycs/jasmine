use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct StructArg {
    pub ident: String,
    pub value: Expression,
}

impl Parse for StructArg {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut value = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::expr => value = Some(Expression::parse(rule)?),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            value: value?,
        })
    }
}

impl ParseMany for StructArg {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut args = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::struct_arg => args.push(Self::parse(rule)?),
                _ => {}
            }
        }

        Some(args)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateStructure {
    pub ident: String,
    pub fields: Vec<StructArg>,
}

impl Parse for CreateStructure {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut fields = vec![];
        let mut ident = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::struct_args => fields = StructArg::parse_many(rule)?,
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            fields,
        })
    }
}

impl CreateStructure {
    pub fn rewrite(&self) -> String {
        let fields = self
            .fields
            .iter()
            .sorted_by(|a, b| a.ident.cmp(&b.ident))
            .map(|n| &n.value)
            .cloned()
            .collect_vec();

        format!(
            "new {}({})",
            self.ident,
            Expression::rewrite_many(fields, ", ")
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Structure {
    pub ident: String,
    pub fields: Vec<Arg>,
    pub generics: Option<GenericArguments>,
    pub where_clause: Option<Vec<WhereUnit>>,
}

impl Parse for Structure {
    fn parse(pair: Pair<'_, Rule>) -> Option<Structure> {
        let mut fields = vec![];
        let mut ident = None;
        let mut generics = None;
        let mut where_clause = None;

        for struct_part in pair.into_inner() {
            match struct_part.as_rule() {
                Rule::ident => {
                    ident = Some(struct_part.as_str().to_string());
                }
                Rule::define_arguments => {
                    let args = Arg::parse_many(struct_part)?;

                    fields = args;
                }
                Rule::generic_args => {
                    generics = Some(GenericArguments::parse(struct_part)?);
                }
                Rule::where_clause => {
                    where_clause = Some(WhereUnit::parse_many(struct_part)?);
                }
                _ => {}
            }
        }

        Some(Structure {
            ident: ident?,
            fields,
            generics,
            where_clause,
        })
    }
}
