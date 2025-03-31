#![allow(dead_code)]

use errors::pretty_error::PrettyError;
use traits::Parser;

pub mod errors;
pub mod inputs;
pub mod parsers;
pub mod traits;
pub mod type_alias;

/// Main parser will hold the source code
pub struct MainParser<P>
where
    P: Parser,
{
    pub src: String,
    pub parser: P,
}

impl<P> MainParser<P>
where
    P: Parser,
{
    pub fn parse(&self, input: &inputs::Input) -> type_alias::ParserRes<P::Output, PrettyError> {
        self.parser
            .parse(input)
            .map_err(|x| PrettyError::from((x, self)))
    }
}

#[cfg(test)]
mod test {}
