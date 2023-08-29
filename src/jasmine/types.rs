use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureArgument {
    pub generic: bool,
    pub ty: Type,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureTypeData {
    pub args: Vec<ClosureArgument>,
    pub ret: Option<Box<ClosureArgument>>,
}

impl Parse for ClosureTypeData {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut args = vec![];
        let mut ret = None;
        let mut next_is_generic = false;

        let mut next_is_arg = true;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ty => {
                    let ty = Type::parse(rule)?;

                    if next_is_arg {
                        args.push(ClosureArgument {
                            generic: next_is_generic,
                            ty: ty,
                        })
                    } else {
                        ret = Some(ClosureArgument {
                            generic: next_is_generic,
                            ty: ty,
                        });
                    }

                    next_is_generic = false;
                }
                Rule::generic_kwd => {
                    next_is_generic = true;
                }
                Rule::rparen => {
                    next_is_arg = false;
                }
                _ => {}
            }
        }

        Some(ClosureTypeData {
            args,
            ret: ret.map(Box::new),
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum WhichType {
    Int,
    Float,
    Bool,
    String,
    Char,
    Ident(String),
    Closure(ClosureTypeData),
    Array { ty: Box<Type>, dimensions: usize },
    Generic { outer: Box<Type>, inner: Vec<Type> },
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub which: WhichType,
}

impl Parse for Type {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut which = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::int_ty => which = Some(WhichType::Int),
                Rule::float_ty => which = Some(WhichType::Float),
                Rule::char_ty => which = Some(WhichType::Char),
                Rule::string_ty => which = Some(WhichType::String),
                Rule::bool_ty => which = Some(WhichType::Bool),
                Rule::closure_ty => {
                    let data = ClosureTypeData::parse(rule)?;
                    which = Some(WhichType::Closure(data))
                }
                Rule::ident_ty => {
                    let ident = rule.as_str().to_string();
                    which = Some(WhichType::Ident(ident));
                }
                Rule::array_ty => {
                    let dimensions = rule
                        .clone()
                        .into_inner()
                        .filter(|n| n.as_rule() == Rule::lbrack)
                        .count();

                    let ty = Type::parse(rule)?;

                    which = Some(WhichType::Array {
                        ty: Box::new(ty),
                        dimensions,
                    });
                }
                Rule::generic_ty => {
                    let outer = Type::parse(rule.clone()).map(Box::new)?;

                    let inner = rule
                        .into_inner()
                        .filter(|r| r.as_rule() == Rule::ty)
                        .map(Type::parse)
                        .filter_map(|n| n)
                        .collect_vec();

                    which = Some(WhichType::Generic { outer, inner })
                }
                _ => {}
            }
        }

        Some(Type { which: which? })
    }
}

impl Type {
    pub fn rewrite(&self) -> String {
        let mut rewritten = "".to_string();

        match self.clone().which {
            WhichType::Array { ty, dimensions } => {
                let unbox_ty = ty.as_ref().clone();

                for _ in 0..dimensions {
                    rewritten.push_str("Vec<");
                }

                rewritten.push_str(&unbox_ty.rewrite());

                for _ in 0..dimensions {
                    rewritten.push('>');
                }
            }
            WhichType::Bool => {
                rewritten.push_str("Boolean");
            }
            WhichType::Char => {
                rewritten.push_str("Character");
            }
            WhichType::Float => {
                rewritten.push_str("Double");
            }
            WhichType::Int => {
                rewritten.push_str("Integer");
            }
            WhichType::Generic { outer, inner } => {
                let unboxed_outer = outer.as_ref().clone();
                let unboxed_inner = inner;

                rewritten.push_str(&format!(
                    "{}<{}>",
                    unboxed_outer.rewrite(),
                    unboxed_inner.iter().map(|n| n.rewrite()).join(", ")
                ))
            }
            WhichType::String => rewritten.push_str("String"),
            WhichType::Ident(ty) => {
                rewritten.push_str(&ty);
            }
            WhichType::Closure(data) => rewritten.push_str(&rewrite::add_closure(data)),
        }

        rewritten
    }
}
