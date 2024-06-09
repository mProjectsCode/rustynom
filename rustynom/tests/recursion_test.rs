use rustynom::{
    atomic_parsers::{EofParser, StringParser},
    combinator_parsers::{AndParser2, SameOrParser2},
    parser::ParserCombinator,
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
