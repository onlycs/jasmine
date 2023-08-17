use super::*;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Impl {
    pub ident: String,
    pub methods: Vec<ImplFunction>,
}

impl Parse for Impl {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut methods = vec![];

        for impl_part in pair.into_inner() {
            match impl_part.as_rule() {
                Rule::ident => {
                    ident = Some(impl_part.as_str().to_string());
                }
                Rule::impl_fn_def => {
                    methods.push(ImplFunction::parse(impl_part)?);
                }
                _ => {}
            }
        }

        Some(Impl {
            ident: ident?,
            methods,
        })
    }
}
