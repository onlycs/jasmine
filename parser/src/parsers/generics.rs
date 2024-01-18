use super::*;
use crate::prelude::*;

fn parse_constraints(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<Vec<UncheckedFullType>, ParserError> {
    let mut constraints = vec![];

    while iterator.peek().is_some() {
        let fulltype = types::parse_full(iterator)?;

        constraints.push(fulltype);

        match iterator.peek() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => {
                iterator.next();
                break;
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '+' => {
                iterator.next();
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
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<Vec<UncheckedGeneric>, ParserError> {
    let mut generics = vec![];

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '<' });

    while let Some(next) = iterator.next() {
        let ident = expect!(on next, TokenTree::Ident(i), ret { i.to_string() });

        match iterator.next() {
            Some(TokenTree::Punct(p)) if p.as_char() == ',' => generics.push(UncheckedGeneric {
                ident,
                constraints: vec![],
            }),
            Some(TokenTree::Punct(p)) if p.as_char() == ':' => {
                generics.push(UncheckedGeneric {
                    ident,
                    constraints: parse_constraints(iterator)?,
                });

                if let Some(TokenTree::Punct(p)) = iterator.peek()
                    && p.as_char() == '>'
                {
                    break;
                }
            }
            Some(TokenTree::Punct(p)) if p.as_char() == '>' => {
                generics.push(UncheckedGeneric {
                    ident,
                    constraints: vec![],
                });

                break;
            }
            Some(next) => bail!(SyntaxError::UnexpectedToken(next.to_string())),
            None => bail!(SyntaxError::UnexpectedEOF),
        }
    }

    Ok(generics)
}
