mod alias;
mod common;
mod enums;
mod function;
mod generics;
mod structs;
mod traits;
mod types;

use crate::prelude::*;

fn _parse(iterator: &mut TokenIterator) -> Result<UncheckedProgram, ParserError> {
    let mut functions = HashMap::new();
    let mut types = HashMap::new();

    while let Some(next) = iterator.next() {
        let ident = expect!(on next, TokenTree::Ident(ref i), ret { i });

        match ident.to_string().as_str() {
            "type" => {
                let alias = alias::parse(iterator)?;
                let ident = alias.ident();

                if types.insert(Arc::clone(&ident), alias).is_some() {
                    bail!(TypeError::DuplicateType(ident.to_string()));
                }
            }
            "struct" => {
                let s = structs::parse(iterator)?;
                let ident = s.ident();

                if types.insert(Arc::clone(&ident), s).is_some() {
                    bail!(TypeError::DuplicateType(ident.to_string()));
                }
            }
            "fn" => {
                let f = function::parse(iterator)?;
                let ident = f.ident();

                if functions.insert(Arc::clone(&ident), f).is_some() {
                    bail!(TypeError::DuplicateFunction(ident.to_string()));
                }
            }
            "enum" => {
                let e = enums::parse(iterator)?;
                let ident = e.ident();

                if types.insert(Arc::clone(&ident), e).is_some() {
                    bail!(TypeError::DuplicateType(ident.to_string()));
                }
            }
            "trait" => {
                let t = traits::parse(iterator)?;
                let ident = t.ident();

                if types.insert(Arc::clone(&ident), t).is_some() {
                    bail!(TypeError::DuplicateType(ident.to_string()));
                }
            }
            i => bail!(SyntaxError::InvalidIdent {
                ident: i.to_string(),
                next: next,
            }),
        }
    }

    Ok(UncheckedProgram { functions, types })
}

pub fn parse(stream: TokenStream) -> Result<UncheckedProgram, FullParserError> {
    let mut iterator = TokenIterator::new(stream);

    match _parse(&mut iterator) {
        Ok(p) => Ok(p),
        Err(e) => Err(FullParserError {
            error: e,
            next_item: iterator.next(),
        }),
    }
}
