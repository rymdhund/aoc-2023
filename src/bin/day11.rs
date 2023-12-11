use std::fs;

fn parse(file: &str) -> Vec<Vec<char>> {
    fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        line.chars().collect()
    }).collect()
}

fn solve(map: &Vec<Vec<char>>, expansion: i64) -> i64 {
    let stars = expand(map, expansion);

    let mut dist = 0;
    for i in 0..stars.len() {
        for j in 0..i {
            dist += (stars[i].0 - stars[j].0).abs() + (stars[i].1 - stars[j].1).abs();
        }
    }
    dist
}

fn expand(map: &Vec<Vec<char>>, expansion: i64) -> Vec<(i64, i64)> {
    let empty_rows: Vec<usize> = map.iter().enumerate().filter_map(|(i, row)| row.iter().all(|c| *c == '.').then_some(i)).collect();
    let empty_cols: Vec<usize> = (0..map[0].len()).filter(|i| map.iter().all(|row| row[*i] == '.')).collect();

    let mut x;
    let mut y = 0;

    let mut stars = vec![];
    for (i, row) in map.iter().enumerate() {
        x = 0;
        for (j, c) in row.iter().enumerate() {
            if *c == '#' {
                stars.push((x, y));
            }
            x += if empty_cols.contains(&j) { expansion } else { 1 };
        }
        y += if empty_rows.contains(&i) { expansion } else { 1 };
    }
    stars
}

fn main() {
    let ex1 = parse("inputs/day11_ex1.txt");
    let inp = parse("inputs/day11.txt");

    let ex1_res = solve(&ex1, 2);
    assert!(ex1_res == 374);

    let sol1 = solve(&inp, 2);
    println!("sol1: {sol1}");
    assert!(sol1 == 10228230);

    let sol2 = solve(&inp, 1000000);
    println!("sol2: {sol2}");
    assert!(sol2 == 447073334102);
}