// I heavily optimized the dfs search in order to make it run fast enough using edgelist and a bitmask for the visited nodes
use std::collections::HashMap;

use aoc2023::coord::{Coord, Dir, CoordMap, Bitmask};
use itertools::Itertools;

fn parse(file: &str) -> Vec<Vec<char>> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        line.chars().collect_vec()
    }).collect_vec()
}

#[derive(Debug, Clone, Copy)]
struct Path {
    len: usize,
    end: usize,
}

fn solve(map: &Vec<Vec<char>>) -> (usize, usize) {
    let startx = map[0].iter().position(|&c| c == '.').unwrap();
    let start = Coord::new_u(startx, 0);

    let goalx = map[map.len()-1].iter().position(|&c| c == '.').unwrap();
    let goal = Coord::new_u(goalx, map.len()-1);

    let start_idx = 0;
    let mut node_idx = HashMap::new();
    node_idx.insert(start, start_idx);

    let mut non_slope_paths = vec![vec![]];
    let mut all_paths = vec![vec![]];

    let mut unhandled = vec![start];
    while !unhandled.is_empty() {
        let pos = unhandled.pop().unwrap();
        let idx = node_idx[&pos];

        let mut non_slope = vec![];
        let mut all = vec![];

        let neigh = pos.neighbours().into_iter()
            .filter(|&p| map.contains(p) && *map.at(p) != '#').collect_vec();

        for n in neigh {
            let (end, cost, has_slope) = find_path(map, pos, n);
            if !node_idx.contains_key(&end) {
                node_idx.insert(end, node_idx.len());
                unhandled.push(end);
                non_slope_paths.push(vec![]);
                all_paths.push(vec![]);
            }

            let path = Path { end: node_idx[&end], len: cost };
            if !has_slope {
                non_slope.push(path);
            }
            all.push(path);
        }
        non_slope_paths[idx] = non_slope;
        all_paths[idx] = all;
    }
    let goal_idx = node_idx[&goal];

    let mut seen = Bitmask::new(node_idx.len());
    seen.add(start_idx);
    let sol1 = dfs(&non_slope_paths, start_idx, goal_idx, &mut seen, false).unwrap();
    let sol2 = dfs(&all_paths, start_idx, goal_idx, &mut seen, true).unwrap();
    (sol1, sol2)
}

fn find_path(map: &Vec<Vec<char>>, prev: Coord, start: Coord) -> (Coord, usize, bool) {
    let mut seen = 1;
    let mut prev = prev;
    let mut pos = start;
    let mut slope = false;

    loop {
        let c = *map.at(pos);
        if (c == '>' && prev == pos.go(Dir::Right)) ||
            (c == '<' && prev == pos.go(Dir::Left)) ||
            (c == 'v' && prev == pos.go(Dir::Down)) ||
            (c == '^' && prev == pos.go(Dir::Up))
        {
            slope = true;
        }
        let neigh = pos.neighbours().into_iter()
            .filter(|&p| map.contains(p) && *map.at(p) != '#')
            .filter(|&p| p != prev).collect_vec();
        if neigh.len() != 1 {
            return (pos, seen, slope)
        }
        prev = pos;
        pos = neigh[0];
        seen += 1;
    }
}

fn dfs(paths: &Vec<Vec<Path>>, start: usize, end: usize, seen: &mut Bitmask, ignore_slopes: bool) -> Option<usize> {
    if start == end {
        return Some(0);
    }
    let mut max = 0;
    for path in paths[start].iter() {
        if !seen.contains(path.end) {
            seen.add(path.end);
            if let Some(cost) = dfs(paths, path.end, end, seen, ignore_slopes) {
                let cost = cost + path.len;
                if cost >= max {
                    max = cost;
                }
            }
            seen.rm(path.end);
        }
    }
    (max > 0).then_some(max)
}

fn main() {
    let test = parse("inputs/day23_ex1.txt");
    let (test1, test2) = solve(&test);
    assert!(test1 == 94);
    assert!(test2 == 154);

    let inp = parse("inputs/day23.txt");
    let (sol1, sol2) = solve(&inp);
    println!("sol1: {}", sol1);
    println!("sol2: {}", sol2);
}