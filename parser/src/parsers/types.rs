use crate::prelude::*;

pub fn parse_tuple(iterator: &mut TokenIterator) -> Result<Vec<UncheckedFullTypeId>, ParserError> {
    let res = iterator
        .split(',')
        .map(|group| parse_full(&mut group.into()))
        .check()?
        .collect_vec();

    Ok(res)
}

pub fn parse_full(iterator: &mut TokenIterator) -> Result<UncheckedFullTypeId, ParserError> {
    if let Some(TokenTree::Group(g)) = iterator.peek()
        && g.delimiter() == Delimiter::Parenthesis
    {
        let inner = g.stream();
        let inner = parse_tuple(&mut inner.into())?;

        return Ok(UncheckedFullTypeId::Tuple(inner));
    }

    let mut refs = iterator
        .collect_while(|item| {
            matches!(item, TokenTree::Punct(p) if p.as_char() == '&')
                || matches!(item, TokenTree::Ident(i) if i.to_string() == "mut")
        })
        .into_iter();

    let outer = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });

    // pathed (path::to::type)
    if iterator.matches("::") {
        let ahead = parse_full(iterator)?;

        return Ok(UncheckedFullTypeId::Path {
            behind: outer,
            ahead: Box::new(ahead),
        });
    }

    let inner = vec![];

    if iterator.matches("<") {
        iterator.permit_if(|p| !matches!(p, TokenTree::Punct(p) if p.as_char() == '>'));

        iterator
            .split(',')
            .map(|iter| parse_full(&mut iter.into()))
            .check()?
            .collect_vec();

        iterator.remove_first_limit();

        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '>' });
    }

    let mut full_type = if inner.is_empty() {
        UncheckedFullTypeId::Simple(outer)
    } else {
        UncheckedFullTypeId::Generic { outer, inner }
    };

    while let Some(next) = refs.next() {
        full_type = match next.to_string().as_str() {
            "&" => UncheckedFullTypeId::Ref(Box::new(full_type)),
            "mut" => UncheckedFullTypeId::RefMut(Box::new(full_type)),
            _ => unreachable!(),
        }
    }

    Ok(full_type)
}
