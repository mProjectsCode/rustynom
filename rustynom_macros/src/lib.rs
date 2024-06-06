extern crate proc_macro;
use proc_macro::TokenStream;

// pub struct AndParser<'a, A, B> {
//     a: Box<&'a dyn Parser<A>>,
//     b: Box<&'a dyn Parser<B>>,
// }

// impl<'a, A, B> AndParser<'a, A, B> {
//     pub fn new(a: &'a dyn Parser<A>, b: &'a dyn Parser<B>) -> AndParser<'a, A, B> {
//         AndParser {
//             a: Box::from(a),
//             b: Box::from(b),
//         }
//     }
// }

// impl<'a, A, B> Parser<(A, B)> for AndParser<'a, A, B> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<(A, B)> {
//         match (self.a.parse(context), self.b.parse(context)) {
//             (ParseResult::Success(a), ParseResult::Success(b)) => ParseResult::Success((a, b)),
//             (ParseResult::Failure(f), _) => ParseResult::Failure(f),
//             (_, ParseResult::Failure(f)) => ParseResult::Failure(f),
//         }
//     }
// }

#[proc_macro]
pub fn and_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    // construct the type arguments T1, T2, ..., Tn
    let type_args: String = (1..=n)
        .map(|i| format!("T{}", i))
        .collect::<Vec<String>>()
        .join(", ");

    let mut output = String::new();

    // construct the struct definition
    output.push_str(format!("pub struct AndParser{}<'a, {}> {{", n, type_args).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: &'a dyn Parser<T{}>,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the impl block for AndParser
    output.push_str(
        format!(
            "impl<'a, {}> AndParser{}<'a, {}> {{",
            type_args, n, type_args
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "pub fn new({}) -> AndParser{}<'a, {}> {{",
            (1..=n)
                .map(|i| format!("p{}: &'a dyn Parser<T{}>", i, i))
                .collect::<Vec<String>>()
                .join(", "),
            n,
            type_args
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
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<'a, {}> Parser<({})> for AndParser{}<'a, {}> {{",
            type_args, type_args, n, type_args
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
    output.push_str("match (");
    output.push_str(
        (1..=n)
            .map(|i| format!("self.p{}.parse(context),", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str(") {");
    output.push_str(
        format!(
            "({}) => ParseResult::Success(({})),",
            (1..=n)
                .map(|i| format!("ParseResult::Success(p{})", i))
                .collect::<Vec<String>>()
                .join(", "),
            (1..=n)
                .map(|i| format!("p{}", i))
                .collect::<Vec<String>>()
                .join(", ")
        )
        .as_str(),
    );
    for i in 1..=n {
        output.push_str(
            format!(
                "({}ParseResult::Failure(f),{}) => ParseResult::Failure(f),",
                "_,".repeat(i - 1),
                "_,".repeat(n - i)
            )
            .as_str(),
        );
    }
    output.push_str("}}}");

    output.parse().unwrap()
}

// #[derive(Debug, Clone, PartialEq)]
// pub enum Variant<T1, T2> {
//     V1(T1),
//     V2(T2),
// }

// pub struct OrParser<'a, A, B> {
//     a: Box<&'a dyn Parser<A>>,
//     b: Box<&'a dyn Parser<B>>,
// }

// impl<'a, A, B> OrParser<'a, A, B> {
//     pub fn new(a: &'a dyn Parser<A>, b: &'a dyn Parser<B>) -> OrParser<'a, A, B> {
//         OrParser {
//             a: Box::from(a),
//             b: Box::from(b),
//         }
//     }
// }

// impl<'a, A, B> Parser<Variant<A, B>> for OrParser<'a, A, B> {
//     fn parse(&self, context: &mut ParsingContext) -> ParseResult<Variant<A, B>> {
//         let mut cloned_context = context.clone();
//         let a = self.a.parse(&mut cloned_context);

//         if let ParseResult::Success(a) = a {
//             return context.succeed_at(cloned_context.position.index, Variant::V1(a));
//         }

//         match self.b.parse(context) {
//             ParseResult::Success(b) => ParseResult::Success(Variant::V2(b)),
//             ParseResult::Failure(f) => ParseResult::Failure(f),
//         }
//     }
// }

#[proc_macro]
pub fn or_parser(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let n = input.parse::<usize>().unwrap();

    // construct the type arguments T1, T2, ..., Tn
    let type_args: String = (1..=n)
        .map(|i| format!("T{}", i))
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
    output.push_str(format!("pub struct OrParser{}<'a, {}> {{", n, type_args).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: &'a dyn Parser<T{}>,", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(
        format!(
            "impl<'a, {}> OrParser{}<'a, {}> {{",
            type_args, n, type_args
        )
        .as_str(),
    );
    output.push_str(
        format!(
            "pub fn new({}) -> OrParser{}<'a, {}> {{",
            (1..=n)
                .map(|i| format!("p{}: &'a dyn Parser<T{}>", i, i))
                .collect::<Vec<String>>()
                .join(", "),
            n,
            type_args
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
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(
        format!(
            "impl<'a, {}> Parser<Variant{}<{}>> for OrParser{}<'a, {}> {{",
            type_args, n, type_args, n, type_args
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

    // construct the struct definition
    output.push_str(format!("pub struct SameOrParser{}<'a, T> {{", n).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: Box<&'a dyn Parser<T>>,", i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}\n");

    // construct the impl block for OrParser
    output.push_str(format!("impl<'a, T> SameOrParser{}<'a, T> {{", n).as_str());
    output.push_str(
        format!(
            "pub fn new({}) -> SameOrParser{}<'a, T> {{",
            (1..=n)
                .map(|i| format!("p{}: &'a dyn Parser<T>", i))
                .collect::<Vec<String>>()
                .join(", "),
            n
        )
        .as_str(),
    );
    output.push_str(format!("SameOrParser{} {{", n).as_str());
    output.push_str(
        (1..=n)
            .map(|i| format!("p{}: Box::from(p{}),", i, i))
            .collect::<Vec<String>>()
            .join("")
            .as_str(),
    );
    output.push_str("}}}\n");

    // construct the impl block for Parser
    output.push_str(format!("impl<'a, T> Parser<T> for SameOrParser{}<'a, T> {{", n).as_str());
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
