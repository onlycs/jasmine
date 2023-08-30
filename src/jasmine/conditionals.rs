use super::*;

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
