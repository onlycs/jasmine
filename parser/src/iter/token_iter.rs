use std::mem;

use crate::prelude::*;

type Inner = <TokenStream as IntoIterator>::IntoIter;
type Item = <TokenStream as IntoIterator>::Item;
type Limiter = Arc<dyn Fn(&Item) -> bool>;

#[derive(Clone, Debug)]
pub struct Cache {
    item: Item,
    next: Option<Box<Cache>>,
}

impl Cache {
    pub fn new(item: Item) -> Self {
        Self { item, next: None }
    }

    pub fn push(&mut self, item: Item) {
        if let Some(next) = self.next.as_mut() {
            next.push(item);
        } else {
            self.next = Some(Box::new(Self::new(item)));
        }
    }

    pub fn get(&self) -> &Item {
        &self.item
    }

    pub fn at(&self, index: usize) -> Option<&Item> {
        if index == 0 {
            Some(&self.item)
        } else {
            self.next.as_ref().and_then(|n| n.at(index - 1))
        }
    }

    pub fn forward(&mut self) -> Option<Item> {
        let mut next = self.next.take()?;
        mem::swap(self, &mut *next);

        Some(next.item)
    }
}

#[derive(Clone)]
pub struct LimitSeries {
    inner: Limiter,
    pub next: Option<Box<LimitSeries>>,
}

impl LimitSeries {
    pub fn new(inner: Limiter) -> Self {
        Self { inner, next: None }
    }

    pub fn push(&mut self, inner: Limiter) {
        if let Some(next) = self.next.as_mut() {
            next.push(inner);
        } else {
            self.next = Some(Box::new(Self::new(inner)));
        }
    }

    pub fn delimit_one(&mut self) {
        let Some(mut next) = self.next.take() else {
            return;
        };

        mem::swap(self, &mut *next);
    }

    pub fn verify_all(&self, item: &Item) -> bool {
        if !(self.inner)(item) {
            false
        } else {
            self.next
                .as_ref()
                .map(|n| n.verify_all(item))
                .unwrap_or(true)
        }
    }
}

#[derive(Clone)]
pub struct TokenIterator {
    pub inner: Inner,
    pub cache: Option<Cache>,
    pub limit: Option<LimitSeries>,
}

impl TokenIterator {
    pub fn new(stream: TokenStream) -> Self {
        let inner = stream.into_iter();

        Self {
            inner,
            limit: None,
            cache: None,
        }
    }

    /// Collects the cached items (up to count)
    pub fn decache(&mut self, count: usize, collect: &mut Vec<Item>) {
        if count == 0 {
            return;
        }

        let next = if self
            .cache
            .as_ref()
            .map(|c| c.next.is_some())
            .unwrap_or(false)
        {
            self.cache.as_mut().and_then(|c| c.forward())
        } else {
            self.cache.take().map(|c| c.item)
        };

        if let Some(next) = next {
            collect.push(next);
        } else {
            return;
        }

        self.decache(count - 1, collect);
    }

    pub fn matches(&mut self, front: impl Into<String>) -> bool {
        let mut front_owned: String = front.into();
        let mut front = front_owned.as_mut_str();
        let mut items_consumed = 0;

        while !front.is_empty() {
            if let Some(cached) = self
                .cache
                .as_ref()
                .and_then(|c| c.at(items_consumed))
                .map(TokenTree::to_string)
            {
                if front.starts_with(&cached) {
                    front = &mut front[cached.len()..];
                    items_consumed += 1;
                } else {
                    return false;
                }
            } else if self.cache_one().is_none() {
                return false;
            }
        }

        self.decache(items_consumed, &mut vec![]);

        true
    }

    pub fn cache_one(&mut self) -> Option<&Item> {
        let next = self.inner.next()?;

        if let Some(cache) = self.cache.as_mut() {
            cache.push(next);
        } else {
            self.cache = Some(Cache::new(next));
        }

        self.cache.as_ref().map(|c| c.get())
    }

    fn _peek_cache(&self) -> Option<&Item> {
        self.cache.as_ref().map(Cache::get)
    }

    pub fn peek(&mut self) -> Option<&Item> {
        if self._peek_cache().is_none() {
            self.cache_one();
        }

        self._peek_cache()
            .filter(|t| self.limit.as_ref().map(|l| l.verify_all(t)).unwrap_or(true))
    }

    pub fn collect_while(&mut self, pred: impl Fn(&Item) -> bool) -> Vec<Item> {
        let mut collected = vec![];

        while let Some(peeked) = self.peek()
            && pred(peeked)
        {
            collected.push(self.next().unwrap());
        }

        collected
    }

    pub fn permit_if(&mut self, pred: impl Fn(&Item) -> bool + 'static) {
        match self.limit.as_mut() {
            Some(limit) => limit.push(Arc::new(pred)),
            None => self.limit = Some(LimitSeries::new(Arc::new(pred))),
        }
    }

    pub fn block_when(&mut self, pred: impl Fn(&Item) -> bool + 'static) {
        self.permit_if(move |t| !pred(t));
    }

    pub fn permitting(&mut self, pred: impl Fn(&Item) -> bool + 'static) -> &mut Self {
        self.permit_if(pred);
        self
    }

    pub fn remove_first_limit(&mut self) {
        if let Some(limit) = self.limit.as_mut()
            && limit.next.is_some()
        {
            limit.delimit_one();
        } else {
            self.limit = None;
        }
    }
}

impl Iterator for TokenIterator {
    type Item = Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(cached) = self.cache.as_ref().map(Cache::get) {
            if self
                .limit
                .clone()
                .map(|l| l.verify_all(cached))
                .unwrap_or(true)
            {
                return self
                    .cache
                    .as_mut()
                    .and_then(|c| c.forward())
                    .or_else(|| self.cache.take().map(|c| c.item));
            } else {
                return None;
            }
        }

        match self.inner.next() {
            Some(next)
                if self
                    .limit
                    .clone()
                    .map(|l| l.verify_all(&next))
                    .unwrap_or(true) =>
            {
                Some(next)
            }
            Some(next) => {
                if let Some(cache) = self.cache.as_mut() {
                    cache.push(next);
                } else {
                    self.cache = Some(Cache::new(next));
                }

                None
            }
            _ => None,
        }
    }
}

impl From<Vec<Item>> for TokenIterator {
    fn from(value: Vec<Item>) -> Self {
        Self {
            inner: value.into_iter().collect::<TokenStream>().into_iter(),
            cache: None,
            limit: None,
        }
    }
}

impl From<TokenStream> for TokenIterator {
    fn from(value: TokenStream) -> Self {
        Self::new(value)
    }
}
