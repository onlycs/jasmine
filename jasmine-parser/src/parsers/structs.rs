use super::*;
use crate::prelude::*;

pub fn parse(iterator: &mut TokenIterator) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);
    let inner = common::parse_composite_data(iterator)?;

    if matches!(inner, UncheckedCompositeData::Tuple(_)) {
        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ';' });
    }

    Ok(UncheckedType {
        ident: Arc::new(type_name),
        kind: UncheckedTypeKind::Struct(UncheckedStruct {
            inner,
            generics,
            methods: HashMap::new(),
            traits: vec![],
        }),
    })
}
