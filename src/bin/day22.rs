use std::{ops::Add, cmp::{min, max}};

use itertools::Itertools;

#[derive (Debug, Clone, Copy)]
struct Coord {
    x: i32,
    y: i32,
    z: i32,
}

impl Coord {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Coord { x, y, z }
    }

}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    a: Coord,
    b: Coord,
}

impl Brick {
    
    fn minx(&self) -> i32 {
        min(self.a.x, self.b.x)
    }

    fn miny(&self) -> i32 {
        min(self.a.y, self.b.y)
    }

    fn maxx(&self) -> i32 {
        max(self.a.x, self.b.x)
    }

    fn maxy(&self) -> i32 {
        max(self.a.y, self.b.y)
    }

    fn minz(&self) -> i32 {
        min(self.a.z, self.b.z)
    }

    fn maxz(&self) -> i32 {
        max(self.a.z, self.b.z)
    }

    fn move_z(&mut self, dz: i32) -> Brick {
        let a = self.a + Coord::new(0, 0, dz);
        let b = self.b + Coord::new(0, 0, dz);
        Brick { a, b}
    }

    fn xy_overlap(&self, other: &Brick) -> bool {
        let not = self.minx() > other.maxx()
            || self.maxx() < other.minx()
            || self.miny() > other.maxy()
            || self.maxy() < other.miny();
        !not
    }
}

fn parse(file: &str) -> Vec<Brick> {
    std::fs::read_to_string(file).unwrap().trim().lines().enumerate().map(|(id, line)| {
        let mut parts = line.split('~');
        let a = parse_coord(parts.next().unwrap());
        let b = parse_coord(parts.next().unwrap());
        Brick { a, b}
    }).collect_vec()
}

fn parse_coord(inp: &str) -> Coord {
    let xs = inp.split(',').map(|n| n.parse::<i32>().unwrap()).collect_vec();
    Coord::new(xs[0], xs[1], xs[2])
}


fn solve1(bricks: &Vec<Brick>) -> (usize, usize) {
    let mut bricks = bricks.clone();
    bricks.sort_by(|a, b| a.minz().cmp(&b.minz()));

    let mut needs: Vec<Vec<usize>> = vec![vec![]; bricks.len()];
    let mut needed: Vec<bool> = vec![false; bricks.len()];

    for id in 0..bricks.len() {
        let mut brick = bricks[id];
        let overlap = xy_overlap(brick, bricks[0..id].iter().collect_vec());
        let max_z = overlap.iter().map(|(_, b)| b.maxz()).max().unwrap_or(0);
        let dz = max_z - brick.minz() + 1;
        assert!(dz <= 0);

        let base = overlap.iter().filter(|(_, b)| b.maxz() == max_z).map(|(id, _)| *id).collect_vec();
        if base.len() == 1 {
            needed[base[0]] = true;
        }
        needs[id] = base;

        bricks[id] = brick.move_z(dz);
    }

    let sol1 = needed.iter().filter(|&a| !*a).count();
    let sol2 = (0..bricks.len()).map(|id| would_fall(id, &bricks, &needs)).sum();

    (sol1, sol2)
}

fn would_fall(id: usize, bricks: &Vec<Brick>, base: &Vec<Vec<usize>>) -> usize {
    let mut falling = vec![id];

    for id in 0..bricks.len() {
        if !base[id].is_empty() && base[id].iter().all(|base| falling.contains(base)) {
            falling.push(id);
        }
    }
    falling.len() - 1
}

fn xy_overlap(brick: Brick, bricks: Vec<&Brick>) -> Vec<(usize, &Brick)> {
    bricks.into_iter().enumerate().filter(|&(_, b)| brick.xy_overlap(b)).collect()
}

fn main() {
    let test1 = parse("inputs/day22_ex.txt");
    let testres = solve1(&test1);
    assert!(testres.0 == 5);
    assert!(testres.1 == 7);

    let inp = parse("inputs/day22.txt");
    let res = solve1(&inp);
    println!("sol1: {}", res.0);
    println!("sol2: {}", res.1);
}