use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum OneInputOp {
    Not,
    Neg,
}

impl Parse for OneInputOp {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.into_inner().next()?.as_rule() {
            Rule::not_op => Some(OneInputOp::Not),
            Rule::neg_op => Some(OneInputOp::Neg),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum TwoInputOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Not,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl Parse for TwoInputOp {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::add_op => Some(TwoInputOp::Add),
            Rule::sub_op => Some(TwoInputOp::Sub),
            Rule::mul_op => Some(TwoInputOp::Mul),
            Rule::div_op => Some(TwoInputOp::Div),
            Rule::mod_op => Some(TwoInputOp::Mod),
            Rule::and_op => Some(TwoInputOp::And),
            Rule::or_op => Some(TwoInputOp::Or),
            Rule::not_op => Some(TwoInputOp::Not),
            Rule::eq_op => Some(TwoInputOp::Eq),
            Rule::neq_op => Some(TwoInputOp::Neq),
            Rule::lt_op => Some(TwoInputOp::Lt),
            Rule::gt_op => Some(TwoInputOp::Gt),
            Rule::lte_op => Some(TwoInputOp::Lte),
            Rule::gte_op => Some(TwoInputOp::Gte),
            _ => None,
        }
    }
}
