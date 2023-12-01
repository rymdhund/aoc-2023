use std::fs;

fn get_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename).unwrap().trim().lines().map(|line| line.to_string()).collect()
}

fn solve1(lines: &Vec<String>) -> u32 {
    lines.iter()
        .map(|line| {
            let digits: Vec<u32> = line.chars().filter_map(|c| c.to_digit(10)).collect();
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum()
}

fn solve2(lines: &Vec<String>) -> u32 {
    lines.iter()
        .map(|line| {
            let digits: Vec<u32> = (0..line.len())
                .filter_map(|i| starting_digit(&line[i..]))
                .collect();
            digits[0] * 10 + digits.last().unwrap()
        })
        .sum()
}

fn starting_digit(input: &str) -> Option<u32> {
    let numbers = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

    let digit = input.chars().next().and_then(|c| c.to_digit(10));
    if digit.is_some() {
        digit
    } else {
        (0..numbers.len()).find_map(|i| {
            input
                .starts_with(numbers[i])
                .then_some((i+1) as u32)
        })
    }
}

fn main() {
    let example1 = get_lines("inputs/day1_ex1.txt");
    let example2 = get_lines("inputs/day1_ex2.txt");
    let input = get_lines("inputs/day1.txt");

    println!("example1: {}", solve1(&example1));
    println!("solution1: {}", solve1(&input));
    println!("example2: {}", solve2(&example2));
    println!("solution2: {}", solve2(&input));
}
