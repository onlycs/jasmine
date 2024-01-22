use crate::prelude::*;

pub fn parse_tuple(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<Vec<UncheckedFullTypeId>, ParserError> {
    let res = iterator
        .split(|val| matches!(val, TokenTree::Punct(p) if p.as_char() == ','))
        .map(|mut group| parse_full(&mut group))
        .check()?
        .collect_vec();

    Ok(res)
}

pub fn parse_full(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<UncheckedFullTypeId, ParserError> {
    if let Some(TokenTree::Group(g)) = iterator.peek()
        && g.delimiter() == Delimiter::Parenthesis
    {
        let inner = g.stream().into_iter();
        let inner = parse_tuple(&mut inner.peekable())?;

        return Ok(UncheckedFullTypeId::Tuple(inner));
    }

    let mut refs = iterator.collect_while(|item| {
        matches!(item, TokenTree::Punct(p) if p.as_char() == '&')
            || matches!(item, TokenTree::Ident(i) if i.to_string() == "mut")
    });

    let outer = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });

    // pathed (path::to::type)
    if iterator
        .next_if(|p| matches!(p, TokenTree::Punct(p) if p.as_char() == ':')) // only consumes first colon
        .is_some()
    {
        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ':' });

        let ahead = parse_full(iterator)?;

        return Ok(UncheckedFullTypeId::Path {
            behind: outer,
            ahead: Box::new(ahead),
        });
    }

    let mut inner = vec![];

    if iterator
        .next_if(|p| matches!(p, TokenTree::Punct(p) if p.as_char() == '<'))
        .is_some()
    {
        let mut generics =
            iterator.copy_while(|p| !matches!(p, TokenTree::Punct(p) if p.as_char() == '>'));

        inner = generics
            .split(|p| matches!(p, TokenTree::Punct(p) if p.as_char() == ','))
            .map(|mut iter| parse_full(&mut iter))
            .check()?
            .collect_vec();

        drop(generics);

        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '>' });
    }

    let mut full_type = if inner.is_empty() {
        UncheckedFullTypeId::Simple(outer)
    } else {
        UncheckedFullTypeId::Generic { outer, inner }
    };

    while let Some(next) = refs.next() {
        full_type = match next {
            TokenTree::Punct(p) if p.as_char() == '&' => {
                UncheckedFullTypeId::Ref(Box::new(full_type))
            }
            TokenTree::Ident(i) if i.to_string() == "mut" => {
                UncheckedFullTypeId::RefMut(Box::new(full_type))
            }
            _ => unreachable!(),
        }
    }

    Ok(full_type)
}