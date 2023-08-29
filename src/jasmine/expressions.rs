use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct FullExpr {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
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
                    lhs = Some(Expression::parse(rule)?);
                }
                Rule::op_expr_recurse if on_rhs => {
                    rhs = Some(Expression::parse(rule)?);
                }
                Rule::two_input_op => {
                    op = Some(BinaryOperator::parse(rule)?);
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

impl Rewrite for FullExpr {
    fn rewrite(&self) -> String {
        format!(
            "({} {} {})",
            self.lhs.rewrite(),
            self.op.rewrite(),
            self.rhs.rewrite()
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AfterDotExprType {
    ObjectFnCall {
        data: FunctionCall,
        after_dot: Option<Box<AfterDotExprType>>,
    },
    ObjectProp {
        data: String,
        after_dot: Option<Box<AfterDotExprType>>,
    },
}

impl AfterDotExprType {
    pub fn push(&mut self, next: AfterDotExprType) {
        let new = Box::new(next);

        match self {
            AfterDotExprType::ObjectFnCall { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(*new);
                } else {
                    *after_dot = Some(new);
                }
            }
            AfterDotExprType::ObjectProp { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(*new);
                } else {
                    *after_dot = Some(new);
                }
            }
        }
    }

    pub fn rewrite(&self) -> String {
        match self {
            AfterDotExprType::ObjectFnCall {
                data, after_dot, ..
            } => {
                if let Some(after_dot) = after_dot {
                    format!("{}.{}", data.rewrite(), after_dot.rewrite())
                } else {
                    data.rewrite()
                }
            }
            AfterDotExprType::ObjectProp {
                data, after_dot, ..
            } => {
                if let Some(after_dot) = after_dot {
                    format!("{}.{}", rewrite_ident(data), after_dot.rewrite())
                } else {
                    rewrite_ident(data)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum BaseExprType {
    FnCall {
        data: FunctionCall,
        after_dot: Option<AfterDotExprType>,
    },
    Ident {
        data: String,
        /// this can also be an Enum creation (with data), but enums are made into objects anyways so it doesnt matter
        static_fn: Option<FunctionCall>,
        /// Enum without data
        unit_enum: Option<String>,
        after_dot: Option<AfterDotExprType>,
    },
}

impl BaseExprType {
    pub fn push(&mut self, next: AfterDotExprType) {
        match self {
            BaseExprType::FnCall { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(next);
                } else {
                    *after_dot = Some(next);
                }
            }
            BaseExprType::Ident { after_dot, .. } => {
                if let Some(after_dot) = after_dot {
                    after_dot.push(next);
                } else {
                    *after_dot = Some(next);
                }
            }
        }
    }

    pub fn rewrite(&self) -> String {
        match self {
            BaseExprType::FnCall { data, after_dot } => {
                let mut formatted = data.rewrite();

                if let Some(after_dot) = after_dot {
                    formatted.push_str(&format!(".{}", after_dot.rewrite()));
                }

                formatted
            }
            BaseExprType::Ident {
                data,
                after_dot,
                static_fn,
                unit_enum,
            } => {
                let mut formatted = rewrite_ident(data);

                if let Some(static_fn) = static_fn {
                    formatted.push_str(&format!(".{}", static_fn.rewrite()))
                }

                if let Some(unit_enum) = unit_enum {
                    formatted.push_str(&format!(".{}()", unit_enum)) // fn call under the hood
                }

                if let Some(after_dot) = after_dot {
                    formatted.push_str(&format!(".{}", after_dot.rewrite()));
                }

                formatted
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BaseExpr {
    pub operators: Vec<UnaryOperator>,
    pub kind: BaseExprType,
}

impl Parse for BaseExpr {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut operators = vec![];
        let mut kind = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::one_input_op => operators.push(UnaryOperator::parse(rule)?),
                Rule::ident => {
                    kind = Some(BaseExprType::Ident {
                        data: rule.as_str().to_string(),
                        static_fn: None,
                        unit_enum: None,
                        after_dot: None,
                    });
                }
                Rule::fn_call => {
                    kind = Some(BaseExprType::FnCall {
                        data: FunctionCall::parse(rule)?,
                        after_dot: None,
                    });
                }
                Rule::static_fn => {
                    let Some(BaseExprType::Ident { static_fn, .. }) = &mut kind else {
                        return None;
                    };

                    *static_fn = Some(FunctionCall::parse(
                        rule.into_inner()
                            .filter(|f| f.as_rule() == Rule::fn_call)
                            .next()?,
                    )?);
                }
                Rule::unit_enum => {
                    let Some(BaseExprType::Ident { unit_enum, .. }) = &mut kind else {
                        return None;
                    };

                    *unit_enum = Some(
                        rule.into_inner()
                            .filter(|f| f.as_rule() == Rule::ident)
                            .next()?
                            .as_str()
                            .trim()
                            .to_string(),
                    );
                }
                Rule::object_fn => {
                    let Some(base_expr) = &mut kind else {
                        return None;
                    };

                    let fn_rule = rule.into_inner().find(|n| n.as_rule() == Rule::fn_call)?;

                    base_expr.push(AfterDotExprType::ObjectFnCall {
                        data: FunctionCall::parse(fn_rule)?,
                        after_dot: None,
                    });
                }
                Rule::object_prop => {
                    let Some(base_expr) = &mut kind else {
                        return None;
                    };

                    let ident = rule.into_inner().find(|n| n.as_rule() == Rule::ident)?;

                    base_expr.push(AfterDotExprType::ObjectProp {
                        data: ident.as_str().to_string(),
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

impl BaseExpr {
    pub fn rewrite(&self) -> String {
        let mut rewritten = "".to_string();

        for op in self.operators.iter() {
            rewritten.push_str(&op.rewrite());
        }

        rewritten.push_str(&self.kind.rewrite());

        rewritten
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Base(BaseExpr),
    Full(FullExpr),
    Definition(Definition),
}

impl Parse for Expression {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner_pr = pair
            .into_inner()
            .filter(|r| r.as_rule() != Rule::lparen)
            .next()?;

        match inner_pr.as_rule() {
            Rule::base_expr => Some(Expression::Base(BaseExpr::parse(inner_pr)?)),
            Rule::op_expr => Some(Expression::Full(FullExpr::parse(inner_pr)?)),
            Rule::definition => Some(Expression::Definition(Definition::parse(inner_pr)?)),
            _ => None,
        }
    }
}

impl Expression {
    pub fn rewrite(&self) -> String {
        match self {
            Expression::Base(expr) => expr.rewrite(),
            Expression::Definition(def) => def.rewrite(),
            Expression::Full(expr) => expr.rewrite(),
        }
    }

    pub fn rewrite_many(all: Vec<Self>, sep: &'static str) -> String {
        all.iter().map(|e| e.rewrite()).join(sep)
    }
}
