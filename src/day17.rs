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

fn read() -> (i64, i64, i64, Vec<i64>) {
    let mut lines = read_lines("day17.input")
        .unwrap()
        .map(Result::unwrap)
        .filter(|line| !line.is_empty());
    let (a, b, c) = lines.by_ref().take(3).collect_tuple().unwrap();
    (
        a.strip_prefix("Register A: ").unwrap().parse().unwrap(),
        b.strip_prefix("Register B: ").unwrap().parse().unwrap(),
        c.strip_prefix("Register C: ").unwrap().parse().unwrap(),
        lines
            .next()
            .unwrap()
            .strip_prefix("Program: ")
            .unwrap()
            .split(',')
            .map(str::parse)
            .map(Result::unwrap)
            .collect(),
    )
}

const fn select_op(inst: i64, op: i64, a: i64, b: i64, c: i64) -> i64 {
    if matches!(inst, 0 | 2 | 5 | 6 | 7) {
        match op {
            4 => a,
            5 => b,
            6 => c,
            op => op,
        }
    } else {
        op
    }
}

pub fn a() {
    let (mut a, mut b, mut c, instructions) = read();
    let mut ip = 0;
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    while let (Some(&inst), Some(&op)) = (
        instructions.get(ip as usize),
        instructions.get(ip as usize + 1),
    ) {
        let op = select_op(inst, op, a, b, c);
        ip += 2;
        match inst {
            0 => a >>= op,
            1 => b ^= op,
            2 => b = op & 7,
            4 => b ^= c,
            6 => b = a >> op,
            7 => c = a >> op,
            3 => {
                if a != 0 {
                    ip = op;
                }
            }
            5 => print!("{},", op & 7),
            _ => unreachable!(),
        }
    }
    println!("\x1b[1D ");
}
pub fn find(a: i64, idx: usize, instructions: &[i64]) -> Option<i64> {
    if idx == instructions.len() {
        return Some(a);
    }
    let n = instructions[instructions.len() - 1 - idx];
    (0..8).find_map(|ap| {
        let a = a << 3 | ap;
        let (mut b, mut c) = (0, 0);
        for (&inst, &op) in instructions.iter().dropping_back(2).tuples() {
            let op = select_op(inst, op, a, b, c);
            match inst {
                0 => assert_eq!(op, 3),
                1 => b ^= op,
                2 => b = op & 7,
                4 => b ^= c,
                6 => b = a >> op,
                7 => c = a >> op,
                5 => {
                    if n == op & 7 {
                        return find(a, idx + 1, instructions);
                    }
                    return None;
                }
                _ => unreachable!(),
            }
        }
        unreachable!()
    })
}

pub fn b() {
    let (_, _, _, instructions) = read();
    assert_eq!(instructions[instructions.len() - 1], 0);
    assert_eq!(instructions[instructions.len() - 2], 3);
    let a = find(0, 0, &instructions).unwrap();
    println!("{a}");
}
