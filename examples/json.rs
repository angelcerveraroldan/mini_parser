use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use mini_parc::{
    parsers::{
        and_p::KeepFirstOutputOnly, string_p::string_parser, ParseMatch, ParseWhile,
        ParseWhileOrNothing,
    },
    traits::Parser,
};

fn parse_integer() -> impl Parser<Output = String> {
    ParseWhile(|c| c.is_numeric())
}

fn parse_float() -> impl Parser<Output = String> {
    let p_whole = ParseWhile(|c| c.is_numeric());
    let p_decimal = ParseWhileOrNothing(|c| c.is_numeric());

    // Parse the whole part
    p_whole
        .and_then(ParseMatch('.'))
        // Delete the separator (we dont need it, we just need to know it occurred)
        .combine(KeepFirstOutputOnly)
        // Parse the decimal part
        .and_then(p_decimal)
        // Now that we know the whole and the decimal part, we can join them
        .with_mapping(&|(whole, decimal)| format!("{whole}.{decimal}"))
}

#[derive(Debug, PartialEq)]
pub enum Primitives {
    True,
    False,
    String(String),
    Number(f64),
    Array(Vec<Primitives>),
    Object(HashMap<String, Primitives>),
}

fn parse_number() -> impl Parser<Output = Primitives> {
    parse_float().otherwise(parse_integer()).with_mapping(&|s| {
        let numb = s.parse::<f64>().unwrap();
        Primitives::Number(numb)
    })
}

fn parse_string() -> impl Parser<Output = Primitives> {
    mini_parc::parsers::string_p::string_parser().with_mapping(&|s| Primitives::String(s))
}

pub fn primitive_parser() -> impl Parser<Output = Primitives> {
    ParseMatch("true")
        .with_mapping(&|_| Primitives::True)
        .otherwise(ParseMatch("false").with_mapping(&|_| Primitives::False))
        .otherwise(parse_number())
        .otherwise(parse_string())
        .otherwise(ArrayParser)
        .otherwise(ObjectParser)
}

struct ArrayParser;

impl Parser for ArrayParser {
    type Output = Primitives;
    fn parse(&self, input: &str) -> mini_parc::type_alias::ParserRes<Self::Output> {
        let whitespace_p = ParseWhileOrNothing(|c| c.is_whitespace());
        let (_, mut inp) = ParseMatch('[').parse(input)?;
        let mut acc = vec![];
        loop {
            let (_, rest) = whitespace_p.parse(&inp).unwrap();
            let Ok((prim, rest)) = primitive_parser().parse(&rest) else {
                break;
            };
            inp = rest;
            acc.push(prim);
            let (_, rest) = whitespace_p.parse(&inp).unwrap();
            let Ok((_, rest)) = ParseMatch(',').parse(&rest) else {
                break;
            };
            inp = rest;
        }
        // Parse closing bracket
        let (_, rest) = ParseMatch(']').parse(&inp)?;
        Ok((Primitives::Array(acc), rest))
    }
}

pub struct ObjectParser;

impl Parser for ObjectParser {
    type Output = Primitives;
    fn parse(&self, input: &str) -> mini_parc::type_alias::ParserRes<Self::Output> {
        let whitespace_p = ParseWhileOrNothing(|c| c.is_whitespace());
        let (_, mut inp) = ParseMatch('{').parse(input)?;
        let mut map: HashMap<String, Primitives> = HashMap::new();

        loop {
            let (_, rest) = whitespace_p.parse(&inp).unwrap();
            let Ok((string, rest)) = string_parser().parse(&rest) else {
                break;
            };

            let (_, rest) = whitespace_p.parse(&rest).unwrap();
            let (_, rest) = ParseMatch(':').parse(&rest)?;
            let (_, rest) = whitespace_p.parse(&rest).unwrap();

            let (prim, rest) = primitive_parser().parse(&rest)?;
            let (_, rest) = whitespace_p.parse(&rest).unwrap();
            map.insert(string, prim);
            inp = rest;

            let Ok((_, rest)) = ParseMatch(',').parse(&inp) else {
                break;
            };
            inp = rest;
        }

        let (_, rest) = ParseMatch('}').parse(&inp)?;
        Ok((Primitives::Object(map), rest))
    }
}

fn main() {
    println!("Please enter a single json line to parse it:");
    let _ = stdout().flush();
    let mut buffer: String = String::new();
    stdin()
        .read_line(&mut buffer)
        .expect("Error reading user input");
    let par = primitive_parser();
    println!("{:?}", par.parse(&buffer));
}
