use crate::{
    errors::{simple_error::ParsingError, ParsingErrorKind},
    inputs::Input,
    traits::Parser,
};

use super::{ParseMatch, ParseWhile};

pub struct StringParser;

// FIXME: Many things are broken here...
// + Error will report the wrong line and column
impl Parser for StringParser {
    type Output = String;
    fn parse(&self, input: &Input) -> crate::type_alias::ParserRes<Self::Output> {
        // First, we will make sure that the first character is "
        let (_, rest) = ParseMatch('"').parse(input)?;

        let mut acc = 1;
        let mut chars = rest.source.chars().peekable();
        loop {
            match chars.next() {
                Some('"') => {
                    acc += 1;
                    break;
                }
                Some('\\') => {
                    let _ = chars.next().unwrap();
                    acc += 1;
                }
                Some(_) => acc += 1,
                None => {
                    let kind = ParsingErrorKind::PatternNotFound(
                        "Did not find closing quote \"".to_string(),
                    );
                    return Err(ParsingError::new(kind, rest.offset));
                }
            };
        }

        Ok((
            // Do not include the '"' as part of the string
            input.source.chars().skip(1).take(acc - 2).collect(),
            input.clone().char_offset(acc),
        ))
    }
}

pub fn string_parser() -> impl Parser<Output = String> {
    StringParser
}

#[cfg(test)]
mod string_parser_test {
    use crate::parsers::string_p::string_parser;
    use crate::traits::Parser;

    #[test]
    fn string_parser_test() {
        let sp = string_parser();
        let (p, inp) = sp
            .parse(&"\"This is some string\" and this is the rest".into())
            .unwrap();
        assert_eq!(p, "This is some string".to_string());
        assert_eq!(inp.source, " and this is the rest".to_string());
        assert_eq!(inp.offset, 21);
    }
}
