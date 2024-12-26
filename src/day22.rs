use itertools::Itertools;
use std::{
    fs,
    collections::HashMap,
    io::{self, BufRead},
};

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> Vec<i128> {
    let lines = read_lines("day22.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|a| a.parse())
        .map(Result::unwrap)
        .collect()
}
const fn next(mut secret: i128) -> i128 {
    secret ^= secret << 6;
    secret = secret.rem_euclid(16_777_216);
    secret ^= secret >> 5;
    secret = secret.rem_euclid(16_777_216);
    secret ^= secret << 11;
    secret = secret.rem_euclid(16_777_216);
    secret
}

pub fn a() {
    let secrets = read();
    let res = secrets
        .into_iter()
        .map(|mut secret| {
            for _ in 0..2000 {
                secret = next(secret);
            }
            secret
        })
        .sum::<i128>();
    println!("{res}");
}
pub fn b() {
    let secrets = read();
    let monke = secrets
        .into_iter()
        .map(|mut secret| {
            (0..2000)
                .map(|_| {
                    secret = next(secret);
                    secret % 10
                })
                .collect_vec()
        })
        .map(|prices| {
            prices
                .iter()
                .scan(0, |state, a| {
                    let prev = *state;
                    *state = *a;
                    Some(a - prev)
                })
                .skip(1)
                .zip(prices.iter().skip(1).copied())
                .collect_vec()
        })
        .collect_vec();

    let mut map = HashMap::new();
    let default = monke.iter().map(|_| None).collect_vec();
    for (idx, p) in monke.iter().enumerate() {
        for (a, b, c, d) in p.iter().tuple_windows() {
            map.entry((a.0, b.0, c.0, d.0))
                .or_insert_with(|| default.clone())[idx]
                .get_or_insert(d.1);
        }
    }
    let res = map
        .values()
        .map(|x| x.iter().filter_map(|&x| x).sum::<i128>())
        .max()
        .unwrap();
    println!("{res}");
}
