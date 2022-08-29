use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 2 Part 1: {:?}", day_2_part_1("input.txt"));
    println!("Day 2 Part 2: {:?}", day_2_part_2("input.txt"));
}

fn day_2_part_1(filename: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;

    let file = File::open(filename).expect("Failed to open file.");
    BufReader::new(file)
        .lines()
        .map(|item| {
            item.expect("Failed to parse line")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|item| match item[0].as_str() {
            "forward" => horizontal += item[1].parse::<i32>().unwrap(),
            "down" => depth += item[1].parse::<i32>().unwrap(),
            "up" => depth -= item[1].parse::<i32>().unwrap(),
            _ => println!("lol"),
        });

    horizontal * depth
}

fn day_2_part_2(filename: &str) -> i32 {
    let mut horizontal = 0;
    let mut depth = 0;
    let mut aim = 0;

    let file = File::open(filename).expect("Failed to open file.");
    BufReader::new(file)
        .lines()
        .map(|item| {
            item.expect("Failed to parse line")
                .split(" ")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .for_each(|item| match item[0].as_str() {
            "forward" => {
                let x = item[1].parse::<i32>().unwrap();
                horizontal += x;
                depth += aim * x;
            }
            "down" => aim += item[1].parse::<i32>().unwrap(),
            "up" => aim -= item[1].parse::<i32>().unwrap(),
            _ => println!("lol"),
        });

    horizontal * depth
}

#[test]
fn day_2_part_1_example() {
    assert_eq!(150, day_2_part_1("example.txt"));
}

#[test]
fn day_2_part_2_example() {
    assert_eq!(900, day_2_part_2("example.txt"));
}
