use super::*;

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

#[derive(Clone, Debug, PartialEq)]
pub struct ImplFunction {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<BlockPart>,
    pub returns: Option<Type>,
    pub is_static: bool,
    pub generics: Option<GenericArguments>,
    pub where_clause: Option<Vec<WhereUnit>>,
}

impl Parse for ImplFunction {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;
        let mut is_static = true;
        let mut generics = None;
        let mut where_clause = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::impl_define_arguments => {
                    for arg_rule in rule.into_inner() {
                        match arg_rule.as_rule() {
                            Rule::define_arguments => {
                                args = Arg::parse_many(arg_rule)?;
                            }
                            Rule::self_kwd => {
                                is_static = false;
                            }
                            _ => {}
                        }
                    }
                }
                Rule::block => {
                    body = BlockPart::parse_many(rule)?;
                }
                Rule::ty => {
                    returns = Some(Type::parse(rule)?);
                }
                Rule::generic_args => {
                    generics = Some(GenericArguments::parse(rule)?);
                }
                Rule::where_clause => where_clause = Some(WhereUnit::parse_many(rule)?),
                _ => {}
            }
        }

        Some(ImplFunction {
            ident: ident?,
            args,
            body,
            returns,
            is_static,
            generics,
            where_clause,
        })
    }
}

impl ImplFunction {
    pub fn rewrite(&self) -> String {
        let args = Arg::rewrite_many(self.args.clone(), ", ");
        let body = BlockPart::rewrite_many(self.body.clone(), "\n");
        let generics = self
            .generics
            .as_ref()
            .map(|n| n.rewrite(self.where_clause.as_ref()))
            .unwrap_or("".to_string());

        format!(
            "public {} {} {} {}({}) {{\n{}\n}}",
            if self.is_static { "static" } else { "" },
            generics,
            self.returns
                .as_ref()
                .map(|n| n.rewrite())
                .unwrap_or("void".to_string()),
            rewrite_ident(&self.ident),
            args,
            body
        )
    }
}
