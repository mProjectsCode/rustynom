use rustynom::{AndParser2, CharParser, OrParser2, ParserCombinator};

#[test]
fn simple_and() {
    let a = CharParser::new('a');
    let b = CharParser::new('b');
    let parser = AndParser2::new(&a, &b);

    let result = parser.parse_str("ab");
    assert!(result.is_success());

    let result = parser.parse_str("cb");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index, 0);
    assert_eq!(failure.expected, vec!["a"]);

    let result = parser.parse_str("ac");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index, 1);
    assert_eq!(failure.expected, vec!["b"]);
}

#[test]
fn simple_or() {
    let a = CharParser::new('a');
    let b = CharParser::new('b');
    let parser = OrParser2::new(&a, &b);

    let result = parser.parse_str("a");
    assert!(result.is_success());

    let result = parser.parse_str("b");
    assert!(result.is_success());

    let result = parser.parse_str("c");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index, 0);
    assert_eq!(failure.expected, vec!["a", "b"]);
}
