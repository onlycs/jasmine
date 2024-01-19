use super::*;
use crate::prelude::*;

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });

    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let mut braced = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into_iter().peekable() }
    );

    let fields = common::parse_kv(&mut braced)?;

    Ok(UncheckedType {
        ident: Arc::new(type_name),
        kind: UncheckedTypeKind::Struct(UncheckedStruct {
            fields,
            generics,
            methods: HashMap::new(),
            traits: vec![],
        }),
    })
}
