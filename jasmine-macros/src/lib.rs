#![feature(stmt_expr_attributes)]

use proc_macro2::*;
use quote::quote;
use syn::parse_macro_input;

macro_rules! expect {
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
		match $tree.next() {
			Some($expected) => {
				if !$check {
					panic!()
				} else {
					$ret
				}
			}
			Some(other) => {
				panic!()
			}
			_ => panic!(),
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

	($tree:expr, comma) => {
		expect!($tree, TokenTree::Punct(p), chk { p.as_char() == ',' })
	}
}

#[proc_macro]
pub fn proc_expect(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let mut stream = parse_macro_input!(input as TokenStream).into_iter();

    let i_iter = expect!(stream, TokenTree::Ident(i), ret { i });

    expect!(stream, comma);

    let mut lit = expect!(
        stream,
        TokenTree::Literal(l),
        ret { l.to_string() }
    );

    lit = lit.trim_matches('"').to_string();

    let mut split = lit
        .split("}")
        .map(|s| s.to_string())
        .map(|s| s.split("{").map(|s| s.to_string()).collect::<Vec<_>>())
        .flatten()
        .filter(|s| !s.is_empty())
        .map(|s| s.replace(" ", ""));

    let mut in_subst = lit.starts_with("{");
    let mut out = vec![];

    while let Some(s) = split.next() {
        if in_subst {
            let zeroth = s.ends_with(":0");

            let s = Ident::new(s.trim_end_matches(":0"), Span::call_site());
            let next = split.next();

            out.push(quote! { let mut #s = vec![]; });

            if let Some(next) = next {
                out.push(quote! {
                    while !#i_iter.matches(#next) { #s.push(#i_iter.next().unwrap()); }
                });
            } else {
                out.push(quote! {
                    #s = #i_iter.collect();
                });
            }

            out.push(quote! {
                let mut #s = TokenIterator::from(#s);
            });

            if zeroth {
                out.push(quote! {
                    let mut #s = #s.next().unwrap();
                });
            }
        } else {
            out.push(quote! {
                #i_iter.matches(#s);
            });

            in_subst = !in_subst;
        }
    }

    out.into_iter()
        .flat_map(|i| i.into_iter())
        .collect::<TokenStream>()
        .into()
}
