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
        fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let rows: Vec<&str> = contents.split("\n").collect();

    let matrix = rows.iter().map(|row| row.split_whitespace());
    let mut arith_row = matrix
        .clone()
        .last()
        .expect("There should be a last operation row")
        .map(|op| op.to_string());
    let formula_count = arith_row.clone().count();
    let rows = matrix.clone().count();

    println!("Formula count: {}", formula_count);

    // Resolve all formula's
    let mut total_count = 0;
    for col in 0..formula_count {
        let mut nums = vec![];
        for row in 0..rows-1 {
            nums.push(
                matrix
                    .clone()
                    .nth(row)
                    .expect("Should be a column")
                    .nth(col)
                    .expect("Should be an entry in the row")
                    .parse::<u64>()
                    .expect("Each entry should be a number")
            );
        }

        let col_count;
        let op = arith_row.nth(0).expect("Should be an operator");
        if op == "+" {
            col_count = nums.iter().fold(0, |acc, x| {
                acc + *x
            })
        } else {
            col_count = nums.iter_mut().fold(1, |acc, x| {
                acc * (*x)
            })
        }
        total_count += col_count
    }

    // Write the result
    println!("Result\nPart 1: {}", total_count);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}", total_count).expect("Should be able to write to output file");
}
