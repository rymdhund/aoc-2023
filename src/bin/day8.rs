use std::{collections::HashMap, fs};

use nom::InputTake;

fn parse(filename: &str) -> (String, HashMap<String, (String, String)>) {
    let mut map = HashMap::new();

    let contents = fs::read_to_string(filename).unwrap();
    let parts: Vec<&str> = contents.trim().split("\n\n").collect();

    parts[1].lines().for_each(|line| {
        let a: Vec<&str> = line.split(" = (").collect();
        let b: Vec<&str> = a[1].split(", ").collect();
        map.insert(a[0].to_string(), (b[0].to_string(), b[1].take(3).to_string()));
    });

    (parts[0].to_string(), map)
}

fn solve1((dirs, map): &(String, HashMap<String, (String, String)>)) -> usize {
    let mut pos = "AAA";
    let dirs = dirs.as_bytes();
    let mut i = 0;
    while pos != "ZZZ" {
        if dirs[i % dirs.len()] == b'L' {
            pos = &map[pos].0;
        } else {
            pos = &map[pos].1;
        }
        i += 1;
    }
    i
}

fn parse2(filename: &str) -> (String, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>) {
    let mut ids = HashMap::new();
    let mut left = vec![];
    let mut right = vec![];
    let mut starts = vec![];
    let mut goals = vec![];

    let contents = fs::read_to_string(filename).unwrap();
    let parts: Vec<&str> = contents.trim().split("\n\n").collect();
    parts[1].lines().enumerate().for_each(|(i, line)| {
        let name: &str = line.split(" = (").next().unwrap();
        ids.insert(name, i);
        if name.ends_with('A') {
            starts.push(i);
        }
        if name.ends_with('Z') {
            goals.push(i);
        }
    });

    parts[1].lines().for_each(|line| {
        let a: Vec<&str> = line.split(" = (").collect();
        let b: Vec<&str> = a[1].split(", ").collect();
        left.push(ids[b[0]]);
        right.push(ids[b[1].take(3)]);
    });

    (parts[0].to_string(), starts, goals, left, right)
}

#[derive(Debug, Copy, Clone)]
struct Path {
    first_goal: usize, // How many steps till we first hit a goal in this cycle
    cycle: usize,  // How long is the cycle
}

// We can treat this problem as n subproblems for each index in `dirs`.
fn solve2((dirs, starts, goals, left, right): &(String, Vec<usize>, Vec<usize>, Vec<usize>, Vec<usize>)) -> usize {
    let dirs = dirs.as_bytes();

    for idx in 0..dirs.len() {
        let paths: Vec<Path> = starts.iter().filter_map(|start| {
            mk_path(*start, idx, dirs, left, right, goals)
        }).collect();

        if !paths.len() == starts.len() {
            // for this index some cycles didn't have a goal
            continue;
        }

        let mut p0 = paths[0];
        for p in &paths[1..] {
            p0 = combine(p0, *p);
        }
        return idx + p0.first_goal * dirs.len();
    }
    panic!("No solution found")
}

fn combine(p1: Path, p2: Path) -> Path {
    let first_goal = first_match(p1, p2);
    let cycle = num::integer::lcm(p1.cycle, p2.cycle);
    Path { first_goal, cycle }
}

fn first_match(c1: Path, c2: Path) -> usize {
    // Solving a diophantine equation by brute force. This is not guaranteed to finish, but for our input it does.
    let mut a1 = c1.first_goal;
    let mut a2 = c2.first_goal;

    while a1 != a2 {
        if a1 < a2 {
            a1 += c1.cycle;
        } else {
            a2 += c2.cycle;
        }
    }
    a1
}

fn mk_path(start: usize, i_start: usize, dirs: &[u8], left: &Vec<usize>, right: &Vec<usize>, goals: &Vec<usize>) -> Option<Path> {
    let mut cycle = vec![];
    let mut pos = start;
    let mut goal = None;

    // walk i_start steps
    (0..i_start).for_each(|j| pos = step(pos, j, dirs, left, right));

    let mut n = 0;
    while !cycle.contains(&pos) {
        cycle.push(pos);
        if goals.contains(&pos) {
            goal = Some(n);
        }
        n += 1;
        
        // Move a full sequence of dirs
        for j in 0..dirs.len() {
            pos = step(pos, i_start + j, dirs, left, right);
        }
    }
    let cycle_start = cycle.iter().position(|p| *p == pos).unwrap();
    let cycle = cycle.len() - cycle_start;
    goal.map(|first_goal| Path { first_goal, cycle })
}

fn step(pos: usize, i: usize, dirs: &[u8], left: &Vec<usize>, right: &Vec<usize>) -> usize {
    if dirs[i% dirs.len()] == b'L' {
        left[pos]
    } else {
        right[pos]
    }
}

fn main() {
    let ex1 = parse("inputs/day8_ex1.txt");
    let ex2 = parse("inputs/day8_ex2.txt");
    let inp = parse("inputs/day8.txt");
    let ex3 = parse2("inputs/day8_ex3.txt");
    let inp2 = parse2("inputs/day8.txt");

    println!("ex1: {}", solve1(&ex1));
    println!("ex2: {}", solve1(&ex2));
    println!("sol1: {}", solve1(&inp));

    println!("ex3: {}", solve2(&ex3));
    println!("sol2: {}", solve2(&inp2));
}