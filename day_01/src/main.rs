use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 1 Part 1: {:?}", day_1_part_1("input_part_1.txt"));
}

fn day_1_part_1(input_part1_filename: &str) -> i32 {
    let file = File::open(input_part1_filename).expect("Failed to open file.");
    let input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line"))
        .map(|item| item.parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

#[test]
fn day_1_part_1_example() {
    assert_eq!(day_1_part_1("example_part_1.txt"), 7);
}
