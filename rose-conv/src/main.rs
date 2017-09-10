#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate roselib;

use std::fs::{File, create_dir_all};
use std::io::{Read, Write};
use std::path::Path;
use std::process::exit;

use roselib::files::ZMS;

mod errors;
use errors::*;

fn main() {
    let yaml = load_yaml!("main.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    // -- Setup input file
    let in_path = Path::new(matches.value_of("file").unwrap());
    let in_file = match File::open(in_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening input file: {}", e);
            exit(1);
        }
    };

    // -- Setup output file
    let out_dir = Path::new(matches.value_of("out_dir").unwrap());
    if let Err(e) = create_dir_all(out_dir) {
        eprintln!("Error creating output directory: {}", e);
        exit(1);
    }
    let out_file = match File::open(out_dir) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating output file: {}", e);
            exit(1);
        }
    };

    // -- Do conversion
    let conv_res = match matches.subcommand_name() {
        Some("zms_to_obj") => zms_to_obj(in_file, out_file),
        _ => Err("Please provide a valid subcommand".into()),
    };

    // -- Handle conversion errors
    if let Err(e) = conv_res {
        eprintln!("Error converting the file: {}", e);
        exit(1);
    }
}

fn zms_to_obj<R: Read, W: Write>(input: R, output: W) -> Result<()> {
    // Create ZMS object from file
    // Create Output writer
    Ok(())
}
