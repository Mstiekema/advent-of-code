//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 3
/// Wrote on: 2025-12-03
/// Last modified: 2025-12-03
/// Written in: Rust
/// https://adventofcode.com/2025/day/3
//////////////////////////////////////////////////
use std::{fs, io::Write};

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input.txt").expect("Should be able to read the input file");
    let banks = contents.split("\n");

    // Part 1: Loop over each bank
    let mut total: u32 = 0;
    let mut total_p2: u64 = 0;
    for bank in banks {
        let mut highest: u8 = 0;
        let mut second: u8 = 0;
        let bank_length = bank.len();

        // Check each battery in the bank
        for (n, battery) in bank.char_indices() {
            let joltage = battery
                .to_digit(10)
                .expect("Battery joltage should be an integer") as u8;

            if n < bank_length - 1 && joltage > highest {
                highest = joltage;
                second = 0;
            } else if joltage > second {
                second = joltage;
            }
        }

        // Add the joltage of the bank to the total
        let bank_joltage: String = "".to_owned() + &highest.to_string() + &second.to_string();
        total += bank_joltage
            .parse::<u32>()
            .expect("Bank joltage should be able to be parsed to an integer");

        // Part 2: Check for a 12 battery length
        let mut total_bank = vec![0; 12];

        // Check each battery in the bank
        for (n, battery) in bank.char_indices() {
            let joltage = battery
                .to_digit(10)
                .expect("Battery joltage should be an integer") as u8;

            let mut updated = false;
            for x in 0..12 {
                if n < bank_length - (11 - x)
                    && joltage > total_bank[x]
                    // && (x == 0 || joltage < total_bank[x - 1])
                    && !updated
                {
                    total_bank[x] = joltage;
                    total_bank[x+1..12].copy_from_slice(&vec![0; 11 - x]);
                    updated = true
                }
            }
        }

        // Add the joltage of the bank to the total
        let bank_joltage: String = total_bank
            .iter()
            .fold(String::from(""), |acc: String, x| acc + &x.to_string());
        total_p2 += bank_joltage
            .parse::<u64>()
            .expect("Bank joltage should be able to be parsed to an integer");
    }

    // Write the result
    println!("Result:\nPart 1: {}\nPart 2: {}", total, total_p2);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}\n{}", total, total_p2).expect("Should be able to write to output file");
}
