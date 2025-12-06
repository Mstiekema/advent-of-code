//////////////////////////////////////////////////
/// Advent of Code 2025
/// Day 4
/// Wrote on: 2025-12-06
/// Last modified: 2025-12-06
/// Written in: Rust
/// https://adventofcode.com/2025/day/4
//////////////////////////////////////////////////
use std::{fs, io::Write};

fn main() {
    // Read the input
    let contents: String =
        fs::read_to_string("input1.txt").expect("Should be able to read the input file");
    let lines: Vec<&str> = contents.split("\n").collect();

    // Create base matrix with extended column
    let matrix: Vec<Vec<char>> = lines
        .iter()
        .map(|line| {
            let mut base = vec!['.'];
            let row: Vec<char> = line.chars().collect();
            base.extend(row);
            base.extend(vec!['.']);
            base
        })
        .collect();

    // Get the size of the matrix
    let row_count = lines.len() + 2;
    let col_count = lines[0].len() + 2;

    // Create full search matrix by pre- and appending empty rows
    let border_row = vec!['.'; col_count];
    let mut search_matrix = vec![border_row.clone()];
    search_matrix.extend(matrix);
    search_matrix.extend(vec![border_row]);

    // Part 1: Perform the search over 1..n-1
    let mut total_rolls = 0;
    for (y, row) in search_matrix.iter().enumerate() {
        if y == 0 || y == row_count - 1 {
            continue;
        }

        for (x, _col) in row.iter().enumerate() {
            if x == 0 || x == col_count - 1 {
                continue;
            }
            if search_matrix[y][x] == '.' {
                continue;
            }
            let neighbours = get_neighbours(&search_matrix, x, y);
            let rolls: Vec<&char> = neighbours.iter().filter(|spot| **spot == '@').collect();
            if rolls.len() < 4 {
                total_rolls += 1;
            }
        }
    }

    // Part 2: Do the same, but keep going until no more matches are found
    let mut search_matrix_p2 = search_matrix.clone();
    let mut total_rolls_p2 = 0;
    let mut prev_matches = -1;
    while prev_matches != 0 {
        prev_matches = 0;
        for (y, row) in search_matrix_p2.clone().iter().enumerate() {
            if y == 0 || y == row_count - 1 {
                continue;
            }

            for (x, _col) in row.iter().enumerate() {
                if x == 0 || x == col_count - 1 {
                    continue;
                }
                if &search_matrix_p2[y][x] == &'.' {
                    continue;
                }
                let neighbours = get_neighbours(&search_matrix_p2, x, y);
                let rolls: Vec<&char> = neighbours.iter().filter(|spot| **spot == '@').collect();
                if rolls.len() < 4 {
                    total_rolls_p2 += 1;
                    prev_matches += 1;
                    search_matrix_p2[y][x] = '.'
                }
            }
        }
    }

    // Write the result
    println!("Result: \nPart 1: {}\nPart 2: {}", total_rolls, total_rolls_p2);
    let mut file = fs::File::create("output").expect("Should be able to create output file");
    write!(&mut file, "{}\n{}", total_rolls, total_rolls_p2).expect("Should be able to write to output file");
}

fn get_neighbours(matrix: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<char> {
    let mut res: Vec<char> = vec![];
    for cx in x - 1..x + 2 {
        for cy in y - 1..y + 2 {
            if x == cx && y == cy {
                continue;
            };
            res.push(matrix[cy][cx]);
        }
    }
    return res;
}
