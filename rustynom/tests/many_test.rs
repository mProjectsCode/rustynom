use rustynom::{
    atomic_parsers::{LiteralParser, LiteralParserNoOutput},
    combinator_parsers::AndParser2,
    parse_str,
    parser::{ParserCombinator, ParserWrapper},
    transformation_parsers::ManyParser,
};

#[test]
fn simple_many() {
    let parser =
        ParserWrapper::<char, Vec<char>>::from_parser(ManyParser::new(LiteralParser::new('a')));

    let result = parse_str!(parser, "aaa");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "b");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);

    let result = parse_str!(parser, "");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_2() {
    let parser = ParserWrapper::<char, Vec<(char, char)>>::from_parser(ManyParser::new(
        AndParser2::new(LiteralParser::new('a'), LiteralParser::new('b')),
    ));

    let result = parse_str!(parser, "ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![('a', 'b')]);

    let result = parse_str!(parser, "ababab");
    assert!(result.is_success());
    assert_eq!(
        result.unwrap_success(),
        vec![('a', 'b'), ('a', 'b'), ('a', 'b')]
    );

    let result = parse_str!(parser, "");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_3() {
    let parser = ParserWrapper::<char, Vec<char>>::from_parser(ManyParser::new(
        LiteralParser::new('a').skip(LiteralParserNoOutput::new('b')),
    ));

    let result = parse_str!(parser, "ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parse_str!(parser, "ababab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_4() {
    let parser = ParserWrapper::<char, Vec<char>>::from_parser(
        LiteralParser::new('a')
            .skip(LiteralParserNoOutput::new('b'))
            .many(),
    );

    let result = parse_str!(parser, "ab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parse_str!(parser, "ababab");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_many_5() {
    let parser = ParserWrapper::<char, Vec<char>>::from_parser(
        LiteralParser::new('a')
            .skip(LiteralParserNoOutput::new('b'))
            .many()
            .skip(LiteralParserNoOutput::new('c')),
    );

    let result = parse_str!(parser, "abc");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result = parse_str!(parser, "abababc");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "c");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec![]);
}

#[test]
fn simple_separated_by() {
    let parser = ParserWrapper::<char, Vec<char>>::from_parser(
        LiteralParser::new('a').separated_by(LiteralParserNoOutput::new(',')),
    );

    // let result = parser.parse_list("a,a,a");
    // assert!(result.is_success());
    // assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "a,a,a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a']);

    let result: rustynom::ParseResult<Vec<char>> = parse_str!(parser, "");
    assert!(result.is_failure());
}

#[test]
fn simple_separated_by_2() {
    let parser = ParserWrapper::<char, Vec<char>>::from_parser(
        LiteralParser::new('a')
            .separated_by(LiteralParserNoOutput::new(','))
            .surround(
                LiteralParserNoOutput::new('['),
                LiteralParserNoOutput::new(']'),
            ),
    );

    // let result = parser.parse_list("a,a,a");
    // assert!(result.is_success());
    // assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result = parse_str!(parser, "[a,a,a]");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!['a', 'a', 'a']);

    let result: rustynom::ParseResult<Vec<char>> = parse_str!(parser, "[]");
    assert!(result.is_failure());
}
