use crate::types::TypeId;

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: String,
    pub generics: Vec<TypeId>,
    pub params: Vec<(String, TypeId)>,
    pub returns: TypeId,

    // in traits
    pub _abstract: bool,
    pub _static: bool,
}
