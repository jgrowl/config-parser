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
////        alt!(space? | space?) ~

#[test]
fn test_key_value_parser() {
    let empty = &b""[..];
    assert_eq!(key_value_parser("varname0 varvalue0".as_bytes()), IResult::Done(empty, ("varname0", "varvalue0")));
}

named!(end_of_line, alt!(eof!() | eol));

named!(comment_start, tag!("#"));

#[test]
fn test_comment_start() {
    let input = "# hey read this comment".as_bytes();
    let remaining = " hey read this comment".as_bytes();
    let consumed = "#".as_bytes();
    let expected = IResult::Done(remaining, consumed);
    let actual = comment_start(input);
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
