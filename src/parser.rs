use std::collections::HashMap;
use std::str;

use winnow::prelude::*;
use winnow::{
    combinator::{delimited, separated_pair},
};
use winnow::ascii::{alpha1, alphanumeric0, multispace0};
use winnow::combinator::{repeat, terminated};
use crate::model::Pkl;

fn parse_field_name<'i>(input: &mut &'i str) -> PResult<&'i str> {
    alpha1.parse_next(input)
}

fn parse_string<'i>(input: &mut &'i str) -> PResult<&'i str> {
    delimited('"', alphanumeric0, '"').parse_next(input)
}

fn parse_field<'i>(input: &mut &'i str) -> PResult<(&'i str, &'i str)> {
    separated_pair(
        parse_field_name,
        delimited(multispace0, '=', multispace0),
        parse_string
    ).parse_next(input)
}

// fn parse_object(input: &mut &str) -> PResult<Pkl> {
//     let fields = repeat(0.., parse_field).parse_next(input);
//     delimited(
//         terminated('{', multispace0),
//         fields,
//         '}',
//     ).map(
//         |fields| {
//             // add types to field_map
//
//             let mut field_map = HashMap::<String, Pkl>::new();
//             for (name, value) in fields {
//                 field_map.insert(name.to_string(), Pkl::String(value.to_string()));
//             }
//             Pkl::Object(field_map)
//         }
//     ).parse_next(input)
// }

#[cfg(test)]
mod test {
    use super::*;

    fn test_parser<'i, O, F>(parser: F, input: &'i str, expected: O)
    where
        F: Fn(&mut &'i str) -> PResult<O>,
        O: std::fmt::Debug + PartialEq,
    {
        let mut input = input;
        let result = parser(&mut input);
        assert_eq!(result, Ok(expected));
        assert_eq!(input, "");
    }

    #[test]
    fn parse_field_test() {
        test_parser(parse_field, r#"name = """#, ("name", ""));
        test_parser(parse_field, r#"name = "value""#, ("name", "value"));
    }
}