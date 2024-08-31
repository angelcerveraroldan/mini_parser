use crate::traits::Parser;

struct StringParser;

impl Parser for StringParser {
    type Output = String;
    fn parse(&self, input: &str) -> crate::type_alias::ParserRes<Self::Output> {
        // First, we will make sure that the first character is "
        let mut chars = input.chars().peekable();
        let mut acc = String::new();

        if chars.next() != Some('"') {
            return Err(crate::errors::ParsingError::PatternNotFound(
                "String must start with \"".to_string(),
            ));
        }

        loop {
            match chars.next() {
                Some('"') => break,
                Some('\\') => {
                    let char_after = chars.next().unwrap();
                    acc.push('\\');
                    acc.push(char_after);
                }
                Some(c) => acc.push(c),
                None => {
                    return Err(crate::errors::ParsingError::PatternNotFound(
                        "Did not find closing quote \"".to_string(),
                    ))
                }
            };
        }

        Ok((acc, chars.collect()))
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
        let x = sp.parse("\"This is some string\" and this is the rest");
        assert_eq!(
            x,
            Ok((
                "This is some string".to_string(),
                " and this is the rest".to_string()
            ))
        );
    }
}
