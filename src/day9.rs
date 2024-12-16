use std::{
    fs,
    io::{self, BufRead},
    iter::repeat_n,
};

use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> Vec<u64> {
    read_lines("day9.input")
        .unwrap()
        .last()
        .unwrap()
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap().into())
        .collect()
}

pub fn a() {
    let disk_map = read();
    let mut disk = disk_map
        .iter()
        .enumerate()
        .flat_map(|(idx, &i)| repeat_n((idx % 2 != 1).then_some(idx / 2), i.try_into().unwrap()));
    let mut checksum = 0;
    let mut idx = 0;
    loop {
        let Some(x) = disk.next() else { break };
        checksum +=
            idx * x.unwrap_or_else(|| disk.rfind(Option::is_some).expect("at least one").unwrap());
        idx += 1;
    }
    println!("{checksum:?}");
}
pub fn b() {
    let disk_map = read();
    let mut disk_map = disk_map
        .iter()
        .enumerate()
        .map(|(idx, &i)| ((idx % 2 != 1).then_some(idx / 2), i))
        .collect_vec();

    let mut idx = disk_map.len();
    let mut remaining = usize::MAX;
    loop {
        if idx == 0 {
            break;
        }
        idx -= 1;
        let Some(&x) = disk_map.get(idx) else {
            break;
        };
        let (Some(id), n) = x else { continue };
        if n == 0 {
            disk_map.remove(idx);
            continue;
        }
        if id >= remaining {
            continue;
        };
        if let Some(found) = disk_map
            .iter()
            .position(|&(id_, n_)| id_.is_none() && n_ >= n)
        {
            if found < idx {
                disk_map[idx].0 = None;
                disk_map[found].1 -= n;
                disk_map.insert(found, x);
            }
        };
        remaining = id;
    }
    let mut acc = 0;
    let mut idx = 0;
    for &(id, n) in &disk_map {
        if let Some(id) = id {
            acc += id as u64 * ((idx..idx + n).sum::<u64>());
        }
        idx += n;
    }
    println!("{acc}");
}
