use super::*;

#[derive(Clone, Debug, PartialEq)]
pub struct IfLetExpression {
    pub ty_ident: String,
    pub variant_ident: String,
    pub data_ident: String,
    pub data_ty: Type,
    pub expr: Expression,
}

impl Parse for IfLetExpression {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ty_ident = None;
        let mut variant_ident = None;
        let mut data_ident = None;
        let mut data_ty = None;
        let mut expr = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if ty_ident.is_none() => ty_ident = Some(rule.as_str().to_string()),
                Rule::ident if variant_ident.is_none() => {
                    variant_ident = Some(rule.as_str().to_string())
                }
                Rule::ident if data_ident.is_none() => data_ident = Some(rule.as_str().to_string()),
                Rule::ty => data_ty = Some(Type::parse(rule)?),
                Rule::expr => expr = Some(Expression::parse(rule)?),
                _ => {}
            }
        }

        Some(IfLetExpression {
            ty_ident: ty_ident?,
            variant_ident: variant_ident?,
            data_ident: data_ident?,
            data_ty: data_ty?,
            expr: expr?,
        })
    }
}

impl IfLetExpression {
    pub fn rewrite(&self) -> String {
        format!(
            "({}).is({}._{})",
            self.expr.rewrite(),
            self.ty_ident,
            self.variant_ident
        )
    }

    pub fn rewrite_data(&self) -> String {
        format!(
            "{} {} = ({})._getData_{}();",
            self.data_ty.rewrite(),
            rewrite_ident(&self.data_ident),
            self.expr.rewrite(),
            self.variant_ident
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum IfExpression {
    Expr(Expression),
    IfLet(IfLetExpression),
}

impl Parse for IfExpression {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let inner = pair.into_inner().next()?;

        match inner.as_rule() {
            Rule::expr => Some(IfExpression::Expr(Expression::parse(inner)?)),
            Rule::if_let => Some(IfExpression::IfLet(IfLetExpression::parse(inner)?)),
            _ => None,
        }
    }
}

impl IfExpression {
    pub fn rewrite(&self) -> String {
        match self {
            IfExpression::Expr(expr) => expr.rewrite(),
            IfExpression::IfLet(if_let) => if_let.rewrite(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ElifStmt {
    pub cond: IfExpression,
    pub body: Vec<BlockPart>,
}

impl Parse for ElifStmt {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut cond = None;
        let mut body = vec![];

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::if_expr => cond = Some(IfExpression::parse(rule)?),
                Rule::block => body = BlockPart::parse_many(rule)?,
                _ => {}
            }
        }

        Some(ElifStmt { cond: cond?, body })
    }
}

impl ElifStmt {
    pub fn rewrite(&self) -> String {
        let data_block = {
            if let IfExpression::IfLet(iflet) = &self.cond {
                format!("{}\n", iflet.rewrite_data())
            } else {
                "".to_string()
            }
        };

        format!(
            " else if ({}) {{\n{}{}\n}}",
            self.cond.rewrite(),
            data_block,
            BlockPart::rewrite_many(self.body.clone(), "\n")
        )
    }

    pub fn rewrite_many(all: Vec<Self>, sep: &'static str) -> String {
        all.iter().map(|n| n.rewrite()).join(sep)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct IfStatement {
    pub cond: IfExpression,
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
                Rule::if_expr => cond = Some(IfExpression::parse(rule)?),
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

impl IfStatement {
    pub fn rewrite(&self) -> String {
        let data_block = {
            if let IfExpression::IfLet(iflet) = &self.cond {
                format!("{}\n", iflet.rewrite_data())
            } else {
                "".to_string()
            }
        };

        format!(
            "if ({}) {{\n{}{}\n}}{}{}",
            self.cond.rewrite(),
            data_block,
            BlockPart::rewrite_many(self.body.clone(), "\n"),
            ElifStmt::rewrite_many(self.else_ifs.clone(), "\n"),
            if let Some(else_body) = &self.else_body {
                format!(
                    " else {{\n{}\n}}",
                    BlockPart::rewrite_many(else_body.clone(), "\n")
                )
            } else {
                "".to_string()
            },
        )
    }
}
