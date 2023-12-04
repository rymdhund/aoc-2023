use std::fs;
use winnow::{
    ascii::digit1,
    combinator::{repeat, preceded},
    Parser,
    PResult,
    token::take_while
};

fn parse(filename: &str) -> Vec<(Vec<i32>, Vec<i32>)> {
    fs::read_to_string(filename).unwrap().trim().lines()
        .map(|line| {
            parse_line(line).unwrap()
        }).collect()
}

fn ws<'a>(input: &mut &'a str) -> PResult<&'a str> {
    take_while(0.., ' ').parse_next(input)
}

fn parse_number(input: &mut &str) -> PResult<i32> {
    preceded(
        ws,
        digit1
    ).map(|digits: &str| digits.parse::<i32>().unwrap()).parse_next(input)
}

fn parse_line(line: &str) -> PResult<(Vec<i32>, Vec<i32>)>{
    let mut input = line;
    let _ = "Card ".parse_next(&mut input)?;
    let _ = parse_number(&mut input)?;
    let _ = ":".parse_next(&mut input)?;
    let winning = repeat(1.., parse_number).parse_next(&mut input)?;
    let _ = " |".parse_next(&mut input)?;
    let numbers = repeat(1.., parse_number).parse_next(&mut input)?;
    Ok((winning, numbers))
}

fn matches(winning: &Vec<i32>, numbers: &Vec<i32>) -> usize {
    numbers.iter().filter(|n| winning.contains(n)).count()
}

fn solve1(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    cards.iter().map(|(winning, numbers)| {
        let c = matches(winning, numbers) as u32;
        if c == 0 {
            0
        } else {
            2i32.pow(c - 1)
        }
    }).sum()
}

// Sum the values from the end
fn solve2(cards: &Vec<(Vec<i32>, Vec<i32>)>) -> i32 {
    let mut values: Vec<i32> = vec![0i32; cards.len()];
    for (i, (winning, numbers)) in cards.iter().rev().enumerate() {
        let c = matches(winning, numbers);
        values[i] = 1 + (1..=c).map(|n| values[i - n]).sum::<i32>();
    };
    values.iter().sum()
}

fn main() {
    let example1 = parse("inputs/day4_ex1.txt");
    let input = parse("inputs/day4.txt");

    println!("example1: {}", solve1(&example1));
    println!("solution1: {}", solve1(&input));
    println!("example2: {}", solve2(&example1));
    println!("solution2: {}", solve2(&input));
}