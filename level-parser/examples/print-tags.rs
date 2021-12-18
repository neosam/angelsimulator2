//! An example so I can test the debug print messages I made while parsing the svg file.

use std::io::Read;
pub fn main() {
    let mut svg_content = String::new();
    std::fs::File::open("level-parser/resources/assembly-hall-1.svg")
        .expect("Could not open svg level")
        .read_to_string(&mut svg_content)
        .expect("Could not read svg level");
    let level = level_parser::parse_level_from_svg(&svg_content).unwrap();
    dbg!(level);
}
