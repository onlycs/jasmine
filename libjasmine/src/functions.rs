use std::collections::HashMap;

use crate::{prelude::Generic, types::TypeId};

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: String,
    pub generics: HashMap<String, Generic>,
    pub params: Vec<(String, TypeId)>,
    pub returns: TypeId,

    // in traits
    pub _abstract: bool,
    pub _static: bool,
}
