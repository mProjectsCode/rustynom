use rustynom::{
    atomic_parsers::CharParser, parser::ParserCombinator, transformation_parsers::ManyParser,
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
