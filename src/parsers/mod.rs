pub mod and_p;
pub mod map_p;
pub mod or_p;
pub mod repeat_p;
pub mod string_p;

use crate::{errors::ParsingError, traits::Parser, type_alias::ParserRes};

/// A parser that will parse an exact input string
///
/// # Example
///
/// ```rust
/// use mini_parc::parsers::ParseMatch;
/// use mini_parc::traits::Parser;
///
/// let parse_if = ParseMatch("if");
/// let answer = parse_if.parse("if and");
///
/// assert_eq!(
///     answer,
///     Ok(("if".to_string(), " and".to_string()))
/// );
/// ```
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

/// Parse a character if a predicate is met, otherwise, return an error.
///
/// # Example
///
/// Parse the first character if it is a numberical character
///
/// ```rust
/// use mini_parc::parsers::ParseIf;
/// use mini_parc::traits::Parser;
///
/// let parse_if = ParseIf(|c| c.is_numeric());
/// let answer = parse_if.parse("12hello");
/// assert_eq!(answer, Ok(('1', "2hello".to_string())));
/// ```
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

/// Keep parsing characters while some predicate is met. If none of the characters
/// meet the predicate, an empty string will be parsed. If you want an error in the
/// case that no characters meet the predicate, try using `ParseWhile`
///
/// # Example
///
/// ```rust
/// use mini_parc::parsers::ParseWhileOrNothing;
/// use mini_parc::traits::Parser;
///
/// let parse_numbers = ParseWhileOrNothing(|c| c.is_numeric());
/// let answer_valid = parse_numbers.parse("123a 1234");
/// assert_eq!(answer_valid, Ok(("123".to_string(), "a 1234".to_string())));
/// ```
///
/// In the following example, and error will be returned, since none of the characters
/// met the predicate `is_numeric`
///
/// ```rust
/// use mini_parc::parsers::ParseWhileOrNothing;
/// use mini_parc::traits::Parser;
/// use mini_parc::errors::ParsingError;
///
/// let parse_numbers = ParseWhileOrNothing(|c| c.is_numeric());
/// let answer_bad = parse_numbers.parse("x123a 1234");
/// assert_eq!(
///     answer_bad,
///     Ok((String::new(), "x123a 1234".to_string()))
/// );
/// ```
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
/// Keep parsing characters while some predicate is met. If none of the characters
/// meet the predicate, and error will be returned. If this is not desired, try
/// using `ParseWhileOrNothing`
///
/// # Example
///
/// ```rust
/// use mini_parc::parsers::ParseWhile;
/// use mini_parc::traits::Parser;
///
/// let parse_numbers = ParseWhile(|c| c.is_numeric());
/// let answer_valid = parse_numbers.parse("123a 1234");
/// assert_eq!(answer_valid, Ok(("123".to_string(), "a 1234".to_string())));
/// ```
///
/// In the following example, and error will be returned, since none of the characters
/// met the predicate `is_numeric`
///
/// ```rust
/// use mini_parc::parsers::ParseWhile;
/// use mini_parc::traits::Parser;
/// use mini_parc::errors::ParsingError;
///
/// let parse_numbers = ParseWhile(|c| c.is_numeric());
/// let answer_bad = parse_numbers.parse("x123a 1234");
/// assert_eq!(
///     answer_bad,
///     Err(ParsingError::PatternNotFound(
///         "no characters matched predicate".to_string()
///     ))
/// );
/// ```
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

#[cfg(test)]
mod test_base_parsers {
    use super::{ParseIf, ParseMatch, ParseWhile};
    use crate::traits::Parser;

    #[test]
    fn match_parser() {
        let parse_if = ParseMatch("if");
        let answer = parse_if.parse("if and");
        assert_eq!(answer, Ok(("if".to_string(), " and".to_string())));
    }

    #[test]
    fn if_parser() {
        let parse_if = ParseIf(|c| c.is_numeric());
        let answer = parse_if.parse("12hello");
        assert_eq!(answer, Ok(('1', "2hello".to_string())));
    }

    #[test]
    fn parse_while() {
        let parse_numbers = ParseWhile(|c| c.is_numeric());

        let answer_valid = parse_numbers.parse("123a 1234");
        assert_eq!(answer_valid, Ok(("123".to_string(), "a 1234".to_string())));

        let answer_bad = parse_numbers.parse("x123a 1234");
        assert_eq!(
            answer_bad,
            Err(crate::errors::ParsingError::PatternNotFound(
                "no characters matched predicate".to_string()
            ))
        );
    }
}
