use std::{collections::HashMap, cmp::{min, max}};

enum Rule {
    Reject,
    Accept,
    Goto(String),
    Cmp(usize, char, usize, String),
}

fn with_min(vals: [usize;4], idx: usize, value: usize) -> [usize;4] {
    let mut ret = vals.clone();
    ret[idx] = min(value, vals[idx]);
    ret
}

fn with_max(vals: [usize;4], idx: usize, value: usize) -> [usize;4] {
    let mut ret = vals.clone();
    ret[idx] = max(value, vals[idx]);
    ret
}

fn parse(file: &str) -> (HashMap<String, Vec<Rule>>, Vec<[usize;4]>) {
    let inp = std::fs::read_to_string(file).unwrap();
    let mut parts = inp.trim().split("\n\n");

    let mut rules: HashMap<String, Vec<Rule>> = parts.next().unwrap().lines().map(|instr| {
        let mut parts = instr.split('{');
        let name = parts.next().unwrap();
        let rest = parts.next().unwrap().strip_suffix('}').unwrap();
        let rules = rest.split(',').map(|rule| {
            match rule {
                "R" => Rule::Reject,
                "A" => Rule::Accept,
                s if s.chars().all(|c| c.is_alphabetic()) => Rule::Goto(s.to_string()),
                s => {
                    let a = match s.chars().nth(0).unwrap() {
                        'x' => 0,
                        'm' => 1,
                        'a' => 2,
                        's' => 3,
                        _ => panic!("unexpected index")
                    };
                    let op = s.chars().nth(1).unwrap();
                    let mut x = s[2..].split(":");
                    let n = x.next().unwrap().parse::<usize>().unwrap();
                    let g = x.next().unwrap().to_string();
                    Rule::Cmp(a, op, n, g)
                }
            }
        }).collect();
        (name.to_string(), rules)
    }).collect();
    rules.insert("R".to_string(), vec![Rule::Reject]);
    rules.insert("A".to_string(), vec![Rule::Accept]);

    let inputs = parts.next().unwrap().lines().map(|line| {
        let v: Vec<usize> = line.strip_prefix('{').unwrap().strip_suffix('}').unwrap().split(',')
            .map(|x| x[2..].parse::<usize>().unwrap())
            .collect();
        [v[0], v[1], v[2], v[3]]
    }).collect();
    (rules, inputs)
}

fn solve1((map, values): &(HashMap<String, Vec<Rule>>, Vec<[usize; 4]>)) -> usize {
    values.iter().filter(|&value| {
        let mut end = value.clone();
        end.iter_mut().for_each(|x| *x += 1);
        combos(*value, end, map, "in", 0) == 1
    })
    .map(|v| v.iter().sum::<usize>())
    .sum()
}

fn solve2(map: &HashMap<String, Vec<Rule>>) -> usize {
    let start = [1, 1, 1, 1];
    let end = [4001, 4001, 4001, 4001];
    return combos(start, end, map, "in", 0);
}

fn score(start: [usize;4], end: [usize;4]) -> usize {
    start.iter().zip(end).map(|(&s, e)| e - s).product()
}

fn combos(start: [usize;4], end: [usize;4], map: &HashMap<String, Vec<Rule>>, lbl: &str, idx: usize) -> usize {
    if start.iter().zip(end).any(|(&s, e)| s >= e) {
        return 0;
    }
    match &map[lbl][idx]{
        Rule::Accept => return score(start, end),
        Rule::Reject => return 0,
        Rule::Goto(r) => return combos(start, end, map, r, 0),
        Rule::Cmp(attr, op, v, then) => {
            match op {
                '<' => {
                    let a = combos(start, with_min(end, *attr, *v), map, then, 0);
                    let b = combos(with_max(start, *attr, *v), end, map, lbl, idx + 1);
                    return a + b;
                },
                '>' => {
                    let a = combos(with_max(start, *attr, *v + 1), end, map, then, 0);
                    let b = combos(start, with_min(end, *attr, *v + 1), map, lbl, idx + 1);
                    return a + b;
                }
                _ => panic!("unexpected op"),
            };
        }
    }
}

fn main() {
    let ex = parse("inputs/day19_ex.txt");
    let inp = parse("inputs/day19.txt");
    assert!(solve1(&ex) == 19114);
    assert!(solve2(&ex.0) == 167409079868000);
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp.0));
}