// Bruteforce
fn solve1((ts, ds): (Vec<i64>, Vec<i64>)) -> usize {
    ts.iter().zip(ds).map(|(t, d)| {
        let n = (1..*t).filter(|x| x * (t-x) > d).count();
        n
    }).product()
}

// Solve using pq-formula
fn solve2((t, d): (f64, f64)) -> f64{
    let a = t / 2.0;
    let b = ((t / 2.0f64).powf(2.0) - d).sqrt();
    let x0 = (a - b).floor()+1.0;
    let x1 = (a + b).ceil();
    x1 - x0
}


fn main() {
    let ex1 = (vec![7, 15, 30], vec![9, 40, 200]);
    let ex2: (f64, f64) = (71530.0, 940200.0);
    let inp1 = (vec![47, 70, 75, 66], vec![282, 1079, 1147, 1062]);
    let inp2: (f64, f64) = (47707566.0, 282107911471062.0);

    println!("ex1: {}", solve1(ex1));
    println!("sol1: {}", solve1(inp1));
    println!("ex2: {}", solve2(ex2));
    println!("sol2: {}", solve2(inp2));
}