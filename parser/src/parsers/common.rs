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
        .filter_map(Result::ok)
        .for_each(|(ident, value)| collector.add(ident, value));

    Ok(collector)
}

pub trait CollectKv {
    fn new() -> Self;
    fn add(&mut self, key: String, value: UncheckedFullType);
}

impl CollectKv for HashMap<String, UncheckedFullType> {
    fn new() -> Self {
        HashMap::new()
    }

    fn add(&mut self, key: String, value: UncheckedFullType) {
        self.insert(key, value);
    }
}

impl CollectKv for Vec<(String, UncheckedFullType)> {
    fn new() -> Self {
        vec![]
    }

    fn add(&mut self, key: String, value: UncheckedFullType) {
        self.push((key, value));
    }
}
