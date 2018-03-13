extern crate roselib;

use std::path::PathBuf;
use roselib::files::ZON;
use roselib::io::RoseFile;


#[test]
fn read_zon() {
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    root.push("data");

    let file = root.join("JGT01.ZON");
    let zon = ZON::from_path(&file).unwrap();

    println!("{:?}", zon);
}
