use super::*;
use crate::prelude::*;

fn parse_variant(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<(String, Option<UncheckedCompositeData>), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let inner = common::parse_composite_data(iterator).ok();

    Ok((ident, inner))
}

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let mut inner = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into_iter().peekable() }
    );

    let variants = inner
        .split(|p| matches!(p, TokenTree::Punct(p) if p.as_char() == ','))
        .map(|mut iter| parse_variant(&mut iter))
        .check()?
        .collect();

    Ok(UncheckedType {
        ident: Arc::new(type_name),
        kind: UncheckedTypeKind::Enum(UncheckedEnum {
            variants,
            generics,
            methods: HashMap::new(),
            traits: vec![],
        }),
    })
}
