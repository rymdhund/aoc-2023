use priority_queue::DoublePriorityQueue;

use aoc2023::coord::{Coord, CoordMap, Dir};

fn parse(file: &str) -> Vec<Vec<u8>> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap().try_into().unwrap()).collect()
    }).collect()
}

fn astar(map: &Vec<Vec<u8>>, min_steps: usize, max_steps: usize) -> usize {
    let width = map.len();
    let start = Coord::new(0, 0);
    let goal = Coord::new_u(width - 1, width - 1);
    let mut open = DoublePriorityQueue::new();
    open.push((start, Dir::Right), 0);
    open.push((start, Dir::Down), 0);

    // one cost for each direction we could be facing
    let mut costs = vec![vec![[usize::MAX; 4]; width]; width];
    costs[0][0][Dir::Right.id()] = 0;
    costs[0][0][Dir::Down.id()] = 0;

    while !open.is_empty() {
        let ((pos, dir), s) = open.pop_min().unwrap();

        if pos == goal{
            return s;
        }

        for ((p2, d2), c) in neighbours(map, min_steps, max_steps, pos, dir) {
            let new_cost = c + costs.at(pos)[dir.id()];
            if new_cost < costs.at(p2)[d2.id()] {
                costs[p2.y_u()][p2.x_u()][d2.id()] = new_cost;
                let score = new_cost + (goal - p2).manhattan();
                open.push((p2, d2), score);
            }
        }
    }
    panic!("unexpected");
}

fn neighbours(map: &Vec<Vec<u8>>, min_steps: usize, max_steps: usize, pos: Coord, dir: Dir) -> Vec<((Coord, Dir), usize)> {
    let mut res = vec![];
    let step = Coord::dir(dir);

    let mut p2 = pos;
    let mut cost = 0;
    for n in 1..=max_steps {
        p2 = p2 + step;
        if !map.contains(p2) {
            break;
        }
        cost += *map.at(p2) as usize;
        if n >= min_steps {
            res.push(((p2, dir.turn_left()), cost));
            res.push(((p2, dir.turn_right()), cost));
        }
    }
    res
}

fn main() {
    let ex = parse("inputs/day17_ex.txt");
    let inp = parse("inputs/day17.txt");
    assert!(astar(&ex, 0, 3) == 102);
    assert!(astar(&ex, 4, 10) == 94);
    println!("sol1: {}", astar(&inp, 0, 3));
    println!("sol2: {}", astar(&inp, 4, 10));
}