pub mod and_p;
pub mod map_p;
pub mod or_p;

use crate::{errors::ParsingError, traits::Parser, type_alias::ParserRes};

pub struct ParseMatch<A>(pub A)
where
    A: Into<String>;

impl<A> Parser for ParseMatch<A>
where
    A: Into<String> + Clone,
{
    type Output = String;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let match_str: String = self.0.clone().into();
        if !input.starts_with(&match_str) {
            return Err(ParsingError::PatternNotFound(format!(
                "{} did not match pattern: {}",
                input, match_str
            )));
        }
        let rest = input.chars().skip(match_str.len()).collect();
        Ok((match_str, rest))
    }
}

pub struct ParseIf(pub fn(char) -> bool);

impl Parser for ParseIf {
    type Output = char;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let maybe_first_char = input.chars().next();
        if let Some(true) = maybe_first_char.map(self.0) {
            return Ok((maybe_first_char.unwrap(), input.chars().skip(1).collect()));
        }
        Err(ParsingError::PatternNotFound(
            "if predicate not met".to_string(),
        ))
    }
}

#[derive(Debug, Clone)]
pub struct ParseWhileOrNothing(pub fn(char) -> bool);

impl Parser for ParseWhileOrNothing {
    type Output = String;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let taken = input.chars().take_while(|&x| self.0(x)).collect::<String>();
        let rest = input.chars().skip_while(|&x| self.0(x)).collect::<String>();
        Ok((taken, rest))
    }
}

#[derive(Debug, Clone)]
pub struct ParseWhile(pub fn(char) -> bool);

impl Parser for ParseWhile {
    type Output = String;
    fn parse(&self, input: &str) -> ParserRes<Self::Output> {
        let taken = input.chars().take_while(|&x| self.0(x)).collect::<String>();
        if taken.is_empty() {
            return Err(ParsingError::PatternNotFound(
                "no characters matched predicate".to_string(),
            ));
        }
        let rest = input.chars().skip_while(|&x| self.0(x)).collect::<String>();
        Ok((taken, rest))
    }
}
