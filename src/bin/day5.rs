use std::{fs, cmp::{max, min}};

#[derive(Debug, Copy, Clone)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn new(start: i64, end: i64) -> Self {
        Range { start, end }
    }

    fn has_overlap(&self, map: &RangeMap) -> bool {
        map.start < self.end && map.end > self.start
    }
}

#[derive(Debug, Copy, Clone)]
struct RangeMap {
    start: i64,
    end: i64,
    shift: i64,
}

impl RangeMap {
    fn new(to: i64, from: i64, len: i64) -> Self {
        RangeMap { start: from, end: from + len, shift: to - from }
    }
}

fn numbers(line: &str) -> Vec<i64> {
    line.split(' ').map(|n| n.parse::<i64>().unwrap()).collect()
}

fn parse(filename: &str) -> (Vec<i64>, Vec<Vec<RangeMap>>) {
    let data = fs::read_to_string(filename).unwrap();
    let parts: Vec<&str> = data.split("\n\n").collect();
    let seeds: Vec<i64> = numbers(&parts[0][7..]);

    let maps = parts[1..].iter()
        .map(|part| {
            part.splitn(2, ':').collect::<Vec<&str>>()[1].trim().lines()
                .map(|line| {
                    let nums = numbers(line);
                    RangeMap::new(nums[0], nums[1], nums[2])
                })
                .collect::<Vec<RangeMap>>()
        })
        .collect();

    (seeds, maps)
}

fn solve1((seeds, category_maps): &(Vec<i64>, Vec<Vec<RangeMap>>)) -> i64 {
    let mut nums = seeds.to_owned();
    for maps in category_maps {
        nums = nums.iter().map(|n| map(*n, maps)).collect();
    }
    *nums.iter().min().unwrap()
}

fn map(n: i64, maps: &Vec<RangeMap>) -> i64 {
    for map in maps {
        if n >= map.start && n < map.end {
            return n + map.shift
        }
    }
    n
}

fn solve2((seeds, category_maps): &(Vec<i64>, Vec<Vec<RangeMap>>)) -> i64 {
    let mut ranges = seeds.chunks_exact(2).map(|range|{
        let start = range[0];
        let len = range[1];
        Range::new(start, start + len)
    }).collect();

    for maps in category_maps.iter() {
        ranges = map_ranges(ranges, maps);
    }

    ranges.iter().map(|r| r.start).min().unwrap()
}

fn map_ranges(ranges: Vec<Range>, maps: &Vec<RangeMap>) -> Vec<Range> {
    let mut unmapped = ranges;
    let mut mapped = vec![];

    while let Some(range) = unmapped.pop() {
        let overlapping_map = maps.iter().find(|map| range.has_overlap(map));

        if let Some(map) = overlapping_map {
            let overlap_start = max(range.start, map.start);
            let overlap_end = min(range.end, map.end);

            mapped.push(Range::new(overlap_start + map.shift, overlap_end + map.shift));

            if range.start < map.start {
                unmapped.push(Range::new(range.start, map.start));
            }
            if range.end > map.end {
                unmapped.push(Range::new(map.end, range.end));
            }
        } else {
            // The range did not have an overlapping map
            mapped.push(range);
        }
    }
    mapped
}

fn main() {
    let ex = parse("inputs/day5_ex1.txt");
    let input = parse("inputs/day5.txt");

    println!("ex1: {}", solve1(&ex));
    println!("solution1: {}", solve1(&input));
    println!("ex2: {}", solve2(&ex));
    println!("solution2: {}", solve2(&input));
}

#[test]
fn test_map() {
    let ex1 = vec![
        RangeMap::new(50, 98, 2),
        RangeMap::new(52, 50, 48),
    ];
    assert_eq!(1, map(1, &ex1));
    assert_eq!(52, map(50, &ex1));
    assert_eq!(99, map(97, &ex1));
    assert_eq!(50, map(98, &ex1));
}