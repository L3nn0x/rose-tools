#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate image;
extern crate roselib;

use std::ffi::OsStr;
use std::fs::{File, create_dir_all};
use std::io::{Write, BufWriter};
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::ArgMatches;
use roselib::files::*;
use roselib::io::RoseFile;

mod errors;
use errors::*;

fn main() {
    let yaml = load_yaml!("main.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    // Setup output directory
    let out_dir = Path::new(matches.value_of("out_dir").unwrap());
    if let Err(e) = create_dir_all(&out_dir) {
        eprintln!("Error creating output directory {}: {}",
                  out_dir.to_str().unwrap_or(""),
                  e);
        exit(1);
    }

    // Run subcommands
    match matches.subcommand() {
        ("terrain", Some(ref matches)) => convert_terrain(matches),
        _ => {
            eprintln!("Subcommand not recognized");
            exit(1);
        }
    }

    /*
    // -- Setup input file
    let in_path = Path::new(matches.value_of("file").unwrap());
    let in_file = match File::open(in_path) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error opening input file: {}", e);
            exit(1);
        }
    };


    let mut out_filepath = PathBuf::from(out_dir);
    out_filepath.push(in_path.file_name().unwrap_or(OsStr::new("out.obj")));
    out_filepath.set_extension("obj");

    let out_file = match File::create(&out_filepath) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Error creating output file {}: {}",
                      out_filepath.to_str().unwrap_or(""),
                      e);
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
    */
}

fn convert_terrain(matches: &ArgMatches) {
    println!("{:?}", matches.value_of("file").unwrap());
    // TODO:
    // Check file exists
    // Get tile coordinates map from directory (e.g. tile[0][0] = "29_29")
    // Assemble heightmap image
    // Load ZON file
    // Convert tile section to JSON
    // Create N alphamap images for each tile index
    //
}

fn zms_to_obj(input: File, output: File) -> Result<()> {
    let mut writer = BufWriter::new(output);

    //let z = ZMS::from_reader(&mut reader)?;
    let z = ZMS::from_file(&input)?;

    writer
        .write(format!("# Exported using {} v{} ({})\n",
                       env!("CARGO_PKG_NAME"),
                       env!("CARGO_PKG_VERSION"),
                       env!("CARGO_PKG_HOMEPAGE"))
                       .as_bytes())?;

    // -- Write vertex data
    for v in &z.vertices {
        writer
            .write(format!("v {} {} {}\n", v.position.x, v.position.y, v.position.z).as_bytes())?;
    }

    for v in &z.vertices {
        writer
            .write(format!("vt {} {}\n", v.uv1.x, 1.0 - v.uv1.y).as_bytes())?;
    }

    for v in &z.vertices {
        writer
            .write(format!("vn {} {} {}\n", v.normal.x, v.normal.y, v.normal.z).as_bytes())?;
    }

    // -- Write face data
    for i in z.indices {
        writer
            .write(format!("f {x}/{x}/{x} {y}/{y}/{y} {z}/{z}/{z}\n",
                           x = i.x + 1,
                           y = i.y + 1,
                           z = i.z + 1)
                           .as_bytes())?;
    }

    Ok(())
}
