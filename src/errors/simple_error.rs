use crate::errors::ParsingErrorKind;
use crate::traits::Parser;

#[derive(Debug, PartialEq)]
/// Information about what error was reached during the parsing process
pub struct ParsingError {
    pub kind: ParsingErrorKind,
    pub offset: usize,
}

impl PartialOrd for ParsingError {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(if self.offset > other.offset {
            std::cmp::Ordering::Greater
        } else if self.offset < other.offset {
            std::cmp::Ordering::Less
        } else {
            std::cmp::Ordering::Equal
        })
    }
}

impl ParsingError {
    pub fn new(kind: ParsingErrorKind, offset: usize) -> Self {
        ParsingError { kind, offset }
    }
}

/// Add a custom error message to some parser
pub struct ErrorParser<'a, P>
where
    P: Parser,
{
    parser: P,
    message: &'a str,
}

impl<'a, P> ErrorParser<'a, P>
where
    P: Parser,
{
    pub fn new(parser: P, message: &'a str) -> Self {
        ErrorParser { parser, message }
    }
}

impl<'a, P> Parser for ErrorParser<'a, P>
where
    P: Parser,
{
    type Output = P::Output;

    fn parse(&self, input: &crate::inputs::Input) -> crate::type_alias::ParserRes<Self::Output> {
        self.parser.parse(input).map_err(|err| {
            let kind = ParsingErrorKind::CustomError(self.message.to_string());
            ParsingError::new(kind, err.offset)
        })
    }
}
