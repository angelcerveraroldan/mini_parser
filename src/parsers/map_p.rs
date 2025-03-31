use std::fmt::Debug;

use crate::{
    errors::{simple_error::ParsingError, ParsingErrorKind},
    inputs::Input,
    traits::Parser,
    type_alias::ParserRes,
};

/// Given a parser with output of type K, and a mapping K -> Z,
/// make a new parser with output of type Z
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
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        self.parser.parse_and_then_map(input, &self.mapping)
    }
}

impl<'a, P, T> Parser for TryMapParser<'a, P, T>
where
    P: Parser,
    T: Debug,
{
    type Output = T;
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        let (p, rest) = self.parser.parse(input)?;
        match (self.try_map)(p) {
            None => {
                let kind = ParsingErrorKind::MappingError(
                    "Parsing worked, but mapping failed".to_string(),
                );
                Err(ParsingError::new(kind, rest.offset))
            }
            Some(mapped_val) => Ok((mapped_val, rest)),
        }
    }
}
