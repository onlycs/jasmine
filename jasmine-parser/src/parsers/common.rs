use super::*;
use crate::prelude::*;

#[allow(unused_variables)]
pub fn parse_kv<Collector: CollectKv>(
    iterator: &mut TokenIterator,
) -> Result<Collector, ParserError> {
    let collector = iterator
        .split(',')
        .map(Vec::<TokenTree>::into)
        .map(|mut iter: TokenIterator| {
            proc_expect!(iter, "{ident:0}:{value}");

            Result::<_, ParserError>::Ok((ident.to_string(), types::parse_full(&mut value)?))
        })
        .check()?
        .fold(
            Result::<_, TypeError>::Ok(Collector::new()),
            |collector, (ident, value)| {
                let mut collector = collector?;

                if collector.add(ident.clone(), value) {
                    bail!(TypeError::DupilicateKV(ident));
                }

                Ok(collector)
            },
        )?;

    Ok(collector)
}

pub fn parse_composite_data(
    iterator: &mut TokenIterator,
) -> Result<UncheckedCompositeData, ParserError> {
    let (mut inner, inner_delim) = expect!(
        iterator,
        TokenTree::Group(g),
        { matches!(g.delimiter(), Delimiter::Brace | Delimiter::Parenthesis) },
        { (g.stream().into(), g.delimiter()) }
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
    fn add(&mut self, key: String, value: UncheckedFullTypeId) -> bool;
}

impl CollectKv for HashMap<String, UncheckedFullTypeId> {
    fn new() -> Self {
        HashMap::new()
    }

    fn add(&mut self, key: String, value: UncheckedFullTypeId) -> bool {
        self.insert(key, value).is_some()
    }
}

impl CollectKv for Vec<(String, UncheckedFullTypeId)> {
    fn new() -> Self {
        vec![]
    }

    fn add(&mut self, key: String, value: UncheckedFullTypeId) -> bool {
        self.push((key, value));
        self.iter().dedup_with_count().any(|(count, _)| count > 1)
    }
}
