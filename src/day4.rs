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

fn read() -> Vec<Vec<char>> {
    let lines = read_lines("day4.input").unwrap();
    lines
        .filter_map(|line| {
            let line = line.ok()?;
            if line.is_empty() {
                None
            } else {
                Some(line.chars().collect())
            }
        })
        .collect()
}
pub fn a() {
    fn count<'a>(search: impl Iterator<Item = impl Iterator<Item = &'a char>>) -> i32 {
        search
            .map(|line| {
                line.tuple_windows()
                    .filter(|w| (&'X', &'M', &'A', &'S') == *w || *w == (&'S', &'A', &'M', &'X'))
                    .count()
            })
            .map(i32::try_from)
            .map(Result::unwrap)
            .sum::<i32>()
    }
    let block = read();
    let len0 = block[0].len();
    let len = block.len();

    let horizontal = block.iter().map(IntoIterator::into_iter);
    let vertical = (0..len0).map(|i| block.iter().map(move |line| &line[i]));
    let diagonal_ul_dr = (0..len0 + len - 1).map(|i| {
        block
            .iter()
            .enumerate()
            .take(i + 1)
            .tail(len + len0 - i - 1)
            .map(move |(j, line)| &line[i - j])
    });
    let diagonal_ur_dl = (0..len0 + len - 1).map(|i| {
        block
            .iter()
            .enumerate()
            .tail(i + 1)
            .take(len + len0 - i - 1)
            .map(move |(j, line)| &line[i + j + 1 - len])
    });
    let res = count(horizontal) + count(vertical) + count(diagonal_ul_dr) + count(diagonal_ur_dl);
    println!("{res}");
}

pub fn b() {
    let block = read();
    let len0 = block[0].len();
    let len = block.len();
    let res = block
        .iter()
        .enumerate()
        .take(len - 1)
        .skip(1)
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .take(len0 - 1)
                .skip(1)
                .map(move |(j, c)| (i, j, *c))
        })
        .filter(|&(i, j, c)| {
            c == 'A'
                && ('M' == block[i - 1][j - 1] && 'S' == block[i + 1][j + 1]
                    || 'S' == block[i - 1][j - 1] && 'M' == block[i + 1][j + 1])
                && ('M' == block[i + 1][j - 1] && 'S' == block[i - 1][j + 1]
                    || 'S' == block[i + 1][j - 1] && 'M' == block[i - 1][j + 1])
        })
        .count();

    println!("{res}");
}
