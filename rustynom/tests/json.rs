#![feature(test)]

use std::fs;

use rustynom::{
    atomic_parsers::{CharParser, StringParser},
    combinator_parsers::{AndParser5, SameOrParser2, SameOrParser6},
    parser::{Parser, ParserCombinator},
    transformation_parsers::RecParser,
    utility_parsers,
};

extern crate test;
use test::Bencher;

#[derive(Debug, Clone, PartialEq)]
enum Value {
    Null,
    Bool(bool),
    Number(f32),
    String(String),
    Array(Vec<Value>),
    Object(Vec<(String, Value)>),
}

fn define_parser() -> impl Parser<Value> {
    let number = utility_parsers::float().map(Value::Number);

    let string = utility_parsers::multi_char_test(|c| c != '"', "string".to_string())
        .trim(StringParser::new("\"".to_string()))
        // .describe("string")
        ;
    let string_value = string.clone().map(Value::String)
    // .describe("string value")
    ;

    let boolean = SameOrParser2::new(
        StringParser::new("true".to_string()).map(|_| Value::Bool(true)),
        StringParser::new("false".to_string()).map(|_| Value::Bool(false)),
    );

    let null = StringParser::new("null".to_string()).map(|_| Value::Null);

    RecParser::new(|rec_ref| {
        let array = rec_ref
            .clone()
            .separated_by_non_empty(CharParser::new(','))
            .surround(CharParser::new('['), CharParser::new(']'))
            // .box_describe("array")
            .map(Value::Array);

        let object_entry = AndParser5::new(
            utility_parsers::optional_whitespace(),
            string.clone(),
            utility_parsers::optional_whitespace(),
            CharParser::new(':'),
            rec_ref.clone(),
        )
        // .box_describe("object property")
        .map(|(_, key, _, _, value)| (key, value));

        let object = object_entry
            .separated_by_non_empty(CharParser::new(','))
            .surround(CharParser::new('{'), CharParser::new('}'))
            // .box_describe("object")
            // .map(|entries| {
            //     let mut map = HashMap::new();
            //     for (key, value) in entries {
            //         map.insert(key, value);
            //     }
            //     Value::Object(map)
            // });
            .map(Value::Object);

        // array works, but object doesn't
        SameOrParser6::new(number, string_value, boolean, null, array, object)
            .trim(utility_parsers::optional_whitespace())
    })
}

// #[test]
// fn test_json() {
//     let parser = define_parser().then_eof();
//     let input = "{\"test\": 1.5}";

//     let result = parser.parse_str(input);
//     assert!(result.is_success());
//     let value = result.unwrap_success();
//     assert_eq!(
//         value,
//         Value::Object({
//             let mut map = HashMap::new();
//             map.insert("test".to_string(), Value::Number(1.5));
//             map
//         })
//     );
// }

// #[test]
// fn test_json_2() {
//     let parser = define_parser().then_eof();
//     let input = r#"
//         {
//             "key1": 123,
//             "key2": "value",
//             "key3": true,
//             "key4": null,
//             "key5": [1, 2, 3],
//             "key6": {
//                 "nested": "object"
//             }
//         }
//     "#;

//     let result = parser.parse_str(input);
//     assert!(result.is_success());
//     let value = result.unwrap_success();
//     assert_eq!(
//         value,
//         Value::Object({
//             let mut map = HashMap::new();
//             map.insert("key1".to_string(), Value::Number(123.0));
//             map.insert("key2".to_string(), Value::String("value".to_string()));
//             map.insert("key3".to_string(), Value::Bool(true));
//             map.insert("key4".to_string(), Value::Null);
//             map.insert(
//                 "key5".to_string(),
//                 Value::Array(vec![
//                     Value::Number(1.0),
//                     Value::Number(2.0),
//                     Value::Number(3.0),
//                 ]),
//             );
//             map.insert(
//                 "key6".to_string(),
//                 Value::Object({
//                     let mut nested_map = HashMap::new();
//                     nested_map.insert("nested".to_string(), Value::String("object".to_string()));
//                     nested_map
//                 }),
//             );
//             map
//         })
//     );
// }


#[test] 
fn test_json_file() {
    let json_string = fs::read_to_string("tests/data/big_json.json").unwrap();
    dbg!(json_string.len());
    let parser = define_parser().then_eof();


    let result = parser.parse_str(&json_string);
    assert!(result.is_success());

}

#[bench]
fn bench_json(b: &mut Bencher) {
    let json_string = fs::read_to_string("tests/data/big_json.json").unwrap();
    let parser = define_parser().then_eof();

    let chars = json_string.chars().collect::<Vec<_>>();

    b.iter(|| {
        parser.parse_chars(&chars)
    })
}