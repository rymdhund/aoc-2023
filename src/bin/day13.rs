use std::{fs, cmp::min};

fn parse(file: &str) -> Vec<Vec<Vec<char>>> {
    fs::read_to_string(file).unwrap().trim().split("\n\n").map(|part| {
        part.lines().map(|line| line.chars().collect()).collect()
    }).collect()
}

fn find_reflection(grid: &Vec<Vec<char>>, smears: usize) -> usize {
    let width = grid[0].len();
    let height = grid.len();
    for i in 0..width-1 {
        let diffs: usize = (0..min(width-i-1, i+1)).map(|j| {
            let a = i - j;
            let b = i + j + 1;
            let diff = (0..height).filter(|&y| grid[y][a] != grid[y][b]).count();
            diff
        }).sum();
        if diffs == smears {
            return i + 1;
        }
    }
    for i in 0..height-1 {
        let diffs: usize = (0..min(height-i-1, i+1)).map(|j| {
            let a = i - j;
            let b = i + j + 1;
            let diff = (0..width).filter(|&x| grid[a][x] != grid[b][x]).count();
            diff
        }).sum();
        if diffs == smears {
            return (i + 1) * 100;
        }
    }
    panic!("unexpected");
}

fn solve(inp: &Vec<Vec<Vec<char>>>, smears: usize) -> usize {
    inp.iter().map(|grid| find_reflection(grid, smears)).sum()
}

fn main() {
    let ex = parse("inputs/day13_ex1.txt");
    let inp = parse("inputs/day13.txt");
    assert!(solve(&ex, 0) == 405);
    assert!(solve(&ex, 1) == 400);
    println!("sol1: {}", solve(&inp, 0));
    println!("sol2: {}", solve(&inp, 1));
}