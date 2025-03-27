extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn and_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    // construct the type arguments T1, T2, ..., Tn
    let full_type_args: String = "ERROR, TIn, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let type_args_decl = "const ERROR: bool, TIn, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let where_clause: String = "TIn: Parsable, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser: RawTestParser<TIn, ERROR>", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let mut output = String::new();

    // construct the struct definition
    output.push_str("#[derive(Clone)]\n");
    output.push_str(
        format!(
            "pub struct AndParser{}<{}> where {} {{",
            n, type_args_decl, where_clause
        )
        .as_str(),
    );
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: T{}Parser,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str(format!("__phantom_in: std::marker::PhantomData<TIn>,").as_str());
    output.push_str("}\n");

    // construct the impl block for AndParser
    output.push_str(
        format!(
            "impl<{}> AndParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "pub fn new({}) -> AndParser{}<{}> {{",
            (1..=n)
                .map(|i| format!("p{}: T{}Parser", i, i))
                .collect::<Vec<String>>()
                .join(", "),
            n,
            full_type_args
        )
        .as_str(),
    );
    output.push_str(format!("AndParser{} {{", n).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{},", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("__phantom_in: std::marker::PhantomData,");
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawTestParser<TIn, ERROR> for AndParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );
    let out_type = format!(
        "({})",
        (1..=n)
            .map(|i| format!("<T{}Parser as RawTestParser<TIn, ERROR>>::TOut", i))
            .collect::<Vec<String>>()
            .join(", ")
    );

    output.push_str(
        format!(
            "type TOut = {};",
            out_type
        )
        .as_str(),
    );

    output.push_str(
        format!(
            "fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<{}> {{",
            out_type
        )
        .as_str(),
    );

    for i in 1..=n {
        output.push_str(format!("let r{} = self.p{}.parse(input, position);", i, i).as_str());
        output.push_str(format!("if let ParseResult::Failure(x) = r{} {{", i).as_str());
        output.push_str("return ParseResult::Failure(x);");
        output.push_str("}");
        output.push_str(format!("let s{} = r{}.unwrap_success();", i, i).as_str());
    }

    output.push_str(
        format!(
            "ParseResult::Success(({}))",
            (1..=n)
                .map(|i| format!("s{}", i))
                .collect::<Vec<String>>()
                .join(", ")
        )
        .as_str(),
    );
    output.push_str("}");

    output.push_str("}");

    output.parse().unwrap()
}

#[proc_macro]
pub fn or_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    let full_type_args: String = "ERROR, TIn, ".to_string()
    + (1..=n)
        .map(|i| format!("T{}Parser", i))
        .collect::<Vec<String>>()
        .join(", ")
        .as_str();

    let type_args_decl = "const ERROR: bool, TIn, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let enum_type_args: String = (1..=n)
        .map(|i| format!("T{}", i))
        .collect::<Vec<String>>()
        .join(", ");

    let where_clause: String = "TIn: Parsable, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser: RawTestParser<TIn, ERROR>", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let mut output = String::new();

    // variant enum definition
    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str(format!("pub enum Variant{}<{}> {{", n, enum_type_args).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("V{}(T{}),", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the struct definition
    output.push_str("#[derive(Clone)]\n");
    output.push_str(
        format!(
            "pub struct OrParser{}<{}> where {} {{",
            n, type_args_decl, where_clause
        )
        .as_str(),
    );
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: T{}Parser,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("__phantom_in: std::marker::PhantomData<TIn>,");
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(
        format!(
            "impl<{}> OrParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "pub fn new({}) -> OrParser{}<{}> {{",
            (1..=n)
                .map(|i| format!("p{}: T{}Parser", i, i))
                .collect::<Vec<String>>()
                .join(", "),
            n,
            full_type_args
        )
        .as_str(),
    );
    output.push_str(format!("OrParser{} {{", n).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{},", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("__phantom_in: std::marker::PhantomData,");
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawTestParser<TIn, ERROR> for OrParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );
    let out_type = format!(
        "Variant{}<{}>",
        n,
        (1..=n)
            .map(|i| format!("<T{}Parser as RawTestParser<TIn, ERROR>>::TOut", i))
            .collect::<Vec<String>>()
            .join(", ")
    );

    output.push_str(
        format!(
            "type TOut = {};",
            out_type
        )
        .as_str(),
    );

    output.push_str(
        format!(
            "fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<{}> {{",
            out_type
        )
        .as_str(),
    );

    output.push_str("let initial_pos = position.clone();");

    for i in 1..n {
        output.push_str(format!("let r{} = self.p{}.parse(input, position);", i, i).as_str());
        output.push_str(format!("if let ParseResult::Success(x) = r{} {{", i).as_str());
        output.push_str(format!("return ParseResult::Success(Variant{}::V{}(x));", n, i).as_str());
        output.push_str("} else {");
        output.push_str("position.advance_to(initial_pos.clone());");
        output.push_str("}");
    }

    output.push_str(format!("let r{} = self.p{}.parse(input, position);", n, n).as_str());
    output.push_str(format!("if let ParseResult::Success(x) = r{} {{", n).as_str());
    output.push_str(format!("return ParseResult::Success(Variant{}::V{}(x));", n, n).as_str());
    output.push('}');

    output.push_str("let mut failure = r1.unwrap_failure();");
    for i in 2..=n {
        output.push_str(
            format!(
                "failure = position.merge_failures(failure, r{}.unwrap_failure());",
                i
            )
            .as_str(),
        );
    }

    output.push_str(format!("ParseResult::Failure(failure)").as_str());
    output.push_str("}");

    output.push_str("}");

    // Same Type

    let full_type_args: String = "ERROR, TIn, ".to_string()
    + (1..=n)
        .map(|i| format!("T{}Parser", i))
        .collect::<Vec<String>>()
        .join(", ")
        .as_str();

    let type_args_decl = "const ERROR: bool, TIn, ".to_string()
        + (1..=n)
            .map(|i| format!("T{}Parser", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    let where_clause: String = "TIn: Parsable, T1Parser: RawTestParser<TIn, ERROR>, ".to_string()
        + (2..=n)
            .map(|i| format!("T{}Parser: RawTestParser<TIn, ERROR, TOut = <T1Parser as RawTestParser<TIn, ERROR>>::TOut>", i))
            .collect::<Vec<String>>()
            .join(", ")
            .as_str();

    // construct the struct definition
    output.push_str("#[derive(Clone)]\n");
    output.push_str(
        format!(
            "pub struct SameOrParser{}<{}> where {} {{",
            n, type_args_decl, where_clause
        )
        .as_str(),
    );
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: T{}Parser,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("__phantom_in: std::marker::PhantomData<TIn>,");
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(
        format!(
            "impl<{}> SameOrParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "pub fn new({}) -> SameOrParser{}<{}> {{",
            (1..=n)
                .map(|i| format!("p{}: T{}Parser", i, i))
                .collect::<Vec<String>>()
                .join(", "),
            n,
            full_type_args
        )
        .as_str(),
    );
    output.push_str(format!("SameOrParser{} {{", n).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{},", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("__phantom_in: std::marker::PhantomData,");
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawTestParser<TIn, ERROR> for SameOrParser{}<{}> where {} {{",
            type_args_decl, n, full_type_args, where_clause
        )
        .as_str(),
    );

    let out_type = "<T1Parser as RawTestParser<TIn, ERROR>>::TOut";
    output.push_str(format!("type TOut = {};", out_type).as_str());

    output.push_str(format!("fn parse(&self, input: &[TIn::T], position: &mut ParsingPosition) -> ParseResult<{}> {{", out_type).as_str());
    output.push_str("let initial_pos = position.clone();");

    for i in 1..n {
        output.push_str(format!("let r{} = self.p{}.parse(input, position);", i, i).as_str());
        output.push_str(format!("if r{}.is_success() {{", i).as_str());
        output.push_str(format!("return r{};", i).as_str());
        output.push_str("} else {");
        output.push_str("position.advance_to(initial_pos.clone());");
        output.push_str("}");
    }

    output.push_str(format!("let r{} = self.p{}.parse(input, position);", n, n).as_str());
    output.push_str(format!("if r{}.is_success() {{", n).as_str());
    output.push_str(format!("return r{};", n).as_str());
    output.push('}');

    output.push_str("let mut failure = r1.unwrap_failure();");
    for i in 2..=n {
        output.push_str(
            format!(
                "failure = position.merge_failures(failure, r{}.unwrap_failure());",
                i
            )
            .as_str(),
        );
    }

    output.push_str(format!("ParseResult::Failure(failure)").as_str());
    output.push_str("}");

    output.push_str("}");

    output.parse().unwrap()
}
