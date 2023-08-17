use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct ClosureTypeData {
    pub args: Vec<Type>,
    pub ret: Option<Box<Type>>,
}

impl Parse for ClosureTypeData {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut args = vec![];
        let mut ret = None;

        let mut next_is_arg = true;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ty => {
                    let ty = Type::parse(rule)?;

                    if next_is_arg {
                        args.push(ty);
                    } else {
                        ret = Some(ty);
                    }
                }
                Rule::comma => {
                    next_is_arg = true;
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
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorrowType {
    Borrow,
    MutBorrow,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Type {
    pub borrows: Vec<BorrowType>,
    pub which: WhichType,
}

impl Parse for Type {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut borrows = vec![];
        let mut which = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::borrow_kwd => borrows.push(BorrowType::Borrow),
                Rule::mut_kwd => {
                    let len = borrows.len();
                    borrows[len - 1] = BorrowType::MutBorrow;
                }
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
                _ => {}
            }
        }

        Some(Type {
            borrows,
            which: which?,
        })
    }
}
