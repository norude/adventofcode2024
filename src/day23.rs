use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> Vec<(String, String)> {
    let lines = read_lines("day23.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split('-')
                .map(ToString::to_string)
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn a() {
    let net = read();
    let key_names = net.iter().flat_map(|(a, b)| [a, b]).unique().collect_vec();
    let key_idxes = key_names
        .iter()
        .enumerate()
        .map(|(idx, &i)| (i, idx))
        .collect::<HashMap<_, _>>();
    let adj = {
        let mut adj = key_names
            .iter()
            .map(|_| key_names.iter().map(|_| false).collect_vec())
            .collect_vec();
        for (a, b) in &net {
            let (a, b) = (key_idxes[a], key_idxes[b]);
            adj[a][b] = true;
            adj[b][a] = true;
        }
        adj
    };

    let s = net
        .iter()
        .flat_map(|(a, b)| {
            let aidx = key_idxes[a];
            let bidx = key_idxes[b];
            adj[aidx]
                .iter()
                .zip(&adj[bidx])
                .enumerate()
                .filter_map(|(idx, (&a, &b))| (a && b).then_some(idx))
                .map(move |idx| [idx, aidx, bidx])
                .map(|x| x.into_iter().sorted().collect_tuple().unwrap())
        })
        .unique()
        .map(|(a, b, c)| (key_names[a], key_names[b], key_names[c]))
        .filter(|&(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count();
    println!("{s}");
}

fn is_clique(adj: &Vec<Vec<bool>>, nodes: &[usize]) -> bool {
    if nodes.len() < 2 {
        return true;
    };
    let a = nodes[0];
    let b = nodes[1];
    if !adj[a][b] {
        return false;
    };
    nodes[2..].iter().all(|&x| adj[a][x] && adj[b][x]) && is_clique(adj, &nodes[2..])
}
pub fn b() {
    let net = read();
    let key_names = net.iter().flat_map(|(a, b)| [a, b]).unique().collect_vec();
    let key_idxes = key_names
        .iter()
        .enumerate()
        .map(|(idx, &i)| (i, idx))
        .collect::<HashMap<_, _>>();
    let adj = {
        let mut adj = key_names
            .iter()
            .map(|_| key_names.iter().map(|_| false).collect_vec())
            .collect_vec();
        for (a, b) in &net {
            let (a, b) = (key_idxes[a], key_idxes[b]);
            adj[a][b] = true;
            adj[b][a] = true;
        }
        adj
    };

    let mut i = 3;
    let s = loop {
        i += 1;
        if let Ok(s) = net
            .iter()
            .flat_map(|(a, b)| {
                let aidx = key_idxes[a];
                let bidx = key_idxes[b];
                adj[aidx]
                    .iter()
                    .zip(&adj[bidx])
                    .enumerate()
                    .filter_map(|(idx, (&a, &b))| (a && b).then_some(idx))
                    .combinations(i - 2)
                    .filter(|comb| is_clique(&adj, comb))
                    .map(move |x| x.into_iter().chain([aidx, bidx]).sorted().collect_vec())
            })
            .unique()
            .exactly_one()
        {
            break s.iter().map(|&x| key_names[x]).sorted().join(",");
        };
    };

    println!("{s}");
}
