use std::{
    fs,
    io::{self, BufRead},
};

use cached::proc_macro::cached;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Color {
    White,
    Blue,
    Black,
    Red,
    Green,
}
impl Color {
    fn from_char(s: char) -> Self {
        match s {
            'w' => Self::White,
            'u' => Self::Blue,
            'r' => Self::Red,
            'g' => Self::Green,
            'b' => Self::Black,
            _ => unreachable!(),
        }
    }
}
fn read() -> (Vec<Vec<Color>>, Vec<Vec<Color>>) {
    let mut lines = read_lines("day19.input")
        .unwrap()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty());
    (
        lines
            .next()
            .unwrap()
            .split(", ")
            .map(|p| p.chars().map(Color::from_char).collect())
            .collect(),
        lines
            .map(|p| p.chars().map(Color::from_char).collect())
            .collect(),
    )
}

#[cached(key = "(usize,usize)", convert = "{(idx,didx)}")]
fn fits(idx: usize, didx: usize, designs: &[Vec<Color>], towels: &[Vec<Color>]) -> usize {
    let design = &designs[didx];
    if idx == design.len() {
        return 1;
    }
    (idx..design.len())
        .map(|i| {
            if towels.iter().any(|p| p == &design[idx..=i]) {
                fits(i + 1, didx, designs, towels)
            } else {
                0
            }
        })
        .sum()
}

pub fn a() {
    let (towels, designs) = read();
    let s = designs
        .iter()
        .enumerate()
        .filter(|design| fits(0, design.0, &designs, &towels) > 0)
        .count();
    println!("{s}");
}
pub fn b() {
    let (towels, designs) = read();
    let s: usize = designs
        .iter()
        .enumerate()
        .map(|design| fits(0, design.0, &designs, &towels))
        .sum();
    println!("{s}");
}
