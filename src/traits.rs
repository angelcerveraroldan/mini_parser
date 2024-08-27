use std::fmt::Debug;

use crate::{
    parsers::{
        and_p::{AndCombinator, AndThenParser, IdentityAndCombinator},
        map_p::{MapParser, TryMapParser},
        or_p::OrThenParser,
    },
    type_alias::ParserRes,
};

/// Parser trait
///
/// A parser will parse some input string into A and also return the rest of the string
/// ( or return an error ). This allows you to recursively keep parsing an input string until
/// the entire input has been parsed.
pub trait Parser
where
    Self: Sized,
{
    type Output: Debug;

    /// Parse the input string, if the parser is sucessful, it will return Ok((parsed, rest)),
    /// where parsed is the data that was parsed from the string, and the rest is what was left
    /// over.
    ///
    /// If parsing did not suceed, then an error will be returned
    fn parse(&self, input: &str) -> ParserRes<Self::Output>;

    /// Parse the output (see parse function), and if sucessful, map the parsed output
    fn parse_and_then_map<F, MappedOutput>(&self, input: &str, f: F) -> ParserRes<MappedOutput>
    where
        F: FnOnce(Self::Output) -> MappedOutput,
    {
        self.parse(input).map(|(a, rest)| (f(a), rest))
    }

    /// Make a new parser that consists of this parser, followed by another parser.
    ///
    /// The output will be sucessful iff both parsers are sucessful
    fn and_then<P>(self, other: P) -> AndThenParser<Self, P, IdentityAndCombinator>
    where
        P: Parser,
    {
        AndThenParser::from((self, other))
    }

    fn and_then_combine_with<P, C>(self, other: P, combinator: C) -> AndThenParser<Self, P, C>
    where
        P: Parser,
        C: AndCombinator<Self::Output, P::Output>,
    {
        AndThenParser::from((self, other, combinator))
    }

    /// Make a new parser that consists of this parser OR another parser.
    ///
    /// This new parser will run both parsers in order, and return the first sucessful one
    fn otherwise<P>(self, other: P) -> OrThenParser<Self, P>
    where
        P: Parser,
    {
        OrThenParser::from((self, other))
    }

    fn with_mapping<'a, T>(self, mapping: &'a dyn Fn(Self::Output) -> T) -> MapParser<Self, T> {
        MapParser {
            parser: self,
            mapping,
        }
    }

    fn with_try_mapping<'a, T>(
        self,
        try_map: &'a dyn Fn(Self::Output) -> Option<T>,
    ) -> TryMapParser<Self, T> {
        TryMapParser {
            parser: self,
            try_map,
        }
    }
}
