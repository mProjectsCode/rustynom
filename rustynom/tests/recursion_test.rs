use rustynom::{
    atomic_parsers::{EofParser, LiteralListParser, LiteralParserNoOutput},
    combinator_parsers::{AndParser2, SameOrParser2},
    parse_str,
    parser::{ParserCombinator, ParserWrapper},
    transformation_parsers::RecParser,
};

#[test]
fn simple_recursion() {
    let a = LiteralListParser::new("a".to_owned());
    let mapped_eof = EofParser::new().map(|_| vec!["eof".to_owned()]);

    let rec = RecParser::new(|rec_ref| {
        let and = AndParser2::new(a, rec_ref).map(|(a, b)| {
            let mut result = vec![a];
            result.extend(b);
            result
        });

        SameOrParser2::new(mapped_eof, and)
    });

    let p = ParserWrapper::<char, Vec<String>>::from_parser(rec);

    let result = parse_str!(p, "");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["eof"]);

    let result = parse_str!(p, "a");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "eof"]);

    let result = parse_str!(p, "aa");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "a", "eof"]);
}

#[test]
fn simple_recursion_2() {
    let a = LiteralListParser::new("a".to_owned());

    let rec = RecParser::new(|rec_ref| {
        SameOrParser2::new(
            a,
            rec_ref.surround(
                LiteralParserNoOutput::new('['),
                LiteralParserNoOutput::new(']'),
            ),
        )
    });

    let p = ParserWrapper::<char, String>::from_parser(rec);

    let result = parse_str!(p, "a");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), "a");

    let result = parse_str!(p, "[a]");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), "a");

    let result = parse_str!(p, "[[a]]");
    dbg!(&result);
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), "a");
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Value {
    String(String),
    Array(Vec<Value>),
}

#[test]
fn simple_recursion_3() {
    let a = LiteralListParser::new("a".to_owned()).map(Value::String);

    let rec = RecParser::new(|rec_ref| {
        let tmp = rec_ref
            .separated_by(LiteralParserNoOutput::new(','))
            .surround(
                LiteralParserNoOutput::new('['),
                LiteralParserNoOutput::new(']'),
            )
            .map(Value::Array);

        SameOrParser2::new(a, tmp)
    });

    let p = ParserWrapper::<char, Value>::from_parser(rec);

    let result = parse_str!(p, "a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), Value::String("a".to_owned()));

    let result = parse_str!(p, "[a]");
    assert!(result.is_success());
    assert_eq!(
        result.unwrap_success(),
        Value::Array(vec![Value::String("a".to_owned())])
    );

    let result = parse_str!(p, "[a,a]");
    assert!(result.is_success());
    assert_eq!(
        result.unwrap_success(),
        Value::Array(vec![
            Value::String("a".to_owned()),
            Value::String("a".to_owned())
        ])
    );

    let result = parse_str!(p, "[[a]]");
    assert!(result.is_success());
    assert_eq!(
        result.unwrap_success(),
        Value::Array(vec![Value::Array(vec![Value::String("a".to_owned())])])
    );

    let result = parse_str!(p, "[b]");
    assert!(result.is_failure());
}
