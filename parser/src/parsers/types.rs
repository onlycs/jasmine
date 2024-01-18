use crate::prelude::*;

pub fn parse_full<'a>(
    iterator: &mut Peekable<impl Iterator<Item = TokenTree>>,
) -> Result<UncheckedFullType, ParserError> {
    let mut refs = iterator.copy_while(|item| {
        matches!(item, TokenTree::Punct(p) if p.as_char() == '&')
            || matches!(item, TokenTree::Ident(i) if i.to_string() == "mut")
    });

    let outer = expect!(iterator, TokenTree::Ident(i), ret { i.to_string() });
    let mut inner = vec![];

    if let Some(TokenTree::Punct(p)) = iterator.peek()
        && p.as_char() == '<'
    {
        while let Some(next) = iterator.next() {
            if let TokenTree::Punct(p) = &next
                && p.as_char() == '>'
            {
                break;
            }

            if let TokenTree::Punct(p) = &next
                && p.as_char() != ','
            {
                bail!(SyntaxError::UnexpectedToken(next.to_string()))
            }

            inner.push(parse_full(iterator)?);
        }
    }

    let mut full_type = if inner.is_empty() {
        UncheckedFullType::Simple(outer)
    } else {
        UncheckedFullType::Generic(outer, inner)
    };

    while let Some(next) = refs.next() {
        full_type = match next {
            TokenTree::Punct(p) if p.as_char() == '&' => {
                UncheckedFullType::Ref(Box::new(full_type))
            }
            TokenTree::Ident(i) if i.to_string() == "mut" => {
                UncheckedFullType::RefMut(Box::new(full_type))
            }
            _ => unreachable!(),
        }
    }

    Ok(full_type)
}

pub trait CopyWhile: Iterator {
    fn copy_while(
        &mut self,
        predicate: impl Fn(&Self::Item) -> bool,
    ) -> Peekable<impl Iterator<Item = Self::Item>>;
}

impl<I> CopyWhile for Peekable<I>
where
    I: Iterator,
{
    fn copy_while(
        &mut self,
        predicate: impl Fn(&Self::Item) -> bool,
    ) -> Peekable<<Vec<Self::Item> as IntoIterator>::IntoIter> {
        let mut collect = vec![];

        while let Some(next) = self.peek()
            && predicate(next)
        {
            collect.push(self.next().unwrap());
        }

        collect.into_iter().peekable()
    }
}
