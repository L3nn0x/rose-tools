#[macro_use] extern crate clap;
#[macro_use] extern crate failure;
extern crate image;
extern crate roselib;

use std::f32;
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::iter;
use std::path::{Path, PathBuf};
use std::process::exit;

use clap::ArgMatches;
use failure::Error;
use image::{GrayImage, ImageBuffer};

use roselib::files::*;
use roselib::io::RoseFile;


fn main() {
    let yaml = load_yaml!("main.yaml");
    let matches = clap::App::from_yaml(yaml).get_matches();

    // Setup output directory
    let out_dir = Path::new(matches.value_of("out_dir").unwrap());
    if let Err(e) = fs::create_dir_all(&out_dir) {
        eprintln!("Error creating output directory {}: {}",
                  out_dir.to_str().unwrap_or(""),
                  e);
        exit(1);
    }

    // Run subcommands
    let res = match matches.subcommand() {
        ("map", Some(matches)) => convert_map(matches),
        _ => {
            eprintln!("ROSE Online Converter. Run with `--help` for more info.");
            exit(1);
        }
    };

    if let Err(e) = res {
        eprintln!("Error occured: {}", e);
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

/// Convert map files:
/// - ZON: JSON
/// - TIL: Combined into 1 JSON file
/// - IFO: Combined into 1 JSON file
/// - HIM: Combined into 1 greyscale png
fn convert_map(matches: &ArgMatches) -> Result<(), Error> {
    let map_dir = Path::new(matches.value_of("map_dir").unwrap());
    if !map_dir.is_dir() {
        bail!("Map path is not a directory: {:?}", map_dir);
    }

    println!("Loading map from: {:?}", map_dir);

    // Collect coordinates from file names (using HIM as reference)
    let mut x_coords: Vec<u32> = Vec::new();
    let mut y_coords: Vec<u32> = Vec::new();

    for f in fs::read_dir(map_dir)? {
        let f = f?;
        let fpath = f.path();
        if !fpath.is_file() {
            continue;
        }

        if fpath.extension().unwrap().to_str().unwrap().to_lowercase() == "him" {
            let fname = fpath.file_stem().unwrap().to_str().unwrap();
            let parts: Vec<&str> = fname.split("_").collect();
            x_coords.push(parts[0].parse()?);
            y_coords.push(parts[1].parse()?);
        }
    }

    x_coords.sort();
    y_coords.sort();

    let x_min = *x_coords.iter().min().unwrap();
    let x_max = *x_coords.iter().max().unwrap();
    let y_min = *y_coords.iter().min().unwrap();
    let y_max = *y_coords.iter().max().unwrap();

    let map_width = (x_max - x_min + 1) * 65;
    let map_height = (y_max - y_min + 1) * 65;

    let mut max_height = f32::NAN;
    let mut min_height = f32::NAN;

    //let mut heights: Vec<f32> = Vec::new();
    let mut heights: Vec<Vec<f32>> = Vec::new();
    heights.resize(
        map_height as usize,
        iter::repeat(f32::NAN).take(map_width as usize).collect()
    );

    for y in y_min..y_max+1 {
        for x in x_min..x_max+1 {
            let fname = format!("{}_{}.HIM", x, y);
            let fpath = map_dir.join(Path::new(&fname));

            //-- Load HIMs
            let him = HIM::from_path(&fpath).unwrap();
            if him.height != 65 || him.width != 65 {
                bail!("Unexpected HIM dimensions. Expected 65x65: {} ({}x{})",
                      &fpath.to_str().unwrap_or(&fname),
                      him.height,
                      him.width);
            }

            for h in 0..him.height {
                for w in 0..him.width {
                    let height = him.heights[h as usize][w as usize];

                    if (height > max_height) || (max_height.is_nan()) {
                        max_height = height;
                    }
                    if (height < min_height) || (min_height.is_nan()) {
                        min_height = height;
                    }

                    let new_x = ((x - x_min) * 65) + w as u32;
                    let new_y = ((y - y_min) * 65) + h as u32;

                    heights[new_y as usize][new_x as usize] = height;
                }
            }

            // TODO:
            // Load TIL data
            // Load IFO data
        }
    }

    // -- HIM
    let delta_height = max_height - min_height;

    let mut height_image: GrayImage = ImageBuffer::new(
        map_width,
        map_height,
    );

    for y in 0..map_height {
        for x in 0..map_width {
            let height = heights[y as usize][x as usize];

            let norm_height = |h| {
               (255.0 * ((h - min_height) / delta_height)) as u8
            };

            let pixel = image::Luma([norm_height(height)]);
            height_image.put_pixel(x, y, pixel);
        }
    }

    // TODO: Change this to outdir + map dir name
    height_image.save("test.png");

    // Load ZON file and export as JSON
    // Export TIL data as JSON
    // EXPORT IFO data as JSON

    Ok(())
}

/*
fn zms_to_obj(input: File, output: File) -> Result<(), Error> {
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
*/
