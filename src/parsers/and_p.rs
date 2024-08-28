use std::fmt::Debug;

use crate::{traits::Parser, type_alias::ParserRes};

/// A parser that consists of first running some parser A, then some parser B.
///
/// If either of the parsers give an error, that error will be returned. If they
/// both suceed, their outputs will be combined using the `AndCombinator`, and
/// that will be reuturned, with the rest of the unparsed string.
pub struct AndThenParser<A, B, C>
where
    A: Parser,
    B: Parser,
    C: AndCombinator<A::Output, B::Output>,
{
    pub first_parse: A,
    pub second_parse: B,
    combinator: C,
}

impl<A, B, C> AndThenParser<A, B, C>
where
    A: Parser,
    B: Parser,
    C: AndCombinator<A::Output, B::Output>,
{
    pub fn combine<NC>(self, combinator: NC) -> AndThenParser<A, B, NC>
    where
        NC: AndCombinator<A::Output, B::Output>,
    {
        AndThenParser {
            first_parse: self.first_parse,
            second_parse: self.second_parse,
            combinator,
        }
    }
}

/// This serves as a way to combine two parser outputs
pub trait AndCombinator<A, B> {
    type Combined;
    fn combine(&self, pair: (A, B)) -> Self::Combined;
}

/// Combine A and B into a tuple (A, B)
pub struct IdentityAndCombinator;

impl<A, B> AndCombinator<A, B> for IdentityAndCombinator {
    type Combined = (A, B);
    fn combine(&self, pair: (A, B)) -> Self::Combined {
        pair
    }
}

/// Keep the output from the first (A) parser, and discard the output
/// of the second parser (B)
pub struct KeepFirstOutputOnly;

impl<A, B> AndCombinator<A, B> for KeepFirstOutputOnly {
    type Combined = A;
    fn combine(&self, (a, _): (A, B)) -> Self::Combined {
        a
    }
}

/// Keep the output from the second (B) parser, and discard the output
/// of the first parser (A)
pub struct KeepSecondOutputOnly;

impl<A, B> AndCombinator<A, B> for KeepSecondOutputOnly {
    type Combined = B;
    fn combine(&self, (_, b): (A, B)) -> Self::Combined {
        b
    }
}

/// Discard the output of both parsers, and return the unit type
pub struct KeepNone;

impl<A, B> AndCombinator<A, B> for KeepNone {
    type Combined = ();
    fn combine(&self, _: (A, B)) -> Self::Combined {
        ()
    }
}

impl<A, B, C> From<(A, B, C)> for AndThenParser<A, B, C>
where
    A: Parser,
    B: Parser,
    C: AndCombinator<A::Output, B::Output>,
{
    fn from((first_parse, second_parse, combinator): (A, B, C)) -> Self {
        Self {
            first_parse,
            second_parse,
            combinator,
        }
    }
}

impl<A, B> From<(A, B)> for AndThenParser<A, B, IdentityAndCombinator>
where
    A: Parser,
    B: Parser,
{
    fn from((first_parse, second_parse): (A, B)) -> Self {
        Self {
            first_parse,
            second_parse,
            combinator: IdentityAndCombinator,
        }
    }
}

impl<A, B, C> Parser for AndThenParser<A, B, C>
where
    A: Parser,
    B: Parser,
    C: AndCombinator<A::Output, B::Output>,
    C::Combined: Debug,
{
    type Output = C::Combined;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let (a, rest) = A::parse(&self.first_parse, input)?;
        let (b, rest) = B::parse(&self.second_parse, &rest)?;
        Ok((C::combine(&self.combinator, (a, b)), rest))
    }
}
