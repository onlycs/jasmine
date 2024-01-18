pub mod fnbody;

use fnbody::*;
use std::collections::HashMap;
use std::sync::Arc;

pub type TypeId = u32;
static mut CURRENT_TYPE_ID: TypeId = 3;

pub fn new_type_id() -> TypeId {
    unsafe {
        CURRENT_TYPE_ID += 1;
        CURRENT_TYPE_ID
    }
}

#[derive(Clone, Debug)]
pub struct FullTypeId {
    pub outer: TypeId,
    pub generics: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub struct Generic {
    pub ident: String,
    pub constraints: Vec<FullTypeId>, // TODO: See if we get performance using hashset
}

#[derive(Clone, Debug)]
pub enum BodyData {
    Abstract,
    WithBody(FnBody),
}

#[derive(Clone, Debug)]
pub enum FunctionSelf {
    None,
    Ref,
    RefMut,
    Consume,
}

#[derive(Clone, Debug)]
pub struct Function {
    pub ident: Arc<String>,
    pub generics: HashMap<String, Generic>,
    pub params: Vec<(String, TypeId)>,
    pub returns: TypeId,
    pub body: BodyData,
    pub self_as: FunctionSelf,

    // in impls or traits
    pub _static: bool,
}

#[derive(Clone, Debug)]
pub struct Struct {
    pub fields: HashMap<String, FullTypeId>,
    pub generics: Vec<Generic>,
    pub methods: HashMap<Arc<String>, Function>,
    pub traits: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub enum EnumVariantData {
    Struct(HashMap<String, FullTypeId>),
    Tuple(Vec<FullTypeId>),
}

#[derive(Clone, Debug)]
pub struct Enum {
    pub variants: HashMap<String, Option<EnumVariantData>>,
    pub generics: HashMap<String, Generic>,
    pub methods: HashMap<String, Function>,
    pub traits: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub struct Trait {
    pub generics: HashMap<String, Generic>,
    pub methods: HashMap<String, Function>,
    pub constraints: Vec<FullTypeId>,
}

#[derive(Clone, Debug)]
pub enum TypeKind {
    Struct(Struct),
    Enum(Enum),
    Alias(String, FullTypeId),
    Trait(Trait),
    Generic(Generic),
    JavaBuiltin,
}

pub struct Type {
    pub id: TypeId,
    pub name: Arc<String>,
    pub kind: TypeKind,
}
