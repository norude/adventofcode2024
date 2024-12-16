use std::{
    fs,
    io::{self, BufRead},
};use cached::proc_macro::cached;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> Vec<i128> {
    read_lines("day11.input")
        .unwrap()
        .next()
        .unwrap()
        .unwrap()
        .split(' ')
        .map(str::parse)
        .map(Result::unwrap)
        .collect()
}

#[cached]
fn count(stone: i128, times: i128) -> i128 {
    if times == 0 {
        return 1;
    };
    let times = times - 1;
    if stone == 0 {
        count(1, times)
    } else if stone.ilog10() % 2 == 0 {
        count(stone * 2024, times)
    } else {
        let pow = 10i128.pow((stone.ilog10() + 1) / 2);
        count(stone / pow, times) + count(stone % pow, times)
    }
}

pub fn a() {
    let stones = read();
    let s: i128 = stones.into_iter().map(|stone| count(stone, 25)).sum();

    println!("{s}");
}
pub fn b() {
    let stones = read();
    let s: i128 = stones.into_iter().map(|stone| count(stone, 75)).sum();

    println!("{s}");
}
