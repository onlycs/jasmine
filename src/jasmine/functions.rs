use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<BlockPart>,
    pub returns: Option<Type>,
    pub generics: Option<GenericArguments>,
    pub where_clause: Option<Vec<WhereUnit>>,
}

impl Parse for Function {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;
        let mut generics = None;
        let mut where_clause = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::define_arguments => {
                    args = Arg::parse_many(rule)?;
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

        Some(Function {
            ident: ident?,
            args,
            body,
            returns,
            generics,
            where_clause,
        })
    }
}

impl Function {
    pub fn rewrite(&self) -> String {
        let generics = self
            .generics
            .as_ref()
            .map(|n| n.rewrite(self.where_clause.as_ref()))
            .unwrap_or("".to_string());

        format!(
            "
			public static {} {} {}({}) {{
				{}
			}}
			",
            generics,
            self.returns
                .as_ref()
                .map(|n| n.rewrite())
                .unwrap_or("void".to_string()),
            rewrite_ident(&self.ident),
            Arg::rewrite_many(self.args.clone(), ", "),
            BlockPart::rewrite_many(self.body.clone(), "\n")
        )
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

#[derive(Clone, Debug, PartialEq)]
pub struct Closure {
    pub args: Vec<Arg>,
    pub body: Vec<BlockPart>,
    pub returns: Option<Type>,
}

impl Parse for Closure {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::define_arguments => {
                    args = Arg::parse_many(rule)?;
                }
                Rule::block => {
                    body = BlockPart::parse_many(rule)?;
                }
                Rule::ty => {
                    returns = Some(Type::parse(rule)?);
                }
                _ => {}
            }
        }

        Some(Closure {
            args,
            body,
            returns,
        })
    }
}

impl Closure {
    pub fn rewrite(&self) -> String {
        let args = Arg::rewrite_many(self.args.clone(), ", ");
        let body = BlockPart::rewrite_many(self.body.clone(), "\n");

        format!("({}) -> {{{}}}", args, body)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionCall {
    pub ident: String,
    pub args: Vec<CallArg>,
}

impl Parse for FunctionCall {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::call_arguments => args = CallArg::parse_many(rule)?,
                _ => {}
            }
        }

        Some(FunctionCall {
            ident: ident?,
            args,
        })
    }
}

impl FunctionCall {
    pub fn rewrite(&self) -> String {
        if self.ident == "panic" {
            return format!(
                "throw new RuntimeException({})",
                CallArg::rewrite_many(self.args.clone(), ", ")
            );
        }

        let mut rewritten = format!("{}(", rewrite_ident(&self.ident));

        rewritten.push_str(&CallArg::rewrite_many((&self.args).clone(), ", "));

        rewritten.push(')');

        rewritten
    }
}
