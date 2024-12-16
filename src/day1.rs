use std::{
    fs,
    io::{self, BufRead},
};

pub fn a() {
    let (mut a, mut b) = read_a_b();
    a.sort_unstable();
    b.sort_unstable();
    println!(
        "{}",
        a.iter()
            .zip(b.iter())
            .map(|(a, b)| (a - b).abs())
            .sum::<i32>()
    );
}
fn transpose<T, U: Iterator<Item = Vec<T>>>(v: U) -> Vec<Vec<T>> {
    // assert!(!v.is_empty());
    let mut iters: Vec<_> = v.map(IntoIterator::into_iter).collect();
    let len = iters[0].len();

    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read_a_b() -> (Vec<i32>, Vec<i32>) {
    let lines = read_lines("day1.input").unwrap();
    <[Vec<_>; 2]>::try_from(transpose(lines.filter_map(|line| {
        let line = line.ok()?;
        if line.is_empty() {
            return None;
        };
        Some(
            line.split("   ")
                .map(str::parse::<i32>)
                .map(Result::unwrap)
                .collect(),
        )
    })))
    .unwrap()
    .into()
}

pub fn b() {
    let (mut a, mut b) = read_a_b();
    a.sort_unstable();
    b.sort_unstable();
    let sum = a
        .iter()
        .map(|&s| <i32>::try_from(b.iter().filter(|&&r| r == s).count()).unwrap() * s)
        .sum::<i32>();
    println!("{sum}");
}
