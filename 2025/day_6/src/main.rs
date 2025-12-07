//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 6
/// Wrote on: 2025-12-06
/// Last modified: 2025-12-06
/// Written in: Rust
/// https://adventofcode.com/2025/day/6
//////////////////////////////////////////////////
use std::{fs, io::Write};

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input1.txt").expect("Should be able to read the input file");
    let rows: Vec<&str> = contents.split("\n").collect();
    let arith_row = rows.last();

    let mut matrix = rows.iter().map(|row| row.split_whitespace());
    let formula_count = matrix.clone().next().iter().count();
    let rows = matrix.clone().count();

    // Resolve all formula's
    let mut total_count = 0;
    for col in 0..formula_count {
        let mut nums = vec![];
        for row in 0..rows - 1 {
            nums.push(
                matrix.clone().into_iter().nth(row).expect("Get a number") // Parse to int!!
            );
        }

        let mut col_count = 0;
        let op = arith_row.iter().nth(0).expect("Should be an operator");
        if op == &&"+" {
            col_count = nums.iter().fold(0, |acc, x| {
                acc + x
                    .nth(0)
                    .expect("There should be a number at the next spot")
                    .parse::<u32>()
                    .expect("The next number should be an integer")
            })
        } else {
            col_count = nums.iter().fold(1, |acc, x| {
                acc * x
                    .nth(0)
                    .expect("There should be a number at the next spot")
                    .parse::<u32>()
                    .expect("The next number should be an integer")
            })
        }
        total_count += col_count
    }

    // Write the result
    println!("Result\nPart 1: {}", total_count);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}", total_count).expect("Should be able to write to output file");
}
