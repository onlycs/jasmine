use super::*;

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
pub enum BaseExprType {
    FnCall {
        data: FnCall,
        after_dot: Option<Box<BaseExprType>>,
    },
    Ident {
        data: String,
        static_fn: Option<FnCall>,
        after_dot: Option<Box<BaseExprType>>,
    },
}

impl BaseExprType {
    pub fn push(&mut self, next: BaseExprType) {
        let new = Box::new(next);

        match self {
            BaseExprType::FnCall { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(*new);
                } else {
                    *after_dot = Some(new);
                }
            }
            BaseExprType::Ident { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(*new);
                } else {
                    *after_dot = Some(new);
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseExpr {
    pub operators: Vec<OneInputOp>,
    pub kind: BaseExprType,
}

impl Parse for BaseExpr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut operators = vec![];
        let mut kind = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::one_input_op => operators.push(OneInputOp::parse(rule)?),
                Rule::ident => {
                    kind = Some(BaseExprType::Ident {
                        data: rule.as_str().to_string(),
                        static_fn: None,
                        after_dot: None,
                    });
                }
                Rule::fn_call => {
                    kind = Some(BaseExprType::FnCall {
                        data: FnCall::parse(rule)?,
                        after_dot: None,
                    });
                }
                Rule::static_fn => {
                    let Some(BaseExprType::Ident { static_fn, .. }) = &mut kind else {
                        return None;
                    };

                    *static_fn = Some(FnCall::parse(rule)?);
                }
                Rule::object_fn => {
                    let Some(base_expr) = &mut kind else {
                        return None;
                    };

                    let fn_rule = rule.into_inner().find(|n| n.as_rule() == Rule::fn_call)?;

                    base_expr.push(BaseExprType::FnCall {
                        data: FnCall::parse(fn_rule)?,
                        after_dot: None,
                    });
                }
                Rule::object_prop => {
                    let Some(base_expr) = &mut kind else {
                        return None;
                    };

                    let ident = rule.into_inner().find(|n| n.as_rule() == Rule::ident)?;

                    base_expr.push(BaseExprType::Ident {
                        data: ident.as_str().to_string(),
                        static_fn: None,
                        after_dot: None,
                    });
                }
                Rule::base_expr => return Some(BaseExpr::parse(rule)?),
                _ => {}
            }
        }

        Some(BaseExpr {
            operators,
            kind: kind?,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Base(BaseExpr),
    Full(FullExpr),
    Definition(Definition),
}

impl Parse for Expr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner_pr = pair.into_inner().next()?;

        match inner_pr.as_rule() {
            Rule::base_expr => Some(Expr::Base(BaseExpr::parse(inner_pr)?)),
            Rule::op_expr => Some(Expr::Full(FullExpr::parse(inner_pr)?)),
            Rule::definition => Some(Expr::Definition(Definition::parse(inner_pr)?)),
            _ => None,
        }
    }
}
