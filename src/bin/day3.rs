use std::fs;

#[derive (Debug)]
struct Number {
    n: i32,
    row: i32,
    col: i32,
    len: i32,
}

impl Number {
    fn new(row: i32, col: i32) -> Self {
        Number { n: 0, row, col, len: 0}
    }
}

#[derive (Debug)]
struct Symbol {
    row: i32,
    col: i32,
    c: char,
}

fn adjacent(n: &Number, s: &Symbol) -> bool {
    let far = (n.row - s.row).abs() > 1 ||
      s.col < n.col - 1 ||
      s.col > n.col + n.len;
    !far
}

fn parse(filename: &str) -> (Vec<Number>, Vec<Symbol>) {
    let mut numbers = vec![];
    let mut symbols = vec![];

    let content = fs::read_to_string(filename).unwrap();
    let rows = content.trim().lines();
    (0i32..).zip(rows).for_each(|(j, row)| {
        let mut current: Option<Number> = None;
        for (i, c) in (0i32..).zip(row.chars()) {
            if let Some(digit) = c.to_digit(10)  {
                if current.is_none() {
                    current = Some(Number::new(j, i));
                }
                current.iter_mut().for_each(|num| {
                    num.n = num.n * 10 + digit as i32;
                    num.len += 1;
                })
            } else {
                // Push if we have a current number
                if let Some(num) = current {
                    numbers.push(num);
                    current = None;
                }

                if c != '.' {
                    symbols.push(Symbol {row: j, col: i, c: c})
                }
            }
        }
        if let Some(num) = current {
            numbers.push(num);
        }
    });
    (numbers, symbols)
}

fn solve1((numbers, symbols): &(Vec<Number>, Vec<Symbol>)) -> i32 {
    numbers.iter().filter(|num| {
        symbols.iter().any(|sym| adjacent(num, sym))
    }).map(|number| number.n).sum()
}

fn solve2((numbers, symbols): &(Vec<Number>, Vec<Symbol>)) -> i32 {
    symbols.iter()
        .filter(|sym| sym.c == '*')
        .filter_map(|sym| {
            let adj_nums: Vec<&Number> = numbers.iter().filter(|num| adjacent(num, sym)).collect();
            (adj_nums.len() == 2).then(|| adj_nums[0].n * adj_nums[1].n)
        }).sum()
}


fn main() {
    let example1 = parse("inputs/day3_ex1.txt");
    let input = parse("inputs/day3.txt");

    println!("example1: {}", solve1(&example1));
    println!("solution1: {}", solve1(&input));
    println!("example2: {}", solve2(&example1));
    println!("solution2: {}", solve2(&input));
}