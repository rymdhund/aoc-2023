//
// Trying out the nom parser library
//
use std::str::FromStr;
use std::{fs, str::Lines};

use nom::{
    IResult,
    combinator::map_res,
    bytes::complete::tag,
    character::complete::{
        digit1,
        alpha1,
    },
    multi::separated_list1,
    sequence::{
        preceded,
        terminated,
        separated_pair,
        tuple
    }
};

fn parse_num<T: FromStr>(input: &str) -> IResult<&str, T> {
    map_res(digit1, |n: &str| n.parse::<T>())(input)
}

fn parse_color(input: &str) -> IResult<&str, usize> {
    map_res(alpha1, |s| {
        match s {
        "red" => Ok(0),
        "green" => Ok(1),
        "blue" => Ok(2),
        _ => Err(())
    }})(input)
}

fn parse_pair(input: &str) -> IResult<&str, (i32, usize)> {
    separated_pair(parse_num, tag(" "), parse_color)(input)
}

fn parse_game(input: &str) -> IResult<&str, Vec<(i32, usize)>> {
    separated_list1(tag(", "), parse_pair)(input)
}

fn parse_games(input: &str) -> IResult<&str, (usize, Vec<Vec<(i32, usize)>>)> {
    let parse_n = terminated(
        preceded(
            tag("Game "),
            parse_num::<usize>
        ),
        tag(": ")
    );
    let parse_games = separated_list1(tag("; "), parse_game);
    tuple((parse_n, parse_games))(input)
}

fn solve1(lines: Lines, limits: Vec<i32>) -> usize {
    lines.filter_map(|line| {
        let (rest, (n, games)) = parse_games(line).unwrap();
        assert!(rest.is_empty());

        let is_possible = games.iter().all(|colors| {
            colors.iter().all(|(n, c)| *n <= limits[*c])
        });

        is_possible.then_some(n)
    }).sum()
}

fn solve2(lines: Lines) -> i32 {
    lines.map(|line| {
        let (_, (_, games)) = parse_games(line).unwrap();
        power(games)
    }).sum()
}

fn power(games: Vec<Vec<(i32, usize)>>) -> i32 {
    (0..3).map(|color| {
        let max_num_for_color = games.iter()
            .flat_map(|colors| {
                colors.iter().filter_map(|(n, c)| (*c == color).then_some(n))
            })
            .max().unwrap();
        max_num_for_color
    }).product()
}

fn main() {
    let example1 = fs::read_to_string("inputs/day2_ex1.txt").unwrap();
    let input = fs::read_to_string("inputs/day2.txt").unwrap();

    println!("example1: {}", solve1(example1.lines(), vec![12, 13, 14]));
    println!("problem1: {}", solve1(input.lines(), vec![12, 13, 14]));
    println!("example2: {}", solve2(example1.lines()));
    println!("problem2: {}", solve2(input.lines()));
}
