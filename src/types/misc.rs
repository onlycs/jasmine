use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Escape {
    Newline,
    Tab,
    CarriageReturn,
    Backslash,
    SingleQuote,
    DoubleQuote,
    NullByte,
    Unicode(String),
}

impl Parse for Escape {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        if let Some(rule) = pair.into_inner().next() {
            let rule_str = rule.as_str();

            match rule.as_rule() {
                Rule::escape_predefined if rule_str == "n" => Some(Escape::Newline),
                Rule::escape_predefined if rule_str == "t" => Some(Escape::Tab),
                Rule::escape_predefined if rule_str == "r" => Some(Escape::CarriageReturn),
                Rule::escape_predefined if rule_str == "\\" => Some(Escape::Backslash),
                Rule::escape_predefined if rule_str == "'" => Some(Escape::SingleQuote),
                Rule::escape_predefined if rule_str == "\"" => Some(Escape::DoubleQuote),
                Rule::escape_predefined if rule_str == "0" => Some(Escape::NullByte),
                Rule::unicode_escape => {
                    // in format u{XXXXXX} (4-6 digits)
                    let mut chars = rule_str.chars();

                    // remove first 2
                    chars.next();
                    chars.next();

                    // remove last 1
                    chars.next_back();

                    let unicode_digits = chars.as_str().to_string();

                    Some(Escape::Unicode(unicode_digits))
                }
                _ => None,
            }
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FnCall {
    pub ident: String,
    pub args: Vec<CallArg>,
}

impl Parse for FnCall {
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

        Some(FnCall {
            ident: ident?,
            args,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum InBlock {
    Var(VarDef),
    Expr(Expr),
    Stmt(Stmt),
    BreakKwd,
    ContinueKwd,
    Return(Option<Expr>),
    If(IfDef),
    While(WhileDef),
    For(ForDef),
}

impl Parse for InBlock {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner = pair.into_inner().next()?;

        match inner.as_rule() {
            Rule::var => Some(InBlock::Var(VarDef::parse(inner)?)),
            Rule::expr => Some(InBlock::Expr(Expr::parse(inner)?)),
            Rule::stmt => Some(InBlock::Stmt(Stmt::parse(inner)?)),
            Rule::break_kwd => Some(InBlock::BreakKwd),
            Rule::continue_kwd => Some(InBlock::ContinueKwd),
            Rule::return_def => {
                let mut expr = None;

                for rule in inner.into_inner() {
                    match rule.as_rule() {
                        Rule::expr => expr = Some(Expr::parse(rule)?),
                        _ => {}
                    }
                }

                Some(InBlock::Return(expr))
            }
            Rule::if_def => Some(InBlock::If(IfDef::parse(inner)?)),
            Rule::while_def => Some(InBlock::While(WhileDef::parse(inner)?)),
            Rule::for_def => Some(InBlock::For(ForDef::parse(inner)?)),
            _ => None,
        }
    }
}

impl ParseMany for InBlock {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut blocks = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::in_block => blocks.push(InBlock::parse(rule)?),
                _ => {}
            }
        }

        Some(blocks)
    }
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum BorrowType {
    Borrow,
    MutBorrow,
}

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub struct ModuleIdentTree {
    pub path: Vec<String>,
}

impl Parse for ModuleIdentTree {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut path = vec![];

        for ident in pair.into_inner() {
            match ident.as_rule() {
                Rule::ident => path.push(ident.as_str().to_string()),
                _ => {}
            }
        }

        Some(ModuleIdentTree { path })
    }
}
