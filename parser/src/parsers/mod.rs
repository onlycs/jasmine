mod alias;
mod function;
mod generics;
mod structs;
mod types;

use crate::prelude::*;

fn _parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedProgram, ParserError> {
    let mut functions = HashMap::new();
    let mut types = HashMap::new();

    while let Some(next) = iterator.next() {
        let ident = expect!(on next, TokenTree::Ident(i), ret { i });

        match ident.to_string().as_str() {
            "type" => {
                let alias = alias::parse(iterator)?;

                types.insert(alias.ident(), alias);
            }
            "struct" => {
                let s = structs::parse(iterator)?;

                types.insert(s.ident(), s);
            }
            "fn" => {
                let f = function::parse(iterator)?;

                functions.insert(f.ident(), f);
            }
            i => bail!(SyntaxError::InvalidIdent(i.to_string())),
        }
    }

    Ok(UncheckedProgram { functions, types })
}

pub fn parse(stream: TokenStream) -> Result<UncheckedProgram, FullParserError> {
    let mut iterator = stream.into_iter().peekable();

    match _parse(&mut iterator) {
        Ok(p) => Ok(p),
        Err(e) => Err(FullParserError {
            error: e,
            next_item: iterator.next(),
        }),
    }
}
