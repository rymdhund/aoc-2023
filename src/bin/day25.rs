use itertools::Itertools;

fn parse(file: &str) -> Vec<Vec<usize>> {
    let mut names: Vec<String> = vec![];
    let mut edge_list = vec![];

    std::fs::read_to_string(file).unwrap().trim().lines().for_each(|line| {
        let mut parts = line.split(": ");
        let lhs = parts.next().unwrap().to_string();
        if !names.contains(&lhs) {
            names.push(lhs.to_string());
            edge_list.push(vec![]);
        }
        let lhs_idx = names.iter().position(|n| n == &lhs).unwrap();
        for rhs in parts.next().unwrap().split(' ') {
            if !names.contains(&rhs.to_string()) {
                names.push(rhs.to_string());
                edge_list.push(vec![]);
            }

            let rhs_idx = names.iter().position(|n| n == rhs).unwrap();
            edge_list[lhs_idx].push(rhs_idx);
            edge_list[rhs_idx].push(lhs_idx);
        }
    });
    edge_list
}

fn solve1(edges: &Vec<Vec<usize>>) -> usize {
    loop {
        let (edges, res) = karger(edges);
        if edges == 3 {
            return res;
        }
    }
}

struct Subset {
    parent: usize,
    rank: usize,
}

fn karger(adj_list: &Vec<Vec<usize>>) -> (usize, usize) {
    let edges: Vec<(usize, usize)> = adj_list.iter().enumerate().flat_map(|(i, conns)| {
        conns.iter().filter_map(move |&j| if i < j { Some((i, j)) } else { None })
    }).collect();

    let mut subsets = (0..adj_list.len()).map(|n| Subset { parent: n, rank: 0}).collect_vec();
    let mut num_vertices = subsets.len();

    while num_vertices > 2 {
        let i = rand::random::<usize>() % edges.len();
        let subset1 = find(&mut subsets, edges[i].0);
        let subset2 = find(&mut subsets, edges[i].1);

        if subset1 == subset2 {
            continue;
        }
        num_vertices -= 1;
        mk_union(&mut subsets, subset1, subset2);
    }

    let mut cutedges = 0;
    for edge in edges {
        let subset1 = find(&mut subsets, edge.0);
        let subset2 = find(&mut subsets, edge.1);
        if subset1 != subset2 {
            cutedges += 1;
        }
    }

    let cnt = subsets.iter().filter(|s| s.parent == subsets[0].parent).count();
    (cutedges, cnt * (subsets.len() - cnt))
}

fn find(subsets: &mut Vec<Subset>, i: usize) -> usize {
    if subsets[i].parent != i {
        subsets[i].parent = find(subsets, subsets[i].parent);
    }
    subsets[i].parent
}

fn mk_union(subsets: &mut Vec<Subset>, x: usize, y: usize) {
    let xroot = find(subsets, x);
    let yroot = find(subsets, y);

    if subsets[xroot].rank < subsets[yroot].rank {
        subsets[xroot].parent = yroot;
    } else if subsets[xroot].rank < subsets[yroot].rank {
        subsets[yroot].parent = xroot;
    } else {
        subsets[yroot].parent = xroot;
        subsets[xroot].rank += 1;
    }
}

fn main() {
    let test = parse("inputs/day25_ex.txt");
    println!("example1: {}", solve1(&test));

    let inp = parse("inputs/day25.txt");
    println!("sol1: {}", solve1(&inp));
}