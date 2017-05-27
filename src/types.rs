use linked_hash_map::LinkedHashMap;
use std::fs::File;

#[derive(Debug,PartialEq,Clone)]
pub enum Out {
    Comment {
        whitespace_1: Option<String>,
        separator: String,
        text: Option<String>,
    },

    KeyValue {
        whitespace_1: Option<String>,
        key: String,
        whitespace_2: Option<String>,
        separator: String,
        whitespace_3: Option<String>,
        value: String,
        whitespace_4: Option<String>
    }
}


#[derive(Debug)]
pub struct Config {
    pub template_lines: Option<Vec<Out>>,
    pub input_variables: LinkedHashMap<String, String>,
    pub output_file: Option<File>
}


impl Config {

    pub fn new() -> Config {
        return Config{template_lines: None, input_variables: LinkedHashMap::new(), output_file: None};
    }

    pub fn output(&mut self) {
        for line in self.template_lines.clone().unwrap() {
            match line {
                Out::Comment { whitespace_1, separator, text } => {
                    println!("{}", text.unwrap());
                },
                Out::KeyValue { whitespace_1, key, whitespace_2, separator, whitespace_3, value, whitespace_4 } => {
                    println!("`{:?}` `{:?}`", key, value);
                }
            }
        }
    }
}
