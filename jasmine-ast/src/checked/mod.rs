pub mod fnbody;

use fnbody::*;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;

pub type TypeId = u32;
static mut CURRENT_TYPE_ID: TypeId = 3;

pub fn new_type_id() -> TypeId {
    unsafe {
        CURRENT_TYPE_ID += 1;
        CURRENT_TYPE_ID
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum FullTypeId {
    Ref(Box<FullTypeId>),
    RefMut(Box<FullTypeId>),
    Generic {
        outer: TypeId,
        inner: Vec<FullTypeId>,
    },
    Tuple(Vec<FullTypeId>),
    Simple(TypeId),
}

#[derive(Clone, Debug)]
pub struct Generic {
    pub ident: String,
    pub constraints: HashSet<FullTypeId>,
}

#[derive(Clone, Debug)]
pub enum BodyData {
    Abstract,
    WithBody(FnBody),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum FunctionSelf {
    None,
    Ref,
    RefMut,
    Consume,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: Arc<String>,
    pub generics: Vec<Generic>,
    pub params: Vec<(String, TypeId)>,
    pub returns: TypeId,
    pub body: BodyData,
    pub self_as: FunctionSelf,
}

#[derive(Clone, Debug)]
pub enum CompositeData {
    Struct(HashMap<String, TypeId>),
    Tuple(Vec<TypeId>),
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub inner: CompositeData,
    pub generics: Vec<Generic>,
    pub methods: HashMap<Arc<String>, Function>,
    pub traits: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub variants: HashMap<String, Option<CompositeData>>,
    pub generics: HashMap<String, Generic>,
    pub methods: HashMap<String, Function>,
    pub traits: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub struct Trait {
    pub generics: HashMap<String, Generic>,
    pub methods: HashMap<String, Function>,
    pub constraints: HashSet<FullTypeId>,
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    Struct(Struct),
    Enum(Enum),
    AliasTo(FullTypeId),
    Trait(Trait),
    Generic(Generic),
    JavaBuiltin,
}

pub struct Type {
    pub id: TypeId,
    pub name: Arc<String>,
    pub kind: TypeKind,
}
