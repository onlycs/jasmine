use std::sync::Arc;

use proc_macro2::Group;

use crate::parse;
use crate::prelude::*;

#[test]
fn aliases() {
    const INPUT: &'static str = r#"
		type Alias = Actual;
		type AliasToTuple = (Alias, Alias);
		type AliasToGeneric = G<Alias>;
	"#;

    let parsed_ast = parse(INPUT).unwrap();
    let manual_ast = UncheckedProgram {
        functions: hashmap! {},
        types: {
            let alias = Arc::new(String::from("Alias"));
            let alias_to_tuple = Arc::new(String::from("AliasToTuple"));
            let alias_to_generic = Arc::new(String::from("AliasToGeneric"));

            hashmap! {
                Arc::clone(&alias) => UncheckedType {
                    ident: Arc::clone(&alias),
                    kind: UncheckedTypeKind::AliasTo(UncheckedFullTypeId::Simple(String::from("Actual"))),
                },
                Arc::clone(&alias_to_tuple) => UncheckedType {
                    ident: Arc::clone(&alias_to_tuple),
                    kind: UncheckedTypeKind::AliasTo(UncheckedFullTypeId::Tuple(vec![
                        UncheckedFullTypeId::Simple(String::from("Alias")),
                        UncheckedFullTypeId::Simple(String::from("Alias")),
                    ])),
                },
                Arc::clone(&alias_to_generic) => UncheckedType {
                    ident: Arc::clone(&alias_to_generic),
                    kind: UncheckedTypeKind::AliasTo(UncheckedFullTypeId::Generic {
                        outer: String::from("G"),
                        inner: vec![UncheckedFullTypeId::Simple(String::from("Alias"))],
                    }),
                },
            }
        },
    };

    assert_eq!(parsed_ast, manual_ast);
}

