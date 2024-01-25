use super::*;
use crate::prelude::*;

fn parse_assoc_type(
    iterator: &mut TokenIterator,
) -> Result<(String, UncheckedAssicatedType), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });

    let constraints = generics::parse_constraints(iterator.permitting(|t| {
        !matches!(t, TokenTree::Punct(p) if p.as_char() == ';')
            && !matches!(t, TokenTree::Punct(p) if p.as_char() == '=')
    }))
    .unwrap_or_default();

    iterator.remove_first_limit();

    let default = if iterator.matches('=') {
        iterator.permit_if(|t| match t {
            TokenTree::Punct(p) if p.as_char() == ';' => false,
            TokenTree::Punct(p) if p.as_char() == '=' => false,
            _ => true,
        });

        let ty = types::parse_full(iterator)?;
        iterator.remove_first_limit();

        Some(ty)
    } else {
        None
    };

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ';' });

    Ok((
        ident,
        UncheckedAssicatedType {
            constraints,
            default,
        },
    ))
}

fn parse_assoc_const(
    iterator: &mut TokenIterator,
) -> Result<(String, UncheckedAssicatedConst), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ':' });

    let ty = types::parse_full(iterator)?;

    let default = if iterator.matches('=') {
        Some(iterator.collect_while(|t| !matches!(t, TokenTree::Punct(p) if p.as_char() == ';')))
    } else {
        None
    };

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ';' });

    Ok((
        ident,
        UncheckedAssicatedConst {
            ty,
            default: default.map(Vec::into_iter).map(Iterator::collect),
        },
    ))
}

pub fn parse(iterator: &mut TokenIterator) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);

    iterator.permit_if(|t| !matches!(t, TokenTree::Group(_)));
    let constraints = generics::parse_constraints(iterator).unwrap_or_default();
    iterator.remove_first_limit();

    let mut inner: TokenIterator = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into() }
    );

    let mut associated_types = HashMap::new();
    let mut consts = HashMap::new();
    let mut methods = HashMap::new();

    while let Some(next) = inner.next() {
        let next = expect!(on next, TokenTree::Ident(ref i), ret { i });

        match next.to_string().as_str() {
            "type" => {
                let (ident, assoc) = parse_assoc_type(&mut inner)?;

                if associated_types.insert(ident.clone(), assoc).is_some() {
                    panic!() // TODO: Type error
                }
            }
            "const" => {
                let (ident, assoc) = parse_assoc_const(&mut inner)?;

                if consts.insert(ident, assoc).is_some() {
                    panic!() // TODO: Type error
                }
            }
            "fn" => {
                let f = function::parse(&mut inner)?;

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
