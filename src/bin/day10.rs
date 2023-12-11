use std::{ops::{Add, Sub}, fs, collections::HashSet};

fn main() {
    let ex1 = parse("inputs/day10_ex.txt");
    let ex2 = parse("inputs/day10_ex2.txt");
    let ex3 = parse("inputs/day10_ex3.txt");
    let inp = parse("inputs/day10.txt");
    assert!(8 == solve1(&ex1));
    assert!(8 == solve2(&ex2));
    assert!(10 == solve2(&ex3));

    let sol1 = solve1(&inp);
    assert!(sol1 == 6717);
    println!("sol1: {sol1}");

    let sol2 = solve2(&inp);
    assert!(sol2 == 381);
    println!("sol2: {sol2}");
}

fn parse(filename: &str) -> Board {
    let chars = fs::read_to_string(filename).unwrap().trim().lines().map(|line| {
        let row: Vec<char> = line.chars().collect();
        row
    }).collect();
    Board { chars }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coord {
    x: i32,
    y: i32,
}

impl Coord {
    fn new(x: i32, y: i32) -> Self {
        Coord {x, y}
    }

    fn up() -> Coord {
        Coord::new(0, -1)
    }

    fn right() -> Coord {
        Coord::new(1, 0)
    }

    fn down() -> Coord {
        Coord::new(0, 1)
    }

    fn left() -> Coord {
        Coord::new(-1, 0)
    }

    fn dirs() -> Vec<Coord> {
        vec![Self::up(), Self::left(), Self::down(), Self::right()]
    }
}

impl Add for Coord {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coord::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Sub for Coord {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord::new(self.x - rhs.x, self.y - rhs.y)
    }
}

struct Board {
    chars: Vec<Vec<char>>
}

impl Board {
    fn get(&self, c: Coord) -> char {
        if c.x < 0 || c.y < 0 || c.x as usize >= self.chars[0].len() || c.y as usize >= self.chars.len() {
            return '.';
        }
        let y: usize = c.y.try_into().unwrap();
        let x: usize = c.x.try_into().unwrap();
        self.chars[y][x]
    }
}


fn solve1(board: &Board) -> usize {
    find_path(board).len() / 2
}

fn solve2(board: &Board) -> usize {
    let path = find_path(board);

    let mut change = vec!['|', 'J', 'L'];
    if (path[1] - path[0]).y == -1 {
        change.push('S');
    }

    let path: HashSet<&Coord> = HashSet::from_iter(path.iter());

    let mut cnt = 0;
    let mut inside = false;
    for (row, y) in board.chars.iter().zip(0..) {
        for (c, x) in row.iter().zip(0..) {
            let coord = Coord::new(x, y);
            if path.contains(&coord) {
                print!("{c}");
                if change.contains(&c) {
                    inside = !inside;
                }
            } else if inside {
                cnt += 1;
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    cnt
}


fn find_path(board: &Board) -> Vec<Coord> {
    let y = board.chars.iter().position(|row| row.contains(&'S')).unwrap();
    let x = board.chars[y].iter().position(|c| *c == 'S').unwrap();
    let start = Coord::new(x.try_into().unwrap(), y.try_into().unwrap());

    Coord::dirs().iter().find_map(|dir| {
        let path = follow(&board, start, start + *dir);
        (path.len() > 0).then_some(path)
    }).unwrap()
}

fn follow(board: &Board, start: Coord, mut next: Coord) -> Vec<Coord> {
    let mut prev = start;
    let mut path = vec![start];

    while next != start {
        path.push(next);
        let ns: Vec<Coord> = neigh(board.get(next)).iter().map(|dir| next + *dir).collect();
        if !ns.contains(&prev) {
            return vec![];
        }
        let nextnext = *ns.iter().filter(|x| **x != prev).next().unwrap();
        prev = next;
        next = nextnext;
    }
    path
}

fn neigh(c: char) -> Vec<Coord> {
    match c {
        '|' => vec![Coord::up(), Coord::down()],
        '-' => vec![Coord::left(), Coord::right()],
        'J' => vec![Coord::up(), Coord::left()],
        'F' => vec![Coord::down(), Coord::right()],
        'L' => vec![Coord::up(), Coord::right()],
        '7' => vec![Coord::down(), Coord::left()],
        '.' => vec![],
        _ => panic!("unexpected"),
    }
}