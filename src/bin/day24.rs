use itertools::Itertools;
use num::ToPrimitive;
use priority_queue::DoublePriorityQueue;
use std::{ops::{Mul, Add, Sub}, cmp::min};


type C3 = (f64, f64, f64);
type C2 = (f64, f64);
type I3 = (i64, i64, i64);


fn parse(file: &str) -> Vec<(C3, C3)> {
    std::fs::read_to_string(file).unwrap().trim().lines().map(|line| {
        let mut parts = line.split(" @ ");
        let pos = parse_coord(parts.next().unwrap());
        let dir = parse_coord(parts.next().unwrap());
        (pos, dir)
    }).collect_vec()
}

fn parse_coord(s: &str) -> C3 {
    if let Some(coord) = s.split(", ").map(|c| {
        c.trim().parse::<f64>().unwrap()
    }).collect_tuple() {
        coord
    } else {
        panic!("Couldn't parse coord")
    }
}

fn solve1(stones: &Vec<(C3, C3)>, start: f64, end: f64) -> usize {
    let eqs = stones.iter().map(|(pos, dest)| {
        let k = f64::from(dest.1) / f64::from(dest.0);
        let m = f64::from(pos.1) - f64::from(pos.0) * k;
        (k, m)
    }).collect_vec();

    let mut cnt = 0;
    for i in 0..eqs.len() {
        for j in 0..i {
            if i != j {
                let (p1, dp1) = stones[i];
                let (p2, dp2) = stones[j];
                let p1 = proj_xy(p1);
                let dp1 = proj_xy(dp1);
                let p2 = proj_xy(p2);
                let dp2 = proj_xy(dp2);
                let t = intersect(p1, dp1, p2, dp2);
                let u = intersect(p2, dp2, p1, dp1);
                if t == None || u == None {
                    continue;
                }
                let t = t.unwrap();
                let u = u.unwrap();
                if t < 0.0 || u < 0.0 {
                    continue;
                }
                let p = (p1.0 + dp1.0 * t, p1.1 + dp1.1 * t);
                if p.0 < start || p.1 < start || p.0 > end || p.1 > end {
                    continue;
                }
                cnt += 1;
            }
        }
    }
    cnt
}

fn proj_xy<T>((a, b, _): (T, T, T)) -> (T, T) {
    (a, b)
}

fn intersect((x1, y1): (f64, f64), (dx1, dy1): (f64, f64), (x2, y2): (f64, f64), (dx2, dy2): (f64, f64)) -> Option<f64> {
    let den = dx1 * dy2 - dy1 * dx2;
    if den == 0.0 {
        return None;
    }
    let nom = (x2 - x1) * dy2 - (y2 - y1) * dx2;
    Some(nom / den)
}

// Project the lines onto a plane for a given normal
// Search for normal that minimises distace beteen intersection points
// When all lines intersect at the same point we have found the direction of our throw
fn solve2(inp: &Vec<(C3, C3)>) -> i64 {
    let n1: I3 = (0, 0, 0);
    let mut processed = vec![n1];
    let mut queue = DoublePriorityQueue::new();
    queue.push(n1, i64::MAX);

    // We don't need all lines, just a few
    let inp = inp.iter().map(|&x| x).take(7).collect_vec();

    let epsilon: f64 = 0.5;
    while !queue.is_empty() {
        let (normal, _) = queue.pop_min().unwrap();

        let neigh = neighbours(normal);
        for n in neigh {
            // ignore negative z axis
            if processed.contains(&n) || n.2 < 0 {
                continue;
            }

            let projected = proj_all(&inp, norm(n));
            let dist = diam(&projected);
            let score = (dist + (1.0 - epsilon)).to_i64().unwrap_or(i64::MAX);

            if score == 0 {
                println!("found normal! {n:?}");

                let t0 = intersect(projected[0].0, projected[0].1, projected[1].0, projected[1].1).unwrap().round();
                let t1 = intersect(projected[1].0, projected[1].1, projected[0].0, projected[0].1).unwrap().round();
                let p0 = add3(inp[0].0, mult3(inp[0].1, t0));
                let p1 = add3(inp[1].0, mult3(inp[1].1, t1));

                // p0 and p1 are on the line we are looking for
                let dt = t1 - t0;
                let dir = mult3(sub3(p1, p0), 1.0/dt);
                let start_pos = sub3(p0, mult3(dir, t0));
                let res = start_pos.0.round() + start_pos.1.round() + start_pos.2.round();
                return res.to_i64().unwrap();
            }
            queue.push(n, score);
            processed.push(n);
        }
    }
    panic!("Unexpected: did not find a solution");
}

