use crate::{errors::ParsingError, traits::Parser};

pub struct RepeatParser<P>(P);

impl<P> RepeatParser<P> {
    pub fn new(p: P) -> Self {
        Self(p)
    }
}

impl<P> Parser for RepeatParser<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;
    fn parse(&self, input: &str) -> crate::type_alias::ParserRes<Self::Output> {
        let mut rest = input.to_string();
        let mut acc = vec![];
        loop {
            let Ok((p, r)) = self.0.parse(&rest) else {
                break;
            };

            rest = r;
            acc.push(p);
        }

        if acc.is_empty() {
            return Err(ParsingError::PatternNotFound(
                "Did not match parser any times".to_string(),
            ));
        }

        Ok((acc, String::new()))
    }
}

#[cfg(test)]
mod parse_many_t {

    use crate::parsers::and_p::KeepSecondOutputOnly;
    use crate::parsers::{ParseWhile, ParseWhileOrNothing};
    use crate::traits::Parser;

    #[test]
    fn parse_many_strings() {
        let sp = ParseWhile(|c| c.is_alphabetic());
        let wp = ParseWhileOrNothing(|c| c.is_whitespace());
        let swp = wp.and_then(sp).combine(KeepSecondOutputOnly);
        let many_string_p = super::RepeatParser(swp);

        let (acc, rest) = many_string_p.parse("hello there this is a text").unwrap();
        let exp = "hello there this is a text".split(' ').collect::<Vec<_>>();
        assert_eq!(acc, exp);
        assert!(rest.is_empty());

        let (acc, rest) = many_string_p.parse("hello").unwrap();
        assert_eq!(acc, vec!["hello".to_string()]);
        assert!(rest.is_empty());

        assert!(many_string_p.parse("").is_err())
    }
}
