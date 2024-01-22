use super::*;
use crate::prelude::*;

fn parse_assoc_type(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<(String, UncheckedAssicatedType), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let constraints = generics::parse_constraints(&mut iterator.copy_while(|t| {
        !matches!(t, TokenTree::Punct(p) if p.as_char() == ';')
            && !matches!(t, TokenTree::Punct(p) if p.as_char() == '=')
    }))
    .unwrap_or_default();

    let default = if iterator
        .next_if(|a| matches!(a, TokenTree::Punct(p) if p.as_char() == '='))
        .is_some()
    {
        let mut default =
            iterator.collect_while(|t| !matches!(t, TokenTree::Punct(p) if p.as_char() == ';'));

        Some(types::parse_full(&mut default)?)
    } else {
        None
    };

    Ok((
        ident,
        UncheckedAssicatedType {
            constraints,
            default,
        },
    ))
}

fn parse_assoc_const(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<(String, UncheckedAssicatedConst), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let ty = types::parse_full(iterator)?;

    let default = if iterator
        .next_if(|a| matches!(a, TokenTree::Punct(p) if p.as_char() == '='))
        .is_some()
    {
        Some(
            iterator
                .collect_while(|t| !matches!(t, TokenTree::Punct(p) if p.as_char() == ';'))
                .collect(),
        )
    } else {
        None
    };

    Ok((ident, UncheckedAssicatedConst { ty, default }))
}

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);
    let constraints = generics::parse_constraints(
        &mut iterator.copy_while(|t| !matches!(t, TokenTree::Group(_))),
    )
    .unwrap_or_default();

    let mut inner = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into_iter().peekable() }
    );

    let mut associated_types = HashMap::new();
    let mut consts = HashMap::new();
    let mut methods = HashMap::new();

    while let Some(next) = inner.next() {
        let next = expect!(on next, TokenTree::Ident(ref i), ret { i });

        match next.to_string().as_str() {
            "type" => {
                let (ident, assoc) = parse_assoc_type(iterator)?;

                if associated_types.insert(ident.clone(), assoc).is_some() {
                    panic!() // TODO: Type error
                }
            }
            "const" => {
                let (ident, assoc) = parse_assoc_const(iterator)?;

                if consts.insert(ident, assoc).is_some() {
                    panic!() // TODO: Type error
                }
            }
            "fn" => {
                let f = function::parse(iterator)?;

                methods.insert(f.ident(), f);
            }
            _ => bail!(SyntaxError::InvalidIdent {
                ident: next.to_string(),
                next: TokenTree::Ident(next.clone()),
            }),
        }
    }

    Ok(UncheckedType {
        ident: Arc::new(type_name),
        kind: UncheckedTypeKind::Trait(UncheckedTrait {
            generics,
            methods,
            constraints,
            associated_types,
            consts,
        }),
    })
}
