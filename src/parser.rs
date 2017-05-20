use std::str;

use std::collections::HashMap;
use std::collections::LinkedList;

use std::str::Utf8Error;

use nom::*;


pub fn from_utf8_option(o: Option<&[u8]>) -> Result<Option<&str>, Utf8Error> {
    match o {
        None => Result::Ok(None),
        Some(x) => {
            match str::from_utf8(o.unwrap()) {
                Ok(v) => Result::Ok(Some(v)),
                Err(e) => Result::Err(e)
            }
        }
    }
}

fn default_key_value<'a>() -> Out<'a> {
    Out::KeyValue {
        whitespace_1: None,
        key: "",
        whitespace_2: None,
        separator: " ",
        whitespace_3: None,
        value: "",
        whitespace_4: None
    }
}

#[derive(Debug,PartialEq)]
enum Out<'a> {
    Comment {
        whitespace_1: Option<&'a str>,
        separator: &'a str,
        text: Option<&'a str>,
    },

    KeyValue {
        whitespace_1: Option<&'a str>,
        key: &'a str,
        whitespace_2: Option<&'a str>,
        separator: &'a str,
        whitespace_3: Option<&'a str>,
        value: &'a str,
        whitespace_4: Option<&'a str>
    }
}

named!(comment<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option)  >>
        tag_s!("#") >>
        text: map_res!(opt!(not_line_ending), from_utf8_option) >>
        opt!(eol) >>
        (Out::Comment{whitespace_1: whitespace_1, separator: "#", text: text})
    )
);

#[test]
fn test_comment() {
    let input = &b"# This is a comment\n"[..];
    let consumed = Out::Comment {
        whitespace_1: None,
        separator: "#",
        text: Some(" This is a comment")
    };
    let expected = expected_done(consumed);
    let actual = comment(input);
    assert_eq!(expected, actual);
}

named!(key_value<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option) >>
        key: map_res!(alt!(alphanumeric | tag_s!("_")), str::from_utf8) >> // Need to support other characters?
        whitespace_2: map_res!(opt!(multispace), from_utf8_option) >>
        separator: map_res!(alt!(tag_s!("=") | space), str::from_utf8) >>
        whitespace_3: map_res!(opt!(multispace), from_utf8_option) >>
        value: map_res!(alphanumeric, str::from_utf8) >>
        whitespace_4: map_res!(opt!(complete!(multispace)), from_utf8_option) >>
        ending: map_res!(opt!(complete!(line_ending)), from_utf8_option) >>
        (Out::KeyValue{whitespace_1: whitespace_1, key: key, whitespace_2: whitespace_2, separator: separator,
            whitespace_3: whitespace_3, value: value, whitespace_4: whitespace_4})
    )
);

named!(key_value2<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option) >>
        key: map_res!(alt!(alphanumeric | tag_s!("_")), str::from_utf8) >>
        separator: map_res!(multispace, str::from_utf8) >>
        value: map_res!(not_line_ending, str::from_utf8) >>
        whitespace_4: map_res!(opt!(complete!(multispace)), from_utf8_option) >>
        ending: map_res!(opt!(complete!(line_ending)), from_utf8_option) >>
        (Out::KeyValue{whitespace_1: whitespace_1, key: key, whitespace_2: None, separator: separator,
            whitespace_3: None, value: value, whitespace_4: whitespace_4})
    )
);


#[test]
fn test_key_value() {
    let expected = expected_done(Out::KeyValue {
        whitespace_1: None,
        key: "key",
        whitespace_2: Some(" "),
        separator: "=",
        whitespace_3: Some(" "),
        value: "value",
        whitespace_4: None
    });

    let input = &b"key = value"[..];
    let actual = key_value(input);
    assert_eq!(expected, actual);
}

fn test_key_value_underscore_key() {
    let expected = expected_done(Out::KeyValue {
        whitespace_1: None,
        key: "key",
        whitespace_2: Some(" "),
        separator: "=",
        whitespace_3: Some(" "),
        value: "value",
        whitespace_4: None
    });

    let input = &b"key = value"[..];
    let actual = key_value(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_key_value2() {
    let expected = expected_done(Out::KeyValue {
        whitespace_1: None,
        key: "key",
        whitespace_2: None,
        separator: " ",
        whitespace_3: None,
        value: "value",
        whitespace_4: None
    });

    let input = &b"key value"[..];
    let actual = key_value2(input);
    assert_eq!(expected, actual);
}
//
////named!(keys_values<&[u8], HashMap<&str, &str> >,
////  map!(
////    many0!(terminated!(key_value, opt!(multispace))),
////    |vec: Vec<_>| vec.into_iter().collect()
////  )
////);


named!(keys_values<&[u8], Vec<Out>>,
    many0!(alt!(comment | key_value | key_value2 ))
);


//    many0!(alt!(key_value))
//  map!(
//    many0!(terminated!(key_value, opt!(multispace))),
//    |vec: Vec<_>| vec.into_iter().collect()
//  )


#[test]
fn test_key_values() {
    let input = &b"key0    value0\nkey1    value1\nkey2    value2"[..];
    let output = vec!(
                     Out::KeyValue {
                         whitespace_1: None,
                         key: "key0",
                         whitespace_2: None,
                         separator: "    ",
                         whitespace_3: None,
                         value: "value0",
                         whitespace_4: Some("\n")
                     },
                     Out::KeyValue {
                         whitespace_1: None,
                         key: "key1",
                         whitespace_2: None,
                         separator: "    ",
                         whitespace_3: None,
                         value: "value1",
                         whitespace_4: Some("\n")
                     },
                     Out::KeyValue {
                         whitespace_1: None,
                         key: "key2",
                         whitespace_2: None,
                         separator: "    ",
                         whitespace_3: None,
                         value: "value2",
                         whitespace_4: None
                     }
                     );
    let expected = expected_done(output);
    let actual = keys_values(input);
    assert_eq!(expected, actual);
}

fn expected_done<'a, I>(result: I) -> IResult<&'a [u8], I> {
    IResult::Done(&b""[..], result)
}
