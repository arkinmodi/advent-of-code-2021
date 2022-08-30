use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    println!("Day 3 Part 1: {:?}", day_3_part_1("input.txt"));
    println!("Day 3 Part 2: {:?}", day_3_part_2("input.txt"));
}

fn day_3_part_1(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file.");
    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line"))
        .collect();

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

    let mut gamma_rate = "".to_owned(); // bit criteria: most common bit
    let mut epsilon_rate = "".to_owned(); // bit criteria: least common bit

    // Compute gamma_rate and epsilon_rate strings
    for c in count {
        if c.0 > c.1 {
            gamma_rate.push('0');
            epsilon_rate.push('1');
        } else {
            gamma_rate.push('1');
            epsilon_rate.push('0');
        }
    }

    i32::from_str_radix(&gamma_rate, 2).unwrap() * i32::from_str_radix(&epsilon_rate, 2).unwrap()
}

fn day_3_part_2(filename: &str) -> i32 {
    let file = File::open(filename).expect("Failed to open file.");
    let input: Vec<String> = BufReader::new(file)
        .lines()
        .map(|item| item.expect("Failed to parse line"))
        .collect();

    fn compute_digit_frequency_at_index(numbers: &Vec<String>, index: usize) -> (usize, usize) {
        let mut frequency = (0, 0);
        for binary in numbers {
            match binary.chars().nth(index).unwrap() {
                '0' => frequency.0 += 1,
                '1' => frequency.1 += 1,
                _ => panic!("Not a binary number"),
            }
        }
        frequency
    }

    let mut i = 0;
    let mut most_common: Vec<String> = input.clone();
    while most_common.len() > 1 {
        let frequency = compute_digit_frequency_at_index(&most_common, i);
        if frequency.0 > frequency.1 {
            most_common.retain(|binary| binary.chars().nth(i).unwrap() == '0');
        } else {
            most_common.retain(|binary| binary.chars().nth(i).unwrap() == '1');
        }
        i += 1;
    }
    let oxygen_generator_rating = &most_common[0]; // bit criteria: most common value

    i = 0;
    let mut least_common: Vec<String> = input.clone();
    while least_common.len() > 1 {
        let frequency = compute_digit_frequency_at_index(&least_common, i);
        if frequency.0 <= frequency.1 {
            least_common.retain(|binary| binary.chars().nth(i).unwrap() == '0');
        } else {
            least_common.retain(|binary| binary.chars().nth(i).unwrap() == '1');
        }
        i += 1;
    }
    let co2_scrubber_rating = &least_common[0]; // bit criteria: least common value

    i32::from_str_radix(oxygen_generator_rating, 2).unwrap()
        * i32::from_str_radix(co2_scrubber_rating, 2).unwrap()
}

#[test]
fn day_3_part_1_example() {
    assert_eq!(198, day_3_part_1("example.txt"));
}

#[test]
fn day_3_part_2_example() {
    assert_eq!(230, day_3_part_2("example.txt"));
}
