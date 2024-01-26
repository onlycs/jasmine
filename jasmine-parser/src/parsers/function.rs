use super::*;
use crate::prelude::*;

pub fn parse(iterator: &mut TokenIterator) -> Result<UncheckedFunction, ParserError> {
    let fn_name = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let generics = generics::parse(iterator).unwrap_or(vec![]);

    let mut args: TokenIterator = expect!(
        iterator,
        TokenTree::Group(g),
        { g.delimiter() == Delimiter::Parenthesis },
        { g.stream().into() }
    );

    let self_as;

    if args.matches("self") {
        self_as = FunctionSelf::Consume;
    } else if args.matches("&self") {
        self_as = FunctionSelf::Ref;
    } else if args.matches("&mut self") {
        self_as = FunctionSelf::RefMut;
    } else {
        self_as = FunctionSelf::None;
    }

    if self_as != FunctionSelf::None && args.peek().is_some() {
        expect!(args, TokenTree::Punct(p), chk { p.as_char() == ',' });
    }

    if self_as == FunctionSelf::Consume {
        args.next();
    }

    let params = common::parse_kv(&mut args)?;
    let returns = if iterator.matches("->") {
        Some(types::parse_full(iterator)?)
    } else {
        None
    };

    let body = match iterator.next() {
        Some(TokenTree::Group(g)) if g.delimiter() == Delimiter::Brace => {
            UncheckedBodyData::WithBody(g)
        }
        Some(TokenTree::Punct(p)) if p.as_char() == ';' => UncheckedBodyData::Abstract,
        Some(bad) => bail!(SyntaxError::UnexpectedToken(bad)),
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
