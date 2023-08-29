use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct Arg {
    pub ident: String,
    pub ty: Type,
}

impl Parse for Arg {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut ty = None;

        for arg_part in pair.into_inner() {
            match arg_part.as_rule() {
                Rule::ident => ident = Some(arg_part.as_str().to_owned()),
                Rule::ty => ty = Some(Type::parse(arg_part)?),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            ty: ty?,
        })
    }
}

impl ParseMany for Arg {
    /// parse from the define_arguments rule
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut args = Vec::new();

        for arg in pair.into_inner() {
            match arg.as_rule() {
                Rule::define_argument => args.push(Self::parse(arg)?),
                _ => {}
            }
        }

        Some(args)
    }
}

impl Arg {
    pub fn rewrite(&self) -> String {
        format!("{} {}", self.ty.rewrite(), rewrite_ident(&self.ident))
    }

    pub fn rewrite_many(args: Vec<Arg>, sep: &'static str) -> String {
        args.iter().map(|n| n.rewrite()).join(sep)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CallArg {
    pub expr: Expression,
}

impl Parse for CallArg {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut expr = None;

        for call_arg_part in pair.into_inner() {
            match call_arg_part.as_rule() {
                Rule::expr => expr = Some(Expression::parse(call_arg_part)?),
                _ => {}
            }
        }

        Some(Self { expr: expr? })
    }
}

impl ParseMany for CallArg {
    /// parse from the call_arguments rule
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut args = Vec::new();

        for arg in pair.into_inner() {
            match arg.as_rule() {
                Rule::call_argument => args.push(Self::parse(arg)?),
                _ => {}
            }
        }

        Some(args)
    }
}

impl CallArg {
    pub fn rewrite(&self) -> String {
        self.expr.rewrite()
    }

    pub fn rewrite_many(args: Vec<CallArg>, sep: &'static str) -> String {
        args.iter().map(|n| n.rewrite()).join(sep)
    }
}
