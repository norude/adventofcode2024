use std::{
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

fn read() -> Vec<Vec<Vec<bool>>> {
    let lines = read_lines("day25.input").unwrap();
    let chunks = lines.map(Result::unwrap).chunks(8);
    chunks
        .into_iter()
        .map(|chunk| {
            chunk
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c == '#').collect())
                .collect()
        })
        .collect()
}

pub fn a() {
    let patts = read();
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    for patt in patts {
        let typ = if patt[0].iter().all(|&x| x) {
            &mut locks
        } else {
            &mut keys
        };
        let p = (0..5)
            .map(|i| patt.iter().filter(|&x| x[i]).count() - 1)
            .collect_vec();
        typ.push(p);
    }

    let s = locks
        .iter()
        .flat_map(|lock| {
            keys.iter()
                .filter(move |key| lock.iter().zip(key.iter()).all(|(a, b)| a + b <= 5))
        })
        .count();

    println!("{s}");
}
