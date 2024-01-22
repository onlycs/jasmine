use super::*;
use crate::prelude::*;

pub fn parse_kv<Collector: CollectKv>(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<Collector, ParserError> {
    let mut collector = Collector::new();

    iterator
        .split(|a| match a {
            TokenTree::Punct(punct) => punct.as_char() == ',',
            _ => false,
        })
        .map(|mut iter| {
            let ident = expect!(iter, TokenTree::Ident(i), ret { i.to_string() });
            expect!(iter, TokenTree::Punct(punct), ret { punct.as_char() == ':' });
            let value = types::parse_full(&mut iter)?;
            Result::<_, ParserError>::Ok((ident, value))
        })
        .check()?
        .for_each(|(ident, value)| collector.add(ident, value));

    Ok(collector)
}

pub fn parse_composite_data(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree> + Clone>,
) -> Result<UncheckedCompositeData, ParserError> {
    let (mut inner, inner_delim) = expect!(
        iterator,
        TokenTree::Group(g),
        { matches!(g.delimiter(), Delimiter::Brace | Delimiter::Parenthesis) },
        { (g.stream().into_iter().peekable(), g.delimiter()) }
    );

    let inner = match inner_delim {
        Delimiter::Parenthesis => UncheckedCompositeData::Tuple(types::parse_tuple(&mut inner)?),
        Delimiter::Brace => UncheckedCompositeData::Struct(common::parse_kv(&mut inner)?),
        bad => bail!(SyntaxError::UnexpectedToken(TokenTree::Group(
            proc_macro2::Group::new(bad, inner.collect())
        ))),
    };

    Ok(inner)
}

pub trait CollectKv {
    fn new() -> Self;
    fn add(&mut self, key: String, value: UncheckedFullTypeId);
}

impl CollectKv for HashMap<String, UncheckedFullTypeId> {
    fn new() -> Self {
        HashMap::new()
    }

    fn add(&mut self, key: String, value: UncheckedFullTypeId) {
        self.insert(key, value);
    }
}

impl CollectKv for Vec<(String, UncheckedFullTypeId)> {
    fn new() -> Self {
        vec![]
    }

    fn add(&mut self, key: String, value: UncheckedFullTypeId) {
        self.push((key, value));
    }
}
