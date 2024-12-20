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
type Pos = (usize, usize);
fn read() -> (Vec<Vec<bool>>, Pos, Pos) {
    let lines = read_lines("day20.input").unwrap();
    let mut start = None;
    let mut end = None;
    (
        lines
            .map(Result::unwrap)
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => true,
                        '.' => false,
                        'S' => {
                            start = Some((x, y));
                            false
                        }
                        'E' => {
                            end = Some((x, y));
                            false
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
        start.unwrap(),
        end.unwrap(),
    )
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
impl Dir {
    const fn step(self, (x, y): Pos) -> Pos {
        match self {
            Self::Up => (x, y.wrapping_sub(1)),
            Self::Down => (x, y.wrapping_add(1)),
            Self::Left => (x.wrapping_sub(1), y),
            Self::Right => (x.wrapping_add(1), y),
        }
    }
}
fn get<T>(track: &[Vec<T>], (x, y): Pos) -> Option<&T> {
    track.get(y).and_then(|row| row.get(x))
}
pub fn a() {
    let (track, start, end) = read();
    let mut path = vec![];
    let mut prev = start;
    let mut x = start;
    loop {
        path.push(x);
        if x == end {
            break;
        }
        let original_x = x;
        if Dir::Left.step(x) != prev && Some(&false) == get(&track, Dir::Left.step(x)) {
            x = Dir::Left.step(x);
        } else if Dir::Right.step(x) != prev && Some(&false) == get(&track, Dir::Right.step(x)) {
            x = Dir::Right.step(x);
        } else if Dir::Up.step(x) != prev && Some(&false) == get(&track, Dir::Up.step(x)) {
            x = Dir::Up.step(x);
        } else if Dir::Down.step(x) != prev && Some(&false) == get(&track, Dir::Down.step(x)) {
            x = Dir::Down.step(x);
        } else {
            unreachable!()
        }
        prev = original_x;
    }
    let mut track: Vec<Vec<Option<usize>>> = track
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x { Some(0) } else { None })
                .collect()
        })
        .collect();
    for (idx, x) in path.iter().enumerate() {
        track[x.1][x.0] = Some(idx);
    }
    let res = path
        .iter()
        .enumerate()
        .flat_map(|(idx, &x)| {
            get(&track, Dir::Left.step(Dir::Left.step(x)))
                .copied()
                .flatten()
                .and_then(|n| n.checked_sub(idx + 2))
                .into_iter()
                .chain(
                    get(&track, Dir::Right.step(Dir::Right.step(x)))
                        .copied()
                        .flatten()
                        .and_then(|n| n.checked_sub(idx + 2)),
                )
                .chain(
                    get(&track, Dir::Up.step(Dir::Up.step(x)))
                        .copied()
                        .flatten()
                        .and_then(|n| n.checked_sub(idx + 2)),
                )
                .chain(
                    get(&track, Dir::Down.step(Dir::Down.step(x)))
                        .copied()
                        .flatten()
                        .and_then(|n| n.checked_sub(idx + 2)),
                )
        })
        .filter(|&s| s >= 100)
        .count();
    println!("{res}");
}
pub fn b() {
    let (track, start, end) = read();
    let mut path = vec![];
    let mut prev = start;
    let mut x = start;
    loop {
        path.push(x);
        if x == end {
            break;
        }
        let original_x = x;
        if Dir::Left.step(x) != prev && Some(&false) == get(&track, Dir::Left.step(x)) {
            x = Dir::Left.step(x);
        } else if Dir::Right.step(x) != prev && Some(&false) == get(&track, Dir::Right.step(x)) {
            x = Dir::Right.step(x);
        } else if Dir::Up.step(x) != prev && Some(&false) == get(&track, Dir::Up.step(x)) {
            x = Dir::Up.step(x);
        } else if Dir::Down.step(x) != prev && Some(&false) == get(&track, Dir::Down.step(x)) {
            x = Dir::Down.step(x);
        } else {
            unreachable!()
        }
        prev = original_x;
    }
    let mut track: Vec<Vec<Option<usize>>> = track
        .iter()
        .map(|row| {
            row.iter()
                .map(|&x| if x { Some(0) } else { None })
                .collect()
        })
        .collect();
    for (idx, x) in path.iter().enumerate() {
        track[x.1][x.0] = Some(idx);
    }
    let ref_track = &track;
    let res = path
        .iter()
        .enumerate()
        .flat_map(move |(idx, &x)| {
            (0..=20).flat_map(move |i| {
                (0..=20 - i).flat_map(move |j| {
                    let skip_timespan = i + j;
                    [(i, j), (-i, j), (i, -j), (-i, -j)]
                        .into_iter()
                        .unique()
                        .filter_map(move |(i, j)| {
                            get(
                                ref_track,
                                (x.0.wrapping_add_signed(i), x.1.wrapping_add_signed(j)),
                            )
                            .copied()
                            .flatten()
                            .and_then(|n| n.checked_sub(idx.wrapping_add_signed(skip_timespan)))
                        })
                })
            })
        })
        .filter(|&s| s >= 100)
        .count();
    println!("{res}");
}
