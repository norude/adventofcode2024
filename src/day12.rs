use std::{
    collections::{HashSet, VecDeque},
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

fn read() -> Vec<Vec<char>> {
    let lines = read_lines("day12.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn floodfil(garden: &[Vec<char>], pos: (usize, usize), key: char) -> HashSet<(usize, usize)> {
    let mut visited = HashSet::new();
    let mut to_visit = VecDeque::new();
    to_visit.push_front(pos);
    while let Some(pos) = to_visit.pop_back() {
        // dfs so that to_visit doesn't contain duplicates
        if pos.0 != 0 && garden[pos.0 - 1][pos.1] == key && !visited.contains(&(pos.0 - 1, pos.1)) {
            to_visit.push_back((pos.0 - 1, pos.1));
        };
        if pos.1 != 0 && garden[pos.0][pos.1 - 1] == key && !visited.contains(&(pos.0, pos.1 - 1)) {
            to_visit.push_back((pos.0, pos.1 - 1));
        }
        if pos.1 != garden[0].len() - 1
            && garden[pos.0][pos.1 + 1] == key
            && !visited.contains(&(pos.0, pos.1 + 1))
        {
            to_visit.push_back((pos.0, pos.1 + 1));
        }
        if pos.0 != garden.len() - 1
            && garden[pos.0 + 1][pos.1] == key
            && !visited.contains(&(pos.0 + 1, pos.1))
        {
            to_visit.push_back((pos.0 + 1, pos.1));
        };
        visited.insert(pos);
    }
    visited
}
fn perimeter(part: &HashSet<(usize, usize)>) -> usize {
    let mut r = part.len() * 4;
    for pos in part {
        if part.contains(&(pos.0 + 1, pos.1)) {
            r -= 2;
        }
        if part.contains(&(pos.0, pos.1 + 1)) {
            r -= 2;
        }
    }
    r
}
pub fn a() {
    let garden = read();
    let mut done: HashSet<(usize, usize)> = HashSet::new();
    let mut parts = Vec::new();
    for (x, (y, &c)) in garden
        .iter()
        .enumerate()
        .flat_map(|(x, line)| line.iter().enumerate().map(move |r| (x, r)))
    {
        if !done.contains(&(x, y)) {
            let set = floodfil(&garden, (x, y), c);
            done.extend(&set);
            parts.push(set);
        }
    }
    let s: usize = parts.iter().map(|part| perimeter(part) * part.len()).sum();
    println!("{s}");
}

fn sides(part: &HashSet<(usize, usize)>) -> usize {
    let mut r = part.len() * 4;
    for pos in part {
        if part.contains(&(pos.0 + 1, pos.1)) {
            r -=
                4 - usize::from(
                    part.contains(&(pos.0 + 1, pos.1 + 1)) || part.contains(&(pos.0, pos.1 + 1)),
                ) - usize::from(
                    part.contains(&(pos.0 + 1, pos.1.wrapping_sub(1)))
                        || part.contains(&(pos.0, pos.1.wrapping_sub(1))),
                );
        }
        if part.contains(&(pos.0, pos.1 + 1)) {
            r -=
                4 - usize::from(
                    part.contains(&(pos.0 + 1, pos.1 + 1)) || part.contains(&(pos.0 + 1, pos.1)),
                ) - usize::from(
                    part.contains(&(pos.0.wrapping_sub(1), pos.1 + 1))
                        || part.contains(&(pos.0.wrapping_sub(1), pos.1)),
                );
        }
    }
    r
}

pub fn b() {
    let garden = read();
    let mut done: HashSet<(usize, usize)> = HashSet::new();
    let mut parts = Vec::new();
    for (x, (y, &c)) in garden
        .iter()
        .enumerate()
        .flat_map(|(x, line)| line.iter().enumerate().map(move |r| (x, r)))
    {
        if !done.contains(&(x, y)) {
            let set = floodfil(&garden, (x, y), c);
            done.extend(&set);
            parts.push(set);
        }
    }
    let s: usize = parts.iter().map(|part| sides(part) * part.len()).sum();
    println!("{s}");
}
