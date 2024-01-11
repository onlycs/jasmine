use itertools::Itertools;

use crate::prelude::*;

fn parse_constraints(
    iterator: &mut impl Iterator<Item = TokenTree>,
    type_ids: &HashMap<String, TypeId>,
    awaiting_types: &mut HashMap<String, TypeId>,
) -> Result<Vec<TypeId>, JasmineParserError> {
    let mut constraints = vec![];

    while let Some(next) = iterator.next() {
        let ident = expect_on!(next, TokenTree::Ident(i), { i.to_string() });

        constraints.push(if let Some(id) = type_ids.get(&ident) {
            *id
        } else if let Some(id) = awaiting_types.get(&ident) {
            *id
        } else {
            let id = new_type_id();
            awaiting_types.insert(ident, id);

            id
        });

        match iterator.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
                break;
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '+' => {
                continue;
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => {
                break;
            }
            Some(next) => bail!(SyntaxError::UnexpectedToken(next.to_string())),
            None => bail!(SyntaxError::UnexpectedEOF),
        }
    }

    Ok(constraints)
}

pub fn parse(
    iterator: &mut (impl Iterator<Item = TokenTree> + Clone),
    type_ids: &HashMap<String, TypeId>,
    awaiting_types: &mut HashMap<String, TypeId>,
) -> Result<HashMap<String, Generic>, JasmineParserError> {
    let mut generics = HashMap::new();

    expect!(iterator, TokenTree::Punct(p), { p.as_char() == '<' });

    let mut iterator = iterator.take_while_ref(|t| {
        if let TokenTree::Punct(p) = t {
            p.as_char() != '>'
        } else {
            true
        }
    });

    while let Some(next) = iterator.next() {
        let ident = expect_on!(next, TokenTree::Ident(i), { i.to_string() });

        match iterator.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
                generics.insert(
                    ident.clone(),
                    Generic {
                        id: new_type_id(),
                        name: ident,
                        constraints: vec![],
                    },
                );
            }
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                let constraints = parse_constraints(&mut iterator, type_ids, awaiting_types)?;

                generics.insert(
                    ident.clone(),
                    Generic {
                        id: new_type_id(),
                        name: ident,
                        constraints,
                    },
                );
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => {
                break;
            }
            Some(next) => bail!(SyntaxError::UnexpectedToken(next.to_string())),
            None => bail!(SyntaxError::UnexpectedEOF),
        }
    }

    Ok(generics)
}
