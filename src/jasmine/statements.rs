use super::*;

#[derive(Clone, Debug, PartialEq)]
pub enum AssignType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Statement {
    pub ident: String,
    pub assign_type: AssignType,
    pub expr: Expression,
}

impl Parse for Statement {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut assign_type = None;
        let mut expr = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rewrite_ident(rule.as_str().to_string())),
                Rule::assign => assign_type = Some(AssignType::Assign),
                Rule::add_assign => assign_type = Some(AssignType::AddAssign),
                Rule::sub_assign => assign_type = Some(AssignType::SubAssign),
                Rule::mul_assign => assign_type = Some(AssignType::MulAssign),
                Rule::div_assign => assign_type = Some(AssignType::DivAssign),
                Rule::mod_assign => assign_type = Some(AssignType::ModAssign),
                Rule::expr => expr = Expression::parse(rule),
                _ => {}
            }
        }

        Some(Self {
            ident: ident?,
            assign_type: assign_type?,
            expr: expr?,
        })
    }
}

impl Statement {
    pub fn rewrite(&self) -> String {
        format!(
            "{} {} {}",
            self.ident,
            match self.assign_type {
                AssignType::Assign => "=",
                AssignType::AddAssign => "+=",
                AssignType::SubAssign => "-=",
                AssignType::MulAssign => "*=",
                AssignType::DivAssign => "/=",
                AssignType::ModAssign => "%=",
            },
            self.expr.rewrite()
        )
    }
}
