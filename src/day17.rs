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

fn read() {
    let lines = read_lines("day17.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .collect()
}

pub fn a() {
    let s = read();
    println!("{s}");
}
pub fn b() {
    let s = read();
    println!("{s}");
}
