use color_eyre::Result;

use super::error::SyntaxError;

pub trait Impetuous: Iterator {
    /// Advance the iterator, erroring if EOF occurs prematurely
    fn advance(&mut self) -> Result<<Self as Iterator>::Item>;

    /// Peek the iterator, erroring if EOF occurs early
    fn peer(&mut self) -> Result<<Self as Iterator>::Item>;
}

impl<T> Impetuous for T
where
    T: Iterator + Clone,
{
    fn peer(&mut self) -> Result<<T as Iterator>::Item> {
        let mut iter = self.clone(); // iterator cloning is cheap
        iter.advance()
    }

    fn advance(&mut self) -> Result<<T as Iterator>::Item> {
        self.next().ok_or_else(|| SyntaxError::UnexpectedEOF.into())
    }
}
