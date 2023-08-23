use color_eyre::Result;

use crate::token::Token;

/// Extension for the [`Iterator`] trait that provides a bunch of optional nice
/// to have functions
pub trait Impetuous: Iterator {
    type Scrutinee;
    fn step(&mut self) -> Option<Self::Item> {
        unimplemented!()
    }

    /// Access the element returned last
    fn prev(&self) -> Option<&Self::Item> {
        unimplemented!()
    }

    /// Advance the iterator, erroring if EOF occurs prematurely
    fn advance(&mut self) -> Result<Self::Item> {
        unimplemented!()
    }

    /// Peek the iterator, erroring if EOF occurs early
    fn peer(&mut self) -> Result<Self::Item> {
        unimplemented!()
    }

    /// Consumes the next item, verifing that it is the right value
    fn eat(&mut self, _expected: Self::Scrutinee) -> Option<Token> {
        unimplemented!()
    }

    /// Peeks the next item, verifing that it is the right value
    fn taste(&mut self, _expected: Self::Scrutinee) -> Result<bool> {
        unimplemented!()
    }
}
