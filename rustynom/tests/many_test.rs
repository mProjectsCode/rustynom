use rustynom::{CharParser, ManyParser, ParserCombinator};

#[test]
fn simple_many() {
    let a = CharParser::new('a');
    let parser = ManyParser::new(&a);

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