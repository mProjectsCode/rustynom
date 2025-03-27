#![feature(test)]

use std::fs;

use rustynom::{
    atomic_parsers::{LiteralListMapParser, LiteralParserNoOutput},
    combinator_parsers::{AndParser5, SameOrParser2, SameOrParser6},
    parser::{ParserCombinator, ParserWrapper},
    transformation_parsers::RecParser,
    utility_parsers,
};

extern crate test;
use test::Bencher;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

fn define_parser() -> ParserWrapper<char, Value, false> {
    let number = utility_parsers::float().map(Value::Number);

    let string = utility_parsers::multi_test(|c| *c != '"', "string".to_string())
        .trim(LiteralParserNoOutput::new('"'));

    let string_value = string.clone().map(Value::String);

    let boolean = SameOrParser2::new(
        LiteralListMapParser::new("true".to_string(), Value::Bool(true)),
        LiteralListMapParser::new("false".to_string(), Value::Bool(false)),
    );

    let null = LiteralListMapParser::new("null".to_string(), Value::Null);

    let rec = RecParser::new(|rec_ref| {
        let array = rec_ref
            .clone()
            .separated_by(LiteralParserNoOutput::new(','))
            .surround(
                LiteralParserNoOutput::new('['),
                LiteralParserNoOutput::new(']'),
            )
            // .box_describe("array")
            .map(Value::Array);

        let object_entry = AndParser5::new(
            utility_parsers::optional_whitespace(),
            string.clone(),
            utility_parsers::optional_whitespace(),
            LiteralParserNoOutput::new(':'),
            rec_ref.clone(),
        )
        // .box_describe("object property")
        .map(|(_, key, _, _, value)| (key, value));

        let object = object_entry
            .separated_by(LiteralParserNoOutput::new(','))
            .surround(
                LiteralParserNoOutput::new('{'),
                LiteralParserNoOutput::new('}'),
            )
            .map(Value::Object);

        // array works, but object doesn't
        SameOrParser6::new(number, string_value, boolean, null, array, object)
            .trim(utility_parsers::optional_whitespace())
    });

    ParserWrapper::from_parser(rec.then_eof())
}

// #[test]
// fn test_json_file() {
//     let json_string = fs::read_to_string("tests/data/big_json.json").unwrap();
//     dbg!(json_string.len());
//     let parser = define_parser().then_eof();

//     let result = parser.parse_str(&json_string);
//     dbg!(&result);
//     assert!(result.is_success());

// }

#[bench]
fn bench_json(b: &mut Bencher) {
    let json_string = fs::read_to_string("tests/data/big_json.json").unwrap();
    let parser = define_parser();

    let chars = json_string.chars().collect::<Vec<_>>();

    b.iter(|| parser.parse_slice(&chars))
}

#[bench]
fn bench_serde_json(b: &mut Bencher) {
    let json_string = fs::read_to_string("tests/data/big_json.json").unwrap();

    b.iter(|| {
        let v: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
        v
    })
}

#[bench]
fn bench_json_min(b: &mut Bencher) {
    let json_string = fs::read_to_string("tests/data/big_json_min.json").unwrap();
    let parser = define_parser();

    let chars = json_string.chars().collect::<Vec<_>>();

    b.iter(|| parser.parse_slice(&chars))
}

#[bench]
fn bench_serde_json_min(b: &mut Bencher) {
    let json_string = fs::read_to_string("tests/data/big_json_min.json").unwrap();

    b.iter(|| {
        let v: Result<serde_json::Value, _> = serde_json::from_str(&json_string);
        v
    })
}
