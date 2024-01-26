pub use crate::{errors::*, iter::*};
pub use itertools::*;
pub use jasmine_ast::prelude::*;
pub use jasmine_macros::proc_expect;
pub use proc_macro2::{Delimiter, TokenStream, TokenTree};
pub use std::collections::{HashMap, HashSet};
pub use std::sync::Arc;

macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
}

macro_rules! expect_mac {
    (on $tree:expr, $expected:pat, $check:block, $ret:block) => {
        match $tree {
            $expected => {
                if !$check {
                    bail!(SyntaxError::ExpectWithCheck {
                        item: stringify!($expected),
                        check: stringify!($check),
                        next: $tree,
                    })
                } else {
                    $ret
                }
            }
            _ => bail!(SyntaxError::ExpectWithCheck {
                item: stringify!($expected),
                check: stringify!($check),
                next: $tree,
            }),
        }
    };

    (on $tree:expr, $expected:pat, ret $ret:block) => {
        expect!(on $tree, $expected, { true }, $ret)
    };

    (on $tree:expr, $expected:pat, chk $check:block) => {
        expect!(on $tree, $expected, $check, {})
    };

    (on $tree:expr, $expected:pat) => {
        expect!(on $tree, $expected, { true }, {})
    };

    ($tree:expr, $expected:pat, $check:block, $ret:block) => {
        #[allow(unused)]
        match $tree.peek() {
            Some($expected) => {
                if !$check {
                    bail!(SyntaxError::ExpectWithCheck {
                        item: stringify!($expected),
                        check: stringify!($check),
                        next: $tree.peek().unwrap().clone(),
                    })
                } else {
                    let Some($expected) = $tree.next() else { panic!() };
                    $ret
                }
            }
            Some(other) => {
                bail!(SyntaxError::ExpectWithCheck {
                    item: stringify!($expected),
                    check: stringify!($check),
                    next: $tree.peek().unwrap().clone(),
                })
            }
            _ => bail!(SyntaxError::UnexpectedEOF),
        }
    };

    ($tree:expr, $expected:pat, ret $ret:block) => {
        expect!($tree, $expected, { true }, $ret)
    };

    ($tree:expr, $expected:pat, chk $check:block) => {
        expect!($tree, $expected, $check, {})
    };

    ($tree:expr, $expected:pat) => {
        expect!($tree, $expected, { true }, {})
    };
}

#[allow(unused_macros)]
macro_rules! hashmap {
    () => {
        {
            use std::collections::HashMap;
            HashMap::new()
        }
    };

    ($($a:expr => $b:expr),+ $(,)?) => {
        {
            use std::collections::HashMap;
            let mut map = HashMap::new();
            $(map.insert($a, $b);)+
            map
        }
    };
}

#[allow(unused_macros)]
macro_rules! hashset {
    () => {
        {
            use std::collections::HashSet;
            HashSet::new()
        }
    };

    ($($a:expr),+ $(,)?) => {
        {
            use std::collections::HashSet;
            let mut set = HashSet::new();
            $(set.insert($a);)+
            set
        }
    };
}

pub(crate) use bail;
pub(crate) use expect_mac as expect;

#[allow(unused_imports)]
pub(crate) use hashmap;

#[allow(unused_imports)]
pub(crate) use hashset;
