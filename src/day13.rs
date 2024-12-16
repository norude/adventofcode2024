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

type Pos = (i128, i128);
fn read() -> Vec<(Pos, Pos, Pos)> {
    let lines = read_lines("day13.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .tuples()
        .map(|(a, b, prize)| {
            (
                a.strip_prefix("Button A: X+")
                    .unwrap()
                    .split(", Y+")
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_tuple()
                    .unwrap(),
                b.strip_prefix("Button B: X+")
                    .unwrap()
                    .split(", Y+")
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_tuple()
                    .unwrap(),
                prize
                    .strip_prefix("Prize: X=")
                    .unwrap()
                    .split(", Y=")
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect_tuple()
                    .unwrap(),
            )
        })
        .collect()
}

pub fn a() {
    let s: i128 = read()
        .iter()
        .map(|((ax, ay), (bx, by), (rx, ry))| {
            let r = by * ax - ay * bx;
            let b = ry * ax - rx * ay;
            let a = rx * by - ry * bx;
            i128::from(a % r == 0) * i128::from(b % r == 0) * (b / r + (a / r) * 3)
        })
        .sum();
    println!("{s}");
}
pub fn b() {
    let s: i128 = read()
        .iter()
        .map(|((ax, ay), (bx, by), (rx, ry))| {
            let err = 10_000_000_000_000;
            let (rx, ry) = (err + rx, err + ry);
            let r = by * ax - ay * bx;
            let b = ry * ax - rx * ay;
            let a = rx * by - ry * bx;
            i128::from(a % r == 0) * i128::from(b % r == 0) * (b / r + (a / r) * 3)
        })
        .sum();
    println!("{s}");
}
