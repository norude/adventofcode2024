use core::panic;
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

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Cell {
    Wall,
    Box,
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
    const fn step_n(self, (x, y): Pos, n: i32) -> Pos {
        #[allow(
            clippy::cast_sign_loss,
            clippy::cast_possible_wrap,
            clippy::cast_possible_truncation
        )]
        match self {
            Self::Up => (x, (y as i32).wrapping_sub(n) as usize),
            Self::Down => (x, (y as i32).wrapping_add(n) as usize),
            Self::Left => ((x as i32).wrapping_sub(n) as usize, y),
            Self::Right => ((x as i32).wrapping_add(n) as usize, y),
        }
    }
}

type Pos = (usize,usize);

fn read() -> (Vec<Vec<Option<Cell>>>, Pos, Vec<Dir>) {
    let mut lines = read_lines("day15.input").unwrap().map(Result::unwrap);
    let mut pos = None;
    (
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => Some(Cell::Wall),
                        '.' => None,
                        'O' => Some(Cell::Box),
                        '@' => {
                            pos = Some((x, y));
                            None
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
        pos.expect("a robot"),
        lines
            .filter(|line| !line.is_empty())
            .flat_map(|line| {
                line.chars()
                    .map(|x| match x {
                        '>' => Dir::Right,
                        'v' => Dir::Down,
                        '<' => Dir::Left,
                        '^' => Dir::Up,
                        _ => panic!(),
                    })
                    .collect::<Vec<_>>()
            })
            .collect(),
    )
}

fn get<T>(grid: &[Vec<T>], (x, y): Pos) -> Option<&T> {
    grid.get(y).and_then(|row| row.get(x))
}
fn get_mut<T>(grid: &mut [Vec<T>], (x, y): Pos) -> Option<&mut T> {
    grid.get_mut(y).and_then(|row| row.get_mut(x))
}
pub fn a() {
    let (mut grid, mut pos, directions) = read();
    for dir in directions {
        for n in 1.. {
            let Some(next) = get(&grid, dir.step_n(pos, n)) else {
                break;
            };
            match next {
                Some(Cell::Wall) => break,
                Some(Cell::Box) => continue,
                None => {
                    *get_mut(&mut grid, dir.step_n(pos, n)).unwrap() =
                        *get(&grid, dir.step(pos)).unwrap();
                    pos = dir.step(pos);
                    *get_mut(&mut grid, pos).unwrap() = None;
                    break;
                }
            };
        }
    }

    let s: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|x| x.1 == &Some(Cell::Box))
                .map(move |x| x.0 + 100 * y)
        })
        .sum();
    println!("{s}");
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Cell2 {
    Wall,
    LBox,
    RBox,
}

fn check_push(grid: &Vec<Vec<Option<Cell2>>>, pos: Pos, dir: Dir) -> bool {
    let pos = dir.step(pos);
    let Some(next) = get(grid, pos) else {
        return false;
    };
    match (next, dir) {
        (Some(Cell2::Wall), _) => false,
        (None, _) => true,
        (Some(Cell2::LBox), Dir::Up | Dir::Down) => {
            check_push(grid, pos, dir) && check_push(grid, Dir::Right.step(pos), dir)
        }
        (Some(Cell2::RBox), Dir::Up | Dir::Down) => {
            check_push(grid, pos, dir) && check_push(grid, Dir::Left.step(pos), dir)
        }
        (Some(Cell2::LBox), Dir::Right) => check_push(grid, Dir::Right.step(pos), dir),
        (Some(Cell2::RBox), Dir::Left) => check_push(grid, Dir::Left.step(pos), dir),
        (Some(Cell2::LBox), Dir::Left) | (Some(Cell2::RBox), Dir::Right) => unreachable!(),
    }
}
fn push(grid: &mut Vec<Vec<Option<Cell2>>>, pos: Pos, dir: Dir) {
    let opos = pos;
    let orig = *get(grid, opos).unwrap();
    let pos = dir.step(pos);
    let Some(next) = get(grid, pos) else {
        unreachable!()
    };
    match (next, dir) {
        (None, _) => (),
        (Some(Cell2::LBox), Dir::Up | Dir::Down) => {
            push(grid, pos, dir);
            push(grid, Dir::Right.step(pos), dir);
        }
        (Some(Cell2::RBox), Dir::Up | Dir::Down) => {
            push(grid, pos, dir);
            push(grid, Dir::Left.step(pos), dir);
        }
        (Some(Cell2::LBox), Dir::Right) | (Some(Cell2::RBox), Dir::Left) => {
            push(grid, dir.step(pos), dir);
            *get_mut(grid, dir.step(pos)).unwrap() = *get(grid, pos).unwrap();
        }
        (Some(Cell2::Wall), _)
        | (Some(Cell2::LBox), Dir::Left)
        | (Some(Cell2::RBox), Dir::Right) => unreachable!(),
    };
    *get_mut(grid, pos).unwrap() = orig;
    *get_mut(grid, opos).unwrap() = None;
}
pub fn b() {
    let (grid, pos, directions) = read();
    let mut grid = grid
        .into_iter()
        .map(|line| {
            line.into_iter()
                .flat_map(|x| match x {
                    Some(Cell::Box) => [Some(Cell2::LBox), Some(Cell2::RBox)],
                    Some(Cell::Wall) => [Some(Cell2::Wall), Some(Cell2::Wall)],
                    None => [None, None],
                })
                .collect_vec()
        })
        .collect_vec();
    let mut pos = (pos.0 * 2, pos.1);

    for dir in directions {
        if check_push(&grid, pos, dir) {
            push(&mut grid, pos, dir);
            pos = dir.step(pos);
        }
    }

    let s: usize = grid
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|x| x.1 == &Some(Cell2::LBox))
                .map(move |x| x.0 + 100 * y)
        })
        .sum();
    println!("{s}");
}
