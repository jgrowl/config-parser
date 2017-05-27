extern crate clap;
use std::collections::HashMap;
use clap::{Arg, App};

use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

extern crate linked_hash_map;
use linked_hash_map::LinkedHashMap;

extern crate slapd_parser;
use slapd_parser::types::*;

fn main() {
    let matches = App::new("varconfig")
        .version("0.0.1")
        .author("Jonathan Rowlands <jonrowlands83@gmail.com>")
        .about("Configure unix configuration files with variables!")
        .args_from_usage("
            -o, --output=[FILE]   'Sets an output file'
            -t, --template=[FILE] 'Sets a template base to work from'
            -v...                 'Sets the level of verbosity'
            [INPUT]...
            ")
        .get_matches();

//    "-c, --config=[FILE] 'Sets a custom config file'
//    <INPUT>              'Sets the input file to use'

    let input_vars = parse_input_vars(matches.values_of("INPUT"));
    let output = parse_output(matches.value_of("output"));
    let mut config: Config = Config::new();
//    config.input_variables = input_vars;

    if let Some(o) = matches.value_of("template") {
        let mut file = File::open(o).expect("Unable to open the file");
        let mut config = Config {
            template_lines: Some(slapd_parser::parse_file(file)),
            input_variables: input_vars,
            output_file: output

        };

        config.output();
    }

//    // Vary the output based on how many times the user used the "verbose" flag
//    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
//    match matches.occurrences_of("v") {
//        0 => println!("No verbose info"),
//        1 => println!("Some verbose info"),
//        2 => println!("Tons of verbose info"),
//        3 | _ => println!("Don't be crazy"),
//    }
}

fn parse_output(output_path: Option<&str>) -> Option<File> {
    match output_path {
        None => return None,
        Some(file_path) => {
            let f = File::open(file_path).expect("Unable to open the file");
            return Some(f)
        }
    }
}

fn parse_input_vars(input: Option<clap::Values>) -> LinkedHashMap<String, String> {
    match input {
        None => return LinkedHashMap::new(),
        Some(kv_strings) => {
            kv_strings
                .map(|kv_string| kv_string.split('='))
                .map(|mut kv| (kv.next().unwrap().into(),
                               kv.next().unwrap().into()))
                .collect::<LinkedHashMap<String, String>>()
        }
    }
}
