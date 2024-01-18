use super::*;
use crate::prelude::*;

pub fn parse(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedFunction, ParserError> {
    // fn myfn<T>(args) -> return type {block}

    let fn_name = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let mut self_as = FunctionSelf::None;
    let mut params = vec![];
    let mut can_self_check = true;

    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '(' });

    loop {
        if matches!(iterator.peek(), Some(TokenTree::Punct(p)) if p.as_char() == ')') {
            iterator.next();
            break;
        }

        if can_self_check {
            match iterator
                .peek()
                .map(|n| n.to_string())
                .as_ref()
                .map(|n| n as &str)
            {
                Some("self") => {
                    self_as = FunctionSelf::Consume;
                    iterator.next();
                }
                Some("&") => match iterator
                    .nth(1)
                    .map(|n| n.to_string())
                    .as_ref()
                    .map(|n| n as &str)
                {
                    Some("self") => self_as = FunctionSelf::Ref,
                    Some("mut") => match iterator
                        .next()
                        .map(|n| n.to_string())
                        .as_ref()
                        .map(|n| n as &str)
                    {
                        Some("self") => self_as = FunctionSelf::RefMut,
                        Some(bad) => bail!(SyntaxError::UnexpectedToken(bad.to_string())),
                        None => bail!(SyntaxError::UnexpectedEOF),
                    },
                    Some(bad) => bail!(SyntaxError::UnexpectedToken(bad.to_string())),
                    None => bail!(SyntaxError::UnexpectedEOF),
                },
                Some(_) => {
                    let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
                    expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ':' });

                    params.push((ident, types::parse_full(iterator)?));
                }
                None => bail!(SyntaxError::UnexpectedEOF),
            }

            can_self_check = false;
        } else {
            let ident = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
            expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == ':' });

            params.push((ident, types::parse_full(iterator)?));
        }

        match iterator.next() {
            Some(TokenTree::Punct(p)) => match p.as_char() {
                ')' => break,
                ',' => continue,
                bad => bail!(SyntaxError::UnexpectedToken(bad.to_string())),
            },
            Some(bad) => bail!(SyntaxError::UnexpectedToken(bad.to_string())),
            None => bail!(SyntaxError::UnexpectedEOF),
        }
    }

    let returns = if let Some(TokenTree::Punct(p)) = iterator.peek()
        && p.as_char() == '-'
    {
        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '-' });
        expect!(iterator, TokenTree::Punct(p), chk { p.as_char() == '>' });

        Some(types::parse_full(iterator)?)
    } else {
        None
    };

    let body = match iterator.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
            UncheckedBodyData::WithBody(g)
        }
        Some(TokenTree::Punct(p)) if p.as_char() == ';' => UncheckedBodyData::Abstract,
        Some(bad) => bail!(SyntaxError::UnexpectedToken(bad.to_string())),
        None => bail!(SyntaxError::UnexpectedEOF),
    };

    Ok(UncheckedFunction {
        ident: Arc::new(fn_name),
        generics,
        params,
        returns,
        self_as,
        body,
    })
}
