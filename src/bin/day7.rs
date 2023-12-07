use std::{fs, cmp::Ordering};

fn parse(filename: &str) -> Vec<(Vec<u32>, usize)> {
    fs::read_to_string(filename).unwrap().lines().filter(|line| !line.is_empty())
    .map(|line| {
        let mut parts = line.split(' ');
        let cards = parts.next().unwrap().chars().map(|c| card_value(c)).collect();
        let bid = parts.next().unwrap().parse::<usize>().unwrap();
        (cards, bid)
    })
    .collect()
}

fn card_value(c: char) -> u32 {
    match c {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        d => d.to_digit(10).unwrap()
    }
}

fn solve1(hands: &Vec<(Vec<u32>, usize)>) -> usize {
    let mut hands = hands.to_owned();
    hands.sort_by(|(cards1, _), (cards2, _)| compare(cards1, cards2));
    hands.iter().enumerate()
      .map(|(i, (_, bid))| (i+1) * bid)
      .sum()
}

fn solve2(hands: &Vec<(Vec<u32>, usize)>) -> usize {
    // For J, replace 11 with 1
    let mut hands = hands.clone();
    hands.iter_mut().for_each(|(cards, _)| {
        cards.iter_mut().for_each(|v| if *v == 11 { *v = 1 })}
    );
    solve1(&hands)
}

fn compare(h1: &Vec<u32>, h2: &Vec<u32>) -> Ordering {
    let hc1 = hand_counts(h1);
    let hc2 = hand_counts(h2);
    let diff = hc1.into_iter().zip(hc2).find(|(x, y)| x != y);
    if let Some((a, b)) = diff {
        // return the first that has more cards of the same value
        return a.cmp(&b);
    }
    let (a, b) = h1.iter().zip(h2).find(|(x, y)| x != y).unwrap();
    a.cmp(b)
}

fn hand_counts(h: &Vec<u32>) -> Vec<usize> {
    let jacks = h.iter().filter(|c| **c == 1).count();
    if jacks == 5 {
        return vec![5];
    }

    let mut seen = vec![];
    let mut counts = vec![];
    while let Some(v) = h.iter().find(|c| **c != 1 && !seen.contains(c)) {
        let cnt = h.iter().filter(|c| *c == v).count();
        seen.push(v);
        counts.push(cnt);
    }

    counts.sort();
    counts.reverse();
    counts[0] += jacks;
    counts
}

fn main() {
    let ex1 = parse("inputs/day7_ex1.txt");
    let inp1 = parse("inputs/day7.txt");

    println!("ex1: {}", solve1(&ex1));
    println!("sol1: {}", solve1(&inp1));
    println!("ex2: {}", solve2(&ex1));
    println!("sol2: {}", solve2(&inp1));
}