use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum UnaryOperator {
    Not,
    Neg,
}

impl Parse for UnaryOperator {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.into_inner().next()?.as_rule() {
            Rule::not_op => Some(UnaryOperator::Not),
            Rule::neg_op => Some(UnaryOperator::Neg),
            _ => None,
        }
    }
}

impl UnaryOperator {
    pub fn rewrite(&self) -> String {
        match self {
            UnaryOperator::Neg => "-",
            UnaryOperator::Not => "!",
        }
        .to_string()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Eq,
    Neq,
    Lt,
    Gt,
    Lte,
    Gte,
}

impl Parse for BinaryOperator {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.into_inner().next()?.as_rule() {
            Rule::add_op => Some(BinaryOperator::Add),
            Rule::sub_op => Some(BinaryOperator::Sub),
            Rule::mul_op => Some(BinaryOperator::Mul),
            Rule::div_op => Some(BinaryOperator::Div),
            Rule::mod_op => Some(BinaryOperator::Mod),
            Rule::and_op => Some(BinaryOperator::And),
            Rule::or_op => Some(BinaryOperator::Or),
            Rule::eq_op => Some(BinaryOperator::Eq),
            Rule::neq_op => Some(BinaryOperator::Neq),
            Rule::lt_op => Some(BinaryOperator::Lt),
            Rule::gt_op => Some(BinaryOperator::Gt),
            Rule::lte_op => Some(BinaryOperator::Lte),
            Rule::gte_op => Some(BinaryOperator::Gte),
            _ => None,
        }
    }
}

impl Rewrite for BinaryOperator {
    fn rewrite(&self) -> String {
        match self {
            BinaryOperator::Add => "+",
            BinaryOperator::Sub => "-",
            BinaryOperator::Mul => "*",
            BinaryOperator::Div => "/",
            BinaryOperator::Mod => "%",
            BinaryOperator::And => "&&",
            BinaryOperator::Or => "||",
            BinaryOperator::Eq => "==",
            BinaryOperator::Neq => "!=",
            BinaryOperator::Lt => "<",
            BinaryOperator::Gt => ">",
            BinaryOperator::Lte => "<=",
            BinaryOperator::Gte => ">=",
        }
        .to_string()
    }
}
