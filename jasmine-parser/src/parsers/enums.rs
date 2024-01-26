use super::*;
use crate::prelude::*;

fn parse_variant(
    iterator: &mut TokenIterator,
) -> Result<(String, Option<UncheckedCompositeData>), ParserError> {
    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let inner = common::parse_composite_data(iterator).ok();

    Ok((ident, inner))
}

pub fn parse(iterator: &mut TokenIterator) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let mut inner: TokenIterator = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into() }
    );

    let variants = inner
        .split(',')
        .map(|iter| parse_variant(&mut iter.into()))
        .check()?
        .fold(
            Result::<_, TypeError>::Ok(HashMap::new()),
            |acc, (ident, variant)| {
                let mut acc = acc?;

                if acc.insert(ident.clone(), variant).is_some() {
                    bail!(TypeError::DuplicateVariant(ident));
                }

                Ok(acc)
            },
        )?;

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
