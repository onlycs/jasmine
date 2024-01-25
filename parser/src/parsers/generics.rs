use super::*;
use crate::prelude::*;

pub fn parse_constraints(
    iterator: &mut TokenIterator,
) -> Result<HashSet<UncheckedFullTypeId>, ParserError> {
    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ':' });

    let constraints = iterator
        .split('+')
        .map(|iter| types::parse_full(&mut iter.into()))
        .check()?
        .collect();

    Ok(constraints)
}

pub fn parse(iterator: &mut TokenIterator) -> Result<Vec<UncheckedGeneric>, ParserError> {
    let mut generics = vec![];

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '<' });

    iterator.block_when(|t| t.to_string() == ">");

    iterator
        .split(",")
        .map(Vec::<TokenTree>::into)
        .map(|mut iter: TokenIterator| {
            let ident = expect!(iter, TokenTree::Ident(i), ret { i.to_string() });
            let constraints = parse_constraints(&mut iter).unwrap_or_default();

            Result::<_, ParserError>::Ok(UncheckedGeneric { ident, constraints })
        })
        .filter_map(Result::ok)
        .for_each(|generic| generics.push(generic));

    iterator.remove_first_limit();

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '>' });

    Ok(generics)
}
