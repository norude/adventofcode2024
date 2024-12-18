use std::{
    collections::VecDeque,
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

fn read() -> Vec<(usize, usize)> {
    let lines = read_lines("day18.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|s| {
            s.split(',')
                .map(str::parse)
                .map(Result::unwrap)
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn a() {
    let mut fails = read().into_iter();
    let mut grid = [[true; 71]; 71];
    for (x, y) in fails.by_ref().take(1024) {
        grid[y][x] = false;
    }
    let mut now: VecDeque<_> = [(0, 0)].into();
    let mut next = VecDeque::new();
    let mut idx = 0;
    let s = 'o: loop {
        for &(x, y) in &now {
            if !grid[y][x] {
                continue;
            };
            grid[y][x] = false;
            if (x, y) == (grid[0].len() - 1, grid.len() - 1) {
                break 'o idx;
            }
            if y > 0 && grid[y - 1][x] {
                next.push_back((x, y - 1));
            }
            if x > 0 && grid[y][x - 1] {
                next.push_back((x - 1, y));
            }
            if y < grid.len() - 1 && grid[y + 1][x] {
                next.push_back((x, y + 1));
            }
            if x < grid[0].len() - 1 && grid[y][x + 1] {
                next.push_back((x + 1, y));
            }
        }
        now.clear();
        if let Some((x, y)) = fails.next() {
            grid[y][x] = false;
        }
        std::mem::swap(&mut next, &mut now);
        idx += 1;
    };
    println!("{s}");
}
pub fn b() {
    let fails = read().into_iter();
    let mut grid = [[true; 71]; 71];
    let mut res = None;
    'o: for (x, y) in fails {
        grid[y][x] = false;
        let grid2 = grid;
        let mut grid2 = grid2;
        let mut next: VecDeque<_> = [(0, 0)].into();
        while let Some((x, y)) = next.pop_front() {
            if !grid2[y][x] {
                continue;
            };
            grid2[y][x] = false;
            if (x, y) == (grid2[0].len() - 1, grid2.len() - 1) {
                continue 'o;
            }
            if y > 0 && grid2[y - 1][x] {
                next.push_back((x, y - 1));
            }
            if x > 0 && grid2[y][x - 1] {
                next.push_back((x - 1, y));
            }
            if y < grid2.len() - 1 && grid2[y + 1][x] {
                next.push_back((x, y + 1));
            }
            if x < grid2[0].len() - 1 && grid2[y][x + 1] {
                next.push_back((x + 1, y));
            }
        }
        res = Some((x, y));
        break;
    }
    let res = res.unwrap();
    println!("{},{}", res.0, res.1);
}
