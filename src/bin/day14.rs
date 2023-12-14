use std::fs;

fn print(b: &Vec<Vec<char>>) {
    for row in b {
        println!("{}", row.iter().collect::<String>());
    }
    println!("\n");
}

fn parse(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn solve1(board: &Vec<Vec<char>>) -> usize {
    let mut board = board.to_owned();
    tilt(&mut board, 0);
    score(&board)
}

fn solve2(board: &Vec<Vec<char>>, rotations: usize) -> usize {
    let mut turt = board.to_owned();
    let mut hare = board.to_owned();
    step(&mut hare);

    // run until turt is in a cycle
    let mut i = 0;
    while turt != hare {
        step(&mut turt);
        step(&mut hare);
        step(&mut hare);
        i += 1;
    }

    // find cycle length
    hare = turt.clone();
    step(&mut hare);
    let mut lam = 1;
    while turt != hare {
        step(&mut hare);
        lam += 1;
    }

    // We assume rotations is big, otherwise we could easily calculate the answer directly
    assert!(rotations > i);

    let steps_left = (rotations - i) % lam;
    (0..steps_left).for_each(|_| step(&mut turt));
    score(&turt)
}

fn step(mut board: &mut Vec<Vec<char>>) {
    for i in 0..4 {
        tilt(&mut board, i);
    }
}

fn tilt(board: &mut Vec<Vec<char>>, dir: usize) {
    let width = board.len(); // width = height
    for a in 0..width {
        let mut free = usize::MAX;
        for b in 0..width {
            let (x, y) = rot(width, a, b, dir);
            if board[y][x] == 'O' && free < b {
                let (x1, y1) = rot(width, a, free, dir);
                board[y1][x1] = 'O';
                board[y][x] = '.';
                free += 1;
            } else if board[y][x] == '.' && free > b {
                free = b;
            } else if board[y][x] == '#' && free < b {
                free = usize::MAX;
            }
        }
    }
}

fn rot(width: usize, x: usize, y: usize, dir: usize) -> (usize, usize) {
    match dir {
        0 => (x, y),
        1 => (y, x),
        2 => (x, width - y - 1),
        3 => (width - y - 1, x),
        _ => panic!("unexpected"),
    }
}

fn score(board: &Vec<Vec<char>>) -> usize {
    (0..board.len()).flat_map(|y| {
        (0..board[0].len()).filter_map(move |x| {
            if board[y][x] == 'O' {
                Some(board.len() - y)
            } else {
                None
            }
        })
    }).sum()
}

fn main() {
    let ex = parse("inputs/day14_ex.txt");
    let inp = parse("inputs/day14.txt");
    assert!(solve1(&ex) == 136);
    assert!(solve2(&ex, 1000000000) == 64);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp, 1000000000));
}