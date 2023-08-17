use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct WhileLoop {
    pub cond: Expression,
    pub body: Vec<BlockPart>,
}

impl Parse for WhileLoop {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expression::parse(rule)?),
                Rule::block => body = BlockPart::parse_many(rule)?,
                _ => {}
            }
        }

        Some(WhileLoop { cond: cond?, body })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ForLoop {
    pub arg: Arg,
    pub iter: Expression,
    pub body: Vec<BlockPart>,
}

impl Parse for ForLoop {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut arg = None;
        let mut iter = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::define_argument => arg = Some(Arg::parse(rule)?),
                Rule::expr => iter = Some(Expression::parse(rule)?),
                Rule::block => body = BlockPart::parse_many(rule)?,
                _ => {}
            }
        }

        Some(ForLoop {
            arg: arg?,
            iter: iter?,
            body,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElifStmt {
    pub cond: Expression,
    pub body: Vec<BlockPart>,
}

impl Parse for ElifStmt {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expression::parse(rule)?),
                Rule::block => body = BlockPart::parse_many(rule)?,
                _ => {}
            }
        }

        Some(ElifStmt { cond: cond?, body })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub cond: Expression,
    pub body: Vec<BlockPart>,
    pub else_ifs: Vec<ElifStmt>,
    pub else_body: Option<Vec<BlockPart>>,
}

impl Parse for IfStatement {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];
        let mut else_ifs = vec![];
        let mut else_body = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => cond = Some(Expression::parse(rule)?),
                Rule::block => body = BlockPart::parse_many(rule)?,
                Rule::else_if_def => else_ifs.push(ElifStmt::parse(rule)?),
                Rule::else_def => {
                    for rule in rule.into_inner() {
                        match rule.as_rule() {
                            Rule::block => else_body = Some(BlockPart::parse_many(rule)?),
                            _ => {}
                        }
                    }
                }
                _ => {}
            }
        }

        Some(IfStatement {
            cond: cond?,
            body,
            else_ifs,
            else_body,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BlockPart {
    Var(Variable),
    Expr(Expression),
    Stmt(Statement),
    BreakKwd,
    ContinueKwd,
    Return(Option<Expression>),
    If(IfStatement),
    While(WhileLoop),
    For(ForLoop),
}

impl Parse for BlockPart {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner = pair.into_inner().next()?;

        match inner.as_rule() {
            Rule::var => Some(BlockPart::Var(Variable::parse(inner)?)),
            Rule::expr => Some(BlockPart::Expr(Expression::parse(inner)?)),
            Rule::stmt => Some(BlockPart::Stmt(Statement::parse(inner)?)),
            Rule::break_kwd => Some(BlockPart::BreakKwd),
            Rule::continue_kwd => Some(BlockPart::ContinueKwd),
            Rule::return_def => {
                let mut expr = None;

                for rule in inner.into_inner() {
                    match rule.as_rule() {
                        Rule::expr => expr = Some(Expression::parse(rule)?),
                        _ => {}
                    }
                }

                Some(BlockPart::Return(expr))
            }
            Rule::if_def => Some(BlockPart::If(IfStatement::parse(inner)?)),
            Rule::while_def => Some(BlockPart::While(WhileLoop::parse(inner)?)),
            Rule::for_def => Some(BlockPart::For(ForLoop::parse(inner)?)),
            _ => None,
        }
    }
}

impl ParseMany for BlockPart {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut blocks = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::in_block => blocks.push(BlockPart::parse(rule)?),
                _ => {}
            }
        }

        Some(blocks)
    }
}
