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
