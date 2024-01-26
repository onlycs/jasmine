use super::*;
use crate::prelude::*;

pub fn parse(iterator: &mut TokenIterator) -> Result<UncheckedType, ParserError> {
    proc_expect!(iterator, "{ident:0} = {actual};");
    let actual = types::parse_full(&mut actual)?;

    Ok(UncheckedType {
        ident: Arc::new(ident.to_string()),
        kind: UncheckedTypeKind::AliasTo(actual),
    })
}
