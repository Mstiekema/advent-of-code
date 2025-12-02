//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 2
/// Wrote on: 2025-12-02
/// Last modified: 2025-12-02
/// Written in: Rust
/// https://adventofcode.com/2025/day/1
//////////////////////////////////////////////////
use std::{fs, io::Write};

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input3.txt").expect("Should be able to read the input file");

    // Write the result
    println!("Result");
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "").expect("Should be able to write to output file");
}
