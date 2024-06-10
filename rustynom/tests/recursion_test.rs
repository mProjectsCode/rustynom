use rustynom::{
    atomic_parsers::{CharParser, CustomParser, EofParser, StringParser},
    combinator_parsers::{AndParser2, SameOrParser2},
    parser::{ParserCombinator, RawParser},
    transformation_parsers::RecParser,
};

#[test]
fn simple_recursion() {
    let a = StringParser::new("a".to_owned());
    let mapped_eof = EofParser::new().map(|_| vec!["eof".to_owned()]);

    let rec = RecParser::new(|rec_ref| {
        let and = AndParser2::new(a, rec_ref).map(|(a, b)| {
            let mut result = vec![a];
            result.extend(b);
            result
        });

        SameOrParser2::new(mapped_eof, and)
    });

    let result = rec.parse_str("");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["eof"]);

    let result = rec.parse_str("a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "eof"]);

    let result = rec.parse_str("aa");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "a", "eof"]);
}

#[test]
fn simple_recursion_2() {
    let a = StringParser::new("a".to_owned());

    let rec = RecParser::new(|rec_ref| {
        SameOrParser2::new(a, rec_ref.surround(CharParser::new('['), CharParser::new(']')))
    });

    let result = rec.parse_str("a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), "a");

    let result = rec.parse_str("[a]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), "a");

    let result = rec.parse_str("[[a]]");
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
    let a = StringParser::new("a".to_owned()).map(Value::String);

    let rec = RecParser::new(|rec_ref| {
        let custom = CustomParser::new(move |context| rec_ref.parse(context));

        let tmp = custom
            .separated_by_non_empty(CharParser::new(','))
            .surround(CharParser::new('['), CharParser::new(']'))
            .map(Value::Array);

        SameOrParser2::new(a, tmp)
    });

    let result = rec.parse_str("a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), Value::String("a".to_owned()));

    let result = rec.parse_str("[a]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), Value::Array(vec![Value::String("a".to_owned())]));

    let result = rec.parse_str("[a,a]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), Value::Array(vec![Value::String("a".to_owned()), Value::String("a".to_owned())]));

    let result = rec.parse_str("[[a]]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), Value::Array(vec![Value::Array(vec![Value::String("a".to_owned())])]));

    let result = rec.parse_str("[b]");
    assert!(result.is_failure());
}
