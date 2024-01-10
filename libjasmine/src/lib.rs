use std::collections::HashMap;

use functions::Function;
use types::{Type, TypeId};

pub mod functions;
pub mod types;

#[derive(Clone, Debug)]
pub struct Program {
    pub functions: HashMap<String, Function>,
    pub type_ids: HashMap<String, TypeId>,
    pub types: HashMap<TypeId, Type>,
}

pub mod prelude {
    pub use crate::functions::Function;
    pub use crate::types::{Enum, EnumVariant, EnumVariantData, Generic, Struct, Type, TypeId};
    pub use crate::Program;
}
