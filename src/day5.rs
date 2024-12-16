use itertools::Itertools;
use std::{
    fs,
    io::{self, BufRead},
};
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> (Vec<(i32, i32)>, Vec<Vec<i32>>) {
    let mut lines = read_lines("day5.input").unwrap().map(|l| l.unwrap());
    let rules: Vec<(i32, i32)> = lines
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| {
            l.split('|')
                .map(|s| s.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .collect();
    let rows: Vec<Vec<i32>> = lines
        .filter(|l| !l.is_empty())
        .map(|l| l.split(',').map(|s| s.parse::<i32>().unwrap()).collect())
        .collect();
    (rules, rows)
}

pub fn a() {
    let (rules, rows) = read();
    let s: i32 = rows
        .iter()
        .filter(|row| {
            rules
                .iter()
                .filter_map(|&(a, b)| {
                    Some((
                        row.iter().position(|&x| x == a)?,
                        row.iter().position(|&x| x == b)?,
                    ))
                })
                .all(|(a, b)| a < b)
        })
        .map(|row| row[(row.len() - 1) / 2])
        .sum();
    println!("{s}");
}
pub fn b() {
    let (rules, rows) = read();

    let is_proper = |row: &Vec<i32>| {
        rules
            .iter()
            .filter_map(|&(a, b)| {
                Some((
                    row.iter().position(|&x| x == a)?,
                    row.iter().position(|&x| x == b)?,
                ))
            })
            .all(|(a, b)| a < b)
    };

    let s: i32 = rows
        .iter()
        .filter(|v| !is_proper(v))
        .map(|row| {
            let mut row = row.clone();
            let mut ord = vec![row.pop().unwrap()];
            let mut idx = 0;
            while !row.is_empty() {
                idx = (idx + 1) % row.len();
                let item = row[idx % row.len()];
                let end = rules
                    .iter()
                    .filter(|b| b.0 == item)
                    .filter_map(|b| ord.iter().position(|&a| a == b.1))
                    .min();
                let start = rules
                    .iter()
                    .filter(|b| b.1 == item)
                    .filter_map(|b| ord.iter().position(|&a| a == b.0))
                    .max();
                match (start, end) {
                    (Some(start), None) if ord.len() - 1 == start => ord.push(item),
                    (None, Some(0)) => ord.insert(0, item),
                    (Some(start), Some(end)) if start + 1 == end => ord.insert(end, item),
                    _ => continue,
                };
                row.remove(idx % row.len());
            }
            assert!(is_proper(&ord), "{ord:?}");
            ord
        })
        .map(|row| row[(row.len() - 1) / 2])
        .sum();
    println!("{s}");
}
