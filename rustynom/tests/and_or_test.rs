use rustynom::{
    atomic_parsers::CharParser,
    combinator_parsers::{AndParser2, OrParser2},
    parser::ParserCombinator,
};

#[test]
fn simple_and() {
    let parser = AndParser2::new(CharParser::<true>::new('a'), CharParser::<true>::new('b'));

    let result = parser.parse_str("ab");
    assert!(result.is_success());

    let result = parser.parse_str("cb");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
    assert_eq!(failure.expected, vec!["a".to_string()]);

    let result = parser.parse_str("ac");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 1);
    assert_eq!(failure.expected, vec!["b".to_string()]);
}

#[test]
fn simple_or() {
    let parser = OrParser2::new(CharParser::<true>::new('a'), CharParser::<true>::new('b'));

    let result = parser.parse_str("a");
    assert!(result.is_success());

    let result = parser.parse_str("b");
    assert!(result.is_success());

    let result = parser.parse_str("c");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
    assert_eq!(failure.expected, vec!["a".to_string(), "b".to_string()]);
}
