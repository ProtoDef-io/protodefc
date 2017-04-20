extern crate protodefc;
use protodefc::errors::*;

#[macro_use]
extern crate clap;

use std::fs::File;
use std::io::{Read, Write};

arg_enum! {
    #[derive(Debug)]
    pub enum CompileTarget {
        Javascript,
        Rust
    }
}

fn main() {
    let matches = clap_app!(
        protodefc =>
            (version: "0.0.1")
            (author: "Hans Elias B. Josephsen <me@hansihe.com>")
            (about: "Universal retargetable compiler for the protodef format")
            (@subcommand compile =>
             (about: "Compiles a single compilation unit")
             (@arg TARGET: +required "Specifies the compilation target")
             (@arg INPUT: +required "Sets the input file")
             (@arg OUTPUT: +required "Sets the output file")
            )
            (@subcommand old_protodef_to_pds =>
             (about: "Converts a protocol.json in the old format into a (likely invalid) PDS file.")
             (@arg INPUT: +required "Sets the input file")
             (@arg OUTPUT: +required "Sets the output file")
            )
    ).get_matches();

    let result = ::std::panic::catch_unwind(|| run(&matches));

    use ::std::io::Write;
    let stderr = &mut ::std::io::stderr();
    let errmsg = "Error writing to stderr";

    match result {
        Ok(inner) => {
            if let Err(ref e) = inner {

                writeln!(stderr, "").expect(errmsg);

                writeln!(stderr, "traceback (most recent call last):").expect(errmsg);
                for e in e.iter() {
                    writeln!(stderr, "- {}", e).expect(errmsg);
                }

                writeln!(stderr, "").expect(errmsg);

                if let Some(backtrace) = e.backtrace() {
                    writeln!(stderr, "{:?}", backtrace).expect(errmsg);
                    writeln!(stderr, "").expect(errmsg);
                } else {
                    writeln!(stderr, "Run with RUST_BACKTRACE=1 to get backtrace.").expect(errmsg);
                }

                writeln!(stderr, "Compilation failed.").expect(errmsg);
                ::std::process::exit(1);
            }
        },
        Err(_) => {
            writeln!(stderr, "====================================================").expect(errmsg);
            writeln!(stderr, "========= PANIC IN COMPILER, THIS IS A BUG! ========").expect(errmsg);
            writeln!(stderr, "====================================================").expect(errmsg);
            writeln!(stderr, "Please open an issue and include the steps to reproduce.").expect(errmsg);
            ::std::process::exit(1);
        }
    }
}

fn run(matches: &clap::ArgMatches) -> Result<()> {
    if let Some(ref matches) = matches.subcommand_matches("compile") {
        let target = value_t!(matches, "TARGET", CompileTarget)
            .unwrap_or_else(|e| e.exit());

        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();

        let mut input_file = File::open(input_file).unwrap();
        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str).unwrap();

        let cu = protodefc::spec_to_final_compilation_unit(&input_str)?;

        let out = match target {
            CompileTarget::Javascript =>
                protodefc::backend::javascript::compilation_unit_to_javascript(&cu)?,
            CompileTarget::Rust =>
                protodefc::backend::rust::compilation_unit_to_rust(&cu)?,
        };

        let mut output_file = File::create(output_file).unwrap();
        output_file.write(out.as_bytes()).unwrap();

    }

    if let Some(ref matches) = matches.subcommand_matches("old_protodef_to_pds") {

        let input_file = matches.value_of("INPUT").unwrap();
        let output_file = matches.value_of("OUTPUT").unwrap();

        let mut input_file = File::open(input_file).unwrap();
        let mut input_str = String::new();
        input_file.read_to_string(&mut input_str).unwrap();

        let out = protodefc::old_protocol_json_to_pds::convert(&input_str)?;

        let mut output_file = File::create(output_file).unwrap();
        output_file.write(out.as_bytes()).unwrap();

    }

    Ok(())
}
