use std::collections::{VecDeque, HashSet};

#[derive(Debug, Clone)]
struct Entry {
    typ: char,
    dests: Vec<usize>,
    sources: Vec<usize>,
}

fn parse(path: &str) -> Vec<Entry> {
    let data = std::fs::read_to_string(path).unwrap();
    let mut lines: Vec<&str> = data.trim().lines().collect();
    lines.sort();
    let names: Vec<&str> = lines.iter().map(|line| &line.split(" -> ").next().unwrap()[1..]).collect();
    let mut map = vec![];

    let get_pos = |name| {
        names.iter().position(|&n| n == name).unwrap_or_else(|| {
            names.len()
        })
    };
    let mut sources = vec![vec![]; names.len() + 1];
    lines.iter().enumerate().for_each(|(cur, line)| {
        let parts: Vec<&str> = line.split(" -> ").collect();
        let typ = parts[0].chars().nth(0).unwrap();
        let dests: Vec<usize> = parts[1].split(", ").map(|s| get_pos(s)).collect();
        for &dest in dests.iter() {
            sources[dest].push(cur)
        }
        map.push(Entry { typ, dests, sources: vec![] });
    });
    map.push(Entry { typ: 'o', dests: vec![], sources: vec![] }); // output
    map.iter_mut().enumerate().for_each(|(cur, e)| {
        e.sources = sources[cur].clone();
    });
    map
}

fn solve1(inp: &Vec<Entry>) -> usize {
    let (mut ff_states, mut con_states) = init_states(inp);

    let mut highs = 0;
    let mut lows = 0;
    for _ in 0..1000 {
        let (h, l, _) = push_button(inp, &mut ff_states, &mut con_states, None);
        highs += h;
        lows += l;
    }
    highs * lows
}

fn solve2(inp: &Vec<Entry>) -> usize {
    // This uses the fact that there are distinct parts of the graph that cycle independently of each other
    let subs = subsets(inp);
    let mut prod = 1;
    for subset in subs {
        let (mut ff_states, mut con_states) = init_states(inp);
        for i in 1.. {
            let (_, _, guard_high) = push_button(inp, &mut ff_states, &mut con_states, Some(&subset));
            if guard_high {
                prod *= i;
                break;
            }
        }
    }
    prod
}

fn subsets(inp: &Vec<Entry>) -> Vec<HashSet<usize>> {
    let mut res = vec![];
    let last = &inp[inp.len() - 1];
    assert!(last.sources.len() == 1);
    let lastguard = last.sources[0];
    for end in inp[lastguard].sources.iter() {
        let mut used = HashSet::new();
        let mut buf = vec![end];
        while !buf.is_empty() {
            let x = buf.pop().unwrap();
            if used.insert(*x) {
                inp[*x].sources.iter().for_each(|y| buf.push(y));
            }
        }
        println!("subgroup: {} / {}", used.len(), inp.len());
        assert!(used.len() < inp.len());
        res.push(used)
    }
    res
}

fn init_states(
    inp: &Vec<Entry>
) -> (Vec<bool>, Vec<Vec<bool>>) {
    let ff_states = vec![false; inp.len()];
    let mut con_states = vec![vec![]; inp.len()];
    inp.iter().enumerate().for_each(|(k, entry)| {
        if entry.typ == '&' {
            con_states[k] =  vec![false; entry.sources.len()]
        }
    });
    (ff_states, con_states)
}

// This function is a bit ugly since we mix the solution logic for part 1 and part 2
fn push_button(
    inp: &Vec<Entry>,
    ff_states: &mut Vec<bool>,
    con_states: &mut Vec<Vec<bool>>,
    subset: Option<&HashSet<usize>>
) -> (usize, usize, bool) {
    let mut pulses = VecDeque::new();
    let mut guard_high = false;
    let mut highs = 0;
    let mut lows = 1;

    let last_guard = if subset.is_none() {
        10000 // a random id that doesn't exist
    } else {
        inp[inp.len()-1].sources[0]
    };
    let start = inp.len() - 2;

    inp[start].dests.iter().for_each(|dest| {
        let do_add = subset.is_none() || subset.is_some_and(|subs| subs.contains(dest));
        if do_add {
            pulses.push_back((start, dest, false))
        }
    });

    while !pulses.is_empty() {
        let (from, to, pulse) = pulses.pop_front().unwrap();
        let entry = &inp[*to];

        if pulse {
            highs += 1;
        } else {
            lows += 1;
        }

        if entry.typ == '%' {
            if pulse == false {
                let new_state = !ff_states[*to];
                ff_states[*to] = new_state;
                entry.dests.iter().for_each(|next| pulses.push_back((*to, next, new_state)));
            }
        } else if entry.typ == '&' {
            let idx = entry.sources.iter().position(|&x| x == from).unwrap();
            con_states[*to][idx] = pulse;
            let out = !con_states[*to].iter().all(|&v| v);
            entry.dests.iter().for_each(|next| pulses.push_back((*to, next, out)));
        } 

        if *to == last_guard {
            if pulse {
                // this input to the guard is high
                guard_high = true;
            }
        }
    }
    (lows, highs, guard_high)
}


fn main() {
    let ex12 = parse("inputs/day20_ex1.txt");
    let ex22 = parse("inputs/day20_ex2.txt");
    let inp2 = parse("inputs/day20.txt");

    println!("ex1: {}", solve1(&ex12));
    println!("ex2: {}", solve1(&ex22));
    println!("sol2: {}", solve1(&inp2));
    println!("sol2: {}", solve2(&inp2));
}