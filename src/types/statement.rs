use super::*;

#[derive(Debug, Clone, PartialEq)]
pub enum AssignType {
    Assign,
    AddAssign,
    SubAssign,
    MulAssign,
    DivAssign,
    ModAssign,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Stmt {
    pub ident: String,
    pub assign_type: AssignType,
    pub expr: Expr,
}

impl Parse for Stmt {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        let mut ident = None;
        let mut assign_type = None;
        let mut expr = None;

        for rule in pair.into_inner() {
            match rule.as_rule() {
                Rule::ident => ident = Some(rule.as_str().to_string()),
                Rule::assign => assign_type = Some(AssignType::Assign),
                Rule::add_assign => assign_type = Some(AssignType::AddAssign),
                Rule::sub_assign => assign_type = Some(AssignType::SubAssign),
                Rule::mul_assign => assign_type = Some(AssignType::MulAssign),
                Rule::div_assign => assign_type = Some(AssignType::DivAssign),
                Rule::mod_assign => assign_type = Some(AssignType::ModAssign),
                Rule::expr => expr = Expr::parse(rule),
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
