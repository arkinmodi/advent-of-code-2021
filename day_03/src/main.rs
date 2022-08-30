use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 3 Part 1: {:?}", day_3_part_1("input.txt"));
}

fn day_3_part_1(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file.");
    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line"))
        .collect();

    // let length = input.len();
    // let width = input[0].len();

    // Initialize array
    let mut count: Vec<(i32, i32)> = Vec::new();
    for _ in 0..input[0].len() {
        count.push((0, 0));
    }

    // Populate count array
    for binary in input {
        for n in 0..binary.len() {
            match binary.chars().nth(n).unwrap() {
                '0' => count[n].0 += 1,
                '1' => count[n].1 += 1,
                _ => panic!("Not a binary number"),
            }
        }
    }

    let mut gamma = "".to_owned(); // most common bit
    let mut epsilon = "".to_owned(); // least common bit

    // Compute gamma and epsilon strings
    for c in count {
        if c.0 > c.1 {
            gamma.push('0');
            epsilon.push('1');
        } else {
            gamma.push('1');
            epsilon.push('0');
        }
    }

    i32::from_str_radix(&gamma, 2).unwrap() * i32::from_str_radix(&epsilon, 2).unwrap()
}

#[test]
fn day_3_part_1_example() {
    assert_eq!(198, day_3_part_1("example.txt"));
}
