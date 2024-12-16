use regex::Regex;
use std::fs;
fn read() -> String {
    fs::read_to_string("day3.input").unwrap()
}

pub fn a() {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let input = read();
    let s: i32 = re
        .captures_iter(&input)
        .filter_map(|m| {
            Some(m.get(1)?.as_str().parse::<i32>().ok()? * m.get(2)?.as_str().parse::<i32>().ok()?)
        })
        .sum();

    println!("{s}");
}
pub fn b() {
    let input = read();
    let re = Regex::new(r"(?:mul\(([0-9]{1,3}),([0-9]{1,3})\))|do\(\)|don't\(\)").unwrap();
    let s: i32 = re
        .captures_iter(&input)
        .scan(true, |enable, c| {
            Some(match c.get(0).unwrap().as_str() {
                "do()" => {
                    *enable = true;
                    None
                }
                "don't()" => {
                    *enable = false;
                    None
                }
                _ => enable.then_some(c),
            })
        })
        .flatten()
        .filter_map(|m| {
            Some(m.get(1)?.as_str().parse::<i32>().ok()? * m.get(2)?.as_str().parse::<i32>().ok()?)
        })
        .sum();
    println!("{s}");
}
