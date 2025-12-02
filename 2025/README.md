# 2025
This year, I am writing the Advent of Code in Rust

## How to create a new entry
1. Go to the `2025/` folder
2. Run `cargo new day_X` for the given day
3. Create an input.txt file with the input for the day
4. Write your code in the `src/main.rs` file

## How to run
1. Enter the folder of the day you want to run
2. Run `cargo run`
3. Check the `output` file and/or the cli to see the result

## Template
```rust
//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 1
/// Wrote on: 2025-12-01
/// Last modified: 2025-12-01
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
```
