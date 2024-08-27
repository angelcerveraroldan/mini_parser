use crate::errors::ParsingError;

pub type ParserRes<A, E = ParsingError> = std::result::Result<(A, String), E>;
