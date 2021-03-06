use std::str;
use std::io::prelude::*;
use std::fs::File;
use std::str::Utf8Error;

use nom::*;
use types::*;


pub fn from_utf8_option2(o: Option<&[u8]>) -> Result<Option<String>, Utf8Error> {
    match o {
        None => Result::Ok(None),
        Some(_) => {
            match str::from_utf8(o.unwrap()) {
                Ok(v) => Result::Ok(Some(v.to_string())),
                Err(e) => Result::Err(e)
            }
        }
    }
}

named!(comment<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option2)  >>
        tag_s!("#") >>
        text: map_res!(opt!(not_line_ending), from_utf8_option2) >>
        opt!(eol) >>
        (Out::Comment{whitespace_1: whitespace_1, separator: "#".to_string(), text: text})
    )
);

#[test]
fn test_comment() {
    let input = &b"# This is a comment\n"[..];
    let consumed = Out::Comment {
        whitespace_1: None,
        separator: "#".to_string(),
        text: Some(" This is a comment".to_string())
    };
    let expected = expected_done(consumed);
    let actual = comment(input);
    assert_eq!(expected, actual);
}

named!(key_value<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option2) >>
        key: map_res!(alt!(alphanumeric | tag_s!("_")), str::from_utf8) >> // Need to support other characters?
        whitespace_2: map_res!(opt!(multispace), from_utf8_option2) >>
        separator: map_res!(alt!(tag_s!("=") | space), str::from_utf8) >>
        whitespace_3: map_res!(opt!(multispace), from_utf8_option2) >>
        value: map_res!(alphanumeric, str::from_utf8) >>
        whitespace_4: map_res!(opt!(complete!(multispace)), from_utf8_option2) >>
        ending: map_res!(opt!(complete!(line_ending)), from_utf8_option2) >>
        (Out::KeyValue{whitespace_1: whitespace_1, key: key.to_string(), whitespace_2: whitespace_2, separator: separator.to_string(),
            whitespace_3: whitespace_3, value: value.to_string(), whitespace_4: whitespace_4})
    )
);

named!(key_value2<Out>,
    do_parse!(
        whitespace_1: map_res!(opt!(multispace), from_utf8_option2) >>
        key: map_res!(alt!(alphanumeric | tag_s!("_")), str::from_utf8) >>
        separator: map_res!(multispace, str::from_utf8) >>
        value: map_res!(not_line_ending, str::from_utf8) >>
        whitespace_4: map_res!(opt!(complete!(multispace)), from_utf8_option2) >>
        ending: map_res!(opt!(complete!(line_ending)), from_utf8_option2) >>
        (Out::KeyValue{whitespace_1: whitespace_1, key: key.to_string(), whitespace_2: None, separator: separator.to_string(),
            whitespace_3: None, value: value.to_string(), whitespace_4: whitespace_4})
    )
);


#[test]
fn test_key_value() {
    let expected = expected_done(Out::KeyValue {
        whitespace_1: None,
        key: "key".to_string(),
        whitespace_2: Some(" ".to_string()),
        separator: "=".to_string(),
        whitespace_3: Some(" ".to_string()),
        value: "value".to_string(),
        whitespace_4: None
    });

    let input = &b"key = value"[..];
    let actual = key_value(input);
    assert_eq!(expected, actual);
}

#[test]
fn test_key_value_underscore_key() {
    let expected = expected_done(Out::KeyValue {
        whitespace_1: None,
        key: "key".to_string(),
        whitespace_2: Some(" ".to_string()),
        separator: "=".to_string(),
        whitespace_3: Some(" ".to_string()),
        value: "value".to_string(),
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
        key: "key".to_string(),
        whitespace_2: None,
        separator: " ".to_string(),
        whitespace_3: None,
        value: "value".to_string(),
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
                         key: "key0".to_string(),
                         whitespace_2: None,
                         separator: "    ".to_string(),
                         whitespace_3: None,
                         value: "value0".to_string(),
                         whitespace_4: Some("\n".to_string())
                     },
                     Out::KeyValue {
                         whitespace_1: None,
                         key: "key1".to_string(),
                         whitespace_2: None,
                         separator: "    ".to_string(),
                         whitespace_3: None,
                         value: "value1".to_string(),
                         whitespace_4: Some("\n".to_string())
                     },
                     Out::KeyValue {
                         whitespace_1: None,
                         key: "key2".to_string(),
                         whitespace_2: None,
                         separator: "    ".to_string(),
                         whitespace_3: None,
                         value: "value2".to_string(),
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

pub fn parse_file<'a>(mut file: File) -> Vec<Out>{
    let mut contents: Vec<u8> = Vec::new();
    file.read_to_end(&mut contents).unwrap();
    return keys_values(&contents[..]).unwrap().1;
}
