use std::fs;

fn parse(filename: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(filename).unwrap().trim().lines()
        .map(|line| line.split(' ').map(|n| n.parse::<i32>().unwrap()).collect())
        .collect()
}

fn solve1(numbers: &Vec<Vec<i32>>) -> i32 {
    numbers.iter().map(|xs| {
        let mut xs = xs.clone();
        for i in 1..xs.len() {
            for j in 0..(xs.len()-i) {
                xs[j] = xs[j+1] - xs[j];
            }
        }
        xs.iter().sum::<i32>()
    }).sum()
}

fn main() {
    let mut numbers = parse("inputs/day9.txt");
    println!("sol1: {}", solve1(&numbers));
    numbers.iter_mut().for_each(|xs| xs.reverse());
    println!("sol2: {}", solve1(&numbers));
}