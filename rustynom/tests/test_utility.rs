use rustynom::{parser::ParserCombinator, utility_parsers};

#[test]
fn simple_uint() {
    let parser = utility_parsers::uint();

    let result = parser.parse_str("123");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 123);

    let result = parser.parse_str("1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1);

    let result = parser.parse_str("abc");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
}

#[test]
fn simple_float() {
    let parser = utility_parsers::float();

    let result = parser.parse_str("123");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 123_f32);

    let result = parser.parse_str("1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1_f32);

    let result = parser.parse_str("1.1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.1_f32);

    let result = parser.parse_str("1.12");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.12_f32);

    let result = parser.parse_str("1.002");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.002_f32);

    let result = parser.parse_str("abc");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
}
