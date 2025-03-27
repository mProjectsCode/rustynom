use rustynom::{
    atomic_parsers::LiteralParser,
    combinator_parsers::{AndParser2, OrParser2},
    parse_str,
    parser::ParserWrapper,
};

#[test]
fn simple_and() {
    let parser = ParserWrapper::<char, _, true>::from_parser(AndParser2::new(
        LiteralParser::new('a'),
        LiteralParser::new('b'),
    ));

    let result = parse_str!(parser, "ab");
    assert!(result.is_success());

    let result = parse_str!(parser, "cb");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
    assert_eq!(failure.expected, Some(vec!["a".to_string()]));

    let result = parse_str!(parser, "ac");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 1);
    assert_eq!(failure.expected, Some(vec!["b".to_string()]));
}

#[test]
fn simple_or() {
    let parser = ParserWrapper::<char, _, true>::from_parser(OrParser2::new(
        LiteralParser::new('a'),
        LiteralParser::new('b'),
    ));

    let result = parse_str!(parser, "a");
    assert!(result.is_success());

    let result = parse_str!(parser, "b");
    assert!(result.is_success());

    let result = parse_str!(parser, "c");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
    assert_eq!(
        failure.expected,
        Some(vec!["a".to_string(), "b".to_string()])
    );
}
