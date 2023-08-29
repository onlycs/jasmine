pub mod arguments;
pub mod blocks;
pub mod chars;
pub mod definitions;
pub mod enums;
pub mod expressions;
pub mod functions;
pub mod generics;
pub mod oop;
pub mod operators;
pub mod statements;
pub mod types;

pub use crate::prelude::*;
pub use arguments::*;
pub use blocks::*;
pub use chars::*;
pub use definitions::*;
pub use enums::*;
pub use expressions::*;
pub use functions::*;
pub use generics::*;
pub use oop::*;
pub use operators::*;
pub use statements::*;
pub use types::*;

#[derive(Clone, Debug, PartialEq)]
pub enum JasmineProgramComponent {
    Struct(Structure),
    Impl(Impl),
    Fn(Function),
    Var(Variable),
    Enum(Enumeration),
}

impl Parse for JasmineProgramComponent {
    fn parse(pair: Pair<'_, Rule>) -> Option<Self> {
        match pair.as_rule() {
            Rule::struct_def => Some(Self::Struct(Structure::parse(pair)?)),
            Rule::impl_def => Some(Self::Impl(Impl::parse(pair)?)),
            Rule::fn_def => Some(Self::Fn(Function::parse(pair)?)),
            Rule::var => Some(Self::Var(Variable::parse(pair)?)),
            Rule::enum_def => Some(Self::Enum(Enumeration::parse(pair)?)),
            _ => None,
        }
    }
}

impl ParseMany for JasmineProgramComponent {
    fn parse_many(pair: Pair<'_, Rule>) -> Option<Vec<Self>> {
        let mut components = vec![];
        for inner_pair in pair.into_inner() {
            if !vec![
                Rule::struct_def,
                Rule::impl_def,
                Rule::fn_def,
                Rule::var,
                Rule::enum_def,
            ]
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
