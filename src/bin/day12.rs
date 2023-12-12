use std::fs;

// Use dynamic programming to optimize
fn combos(broken: &[usize], pattern: &str) -> usize {
    // solution array for each prefix of pattern and broken
    let mut res = vec![vec![0; broken.len()+1]; pattern.len()+1];

    for i in 0..pattern.len()+1 {
        if !pattern[0..i].contains('#') {
            res[i][0] = 1;
        }
    }

    for b_prefix in 1..broken.len()+1 {
        let broken_size = broken[b_prefix-1];

        for p_prefix in broken_size..pattern.len()+1 {
            let p = pattern.as_bytes()[p_prefix-1];
            if p == b'.' {
                res[p_prefix][b_prefix] = res[p_prefix-1][b_prefix];
            } else {
                // '#' or '?'
                let can_be_broken = !pattern[p_prefix-broken_size..p_prefix].contains('.');
                let combos_with_broken = if !can_be_broken {
                    0
                } else if p_prefix == broken_size {
                    res[0][b_prefix-1]
                } else if pattern.as_bytes()[p_prefix-broken_size-1] != b'#' {
                    res[p_prefix-broken_size-1][b_prefix-1]
                } else {
                    0
                };

                if p == b'#' {
                    res[p_prefix][b_prefix] = combos_with_broken;
                } else {
                    let combos_not_broken = res[p_prefix-1][b_prefix];
                    res[p_prefix][b_prefix] = combos_with_broken + combos_not_broken;
                }
            }
        }
    }
    res[pattern.len()][broken.len()]
}

#[test]
fn test() {
    assert!(combos(&[3], ".#?") == 0);
    assert!(combos(&[1], "#") == 1);
    assert!(combos(&[1], "?.") == 1);
    assert!(combos(&[1], "?") == 1);
    assert!(combos(&[1], ".#.") == 1);
    assert!(combos(&[1], ".?.") == 1);
    assert!(combos(&[3], "?###") == 1);
    assert!(combos(&[3], "?###?") == 1);
    assert!(combos(&[3], "?###??") == 1);
    assert!(combos(&[2], "#??") == 1);
    assert!(combos(&[2], "???") == 2);
    assert!(combos(&[3], "##.") == 0);
    assert!(combos(&[3], "##") == 0);
    assert!(combos(&[3], "?##.") == 1);
    assert!(combos(&[3, 2], "?###???") == 1);
    assert!(combos(&[3, 2], ".#????#?.???") == 2);
    assert!(combos(&[1,1,3], "???.###") == 1);
    assert!(combos(&[1,1,3], ".??..??...?##.") == 4);
    assert!(combos(&[3,2,1], "?###????????") == 10);
    assert!(combos(&[1,6,5], "????.######..#####.") == 4);
}

fn solve1(input: &Vec<(String, Vec<usize>)>) -> usize {
    input.iter().map(|(pattern, broken)| combos(broken, pattern)).sum()
}

fn solve2(input: &Vec<(String, Vec<usize>)>) -> usize {
    let input2: Vec<(String, Vec<usize>)> = input.iter().map(|(pattern, broken)| {
        (
            [pattern.as_str(); 5].join("?"),
            broken.repeat(5)
        )
    }).collect();
    solve1(&input2)
}

fn parse(file: &str) -> Vec<(String, Vec<usize>)> {
    fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        let parts: Vec<&str> = line.split(" ").collect();
        let broken = parts[1].split(",").map(|n| n.parse::<usize>().unwrap()).collect();
        (parts[0].to_string(), broken)
    }).collect()
}

fn main() {
    let ex1 = parse("inputs/day12_ex1.txt");
    let inp = parse("inputs/day12.txt");
    assert!(solve1(&ex1) == 21);
    assert!(solve2(&ex1) == 525152);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));
}