use crate::{
    errors::{simple_error::ParsingError, ParsingErrorKind},
    inputs::Input,
    traits::Parser,
};

/// Run the same parser repeatedly
///
/// Optionally, you can set a range. The minimum number of times the parser must
/// be run, and the limit.
pub struct RepeatParser<P>
where
    P: Parser,
{
    parser: P,
    lower_bound: usize,
    upper_bound: Option<usize>,
}

impl<P> RepeatParser<P>
where
    P: Parser,
{
    /// By default, the parser must run *at least* 1 time, with no maximum
    pub fn new(p: P) -> Self {
        Self {
            parser: p,
            lower_bound: 1,
            upper_bound: None,
        }
    }

    pub fn minm(mut self, l: usize) -> Self {
        self.lower_bound = l;
        self
    }

    pub fn maxm(mut self, l: usize) -> Self {
        self.upper_bound = Some(l);
        self
    }
}

impl<P> Parser for RepeatParser<P>
where
    P: Parser,
{
    type Output = Vec<P::Output>;
    fn parse(&self, input: &Input) -> crate::type_alias::ParserRes<Self::Output> {
        let mut rest = input.clone();
        let mut acc = vec![];
        loop {
            if let Some(limit) = self.upper_bound {
                if acc.len() >= limit {
                    break;
                }
            }

            let Ok((p, r)) = self.parser.parse(&rest) else {
                break;
            };

            rest = r;
            acc.push(p);
        }

        if acc.len() < self.lower_bound {
            let err_kind = ParsingErrorKind::PatternNotFound(
                "Parser did not run the minum number of times".to_string(),
            );
            return Err(ParsingError::new(err_kind, rest.offset));
        }

        Ok((acc, rest))
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
        let many_string_p = super::RepeatParser::new(swp);

        let (acc, rest) = many_string_p
            .parse(&"hello there this is a text".into())
            .unwrap();
        let exp = "hello there this is a text".split(' ').collect::<Vec<_>>();
        assert_eq!(acc, exp);
        assert!(rest.source.is_empty());

        let (acc, rest) = many_string_p.parse(&"hello".into()).unwrap();
        assert_eq!(acc, vec!["hello".to_string()]);
        assert!(rest.source.is_empty());

        assert!(many_string_p.parse(&"".into()).is_err())
    }

    #[test]
    fn parse_many_with_limit() {
        let sp = ParseWhile(|c| c.is_alphabetic());
        let wp = ParseWhileOrNothing(|c| c.is_whitespace());
        let swp = wp.and_then(sp).combine(KeepSecondOutputOnly);
        let many_string_p = super::RepeatParser::new(swp).maxm(2);

        let (acc, rest) = many_string_p
            .parse(&"hello there this is a text".into())
            .unwrap();
        let exp = "hello there".split(' ').collect::<Vec<_>>();
        assert_eq!(acc, exp);
        assert_eq!(rest.source, " this is a text".to_string());
    }
}
