pub use crate::errors::*;
pub use libjasmine::prelude::*;
pub use proc_macro2::{Delimiter, TokenStream, TokenTree};
pub use std::collections::{HashMap, HashSet};

macro_rules! bail {
    ($err:expr) => {
        return Err($err.into())
    };
}

macro_rules! expect_mac {
    ($tree:expr, $expected:pat, $check:block) => {
        expect!($tree, $expected, $check, {})
    };
    ($tree:expr, $expected:pat, $check:block, $ret:block) => {
        match $tree.next() {
            Some($expected) => {
                if !$check {
                    bail!(SyntaxError::ExpectWithCheck(
                        stringify!($expected),
                        stringify!($check)
                    ))
                } else {
                    $ret
                }
            }
            _ => bail!(SyntaxError::ExpectWithCheck(
                stringify!($expected),
                stringify!($check)
            )),
        }
    };
}

macro_rules! expect_on {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree {
            $expected => $ret,
            _ => bail!(SyntaxError::ExpectWithoutCheck(stringify!($expected))),
        }
    };
}

macro_rules! expect_ret {
    ($tree:expr, $expected:pat, $ret:block) => {
        match $tree.next().unwrap() {
            $expected => $ret,
            _ => bail!(SyntaxError::ExpectWithoutCheck(stringify!($expected))),
        }
    };
}

pub(crate) use bail;
pub(crate) use expect_mac as expect;
pub(crate) use expect_on;
pub(crate) use expect_ret;
