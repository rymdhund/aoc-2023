use aoc2023::coord::{Coord, Dir};

fn parse(file: &str) -> Vec<(Dir, i64)> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        let dir = match parts[0] {
            "U" => Dir::Up,
            "L" => Dir::Left,
            "R" => Dir::Right,
            "D" => Dir::Down,
            _ => panic!("unexpected dir"),
        };
        (dir, parts[1].parse::<i64>().unwrap())
    }).collect()
}

fn parse2(file: &str) -> Vec<(Dir, i64)> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        let hex = i64::from_str_radix(&parts[2][2..7], 16).unwrap();
        let dir = match parts[2].chars().nth(7).unwrap() {
            '0' => Dir::Right,
            '1' => Dir::Down,
            '2' => Dir::Left,
            '3' => Dir::Up,
            c => panic!("unexpected dir {c:?}"),
        };
        (dir, hex)
    }).collect()
}

// Only works for clockwise paths
fn solve(inp: &Vec<(Dir, i64)>) -> i64 {
    let mut area = 0;
    let mut last_dir = inp[inp.len()-1].0;

    let mut pos = Coord::new(0, 0);
    for &(dir, n) in inp {
        area += match dir {
            Dir::Down => n * pos.x, 
            Dir::Up => - n * pos.x, 
            _ => 0
        };
        // Expand one unit to the left and down
        if dir == Dir::Down || dir == Dir::Left {
            area += n;
        }
        if last_dir == Dir::Down && dir == Dir::Left {
            area += 1;
        }
        if last_dir == Dir::Left && dir == Dir::Down {
            area -= 1;
        }
        pos = pos + Coord::dir(dir) * n;
        last_dir = dir;
    }

    area
}

fn main() {
    let ex = parse("inputs/day18_ex.txt");
    let inp = parse("inputs/day18.txt");
    let ex2 = parse2("inputs/day18_ex.txt");
    let inp2 = parse2("inputs/day18.txt");
    assert!(solve(&ex) == 62);
    assert!(solve(&ex2) == 952408144115);
    println!("sol1: {}", solve(&inp));
    println!("sol2: {}", solve(&inp2));
}