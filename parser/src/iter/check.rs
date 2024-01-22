pub trait Check<T, E>: Iterator<Item = Result<T, E>> {
    /// If all are Ok, return an iterator of the Ok values.
    /// If any are Err, return the first Err
    fn check(&mut self) -> Result<impl Iterator<Item = T>, E>;
}

impl<T, E, I> Check<T, E> for I
where
    I: Iterator<Item = Result<T, E>>,
{
    fn check(&mut self) -> Result<impl Iterator<Item = T>, E> {
        let mut vec = vec![];

        while let Some(next) = self.next() {
            vec.push(next?);
        }

        Ok(vec.into_iter())
    }
}
