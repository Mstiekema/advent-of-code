//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 2
/// Wrote on: 2025-12-02
/// Last modified: 2025-12-02
/// Written in: Rust
/// https://adventofcode.com/2025/day/2
//////////////////////////////////////////////////
use std::{fs, io::Write};

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let ranges: std::str::Split<'_, &str> = contents.split(",");

    // Part 1 - Get invalid IDs
    let mut count: u64 = 0;
    for range in ranges {
        let mut split = range.split("-");

        let start: u64 = split
            .nth(0)
            .expect("There should be a start id")
            .parse::<u64>()
            .expect("The ID should be an integer");
        let end: u64 = split
            .nth(0)
            .expect("There should be an end id")
            .parse::<u64>()
            .expect("The ID should be an integer");

        for id in start..end+1 {
            let id_str = id.to_string();
            let id_div = id_str.len() / 2;

            // Only len of 2n can be invalid
            if id_str.len() % 2 == 0 {
                let id_start = &id_str[0..id_div];
                let id_end = &id_str[id_div..id_str.len()];
                if id_start == id_end {
                    count += id;
                    println!("Invalid ID: {}", id);
                }
            }
        }
    }

    // Part 2 - ???

    // Write the result
    println!("Result: {}", count);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}", count).expect("Should be able to write to output file");
}
