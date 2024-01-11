mod alias;

use crate::prelude::*;

pub fn parse(input: TokenStream) -> Result<Program, JasmineParserError> {
    let mut iterator = input.into_iter();

    let mut functions = HashMap::new();
    let mut type_ids = HashMap::new();
    let mut types = HashMap::new();
    let mut awaiting_types = HashMap::new();

    while let Some(next) = iterator.next() {
        let ident = expect_on!(next, TokenTree::Ident(i), { i });

        match ident.to_string().as_str() {
            "type" => {
                let (alias, real) = alias::parse(&mut iterator, &type_ids, &mut awaiting_types)?;

                if let Some(id) = awaiting_types.remove(&alias) {
                    type_ids.insert(alias.clone(), id);
                    types.insert(id, Type::Alias(alias, real));
                } else {
                    let new_id = new_type_id();

                    type_ids.insert(alias.clone(), new_id);
                    types.insert(new_id, Type::Alias(alias, real));
                }
            }
            "struct" => {}
            i => bail!(SyntaxError::InvalidIdent(i.to_string())),
        }
    }

    for (name, id) in awaiting_types {
        if name.starts_with("__ext_java_") {
            type_ids.insert(name.clone(), id);
            types.insert(
                id,
                Type::JavaBuiltin(name.trim_start_matches("__ext_java_").to_string()),
            );
            continue;
        }

        bail!(TypeError::UnresolvedType(name));
    }

    Ok(Program {
        functions,
        type_ids,
        types,
    })
}
