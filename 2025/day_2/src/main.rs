//////////////////////////////////////////////////
use fancy_regex::Regex;
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

    // Regex for matching in part 2
    let re = Regex::new(r"\b(\w+)(\1)+\b").expect("Valid regex to check the id");

    // Initialize looping
    let mut count_p1: u64 = 0;
    let mut count_p2: u64 = 0;
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

        for id in start..end + 1 {
            // Part 1 - Get invalid IDs by splitting
            let id_str = &id.to_string();
            let id_div = id_str.len() / 2;

            // Only len of 2n can be invalid
            if id_str.len() % 2 == 0 {
                let id_start = &id_str[0..id_div];
                let id_end = &id_str[id_div..id_str.len()];
                if id_start == id_end {
                    count_p1 += id;
                    println!("Invalid ID - P1: {}", id);
                }
            }

            // Part 2 - Use regex to see whether the ID is invalid
            let match_re = re
                .is_match(id_str)
                .expect("Should be boolean outcome on the regex match");
            if match_re {
                count_p2 += id;
                println!("Invalid ID - P2: {}", id);
            }
        }
    }

    // Part 2 - Use

    // Write the result
    println!("Result:\nPart 1: {}\nPart 2: {}", count_p1, count_p2);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}\n{}", count_p1, count_p2)
        .expect("Should be able to write to output file");
}
