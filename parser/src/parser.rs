use libjasmine::types::new_type_id;

use crate::prelude::*;

macro_rules! expect {
    ($tree:expr, $expected:pat, $check:block) => {
        expect!($tree, $expected, $check, {})
    };
    ($tree:expr, $expected:pat, $check:block, $ret:block) => {
        match $tree.next() {
            Some($expected) => {
                if !$check {
                    panic!(
                        "Expected {} such that {}",
                        stringify!($expected),
                        stringify!($check),
                    );
                } else {
                    $ret
                }
            }
            _ => panic!(
                "Expected {} such that {}",
                stringify!($expected),
                stringify!($check),
            ),
        }
    };
}

macro_rules! expect_on {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree {
            $expected => $ret,
            _ => panic!("Expected {}", stringify!($expected),),
        }
    };
}

macro_rules! expect_ret {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree.next().unwrap() {
            $expected => $ret,
            _ => panic!("Expected {}", stringify!($expected),),
        }
    };
}

pub fn parse_alias(
    iterator: &mut impl Iterator<Item = TokenTree>,
    type_ids: &HashMap<String, TypeId>,
    awaiting_types: &mut HashMap<String, TypeId>,
) -> (String, TypeId) {
    let alias_ident = expect_ret!(iterator, TokenTree::Ident(ident), { ident.to_string() });

    if type_ids.get(&alias_ident).is_some() {
        panic!("type {} already exists.", alias_ident);
    }

    expect!(iterator, TokenTree::Punct(p), { p.as_char() == '=' });

    let to_ty = expect_ret!(iterator, TokenTree::Ident(i), { i.to_string() });

    let to_tyid = if let Some(id) = type_ids.get(&to_ty) {
        *id
    } else if let Some(id) = awaiting_types.get(&to_ty) {
        *id
    } else {
        let id = new_type_id();
        awaiting_types.insert(to_ty, id);

        id
    };

    expect!(iterator, TokenTree::Punct(p), { p.as_char() == ';' });

    (alias_ident, to_tyid)
}

pub fn parser(input: TokenStream) -> Program {
    let mut iterator = input.into_iter();

    let mut functions = HashMap::new();
    let mut type_ids = HashMap::new();
    let mut types = HashMap::new();
    let mut awaiting_types = HashMap::new();

    while let Some(next) = iterator.next() {
        let ident = expect_on!(next, TokenTree::Ident(i), { i });

        match ident.to_string().as_str() {
            "type" => {
                let (alias, real) = parse_alias(&mut iterator, &type_ids, &mut awaiting_types);

                if let Some(id) = awaiting_types.remove(&alias) {
                    type_ids.insert(alias.clone(), id);
                    types.insert(id, Type::Alias(alias, real));
                } else {
                    let new_id = new_type_id();

                    type_ids.insert(alias.clone(), new_id);
                    types.insert(new_id, Type::Alias(alias, real));
                }
            }
            _ => panic!("invalid ident"),
        }
    }

    for t in awaiting_types.keys() {
        panic!("Could not find type {t}");
    }

    Program {
        functions,
        type_ids,
        types,
    }
}
