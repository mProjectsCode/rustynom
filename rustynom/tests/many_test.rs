use rustynom::{
    atomic_parsers::CharParser, combinator_parsers::AndParser2, parser::ParserCombinator,
    transformation_parsers::ManyParser,
};

#[test]
fn simple_many() {
    let parser = ManyParser::new(CharParser::new('a'));

    let result = parser.parse_str("aaa");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("b");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);

    let result = parser.parse_str("");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_2() {
    let parser = ManyParser::new(AndParser2::new(CharParser::new('a'), CharParser::new('b')));

    let result = parser.parse_str("ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![('a', 'b')]);

    let result = parser.parse_str("ababab");
    assert!(result.is_success());
    assert_eq!(
        result.unwrap_success(),
        vec![('a', 'b'), ('a', 'b'), ('a', 'b')]
    );

    let result = parser.parse_str("");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_3() {
    let parser = ManyParser::new(CharParser::new('a').skip(CharParser::new('b')));

    let result = parser.parse_str("ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parser.parse_str("ababab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_4() {
    let parser = CharParser::new('a').skip(CharParser::new('b')).many();

    let result = parser.parse_str("ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parser.parse_str("ababab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_5() {
    let parser = CharParser::new('a')
        .skip(CharParser::new('b'))
        .many()
        .skip(CharParser::new('c'));

    let result = parser.parse_str("abc");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parser.parse_str("abababc");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("c");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_separated_by() {
    let parser = CharParser::new('a').separated_by_non_empty(CharParser::new(','));

    // let result = parser.parse_str("a,a,a");
    // assert!(result.is_success());
    // assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("a,a,a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result: rustynom::ParseResult<Vec<char>> = parser.parse_str("");
    assert!(result.is_failure());
}

#[test]
fn simple_separated_by_2() {
    let parser = CharParser::new('a')
        .separated_by_non_empty(CharParser::new(','))
        .surround(CharParser::new('['), CharParser::new(']'));

    // let result = parser.parse_str("a,a,a");
    // assert!(result.is_success());
    // assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parser.parse_str("[a,a,a]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result: rustynom::ParseResult<Vec<char>> = parser.parse_str("[]");
    assert!(result.is_failure());
}
