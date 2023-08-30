use super::*;

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
    Match(MatchStatement),
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
            Rule::match_def => Some(BlockPart::Match(MatchStatement::parse(inner)?)),
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

impl BlockPart {
    pub fn rewrite(&self) -> String {
        match self {
            BlockPart::Var(var) => format!("{};", var.rewrite()),
            BlockPart::Expr(expr) => format!("{};", expr.rewrite()),
            BlockPart::Stmt(stmt) => format!("{};", stmt.rewrite()),
            BlockPart::BreakKwd => "break;".to_string(),
            BlockPart::ContinueKwd => "continue;".to_string(),
            BlockPart::Return(expr) => {
                if let Some(expr) = expr {
                    format!("return {};", expr.rewrite())
                } else {
                    "return;".to_string()
                }
            }
            BlockPart::If(if_stmt) => if_stmt.rewrite(),
            BlockPart::While(while_loop) => while_loop.rewrite(),
            BlockPart::For(for_loop) => for_loop.rewrite(),
            BlockPart::Match(match_stmt) => match_stmt.rewrite(),
        }
    }

    pub fn rewrite_many(all: Vec<Self>, sep: &'static str) -> String {
        all.iter().map(|n| n.rewrite()).join(sep)
    }
}
