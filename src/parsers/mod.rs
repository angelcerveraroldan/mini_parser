pub mod and_p;
pub mod map_p;
pub mod or_p;
pub mod repeat_p;
pub mod string_p;

use crate::{
    errors::{simple_error::ParsingError, ParsingErrorKind},
    inputs::Input,
    traits::Parser,
    type_alias::ParserRes,
};

/// A parser that will parse an exact input string
///
/// # Example
///
/// ```rust
/// use parlib::parsers::ParseMatch;
/// use parlib::traits::Parser;
///
/// let parse_if = ParseMatch("if");
/// let (parsed, rest) = parse_if.parse(&"if and".into()).unwrap();
///
/// assert_eq!(parsed, "if".to_string());
/// ```
pub struct ParseMatch<A>(pub A)
where
    A: Into<String>;

impl<A> Parser for ParseMatch<A>
where
    A: Into<String> + Clone,
{
    type Output = String;
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        let match_str: String = self.0.clone().into();
        if !input.source.starts_with(&match_str) {
            let kind = ParsingErrorKind::PatternNotFound(format!(
                "{} did not match pattern: {}",
                input.source, match_str
            ));
            return Err(ParsingError::new(kind, input.offset));
        }

        let rest = input.clone().char_offset(match_str.len());
        Ok((match_str, rest))
    }
}

/// Parse a character if a predicate is met, otherwise, return an error.
///
/// # Example
///
/// Parse the first character if it is a numberical character
///
/// ```rust
/// use parlib::parsers::ParseIf;
/// use parlib::traits::Parser;
///
/// let parse_if = ParseIf(|c| c.is_numeric());
/// let (parsed, _) = parse_if.parse(&"12hello".into()).unwrap();
/// assert_eq!(parsed, '1');
/// ```
pub struct ParseIf(pub fn(char) -> bool);

impl Parser for ParseIf {
    type Output = char;
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        let maybe_first_char = input.source.chars().next();
        if let Some(true) = maybe_first_char.map(self.0) {
            return Ok((maybe_first_char.unwrap(), input.clone().char_offset(1)));
        }
        let kind = ParsingErrorKind::PatternNotFound("if predicate not met".to_string());
        Err(ParsingError::new(kind, input.offset))
    }
}

/// Keep parsing characters while some predicate is met. If none of the characters
/// meet the predicate, an empty string will be parsed. If you want an error in the
/// case that no characters meet the predicate, try using `ParseWhile`
///
/// # Example
///
/// ```rust
/// use parlib::parsers::ParseWhileOrNothing;
/// use parlib::traits::Parser;
///
/// let parse_numbers = ParseWhileOrNothing(|c| c.is_numeric());
/// let (p, i) = parse_numbers.parse(&"123a 1234".into()).unwrap();
/// assert_eq!(p, "123".to_string());
/// assert_eq!(i.source, "a 1234".to_string());
/// ```
///
/// `
/// In the following example, and error will be returned, since none of the characters
/// met the predicate `is_numeric`
///
/// ```rust
/// use parlib::parsers::ParseWhile;
/// use parlib::traits::Parser;
/// use parlib::errors::simple_error::ParsingError;
///
/// let parse_numbers = ParseWhile(|c| c.is_numeric());
/// let answer_bad = parse_numbers.parse(&"x123a 1234".into());
/// assert!(answer_bad.is_err());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ParseWhileOrNothing<F>(pub F)
where
    F: Fn(char) -> bool;

impl<F> Parser for ParseWhileOrNothing<F>
where
    F: Fn(char) -> bool,
{
    type Output = String;
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        let taken = input
            .source
            .chars()
            .take_while(|&x| self.0(x))
            .collect::<String>();
        let len = taken.len();
        Ok((taken, input.clone().char_offset(len)))
    }
}
/// Keep parsing characters while some predicate is met. If none of the characters
/// meet the predicate, and error will be returned. If this is not desired, try
/// using `ParseWhileOrNothing`
///
/// # Example
///
/// ```rust
/// use parlib::parsers::ParseWhile;
/// use parlib::traits::Parser;
///
/// let parse_numbers = ParseWhile(|c| c.is_numeric());
/// let (p, i) = parse_numbers.parse(&"123a 1234".into()).unwrap();
/// assert_eq!(p, "123".to_string());
/// assert_eq!(i.source, "a 1234".to_string());
/// ```
///
/// In the following example, and error will be returned, since none of the characters
/// met the predicate `is_numeric`
///
/// ```rust
/// use parlib::parsers::ParseWhile;
/// use parlib::traits::Parser;
/// use parlib::errors::simple_error::ParsingError;
///
/// let parse_numbers = ParseWhile(|c| c.is_numeric());
/// let answer_bad = parse_numbers.parse(&"x123a 1234".into());
/// assert!(answer_bad.is_err());
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ParseWhile<F>(pub F)
where
    F: Fn(char) -> bool;

impl<F> Parser for ParseWhile<F>
where
    F: Fn(char) -> bool,
{
    type Output = String;
    fn parse(&self, input: &Input) -> ParserRes<Self::Output> {
        let taken: String = input.source.chars().take_while(|&x| self.0(x)).collect();
        if taken.is_empty() {
            let kind =
                ParsingErrorKind::PatternNotFound("no characters matched predicate".to_string());
            return Err(ParsingError::new(kind, input.offset));
        }
        let len = taken.len();
        Ok((taken, input.clone().char_offset(len)))
    }
}

#[cfg(test)]
mod test_base_parsers {
    use super::{ParseIf, ParseMatch, ParseWhile};
    use crate::traits::Parser;

    #[test]
    fn match_parser() {
        let parse_if = ParseMatch("if");
        let (p, i) = parse_if.parse(&"if and".into()).unwrap();
        assert_eq!(p, "if".to_string());
        assert_eq!(i.source, " and".to_string());
    }

    #[test]
    fn if_parser() {
        let parse_if = ParseIf(|c| c.is_numeric());
        let (p, i) = parse_if.parse(&"12hello".into()).unwrap();
        assert_eq!(p, '1');
        assert_eq!(i.source, "2hello".to_string());
    }

    #[test]
    fn parse_while() {
        let parse_numbers = ParseWhile(|c| c.is_numeric());

        let (p, i) = parse_numbers.parse(&"123a 1234".into()).unwrap();
        assert_eq!(p, "123".to_string());
        assert_eq!(i.source, "a 1234".to_string());

        let answer_bad = parse_numbers.parse(&"x123a 1234".into());
        assert!(answer_bad.is_err());
    }
}
