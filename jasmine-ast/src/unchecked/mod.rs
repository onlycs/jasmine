use proc_macro2::{Group, TokenStream};
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

use crate::prelude::FunctionSelf;

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedGeneric {
    pub ident: String,
    pub constraints: HashSet<UncheckedFullTypeId>,
}

#[derive(Clone, PartialEq, Hash, Eq, Debug)]
pub enum UncheckedFullTypeId {
    Ref(Box<UncheckedFullTypeId>),
    RefMut(Box<UncheckedFullTypeId>),
    Generic {
        outer: String,
        inner: Vec<UncheckedFullTypeId>,
    },
    // recursive, so we get Path("moda", Path("modb", Simple("abc")) corresponding to moda::modb::abc
    Path {
        behind: String,
        ahead: Box<UncheckedFullTypeId>,
    },
    Tuple(Vec<UncheckedFullTypeId>),
    Simple(String),
}

impl UncheckedFullTypeId {
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

impl PartialEq for UncheckedBodyData {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Abstract, Self::Abstract) => true,
            (Self::WithBody(_), Self::WithBody(_)) => true,
            _ => false,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedFunction {
    pub ident: Arc<String>,
    pub generics: Vec<UncheckedGeneric>,
    pub params: Vec<(String, UncheckedFullTypeId)>,
    pub returns: Option<UncheckedFullTypeId>,
    pub self_as: FunctionSelf,
    pub body: UncheckedBodyData,
}

impl UncheckedFunction {
    pub fn ident(&self) -> Arc<String> {
        Arc::clone(&self.ident)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum UncheckedCompositeData {
    Struct(HashMap<String, UncheckedFullTypeId>),
    Tuple(Vec<UncheckedFullTypeId>),
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedStruct {
    pub inner: UncheckedCompositeData,
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub traits: Vec<UncheckedFullTypeId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedEnum {
    pub variants: HashMap<String, Option<UncheckedCompositeData>>,
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub traits: Vec<UncheckedFullTypeId>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedAssociatedType {
    pub constraints: HashSet<UncheckedFullTypeId>,
    pub default: Option<UncheckedFullTypeId>,
}

#[derive(Clone, Debug)]
pub struct UncheckedAssociatedConst {
    pub ty: UncheckedFullTypeId,
    pub default: Option<TokenStream>, /* storing expr */
}

impl PartialEq for UncheckedAssociatedConst {
    fn eq(&self, other: &Self) -> bool {
        self.ty == other.ty && self.default.is_some() == other.default.is_some()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedTrait {
    pub generics: Vec<UncheckedGeneric>,
    pub methods: HashMap<Arc<String>, UncheckedFunction>,
    pub constraints: HashSet<UncheckedFullTypeId>,
    pub associated_types: HashMap<String, UncheckedAssociatedType>,
    pub consts: HashMap<String, UncheckedAssociatedConst>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum UncheckedTypeKind {
    Struct(UncheckedStruct),
    Enum(UncheckedEnum),
    AliasTo(UncheckedFullTypeId),
    Trait(UncheckedTrait),
    Generic(UncheckedGeneric),
    JavaBuiltin,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedType {
    pub ident: Arc<String>,
    pub kind: UncheckedTypeKind,
}

impl UncheckedType {
    pub fn ident(&self) -> Arc<String> {
        Arc::clone(&self.ident)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct UncheckedProgram {
    pub functions: HashMap<Arc<String>, UncheckedFunction>,
    pub types: HashMap<Arc<String>, UncheckedType>,
}
