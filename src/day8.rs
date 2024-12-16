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

fn read() -> Vec<Vec<Option<char>>> {
    let lines = read_lines("day8.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|x| (x != '.').then_some(x)).collect())
        .collect()
}

pub fn a() {
    let board = read();
    let len = <i64>::try_from(board.len()).unwrap();
    let len1 = <i64>::try_from(board[0].len()).unwrap();
    let s = board
        .iter()
        .flat_map(IntoIterator::into_iter)
        .filter_map(|&a| a)
        .unique()
        .flat_map(|freq| {
            board
                .iter()
                .enumerate()
                .flat_map(move |(x, v)| {
                    v.iter()
                        .enumerate()
                        .filter(move |&x| x.1 == &Some(freq))
                        .map(move |(y, _)| (x, y))
                })
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let [x1, y1, x2, y2]: [i64; 4] =
                        [x1, y1, x2, y2].map(TryInto::try_into).map(Result::unwrap);
                    [(2 * x1 - x2, 2 * y1 - y2), (2 * x2 - x1, 2 * y2 - y1)]
                })
                .filter(|&(x, y)| 0 <= x && x < len && 0 <= y && y < len1)
        })
        .unique()
        .count();
    println!("{s}");
}
pub fn b() {
    let board = read();
    let len = <i64>::try_from(board.len()).unwrap();
    let len1 = <i64>::try_from(board[0].len()).unwrap();
    let s = board
        .iter()
        .flat_map(IntoIterator::into_iter)
        .filter_map(|&a| a)
        .unique()
        .flat_map(|freq| {
            board
                .iter()
                .enumerate()
                .flat_map(move |(x, v)| {
                    v.iter()
                        .enumerate()
                        .filter(move |&x| x.1 == &Some(freq))
                        .map(move |(y, _)| (x, y))
                })
                .tuple_combinations()
                .flat_map(|((x1, y1), (x2, y2))| {
                    let [x1, y1, x2, y2]: [i64; 4] =
                        [x1, y1, x2, y2].map(TryInto::try_into).map(Result::unwrap);
                    let diff = (x1 - x2, y1 - y2);
                    (-len..=len).map(move |i| (x2 + i * diff.0, y2 + i * diff.1))
                })
                .filter(|&(x, y)| 0 <= x && x < len && 0 <= y && y < len1)
        })
        .unique()
        .count();
    println!("{s}");
}
