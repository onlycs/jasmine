use super::*;
use crate::prelude::*;

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedType, ParserError> {
    let alias_ident = expect!(iterator, TokenTree::Ident(ident), ret { ident.to_string() });

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '=' });

    let actual = types::parse_full(iterator)?;

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ';' });

    Ok(UncheckedType {
        ident: Arc::new(alias_ident),
        kind: UncheckedTypeKind::AliasTo(actual),
    })
}
