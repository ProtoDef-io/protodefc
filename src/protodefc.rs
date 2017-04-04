extern crate protodefc;

#[macro_use]
extern crate clap;
use clap::{Arg, App, SubCommand};

use std::fs::File;
use std::io::{Read, Write};

fn main() {
    let matches = clap_app!(
        protodefc =>
            (version: "0.0.1")
            (author: "Hans Elias B. Josephsen <me@hansihe.com>")
            (about: "Universal retargetable compiler for the protodef format")
            (@subcommand compile =>
             (about: "Compiles a single compilation unit")
             (@arg INPUT: +required "Sets the input file")
             (@arg OUTPUT: +required "Sets the output file")
            )
    ).get_matches();

    if let Some(ref matches) = matches.subcommand_matches("compile") {

        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();

        let mut input_file = File::open(input_file).unwrap();
        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str).unwrap();

        let cu = protodefc::spec_to_final_compilation_unit(&input_str).unwrap();
        let js = protodefc::backend::javascript::compilation_unit_to_javascript(&cu).unwrap();

        let mut output_file = File::create(output_file).unwrap();
        output_file.write(js.as_bytes());

    }

}
