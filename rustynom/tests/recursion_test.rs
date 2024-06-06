use rustynom::{AndParser2, EofParser, ParserCombinator, RecParser, SameOrParser2, StringParser};

#[test]
fn simple_recursion() {
    let a = StringParser::new("a".to_owned());
    let mapped_eof = EofParser::new().owned_map(|_| vec!["eof".to_owned()]);
    let rec = RecParser::<Vec<String>>::new();

    let mapped_and = AndParser2::new(&a, &rec).owned_map(|(a, b)| {
        let mut result = vec![a];
        result.extend(b);
        result
    });

    let or = SameOrParser2::new(&mapped_eof, &mapped_and);

    rec.set(&or);

    let result = rec.parse_str("a");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "eof"]);

    let result = rec.parse_str("aa");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), vec!["a", "a", "eof"]);
}
