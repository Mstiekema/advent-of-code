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

    // Initialize the script
    let mut rotation: i16 = 50;
    let mut password_p1: u16 = 0;
    let mut password_p2: u16 = 0;
    let steps: std::str::Split<'_, &str> = contents.split("\n");

    // Write some output to test
    println!("The problem contains {} lines", steps.clone().count());

    // Loop through each line
    for step in steps {
        // Prepare the step
        let direction = step
            .get(..1)
            .expect("String should be atleast 1 character long");
        let count_str: &str = step
            .get(1..)
            .expect("There should be some amount after the direction");
        let count: i16 = count_str
            .parse::<i16>()
            .expect("The count should be an integer");

        // Execute the step
        let base = if direction == "R" {
            rotation + count
        } else {
            rotation - count
        };

        // Part 1: Increase the password if we end up at 0
        let prev_rotation = rotation;
        rotation = base.rem_euclid(100);
        if rotation == 0 {
            password_p1 += 1;
        }

        // Part 2: Also add to 0 for each pass
        let passes = (base / (100)).abs() as u16;

        if rotation == 0 {
            password_p2 += passes - 1;
        } else {
            password_p2 += passes;
            if base <= 0 { // Direction == L
                if prev_rotation != 0 {
                    password_p2 += 1;
                }
            } else if base >= 100 { // Direction == R
                password_p2 -= 1;
            }
        }

        // Print debugging step
        println!("{} | {} | {}", passes, rotation, base);

        // Wrap around if the result is negative
        if rotation < 0 {
            rotation = 100 + rotation;
        }
    }

    // Write the result
    println!("Final rotation: {}\nPasswords:\n1: {}\n2: {}", rotation, password_p1, password_p2);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}\n{}", password_p1, password_p2).expect("Should be able to write to output file");
}

// 5516 should be close
// 5604 should also be close
