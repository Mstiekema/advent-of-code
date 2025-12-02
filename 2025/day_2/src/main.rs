//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 2
/// Wrote on: 2025-12-02
/// Last modified: 2025-12-02
/// Written in: Rust
/// https://adventofcode.com/2025/day/2
//////////////////////////////////////////////////
use std::{fs, io::Write};
use fancy_regex::Regex;

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let ranges: std::str::Split<'_, &str> = contents.split(",");

    // Part 1 - Get invalid IDs
    let mut count: u16 = 0;
    let re = Regex::new(r"\b(\w+)(\1)+\b").expect("Valid regex to check the id");
    for range in ranges {
        let split = range.split("-");
        let start = split.nth(0);
        let end = split.nth(1);

        for entry in [start..end] {
            let id: i16 = entry.parse().expect("The ID should be an integer");

            // Check if ID is valid
            if re.is_match(id) {
                count += id;
            }
        }
    }

    // Part 2 - ???

    // Write the result
    println!("Result: {}", count);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}", count).expect("Should be able to write to output file");
}
