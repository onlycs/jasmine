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

impl WhileLoop {
    pub fn rewrite(&self) -> String {
        format!(
            "while ({}) {{\n{}\n}}",
            self.cond.rewrite(),
            BlockPart::rewrite_many(self.body.clone(), "\n")
        )
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

impl ForLoop {
    pub fn rewrite(&self) -> String {
        format!(
            "for ({} : {}) {{\n{}\n}}",
            self.arg.rewrite(),
            self.iter.rewrite(),
            BlockPart::rewrite_many(self.body.clone(), "\n")
        )
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

impl ElifStmt {
    pub fn rewrite(&self) -> String {
        format!(
            " else if ({}) {{\n{}\n}}",
            self.cond.rewrite(),
            BlockPart::rewrite_many(self.body.clone(), "\n")
        )
    }

    pub fn rewrite_many(all: Vec<Self>, sep: &'static str) -> String {
        all.iter().map(|n| n.rewrite()).join(sep)
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

impl IfStatement {
    pub fn rewrite(&self) -> String {
        format!(
            "if ({}) {{\n{}\n}}{}{}",
            self.cond.rewrite(),
            BlockPart::rewrite_many(self.body.clone(), "\n"),
            ElifStmt::rewrite_many(self.else_ifs.clone(), "\n"),
            if let Some(else_body) = &self.else_body {
                format!(
                    " else {{\n{}\n}}",
                    BlockPart::rewrite_many(else_body.clone(), "\n")
                )
            } else {
                "".to_string()
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchArm {
    pub ty_ident: String,
    pub variant_ident: String,
    pub data_ident: Option<String>,
    pub data_ident_ty: Option<Type>,
    pub body: Vec<BlockPart>,
}

impl Parse for MatchArm {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ty_ident = None;
        let mut variant_ident = None;
        let mut after_dblcln = false;

        let mut data_ident = None;
        let mut body = vec![];
        let mut ty = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident if !after_dblcln => ty_ident = Some(rule.as_str().to_string()),
                Rule::ident if after_dblcln && variant_ident.is_none() => {
                    variant_ident = Some(rule.as_str().to_string())
                }
                Rule::ident if after_dblcln && variant_ident.is_some() => {
                    data_ident = Some(rule.as_str().to_string())
                }
                Rule::block => body = BlockPart::parse_many(rule)?,
                Rule::dblcln => after_dblcln = true,
                Rule::ty => ty = Some(Type::parse(rule)?),
                _ => {}
            }
        }

        Some(MatchArm {
            ty_ident: ty_ident?,
            variant_ident: variant_ident?,
            data_ident,
            data_ident_ty: ty,
            body,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct MatchStatement {
    pub expr: Expression,
    pub arms: Vec<MatchArm>,
    pub final_arm: Option<Vec<BlockPart>>,
}

impl Parse for MatchStatement {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut expr = None;
        let mut arms = vec![];
        let mut final_arm = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::expr => expr = Some(Expression::parse(rule)?),
                Rule::match_arm => arms.push(MatchArm::parse(rule)?),
                Rule::block => final_arm = Some(BlockPart::parse_many(rule)?),
                _ => {}
            }
        }

        Some(MatchStatement {
            expr: expr?,
            arms,
            final_arm,
        })
    }
}

impl MatchStatement {
    pub fn rewrite(&self) -> String {
        let arms_iter = self.arms.iter();
        let mut rewritten = "".to_string();

        for arm in arms_iter {
            let data_sect = 'block: {
                let Some(ident) = &arm.data_ident else { break 'block String::from("") };
                let Some(ty) = &arm.data_ident_ty else { break 'block String::from("") };

                format!(
                    "{} {} = {}._getData_{}();\n",
                    ty.rewrite(),
                    rewrite_ident(ident),
                    self.expr.rewrite(),
                    rewrite_ident(&arm.variant_ident)
                )
            };

            rewritten.push_str(&format!(
                "else if ({}.is({}.{})) {{
					{}{}
				}} ",
                self.expr.rewrite(),
                rewrite_ident(&arm.ty_ident),
                format!("_{}", rewrite_ident(&arm.variant_ident)),
                data_sect,
                BlockPart::rewrite_many(arm.body.clone(), "\n")
            ));
        }

        rewritten.push_str(&format!(
            "else {{
				{}
			}} ",
            self.final_arm
                .as_ref()
                .map(|a| BlockPart::rewrite_many(a.to_vec(), "\n"))
                .unwrap_or(
                    "throw new RuntimeException(\"Not all match arms were covered in this statement\");".to_string()
                )
        ));

        rewritten.split("else").skip(1).join("else")
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
