use std::{collections::HashSet, cmp::max};

fn parse(file: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn solve1(map: &Vec<Vec<char>>, start: (i32, i32, i32)) -> usize {
    let mut stack: Vec<(i32, i32, i32)> = vec![start];
    let mut seen = HashSet::new();
    while !stack.is_empty() {
        let (x, y, d) = stack.pop().unwrap();
        if x < 0 || y < 0 || x >= map[0].len() as i32 || y >= map.len() as i32 {
            continue;
        }
        if seen.contains(&(x, y, d)) {
            continue;
        }
        seen.insert((x, y, d));
        match map[y as usize][x as usize] {
            '.' => stack.push(step(x, y, d)),
            '\\' => {
                let d2 = match d {
                    0 => 1,
                    1 => 0,
                    2 => 3,
                    3 => 2,
                    _ => panic!("unexpected"),
                };
                stack.push(step(x, y, d2));
            },
            '/' => {
                let d2 = match d {
                    0 => 3,
                    1 => 2,
                    2 => 1,
                    3 => 0,
                    _ => panic!("unexpected"),
                };
                stack.push(step(x, y, d2));
            }
            '|' if d == 1 || d == 3 => stack.push(step(x, y, d)),
            '|' => {
                stack.push(step(x, y, (d + 1) % 4));
                stack.push(step(x, y, (d + 3) % 4));
            },
            '-' if d == 0 || d == 2 => stack.push(step(x, y, d)),
            '-' => {
                stack.push(step(x, y, (d + 1) % 4));
                stack.push(step(x, y, (d + 3) % 4));
            },
            _ => panic!("unexpected"),
        }
    }
    seen.iter().map(|&(x, y, d)| (x, y)).collect::<HashSet<(i32, i32)>>().len()
}

fn solve2(map: &Vec<Vec<char>>) -> usize {
    let row_max = (0..map.len()).map(|y| {
        let r = solve1(map, (0, y as i32, 0));
        let l = solve1(map, (map[0].len() as i32 -1, y as i32, 2));
        max(r, l)
    }).max().unwrap();
    let col_max = (0..map[0].len()).map(|x| {
        let r = solve1(map, (x as i32, 0, 1));
        let l = solve1(map, (x as i32, map.len() as i32 -1, 3));
        max(r, l)
    }).max().unwrap();
    max(row_max, col_max)
}

fn step(x: i32,y: i32, d: i32) -> (i32, i32, i32) {
    match d {
        0 => (x + 1, y, d),
        1 => (x, y + 1, d),
        2 => (x - 1, y, d),
        3 => (x, y - 1, d),
        _ => panic!("unexpected")
    }
}

fn main() {
    let ex = parse("inputs/day16_ex1.txt");
    let inp = parse("inputs/day16.txt");
    assert!(solve1(&ex, (0, 0, 0)) == 46);
    assert!(solve2(&ex) == 51);
    println!("sol1: {}", solve1(&inp, (0, 0, 0)));
    println!("sol2: {}", solve2(&inp));
}