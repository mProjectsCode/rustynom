use rustynom::{
    parse_str,
    parser::{ParserCombinator, ParserWrapper},
    utility_parsers,
};

#[test]
fn simple_uint() {
    let parser = utility_parsers::uint().then_eof();
    let p = ParserWrapper::<_, _, false>::from_parser(parser);

    let result = parse_str!(p, "123");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 123);

    let result = parse_str!(p, "1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1);

    let result = parse_str!(p, "abc");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
}

#[test]
fn simple_float() {
    let parser = utility_parsers::float().then_eof();
    let p = ParserWrapper::<_, _, false>::from_parser(parser);

    let result = parse_str!(p, "123");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 123_f64);

    let result = parse_str!(p, "1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1_f64);

    let result = parse_str!(p, "1.1");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.1_f64);

    let result = parse_str!(p, "1.12");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.12_f64);

    let result = parse_str!(p, "1.002");
    assert!(result.is_success());
    assert_eq!(result.unwrap_success(), 1.002_f64);

    let result = parse_str!(p, "abc");
    assert!(result.is_failure());
    let failure = result.unwrap_failure();
    assert_eq!(failure.furthest.index(), 0);
}
