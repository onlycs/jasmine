use super::*;
use crate::prelude::*;

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedType, ParserError> {
    let type_name = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });

    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let (mut inner, inner_delim) = expect!(
        iterator,
        TokenTree::Group(g),
        { matches!(g.delimiter(), Delimiter::Brace | Delimiter::Parenthesis) },
        { (g.stream().into_iter().peekable(), g.delimiter()) }
    );

    let inner = match inner_delim {
        Delimiter::Parenthesis => UncheckedCompositeData::Tuple(types::parse_tuple(&mut inner)?),
        Delimiter::Brace => UncheckedCompositeData::Struct(common::parse_kv(&mut inner)?),
        bad => bail!(SyntaxError::UnexpectedToken(TokenTree::Group(
            proc_macro2::Group::new(bad, inner.collect())
        ))),
    };

    if inner_delim == Delimiter::Parenthesis {
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