fn norm(n: I3) -> C3 {
    let n2 = (n.0 as f64, n.1 as f64, n.2 as f64);
    normalize(n2)
}

fn mult3<T>(v: (T, T, T), c: T) -> (T, T, T) where T: Mul<Output = T>, T: Copy {
    (v.0 * c, v.1 * c, v.2 * c)
}

fn add3<T>(v: (T, T, T), u: (T, T, T)) -> (T, T, T) where T: Add<Output = T> {
    (v.0 + u.0 , v.1 + u.1, v.2 + u.2)
}

fn sub3<T>(v: (T, T, T), u: (T, T, T)) -> (T, T, T) where T: Sub<Output = T> {
    (v.0 - u.0 , v.1 - u.1, v.2 - u.2)
}

fn normalize(v: C3) -> C3 {
    let d = c3_len(v);
    (v.0 / d, v.1 / d, v.2 / d)
}

fn c3_len(v: C3) -> f64 {
    (v.0 * v.0 + v.1 * v.1 + v.2 * v.2).sqrt()
}

fn neighbours(v: I3) -> Vec<I3> {
    let w = 1;
    (-w..=w).flat_map(|x| 
        (-w..=w).flat_map(move |y| 
            (-w..=w).map(move |z|
                add3(v, (x, y, z)
            )
        )
    )).filter(|&u| u != v).collect_vec()
}

fn diam(stones: &Vec<(C2, C2)>) -> f64 {
    let res = intersections(&stones[0..stones.len()]);
    let mut max_dist = 0.0;
    for i in 0..res.len() {
        for j in 0..i {
            let d = dist(res[i], res[j]);
            if d > max_dist {
                max_dist = d;
            }
        }
    }
    return max_dist;
}

fn intersections(stones: &[(C2, C2)]) -> Vec<C2> {
    let mut results = vec![];
    for i in 0..stones.len() {
        for j in 0..min(i, 3) {
            let (p1, dp1) = stones[i];
            let (p2, dp2) = stones[j];
            let t = intersect(p1, dp1, p2, dp2);
            let u = intersect(p2, dp2, p1, dp1);
            if t == None || u == None{
                // parallell
                continue;
            }
            let t = t.unwrap();
            let u = u.unwrap();
            if t < 0.0 {
                results.push(p1);
            }
            if u < 0.0 {
                results.push(p1);
            }
            if u >= 0.0 && t >= 0.0 {
                let p = (p1.0 + dp1.0 * t, p1.1 + dp1.1 * t);
                results.push(p);
            }
        }
    }
    results
}

fn dist(a: C2, b: C2) -> f64 {
    ((a.0 - b.0) * (a.0 - b.0) + (a.1 - b.1) * (a.1 - b.1)).sqrt()
}

fn proj_all(inp: &Vec<(C3, C3)>, normal: C3) -> Vec<(C2, C2)> {
    let e1 = normalize(orth(normal));
    let e2 = normalize(cross(normal, e1));
    inp.iter().map(|(v, d)| (proj_e(*v, e1, e2), proj_e(*d, e1, e2))).collect_vec()
}

    
fn proj_e(x: C3, e1: C3, e2: C3) -> C2 {
    (dot(x, e1), dot(x, e2))
}

fn orth(a: C3) -> C3 {
    if a.0 != 0.0 || a.1 != 0.0 {
        (-a.1, a.0, 0.0)
    } else {
        (-a.2, 0.0, a.1)
    }
}

fn cross(a: C3, b: C3) -> C3 {
    let x = a.1 * b.2 - a.2 * b.1;
    let y = a.2 * b.0 - a.0 * b.2;
    let z = a.0 * b.1 - a.1 * b.0;
    (x, y, z)
}

fn dot(a: C3, b: C3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

fn main() {
    let a = 200000000000000.0;
    let b = 400000000000000.0;
    let test = parse("inputs/day24_ex.txt");
    let inp = parse("inputs/day24.txt");

    println!("example1: {}", solve1(&test, 7.0, 27.0));
    println!("sol1: {}", solve1(&inp, a, b));

    println!("example2: {}", solve2(&test));
    println!("sol2: {}", solve2(&inp));
}