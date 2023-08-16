use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum ExprType {
    Definition(Definition),
    FnCall(FnCall),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub enum AfterDotExprType {
    FnCall(FnCall),
    Ident(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct FullExpr {
    pub lhs: Box<Expr>,
    pub op: TwoInputOp,
    pub rhs: Box<Expr>,
}

impl Parse for FullExpr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut lhs = None;
        let mut rhs = None;
        let mut op = None;
        let mut on_rhs = false;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::op_expr_recurse if !on_rhs => {
                    lhs = Some(Expr::parse(rule)?);
                }
                Rule::op_expr_recurse if on_rhs => {
                    rhs = Some(Expr::parse(rule)?);
                }
                Rule::two_input_op => {
                    op = Some(TwoInputOp::parse(rule)?);
                    on_rhs = true;
                }
                _ => {}
            }
        }

        Some(FullExpr {
            lhs: Box::new(lhs?),
            op: op?,
            rhs: Box::new(rhs?),
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseExpr {
    pub expr_type: ExprType,
    pub operators: Vec<OneInputOp>,
    pub dot: Vec<AfterDotExprType>,
}

impl Parse for BaseExpr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut after_dot = false;
        let mut ops = vec![];
        let mut dot = vec![];
        let mut expr_type = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::one_input_op => {
                    ops.push(OneInputOp::parse(rule)?);
                }
                Rule::fn_call if !after_dot => {
                    expr_type = Some(ExprType::FnCall(FnCall::parse(rule)?));
                    after_dot = true;
                }
                Rule::definition if !after_dot => {
                    expr_type = Some(ExprType::Definition(Definition::parse(rule)?));
                    after_dot = true;
                }
                Rule::ident if !after_dot => {
                    expr_type = Some(ExprType::Ident(rule.as_str().to_string()));
                    after_dot = true;
                }
                Rule::fn_call if after_dot => {
                    dot.push(AfterDotExprType::FnCall(FnCall::parse(rule)?));
                }
                Rule::ident if after_dot => {
                    dot.push(AfterDotExprType::Ident(rule.as_str().to_string()));
                }
                _ => {}
            }
        }

        Some(BaseExpr {
            expr_type: expr_type?,
            operators: ops,
            dot,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Base(BaseExpr),
    Full(FullExpr),
}

impl Parse for Expr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner_pr = pair.into_inner().next()?;

        match inner_pr.as_rule() {
            Rule::base_expr => Some(Expr::Base(BaseExpr::parse(inner_pr)?)),
            Rule::op_expr => Some(Expr::Full(FullExpr::parse(inner_pr)?)),
            _ => None,
        }
    }
}
