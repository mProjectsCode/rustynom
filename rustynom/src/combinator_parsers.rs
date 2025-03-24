use crate::{ParseResult, ParsingContext, parser::RawParser};
use rustynom_macros::{and_parser, or_parser};

// ---------------
// And parser
// ---------------

and_parser!(2);
and_parser!(3);
and_parser!(4);
and_parser!(5);
and_parser!(6);
and_parser!(7);
and_parser!(8);

// ---------------
// Or parser
// ---------------

or_parser!(2);
or_parser!(3);
or_parser!(4);
or_parser!(5);
or_parser!(6);
or_parser!(7);
or_parser!(8);
