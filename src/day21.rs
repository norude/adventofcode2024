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

fn read() -> Vec<String> {
    let lines = read_lines("day21.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .collect()
}

fn pad0(num: &str) -> String {
    let mut res = String::new();
    let (mut x, mut y) = (2, 3);
    for n in num.chars() {
        let (i, j) = match n {
            '7' => (0, 0),
            '8' => (1, 0),
            '9' => (2, 0),
            '4' => (0, 1),
            '5' => (1, 1),
            '6' => (2, 1),
            '1' => (0, 2),
            '2' => (1, 2),
            '3' => (2, 2),
            '0' => (1, 3),
            'A' => (2, 3),
            _ => unreachable!(),
        };

        if x == 0 && j == 3 {
            res += &">".repeat(i - x);
            res += &"v".repeat(j - y);
        } else if y == 3 && i == 0 {
            res += &"^".repeat(y - j);
            res += &"<".repeat(x - i);
        } else {
            if x > i {
                res += &"<".repeat(x - i);
            }
            if j > y {
                res += &"v".repeat(j - y);
            }
            if y > j {
                res += &"^".repeat(y - j);
            }
            if i > x {
                res += &">".repeat(i - x);
            }
        }

        res.push('A');
        (x, y) = (i, j);
    }
    res
}

fn pad1(num: &str) -> Vec<String> {
    let mut res: Vec<String> = vec![];
    let mut prev = 'A';
    for n in num.chars() {
        res.push(
            match (prev, n) {
                ('^', '^') | ('v', 'v') | ('A', 'A') | ('<', '<') | ('>', '>') => "A",
                ('A', '^') | ('v', '<') | ('>', 'v') => "<A",
                ('^', 'A') | ('<', 'v') | ('v', '>') => ">A",
                ('v', '^') | ('>', 'A') => "^A",
                ('^', 'v') | ('A', '>') => "vA",
                ('^', '>') => "v>A",
                ('>', '^') => "<^A",
                ('A', 'v') => "<vA",
                ('v', 'A') => "^>A",

                ('<', '^') => ">^A",
                ('^', '<') => "v<A",
                ('<', '>') => ">>A",
                ('>', '<') => "<<A",
                ('<', 'A') => ">>^A",
                ('A', '<') => "v<<A",
                _ => unreachable!(),
            }
            .to_string(),
        );
        prev = n;
    }
    res
}

#[cached]
fn pad1_repeat(num: String, i: u32) -> u128 {
    if i == 0 {
        return num.len() as u128;
    }
    pad1(&num)
        .into_iter()
        .map(|num| pad1_repeat(num, i - 1))
        .sum()
}

pub fn a() {
    let res = read()
        .iter()
        .map(|num| {
            pad1_repeat(pad0(num), 2) * num.strip_suffix(&"A").unwrap().parse::<u128>().unwrap()
        })
        .sum::<u128>();
    println!("{res}");
}

pub fn b() {
    let res = read()
        .iter()
        .map(|num| {
            pad1_repeat(pad0(num), 25) * num.strip_suffix(&"A").unwrap().parse::<u128>().unwrap()
        })
        .sum::<u128>();
    println!("{res}");
}
