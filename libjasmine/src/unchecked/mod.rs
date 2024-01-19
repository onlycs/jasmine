use proc_macro2::Group;
use std::collections::HashMap;
use std::sync::Arc;

use crate::prelude::FunctionSelf;

// pre-typechecked structs. uses strings as idents.
// resolved to checked structs. they use u32's as idents
// and are fully typechecked

#[derive(Clone, Debug)]
pub struct UncheckedGeneric {
    pub ident: String,
    pub constraints: Vec<UncheckedFullType>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UncheckedFullType {
    Ref(Box<UncheckedFullType>),
    RefMut(Box<UncheckedFullType>),
    Generic(String, Vec<UncheckedFullType>),
    Tuple(Vec<UncheckedFullType>),
    Simple(String),
}

impl UncheckedFullType {
    pub fn is_ref(&self) -> bool {
        match self {
            Self::Ref(_) | Self::RefMut(_) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug)]
pub enum UncheckedBodyData {
    Abstract,
    WithBody(Group), // TokenTree::Group { delim: brace }
}

#[derive(Clone, Debug)]
pub struct UncheckedFunction {
    pub ident: Arc<String>,
    pub generics: Vec<UncheckedGeneric>,
    pub params: Vec<(String, UncheckedFullType)>,
    pub returns: Option<UncheckedFullType>,
    pub self_as: FunctionSelf,
    pub body: UncheckedBodyData,
}

impl UncheckedFunction {
    pub fn ident(&self) -> Arc<String> {
        Arc::clone(&self.ident)
    }
}

#[derive(Clone, Debug)]
pub struct UncheckedStruct {
    pub fields: HashMap<String, UncheckedFullType>,
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub traits: Vec<UncheckedFullType>,
}

#[derive(Clone, Debug)]
pub enum UncheckedEnumVariantData {
    Struct(HashMap<String, UncheckedFullType>),
    Tuple(Vec<UncheckedFullType>),
}

#[derive(Clone, Debug)]
pub struct UncheckedEnum {
    pub variants: HashMap<String, Option<UncheckedEnumVariantData>>,
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub traits: Vec<UncheckedFullType>,
}

#[derive(Clone, Debug)]
pub struct UncheckedTrait {
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub constraints: Vec<UncheckedFullType>,
}

#[derive(Clone, Debug)]
pub enum UncheckedTypeKind {
    Struct(UncheckedStruct),
    Enum(UncheckedEnum),
    AliasTo(UncheckedFullType),
    Trait(UncheckedTrait),
    Generic(UncheckedGeneric),
    JavaBuiltin,
}

#[derive(Clone, Debug)]
pub struct UncheckedType {
    pub ident: Arc<String>,
    pub kind: UncheckedTypeKind,
}

impl UncheckedType {
    pub fn ident(&self) -> Arc<String> {
        Arc::clone(&self.ident)
    }
}

#[derive(Clone, Debug)]
pub struct UncheckedProgram {
    pub functions: HashMap<Arc<String>, UncheckedFunction>,
    pub types: HashMap<Arc<String>, UncheckedType>,
}
