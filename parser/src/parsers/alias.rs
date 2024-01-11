use crate::prelude::*;

pub fn parse(
    iterator: &mut impl Iterator<Item = TokenTree>,
    type_ids: &HashMap<String, TypeId>,
    awaiting_types: &mut HashMap<String, TypeId>,
) -> Result<(String, TypeId), JasmineParserError> {
    let alias_ident = expect_ret!(iterator, TokenTree::Ident(ident), { ident.to_string() });

    if type_ids.get(&alias_ident).is_some() {
        bail!(TypeError::DuplicateType(alias_ident));
    }

    expect!(iterator, TokenTree::Punct(p), { p.as_char() == '=' });

    let to_ty = expect_ret!(iterator, TokenTree::Ident(i), { i.to_string() });

    let to_tyid = if let Some(id) = type_ids.get(&to_ty) {
        *id
    } else if let Some(id) = awaiting_types.get(&to_ty) {
        *id
    } else {
        let id = new_type_id();
        awaiting_types.insert(to_ty, id);

        id
    };

    expect!(iterator, TokenTree::Punct(p), { p.as_char() == ';' });

    Ok((alias_ident, to_tyid))
}
