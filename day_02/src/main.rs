use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 2 Part 1: {:?}", day_2_part_1("input.txt"));
}

// struct Command {
//     direction: &str,
//     amount: i32,
// }

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

#[test]
fn day_2_part_1_example() {
    assert_eq!(150, day_2_part_1("example.txt"));
}
