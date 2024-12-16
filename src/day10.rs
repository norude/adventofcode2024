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

fn read() -> Vec<Vec<i32>> {
    let lines = read_lines("day10.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
                .collect()
        })
        .collect()
}

fn score(
    map: &Vec<Vec<i32>>,
    (x, y): (usize, usize),
    height: i32,
) -> Box<dyn Iterator<Item = (usize, usize)>> {
    if height == 9 {
        return Box::new(std::iter::once((x, y)));
    };

    let mut res: Box<dyn Iterator<Item = (usize, usize)>> = Box::new(std::iter::empty());
    if x > 0 && map[y][x - 1] == height + 1 {
        res = Box::new(res.chain(score(map, (x - 1, y), height + 1)));
    };
    if y > 0 && map[y - 1][x] == height + 1 {
        res = Box::new(res.chain(score(map, (x, y - 1), height + 1)));
    };
    if y + 1 < map.len() && map[y + 1][x] == height + 1 {
        res = Box::new(res.chain(score(map, (x, y + 1), height + 1)));
    };
    if x + 1 < map[0].len() && map[y][x + 1] == height + 1 {
        res = Box::new(res.chain(score(map, (x + 1, y), height + 1)));
    };
    res
}

pub fn a() {
    let map = read();
    let s: usize = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|s| *s.1 == 0)
                .map(move |x| (x.0, y))
        })
        .map(|pos| score(&map, pos, 0).unique().count())
        .sum();
    println!("{s}");
}
fn rating(map: &Vec<Vec<i32>>, (x, y): (usize, usize), height: i32) -> usize {
    if height == 9 {
        return 1;
    };

    let mut res = 0;
    if x > 0 && map[y][x - 1] == height + 1 {
        res += rating(map, (x - 1, y), height + 1);
    };
    if y > 0 && map[y - 1][x] == height + 1 {
        res += rating(map, (x, y - 1), height + 1);
    };
    if y + 1 < map.len() && map[y + 1][x] == height + 1 {
        res += rating(map, (x, y + 1), height + 1);
    };
    if x + 1 < map[0].len() && map[y][x + 1] == height + 1 {
        res += rating(map, (x + 1, y), height + 1);
    };
    res
}
pub fn b() {
    let map = read();
    let s: usize = map
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            line.iter()
                .enumerate()
                .filter(|s| *s.1 == 0)
                .map(move |x| (x.0, y))
        })
        .map(|pos| rating(&map, pos, 0))
        .sum();
    println!("{s}");
}
