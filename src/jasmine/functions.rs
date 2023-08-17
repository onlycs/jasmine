use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Function {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<BlockPart>,
    pub returns: Option<Type>,
}

impl Parse for Function {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;

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
                _ => {}
            }
        }

        Some(Function {
            ident: ident?,
            args,
            body,
            returns,
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ImplFnType {
    Consume,
    Ref,
    MutRef,
    Static,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ImplFunction {
    pub ident: String,
    pub args: Vec<Arg>,
    pub body: Vec<BlockPart>,
    pub returns: Option<Type>,
    pub self_type: ImplFnType,
}

impl Parse for ImplFunction {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut args = vec![];
        let mut body = vec![];
        let mut returns = None;
        let mut self_type = ImplFnType::Static;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::impl_define_arguments => {
                    for arg_rule in rule.into_inner() {
                        match arg_rule.as_rule() {
                            Rule::define_arguments => {
                                args = Arg::parse_many(arg_rule)?;
                            }
                            Rule::borrow_kwd => {
                                self_type = ImplFnType::Ref;
                            }
                            Rule::mut_kwd => {
                                self_type = ImplFnType::MutRef;
                            }
                            Rule::self_kwd if self_type == ImplFnType::Static => {
                                self_type = ImplFnType::Consume;
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
                _ => {}
            }
        }

        Some(ImplFunction {
            ident: ident?,
            args,
            body,
            returns,
            self_type,
        })
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
