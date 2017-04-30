use std::str;

//use nom::{IResult, space, alpha, alphanumeric, digit};
use nom::*;

named!(key_value_parser< &[u8],(&str, &str)>,
    do_parse!(
        name: map_res!(alphanumeric, str::from_utf8) >>
        opt!(multispace) >>
        value: map_res!(alphanumeric, str::from_utf8) >>
        (name, value)
    )
);

#[test]
fn test_key_value_parser() {
    let input = "varname0 varvalue0".as_bytes();
    let output = ("varname0", "varvalue0");
    let remaining = &b""[..];
    let expected = IResult::Done(remaining, output);
    let actual = key_value_parser(input);
    assert_eq!(expected, actual);
}

named!(comment_line_parser<&str>,
    do_parse!(
        tag_s!("#") >>
        comment: map_res!(not_line_ending, str::from_utf8) >>
        opt!(eol) >>
        (comment)
    )
);

#[test]
fn test_comment_line_parser() {
    let input = "#varname0 varvalue0\n".as_bytes();
    let remaining = "".as_bytes();
    let consumed = "varname0 varvalue0";
    let expected = IResult::Done(remaining, consumed);
    let actual = comment_line_parser(input);
    assert_eq!(expected, actual);
}
