use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

use parlib::{
    inputs::Input,
    parsers::{
        and_p::{KeepFirstOutputOnly, KeepSecondOutputOnly},
        string_p::string_parser,
        ParseMatch, ParseWhile, ParseWhileOrNothing,
    },
    traits::Parser,
    MainParser,
};

#[derive(Debug, PartialEq)]
pub enum Primitives {
    True,
    False,
    String(String),
    Number(f64),
    Array(Vec<Primitives>),
    Object(HashMap<String, Primitives>),
}

fn parse_bool() -> impl Parser<Output = Primitives> {
    ParseMatch("true")
        .with_mapping(&|_| Primitives::True)
        .otherwise(ParseMatch("true").with_mapping(&|_| Primitives::False))
}

fn parse_string() -> impl Parser<Output = Primitives> {
    string_parser().with_mapping(&|s| Primitives::String(s))
}

fn parse_integer() -> impl Parser<Output = String> {
    ParseWhile(|c| c.is_numeric())
}

fn parse_float() -> impl Parser<Output = String> {
    let p_whole = ParseWhile(|c| c.is_numeric());
    let p_decimal = ParseWhileOrNothing(|c| c.is_numeric());

    p_whole
        .and_then(ParseMatch('.'))
        .combine(KeepFirstOutputOnly)
        .and_then(p_decimal)
        .with_mapping(&|(whole, decimal)| format!("{whole}.{decimal}"))
}

fn parse_number() -> impl Parser<Output = Primitives> {
    parse_float().otherwise(parse_integer()).with_mapping(&|s| {
        let numb = s.parse::<f64>().unwrap();
        Primitives::Number(numb)
    })
}

pub fn parse_prim() -> impl Parser<Output = Primitives> {
    parse_bool()
        .otherwise(parse_number())
        .otherwise(parse_string())
}

// (add 2 1)
#[derive(Debug)]
pub enum Expression {
    Prim(Primitives),
    Compound {
        ident: String,
        params: Vec<Expression>,
    },
}

pub fn expression_parse() -> impl Parser<Output = Expression> {
    ParseWhileOrNothing(|x| [' ', '\t'].contains(&x))
        .and_then(
            parse_prim()
                .with_mapping(&|x| Expression::Prim(x))
                .otherwise(CompoundParser),
        )
        .combine(KeepSecondOutputOnly)
}

struct CompoundParser;
impl Parser for CompoundParser {
    type Output = Expression;
    //  ( [a-z]+ <expression>* )
    fn parse(&self, input: &Input) -> parlib::type_alias::ParserRes<Self::Output> {
        let ob = ParseMatch("(");
        let cb = ParseMatch(")");
        let ws = ParseWhileOrNothing(|x| [' ', '\t'].contains(&x));
        let ident = ParseWhile(|x| x.is_alphabetic());

        // whitespace + open bracket
        let (_, rest) = ws.and_then(ob).parse(input)?;
        // Whitespace + ident
        let (i, mut rest) = ws
            .and_then(ident)
            .combine(KeepSecondOutputOnly)
            .parse(&rest)?;

        let mut v: Vec<Expression> = vec![];
        loop {
            if let Ok((exp, r)) = expression_parse().parse(&rest) {
                v.push(exp);
                rest = r;
            } else {
                break;
            }
        }
        // find the closing bracket
        let (_, f_rest) = ws
            .and_then(cb)
            .with_error("Expected closing bracket or expression")
            .parse(&rest)?;
        Ok((
            Expression::Compound {
                ident: i,
                params: v,
            },
            f_rest,
        ))
    }
}

fn main() -> miette::Result<()> {
    loop {
        println!("Please enter a single lisp line to parse it:");
        let _ = stdout().flush();
        let mut buffer: String = String::new();
        stdin()
            .read_line(&mut buffer)
            .expect("Error reading user input");
        let par = MainParser {
            src: buffer.clone(),
            parser: expression_parse(),
        };
        if buffer == String::from("exit\n") {
            return Ok(());
        }
        println!("{:?}", par.parse(&buffer.into())?);
    }
}
