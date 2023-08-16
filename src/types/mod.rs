pub mod arguments;
pub mod definitions;
pub mod expr;
pub mod misc;
pub mod operators;
pub mod statement;

pub use crate::prelude::*;
pub use arguments::*;
pub use definitions::*;
pub use expr::*;
pub use misc::*;
pub use operators::*;
pub use statement::*;

#[derive(Debug, Clone, PartialEq)]
pub enum JasmineProgramComponent {
    Struct(StructDef),
    Impl(ImplDef),
    Fn(FnDef),
    Var(VarDef),
}

impl Parse for JasmineProgramComponent {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::struct_def => Some(Self::Struct(StructDef::parse(pair)?)),
            Rule::impl_def => Some(Self::Impl(ImplDef::parse(pair)?)),
            Rule::fn_def => Some(Self::Fn(FnDef::parse(pair)?)),
            Rule::var => Some(Self::Var(VarDef::parse(pair)?)),
            _ => todo!("Incomplete"),
        }
    }
}

impl ParseMany for JasmineProgramComponent {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut components = vec![];
        for inner_pair in pair.into_inner() {
            if !vec![Rule::struct_def, Rule::impl_def, Rule::fn_def, Rule::var]
                .contains(&inner_pair.as_rule())
            {
                continue;
            }

            let component = Self::parse(inner_pair)?;
            components.push(component);
        }
        Some(components)
    }
}
