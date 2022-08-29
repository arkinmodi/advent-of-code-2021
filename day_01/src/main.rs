use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 1 Part 1: {:?}", day_1_part_1("input.txt"));
    println!("Day 1 Part 2: {:?}", day_1_part_2("input.txt"));
}

fn day_1_part_1(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file.");
    let input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line").parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    for i in 1..input.len() {
        if input[i - 1] < input[i] {
            count += 1;
        }
    }
    count
}

fn day_1_part_2(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file.");
    let input: Vec<i32> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line").parse::<i32>().unwrap())
        .collect();

    let mut count = 0;
    for i in 3..input.len() {
        if input[i - 3] < input[i] {
            count += 1;
        }
    }
    count
}

#[test]
fn day_1_part_1_example() {
    assert_eq!(7, day_1_part_1("example.txt"));
}

#[test]
fn day_1_part_2_example() {
    assert_eq!(5, day_1_part_2("example.txt"));
}
