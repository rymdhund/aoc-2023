use std::collections::HashSet;

use aoc2023::coord::{Coord, Dir, CoordMap};

fn parse(path: &str) -> (Vec<Vec<char>>, Coord) {
    let map: Vec<Vec<char>> = std::fs::read_to_string(path).unwrap().trim().lines().map(|line| {
        line.chars().collect()
    }).collect();

    let start = map.iter().enumerate().find_map(|(y, row)| {
        row.iter().position(|&c| c == 'S').map(|x| Coord::new_u(x, y))
    }).unwrap();

    (map, start)
}


fn neigh(map: &Vec<Vec<char>>, pos: Coord) -> Vec<Coord> {
    (0..4).filter_map(|i| {
        let n = pos + Coord::dir(Dir::of_id(i));
        (map.contains(n) && *map.at(n) != '#').then_some(n)
    }).collect()
}

fn solve2b((map, start): &(Vec<Vec<char>>, Coord), steps: usize) -> usize {
    // assume:
    // - width == height
    // - no obstacles up left right or donw
    // - start is in center
    let w = map.len();
    let mid = w / 2;
    assert!(start.x_u() == mid && start.y_u() == mid);
    assert!(map.len() == map[0].len());

    let entry_points = vec![
        Coord::new_u(0, mid), // right
        Coord::new_u(0, 0), // down-right
        Coord::new_u(mid, 0), // down
        Coord::new_u(w-1, 0), // down-left
        Coord::new_u(w-1, mid), // left
        Coord::new_u(w-1, w-1), // up-left
        Coord::new_u(mid, w-1), // up
        Coord::new_u(0, w-1), // up-right
        Coord::new_u(mid, mid), // center
    ];

    let plots_for_steps_by_dir: Vec<Vec<usize>> = entry_points.iter().map(|&ep| calculate_plots_for_steps(map, ep)).collect();

    // maximum number of steps before a block is fully covered, given any entrypoint
    let max_steps_in_block = plots_for_steps_by_dir.iter().map(|pfs| pfs.len()).max().unwrap();

    // Start block
    let mut tot = get_plots_for_steps(&plots_for_steps_by_dir[8], steps);

    // Blocks straight up / down / left / right
    let n = straight_reach_blocks(w, steps);
    for i in 1..=n {
        for dir in 0..4 {
            let d = dir * 2; // straight
            let steps_to_block = steps_to_block_straight(w, i);
            let steps_in_block = steps - steps_to_block - 1;
            let plots = get_plots_for_steps(&plots_for_steps_by_dir[d], steps_in_block);
            tot += plots;
        }
    }

    // Diagonal blocks
    let n = diag_reach_blocks(w, steps);
    for i in 1..=n {
        let m = n - i + 1;
        for dir in 0..4 {
            let d = dir * 2 + 1; // diagonals
            let plots_for_steps = &plots_for_steps_by_dir[d];
            for j in (1..=m).rev() {
                let steps_to_block = steps_to_block_diag(w, i, j);
                let steps_in_block = steps - steps_to_block - 1;
                if steps_in_block > max_steps_in_block {
                    let oddish = j / 2;
                    let evenish = j - oddish;
                    // 1000 is just an arbitrary even number that is > max_steps_in_block
                    tot += oddish * get_plots_for_steps(plots_for_steps, i + 1 + 1000);
                    tot += evenish * get_plots_for_steps(plots_for_steps, i + 1000);
                    break;
                } else {
                    let plots = get_plots_for_steps(plots_for_steps, steps_in_block);
                    tot += plots;
                }
            }
        }
    }

    tot
}

// i > 0 and j > 0
fn steps_to_block_diag(w: usize, i: usize, j: usize) -> usize {
    let mid = w / 2;
    mid + mid + 1 + (i - 1 + j - 1) * w
}

fn steps_to_block_straight(w: usize, i: usize) -> usize {
    let mid = w / 2;
    mid + (i - 1) * w
}

fn diag_reach_blocks(w: usize, steps: usize) -> usize {
    (steps - 1) / w
}

fn straight_reach_blocks(w: usize, steps: usize) -> usize {
    let mid = w / 2;
    (steps + mid) / w
}

fn calculate_plots_for_steps(map: &Vec<Vec<char>>, entry_point: Coord) -> Vec<usize> {
    let mut coverage = [HashSet::new(), HashSet::new()];
    coverage[0].insert(entry_point);
    let mut buf: Vec<Coord> = vec![entry_point];

    let mut plots_for_steps = vec![1];
    for i in 1.. {
        let mut new: Vec<Coord> = vec![];
        while !buf.is_empty() {
            let next = buf.pop().unwrap();
            for step in neigh(map, next) {
                if coverage[i%2].insert(step) {
                    new.push(step);
                }
            }
        }
        plots_for_steps.push(coverage[i%2].len());
        if new.len() == 0 {
            return plots_for_steps;
        }
        buf = new;
    }
    panic!("unreachable")
}

fn get_plots_for_steps(plots_for_steps: &Vec<usize>, steps: usize) -> usize {
    if steps >= plots_for_steps.len() {
        let d = (steps - (plots_for_steps.len() - 1)) % 2;
        plots_for_steps[plots_for_steps.len() - 1 - d]
    } else {
        plots_for_steps[steps]
    }
}

fn main() {
    let inp = parse("inputs/day21.txt");
    println!("sol1: {}", solve2b(&inp, 64));
    println!("sol2: {}", solve2b(&inp, 26501365));
}