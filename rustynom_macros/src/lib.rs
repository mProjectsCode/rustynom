extern crate proc_macro;
use proc_macro::TokenStream;

#[proc_macro]
pub fn and_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    // construct the type arguments T1, T2, ..., Tn
    let full_type_args: String = (1..=n)
        .map(|i| format!("T{}Parser, T{}", i, i))
        .collect::<Vec<String>>()
        .join(", ");

    let type_args: String = (1..=n)
        .map(|i| format!("T{}", i))
        .collect::<Vec<String>>()
        .join(", ");

    let where_clause: String = (1..=n)
        .map(|i| format!("T{}Parser: RawParser<T{}>, T{}: Clone", i, i, i))
        .collect::<Vec<String>>()
        .join(", ");

    let mut output = String::new();

    // construct the struct definition
    output.push_str("#[derive(Clone)]\n");
    output.push_str(
        format!(
            "pub struct AndParser{}<{}> where {} {{",
            n, full_type_args, where_clause
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
    output.push_str(
        (1..=n)
            .map(|i| format!("__phantom{}: std::marker::PhantomData<T{}>,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the impl block for AndParser
    output.push_str(
        format!(
            "impl<{}> AndParser{}<{}> where {} {{",
            full_type_args, n, full_type_args, where_clause
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
    output.push_str(
        (1..=n)
            .map(|i| format!("__phantom{}: std::marker::PhantomData,", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawParser<({})> for AndParser{}<{}> where {} {{",
            full_type_args, type_args, n, full_type_args, where_clause
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "fn parse(&self, context: &mut ParsingContext) -> ParseResult<({})> {{",
            type_args
        )
        .as_str(),
    );
    // output.push_str("match (");
    // output.push_str(
    //     (1..=n)
    //         .map(|i| format!("self.p{}.parse(context),", i))
    //         .collect::<Vec<String>>()
    //         .join("")
    //         .as_str(),
    // );
    // output.push_str(") {");
    // output.push_str(
    //     format!(
    //         "({}) => ParseResult::Success(({})),",
    //         (1..=n)
    //             .map(|i| format!("ParseResult::Success(p{})", i))
    //             .collect::<Vec<String>>()
    //             .join(", "),
    //         (1..=n)
    //             .map(|i| format!("p{}", i))
    //             .collect::<Vec<String>>()
    //             .join(", ")
    //     )
    //     .as_str(),
    // );
    // for i in 1..=n {
    //     output.push_str(
    //         format!(
    //             "({}ParseResult::Failure(f),{}) => ParseResult::Failure(f),",
    //             "_,".repeat(i - 1),
    //             "_,".repeat(n - i)
    //         )
    //         .as_str(),
    //     );
    // }

    for i in 1..=n {
        output.push_str(format!("let r{} = self.p{}.parse(context);", i, i).as_str());
        output.push_str(format!("if let ParseResult::Failure(x) = r{} {{", i).as_str());
        output.push_str("return ParseResult::Failure(x);");
        output.push_str("}");
        output.push_str(format!("let s{} = r{}.unwrap_success();", i, i).as_str());
    }

    output.push_str(format!("ParseResult::Success(({}))", (1..=n).map(|i| format!("s{}", i)).collect::<Vec<String>>().join(", ")).as_str());

    output.push_str("}}");

    output.parse().unwrap()
}

#[proc_macro]
pub fn or_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    // construct the type arguments T1, T2, ..., Tn
    let full_type_args: String = (1..=n)
        .map(|i| format!("T{}Parser, T{}", i, i))
        .collect::<Vec<String>>()
        .join(", ");

    let type_args: String = (1..=n)
        .map(|i| format!("T{}", i))
        .collect::<Vec<String>>()
        .join(", ");

    let where_clause: String = (1..=n)
        .map(|i| format!("T{}Parser: RawParser<T{}>, T{}: Clone", i, i, i))
        .collect::<Vec<String>>()
        .join(", ");

    let mut output = String::new();

    // variant enum definition
    output.push_str("#[derive(Debug, Clone, PartialEq)]\n");
    output.push_str(format!("pub enum Variant{}<{}> {{", n, type_args).as_str());
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
            n, full_type_args, where_clause
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
    output.push_str(
        (1..=n)
            .map(|i| format!("__phantom{}: std::marker::PhantomData<T{}>,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(
        format!(
            "impl<{}> OrParser{}<{}> where {} {{",
            full_type_args, n, full_type_args, where_clause
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
    output.push_str(
        (1..=n)
            .map(|i| format!("__phantom{}: std::marker::PhantomData,", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawParser<Variant{}<{}>> for OrParser{}<{}> where {} {{",
            full_type_args, n, type_args, n, full_type_args, where_clause
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "fn parse(&self, context: &mut ParsingContext) -> ParseResult<Variant{}<{}>> {{",
            n, type_args
        )
        .as_str(),
    );

    for i in 1..n {
        output.push_str("let mut cloned_context = context.clone();");
        output.push_str(format!("let r{} = self.p{}.parse(&mut cloned_context);", i, i).as_str());
        output.push_str(format!("if let ParseResult::Success(x) = r{} {{", i).as_str());
        output.push_str(
            format!(
                "return context.succeed_at(cloned_context.position.index, Variant{}::V{}(x));",
                n, i
            )
            .as_str(),
        );
        output.push('}');
        if i == 1 {
            output.push_str(format!("let mut failure = r{}.unwrap_failure();", i).as_str());
        } else {
            output.push_str(
                format!(
                    "failure = context.merge_failures(failure, r{}.unwrap_failure());",
                    i
                )
                .as_str(),
            );
        }
    }

    output.push_str(format!("match self.p{}.parse(context) {{", n).as_str());
    output.push_str(
        format!(
            "ParseResult::Success(x) => ParseResult::Success(Variant{}::V{}(x)),",
            n, n
        )
        .as_str(),
    );
    output.push_str(
        "ParseResult::Failure(f) => ParseResult::Failure(context.merge_failures(failure, f)),",
    );
    output.push_str("}}}\n");

    // Same Type

    let mut full_same_type_args: String = (1..=n)
        .map(|i| format!("T{}Parser", i))
        .collect::<Vec<String>>()
        .join(", ");

    full_same_type_args.push_str(", T");

    let mut full_where_clause: String = (1..=n)
        .map(|i| format!("T{}Parser: RawParser<T>", i))
        .collect::<Vec<String>>()
        .join(", ");

    full_where_clause.push_str(", T: Clone");

    // construct the struct definition
    output.push_str("#[derive(Clone)]\n");
    output.push_str(
        format!(
            "pub struct SameOrParser{}<{}> where {} {{",
            n, full_same_type_args, full_where_clause
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
    output.push_str("__phantom: std::marker::PhantomData<T>,");
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(
        format!(
            "impl<{}> SameOrParser{}<{}> where {} {{",
            full_same_type_args, n, full_same_type_args, full_where_clause
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
            full_same_type_args
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
    output.push_str("__phantom: std::marker::PhantomData,");
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<{}> RawParser<T> for SameOrParser{}<{}> where {} {{",
            full_same_type_args, n, full_same_type_args, full_where_clause
        )
        .as_str(),
    );
    output.push_str("fn parse(&self, context: &mut ParsingContext) -> ParseResult<T> {");

    for i in 1..n {
        output.push_str("let mut cloned_context = context.clone();");
        output.push_str(format!("let r{} = self.p{}.parse(&mut cloned_context);", i, i).as_str());
        output.push_str(format!("if let ParseResult::Success(x) = r{} {{", i).as_str());
        output.push_str("return context.succeed_at(cloned_context.position.index, x);");
        output.push('}');
        if i == 1 {
            output.push_str(format!("let mut failure = r{}.unwrap_failure();", i).as_str());
        } else {
            output.push_str(
                format!(
                    "failure = context.merge_failures(failure, r{}.unwrap_failure());",
                    i
                )
                .as_str(),
            );
        }
    }

    output.push_str(format!("match self.p{}.parse(context) {{", n).as_str());
    output.push_str("ParseResult::Success(x) => ParseResult::Success(x),");
    output.push_str(
        "ParseResult::Failure(f) => ParseResult::Failure(context.merge_failures(failure, f)),",
    );
    output.push_str("}}}\n");

    output.parse().unwrap()
}
