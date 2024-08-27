use std::fmt::Debug;

use crate::{errors::ParsingError, traits::Parser, type_alias::ParserRes};

pub struct OrThenParser<A, B>
where
    A: Parser,
    B: Parser,
{
    pub first_parse: A,
    pub second_parse: B,
}

impl<A, B> From<(A, B)> for OrThenParser<A, B>
where
    A: Parser,
    B: Parser,
{
    fn from((ap, bp): (A, B)) -> Self {
        Self {
            first_parse: ap,
            second_parse: bp,
        }
    }
}

impl<A, B, CommonOut> Parser for OrThenParser<A, B>
where
    A: Parser<Output = CommonOut>,
    B: Parser<Output = CommonOut>,
    CommonOut: Debug,
{
    type Output = CommonOut;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let aparse = self.first_parse.parse(input);
        if aparse.is_ok() {
            return aparse;
        }

        let bparse = self.second_parse.parse(input);
        if bparse.is_ok() {
            return bparse;
        };

        Err(ParsingError::PatternNotFound(
            "Or Parser didnt match either of the branches".to_string(),
        ))
    }
}
