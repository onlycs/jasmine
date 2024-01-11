use crate::{parsers::*, prelude::*};

pub fn parse(
    iterator: &mut (impl Iterator<Item = TokenTree> + Clone),
    type_ids: &HashMap<String, TypeId>,
    awaiting_types: &mut HashMap<String, TypeId>,
) -> Result<Struct, JasmineParserError> {
    let type_name = expect_ret!(iterator, TokenTree::Ident(ident), { ident.to_string() });

    if type_ids.get(&type_name).is_some() {
        bail!(TypeError::DuplicateType(type_name));
    }

    let generics = if let Some(TokenTree::Punct(p)) = iterator.clone().next()
        && p.as_char() == '<'
    {
        generics::parse(iterator, type_ids, awaiting_types)?
    } else {
        HashMap::new()
    };

    let mut braced = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Brace },
        { g.stream().into_iter() }
    );

    let mut fields = HashMap::new();

    while let Some(next) = braced.next() {
        let ident = expect_on!(next, TokenTree::Ident(i), { i.to_string() });

        expect!(braced, TokenTree::Punct(p), { p.as_char() == ':' });

        let ty = expect_ret!(braced, TokenTree::Ident(i), { i.to_string() });

        if let Some(try_comma) = braced.next() {
            expect_on!(try_comma, TokenTree::Punct(p), { p.as_char() == ',' });
        }

        let tyid = if let Some(generic) = generics.get(&ty) {
            generic.id
        } else if let Some(id) = type_ids.get(&ty) {
            *id
        } else if let Some(id) = awaiting_types.get(&ty) {
            *id
        } else {
            let id = new_type_id();
            awaiting_types.insert(ty, id);

            id
        };

        fields.insert(ident, tyid);
    }

    Ok(Struct {
        id: new_type_id(),
        name: type_name,
        generics,
        fields,
        methods: HashMap::new(),
    })
}
