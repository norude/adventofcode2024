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

fn read() -> Vec<(i64, Vec<i64>)> {
    let lines = read_lines("day7.input").unwrap();

    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let mut s = line.split(": ");
            (
                s.next().unwrap().parse().unwrap(),
                s.next()
                    .unwrap()
                    .split(' ')
                    .map(str::parse)
                    .map(Result::unwrap)
                    .collect(),
            )
        })
        .collect()
}
fn corr(mut s: i64, mut v: &[i64]) -> bool {
    loop {
        let a = v[v.len() - 1];
        v = &v[..v.len() - 1];
        if v.is_empty() {
            return s == a;
        }
        if s % a == 0 {
            return corr(s - a, v) || corr(s / a, v);
        }
        s -= a;
    }
}
pub fn a() {
    let s = read()
        .iter()
        .filter(|(s, v)| corr(*s, v))
        .map(|x| x.0)
        .sum::<i64>();
    println!("{s}");
}
fn corr2(s: i64, mut v: &[i64]) -> bool {
    let a = v[v.len() - 1];
    v = &v[..v.len() - 1];
    if v.is_empty() {
        return s == a;
    }
    let mut res = corr2(s - a, v);
    if s % a == 0 {
        res |= corr2(s / a, v);
    }
    let pow = 10i64.pow(a.ilog10() + 1);
    if a == s % pow {
        res |= corr2(s / pow, v);
    }
    res
}
pub fn b() {
    let s = read()
        .iter()
        .filter(|(s, v)| corr2(*s, v))
        .map(|x| x.0)
        .sum::<i64>();
    println!("{s}");
}
