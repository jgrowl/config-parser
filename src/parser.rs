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

named!(comment_line_parser<&str>,
    do_parse!(
        tag_s!("#") >>
//        comment: map_res!(alphanumeric, str::from_utf8) >>
//        comment: map_res!(take_until!("\n"), str::from_utf8) >>
        comment: take_until!("\n") >>

//        terminated!(alphanumeric, end_of_line) >>
        (comment)

//          terminated!(alphanumeric, end_of_line) >>
//          str::from_utf8

    )
);

#[test]
fn test_comment_line_parser() {
    let empty = &b""[..];
    assert_eq!(comment_line_parser("#varname0 varvalue0".as_bytes()), IResult::Done(empty, "varname0"));
//    assert_eq!(comment_line_parser("#varname0 varvalue0".as_bytes()), IResult::Done(empty, "varname0 varvalue0"));
}
