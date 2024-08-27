use std::fmt::Debug;

use crate::{errors::ParsingError, traits::Parser, type_alias::ParserRes};

pub struct MapParser<'a, P: Parser, T> {
    pub parser: P,
    pub mapping: &'a dyn Fn(P::Output) -> T,
}

pub struct TryMapParser<'a, P: Parser, T> {
    pub parser: P,
    pub try_map: &'a dyn Fn(P::Output) -> Option<T>,
}

impl<'a, P, T> Parser for MapParser<'a, P, T>
where
    P: Parser,
    T: Debug,
{
    type Output = T;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        self.parser.parse_and_then_map(input, &self.mapping)
    }
}

impl<'a, P, T> Parser for TryMapParser<'a, P, T>
where
    P: Parser,
    T: Debug,
{
    type Output = T;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let (p, rest) = self.parser.parse(input)?;
        match (self.try_map)(p) {
            None => Err(ParsingError::MappingError("mapping failed".to_string())),
            Some(mapped_val) => Ok((mapped_val, rest)),
        }
    }
}
