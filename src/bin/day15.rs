fn parse(file: &str) -> Vec<String> {
    std::fs::read_to_string(file).unwrap().trim().split(',').map(|s| s.to_string()).collect()
}

fn hash(s: &str) -> usize {
    s.as_bytes().iter().fold(0, |h, &b| (h + b as usize) * 17 % 256)
}

fn solve1(inp: &Vec<String>) -> usize {
    inp.iter().map(|s| hash(s)).sum()
}

type Boxes<'a> = Vec<Vec<(&'a str, usize)>>;

fn solve2(inp: &Vec<String>) -> usize {
    let mut boxes: Boxes = vec![vec![]; 256];
    for op in inp {
        if op.ends_with('-') {
            let lbl = &op[..op.len()-1];
            remove(&mut boxes, lbl);
        } else {
            let mut parts = op.split('=');
            let lbl = parts.next().unwrap();
            let val = parts.next().unwrap().parse::<usize>().unwrap();
            put(&mut boxes, lbl, val);
        }
    }
    boxes.iter().enumerate().map(|(i, b)| {
        b.iter().enumerate().map(|(j, &(_, v))| (j+1) * v).sum::<usize>() * (i+1)
    }).sum()
}

fn remove(boxes: &mut Boxes, lbl: &str) {
    let b = hash(lbl);
    let idx_opt = boxes[b].iter().position(|&(l, _)| l == lbl);
    if let Some(idx) = idx_opt {
        boxes[b].remove(idx);
    }
}

fn put<'a>(boxes: &mut Boxes<'a>, lbl: &'a str, val: usize) {
    let b = hash(lbl);
    let idx_opt = boxes[b].iter().position(|&(l, _)| l == lbl);
    if let Some(idx) = idx_opt {
        boxes[b][idx] = (lbl, val);
    } else {
        boxes[b].push((lbl, val));
    }
}

fn main() {
    let inp = parse("inputs/day15.txt");
    println!("sol1: {}", solve1(&inp));
    println!("sol2: {}", solve2(&inp));

}