#[test]
fn structs() {
    const INPUT: &'static str = r#"
		struct Normal {
			field: TypeOne,
			field2: TypeTwo,
		}

		struct Tuple(TypeOne, TypeTwo);

		struct NormalG<T: Constraint1 + Constraint2> {
			field: T,
			field2: TypeTwo,
		}

		struct TupleG<T>(T, TypeTwo);
	"#;

    let parsed_ast = parse(INPUT).unwrap();
    let manual_ast = UncheckedProgram {
        functions: hashmap! {},
        types: {
            let normal = Arc::new(String::from("Normal"));
            let tuple = Arc::new(String::from("Tuple"));
            let normal_g = Arc::new(String::from("NormalG"));
            let tuple_g = Arc::new(String::from("TupleG"));

            hashmap! {
                Arc::clone(&normal) => UncheckedType {
                    ident: Arc::clone(&normal),
                    kind: UncheckedTypeKind::Struct(UncheckedStruct {
                        inner: UncheckedCompositeData::Struct(hashmap! {
                            String::from("field") => UncheckedFullTypeId::Simple(String::from("TypeOne")),
                            String::from("field2") => UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                        }),
                        generics: vec![],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
                Arc::clone(&tuple) => UncheckedType {
                    ident: Arc::clone(&tuple),
                    kind: UncheckedTypeKind::Struct(UncheckedStruct {
                        inner: UncheckedCompositeData::Tuple(vec![
                            UncheckedFullTypeId::Simple(String::from("TypeOne")),
                            UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                        ]),
                        generics: vec![],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
                Arc::clone(&normal_g) => UncheckedType {
                    ident: Arc::clone(&normal_g),
                    kind: UncheckedTypeKind::Struct(UncheckedStruct {
                        inner: UncheckedCompositeData::Struct(hashmap! {
                            String::from("field") => UncheckedFullTypeId::Simple(String::from("T")),
                            String::from("field2") => UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                        }),
                        generics: vec![UncheckedGeneric {
                            ident: String::from("T"),
                            constraints: hashset![
                                UncheckedFullTypeId::Simple(String::from("Constraint1")),
                                UncheckedFullTypeId::Simple(String::from("Constraint2")),
                            ],
                        }],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
                Arc::clone(&tuple_g) => UncheckedType {
                    ident: Arc::clone(&tuple_g),
                    kind: UncheckedTypeKind::Struct(UncheckedStruct {
                        inner: UncheckedCompositeData::Tuple(vec![
                            UncheckedFullTypeId::Simple(String::from("T")),
                            UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                        ]),
                        generics: vec![UncheckedGeneric {
                            ident: String::from("T"),
                            constraints: hashset![],
                        }],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
            }
        },
    };

    assert_eq!(parsed_ast, manual_ast);
}

#[test]
fn enums() {
    const INPUT: &'static str = r#"
		enum Normal {
			Unit,
			Tuple(TypeOne, TypeTwo),
			Struct {
				field: TypeOne,
				field2: TypeTwo,
			},
		}

		enum NormalG<T: Constraint1 + Constraint2> {
			Unit,
			Tuple(T, TypeTwo),
			Struct {
				field: T,
				field2: TypeTwo,
			},
		}
	"#;

    let parsed_ast = parse(INPUT).unwrap();
    let manual_ast = UncheckedProgram {
        functions: hashmap! {},
        types: {
            let normal = Arc::new(String::from("Normal"));
            let normal_g = Arc::new(String::from("NormalG"));

            hashmap! {
                Arc::clone(&normal) => UncheckedType {
                    ident: Arc::clone(&normal),
                    kind: UncheckedTypeKind::Enum(UncheckedEnum {
                        variants: hashmap! {
                            String::from("Unit") => None,
                            String::from("Tuple") => Some(UncheckedCompositeData::Tuple(vec![
                                UncheckedFullTypeId::Simple(String::from("TypeOne")),
                                UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                            ])),
                            String::from("Struct") => Some(UncheckedCompositeData::Struct(hashmap! {
                                String::from("field") => UncheckedFullTypeId::Simple(String::from("TypeOne")),
                                String::from("field2") => UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                            })),
                        },
                        generics: vec![],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
                Arc::clone(&normal_g) => UncheckedType {
                    ident: Arc::clone(&normal_g),
                    kind: UncheckedTypeKind::Enum(UncheckedEnum {
                        variants: hashmap! {
                            String::from("Unit") => None,
                            String::from("Tuple") => Some(UncheckedCompositeData::Tuple(vec![
                                UncheckedFullTypeId::Simple(String::from("T")),
                                UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                            ])),
                            String::from("Struct") => Some(UncheckedCompositeData::Struct(hashmap! {
                                String::from("field") => UncheckedFullTypeId::Simple(String::from("T")),
                                String::from("field2") => UncheckedFullTypeId::Simple(String::from("TypeTwo")),
                            })),
                        },
                        generics: vec![UncheckedGeneric {
                            ident: String::from("T"),
                            constraints: hashset![
                                UncheckedFullTypeId::Simple(String::from("Constraint1")),
                                UncheckedFullTypeId::Simple(String::from("Constraint2")),
                            ],
                        }],
                        methods: hashmap! {},
                        traits: vec![],
                    }),
                },
            }
        },
    };

    assert_eq!(parsed_ast, manual_ast);
}

#[test]
fn traits() {
    const INPUT: &'static str = r#"
		trait TraitA {
			type AssocType;
			const AssocConst: Self::AssocType;
			fn abstract_method(&self) -> Type;
			fn method(&self) -> Type {}
		}

		trait TraitG<T: Constraint1 + Constraint2>: Constraint3 {
			type AssocType = DefaultType;
			type AssocType2: Constraint4 + Constraint5 = T;
			const AssocConst: Self::AssocType;
			fn abstract_method(&self) -> Type;
			fn method(&self) -> T {}
		}
	"#;

    let parsed_ast = parse(INPUT).unwrap();
    let manual_ast = UncheckedProgram {
        functions: hashmap! {},
        types: {
            let trait_a = Arc::new(String::from("TraitA"));
            let trait_g = Arc::new(String::from("TraitG"));

            hashmap! {
                Arc::clone(&trait_a) => UncheckedType {
                    ident: Arc::clone(&trait_a),
                    kind: UncheckedTypeKind::Trait(UncheckedTrait {
                        generics: vec![],
                        methods: {
                            let abstract_method = Arc::new(String::from("abstract_method"));
                            let method = Arc::new(String::from("method"));

                            hashmap! {
                                Arc::clone(&abstract_method) => UncheckedFunction {
                                    ident: Arc::clone(&abstract_method),
                                    params: vec![],
                                    returns: Some(UncheckedFullTypeId::Simple(String::from("Type"))),
                                    body: UncheckedBodyData::Abstract,
                                    self_as: FunctionSelf::Ref,
                                    generics: vec![],
                                },
                                Arc::clone(&method) => UncheckedFunction {
                                    ident: Arc::clone(&method),
                                    params: vec![],
                                    returns: Some(UncheckedFullTypeId::Simple(String::from("Type"))),
                                    body: UncheckedBodyData::WithBody(Group::new(Delimiter::Brace, TokenStream::new())),
                                    self_as: FunctionSelf::Ref,
                                    generics: vec![],
                                },
                            }
                        },
                        associated_types: hashmap! {
                            String::from("AssocType") => UncheckedAssociatedType {
                                constraints: hashset![],
                                default: None,
                            },
                        },
                        consts: hashmap! {
                            String::from("AssocConst") => UncheckedAssociatedConst {
                                ty: UncheckedFullTypeId::Path {
                                    behind: String::from("Self"),
                                    ahead: Box::new(UncheckedFullTypeId::Simple(String::from("AssocType")))
                                },
                                default: None,
                            },
                        },
                        constraints: hashset![],
                    }),
                },
                Arc::clone(&trait_g) => UncheckedType {
                    ident: Arc::clone(&trait_g),
                    kind: UncheckedTypeKind::Trait(UncheckedTrait {
                        generics: vec![UncheckedGeneric {
                            ident: String::from("T"),
                            constraints: hashset![
                                UncheckedFullTypeId::Simple(String::from("Constraint1")),
                                UncheckedFullTypeId::Simple(String::from("Constraint2")),
                            ],
                        }],
                        methods: {
                            let abstract_method = Arc::new(String::from("abstract_method"));
                            let method = Arc::new(String::from("method"));

                            hashmap! {
                                Arc::clone(&abstract_method) => UncheckedFunction {
                                    ident: Arc::clone(&abstract_method),
                                    params: vec![],
                                    returns: Some(UncheckedFullTypeId::Simple(String::from("Type"))),
                                    body: UncheckedBodyData::Abstract,
                                    self_as: FunctionSelf::Ref,
                                    generics: vec![],
                                },
                                Arc::clone(&method) => UncheckedFunction {
                                    ident: Arc::clone(&method),
                                    params: vec![],
                                    returns: Some(UncheckedFullTypeId::Simple(String::from("T"))),
                                    body: UncheckedBodyData::WithBody(Group::new(Delimiter::Brace, TokenStream::new())),
                                    self_as: FunctionSelf::Ref,
                                    generics: vec![],
                                },
                            }
                        },
                        associated_types: hashmap! {
                            String::from("AssocType") => UncheckedAssociatedType {
                                constraints: hashset![],
                                default: Some(UncheckedFullTypeId::Simple(String::from("DefaultType"))),
                            },
                            String::from("AssocType2") => UncheckedAssociatedType {
                                constraints: hashset![
                                    UncheckedFullTypeId::Simple(String::from("Constraint4")),
                                    UncheckedFullTypeId::Simple(String::from("Constraint5")),
                                ],
                                default: Some(UncheckedFullTypeId::Simple(String::from("T"))),
                            },
                        },
                        consts: hashmap! {
                            String::from("AssocConst") => UncheckedAssociatedConst {
                                ty: UncheckedFullTypeId::Path {
                                    behind: String::from("Self"),
                                    ahead: Box::new(UncheckedFullTypeId::Simple(String::from("AssocType")))
                                },
                                default: None,
                            },
                        },
                        constraints: hashset![UncheckedFullTypeId::Simple(String::from("Constraint3"))],
                    }),
                },
            }
        },
    };

    println!("parsed: {:#?}", parsed_ast);
    println!("manual: {:#?}", manual_ast);

    assert_eq!(parsed_ast, manual_ast);
}
