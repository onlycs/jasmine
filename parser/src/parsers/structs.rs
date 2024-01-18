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

    let mut fields = HashMap::new();

    while let Some(next) = braced.next() {
        let ident = expect!(on next, TokenTree::Ident(i), ret { i.to_string() });

        expect!(braced, TokenTree::Punct(p), chk { p.as_char() == ':' });

        let full_type = types::parse_full(&mut braced)?;

        fields.insert(ident, full_type);

        expect!(braced, TokenTree::Punct(p), chk { p.as_char() == ',' });
    }

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
