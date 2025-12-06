//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 5
/// Wrote on: 2025-12-06
/// Last modified: 2025-12-06
/// Written in: Rust
/// https://adventofcode.com/2025/day/5
//////////////////////////////////////////////////
use std::{fs, io::Write};
use std::collections::HashSet;

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let mut ranges_ids_split = contents.split("\n\n");
    let range_str = ranges_ids_split
        .nth(0)
        .expect("First part of the input should be a list of ranges");
    let id_str = ranges_ids_split
        .nth(0)
        .expect("Second part of the input should be a list of IDs");

    // Parse the ranges
    let range_list = range_str.split("\n");
    let mut base_ranges: Vec<std::ops::Range<u64>> = range_list.map(|range| {
        let mut range_info = range.split("-");
        let low = range_info
            .nth(0)
            .expect("There should be a start to the range")
            .parse::<u64>()
            .expect("Range start should be an integer");
        let high = range_info
            .nth(0)
            .expect("There should be a end to the range")
            .parse::<u64>()
            .expect("Range end should be an integer");

        low..high+1
    }).collect();

    // Fix overlapping ranges
    base_ranges.sort_by(|a, b| a.start.cmp(&b.start));
    let mut merged: Vec<std::ops::Range<u64>> = Vec::new();
    for range in base_ranges.clone() {
        if let Some(last) = merged.last_mut() {
            if last.end >= range.start {
                last.end = last.end.max(range.end);
            } else {
                merged.push(range);
            }
        } else {
            merged.push(range);
        }
    }
    let ranges: HashSet<std::ops::Range<u64>> = merged.into_iter().collect();

    // Check each ID to see whether its in any of the range
    let mut fresh = 0;
    let ids = id_str
        .split("\n")
        .map(|id| id.parse::<u64>().expect("ID should be a valid integer"));

    for id in ids {
        for range in base_ranges.clone() {
            if range.contains(&id) {
                fresh += 1;
                break;
            }
        }
    }

    // Part 2: Check total fresh ingredients
    let mut total_fresh: u64 = 0;
    for range in ranges.clone() {
        let cnt = range.clone().count();
        println!("Range: {:?} | Count: {}", range, cnt);
        total_fresh += cnt as u64;
    }

    // Write the result
    println!("Result\nPart 1: {}\nPart 2: {}", fresh, total_fresh);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "").expect("Should be able to write to output file");
}

// 323257644536867 = too low
// 323257644536938 = too low
