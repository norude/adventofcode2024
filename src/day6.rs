use std::{
    collections::HashSet,
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

#[derive(PartialEq, Eq, Debug, Clone)]
enum Cell {
    Open,
    Obstacle,
    Guard,
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
impl Direction {
    const fn step(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (x, y - 1),
            Self::Down => (x, y + 1),
            Self::Left => (x - 1, y),
            Self::Right => (x + 1, y),
        }
    }
    const fn rotate_right(self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }
    const fn unstep(self, (x, y): (usize, usize)) -> (usize, usize) {
        match self {
            Self::Up => (x, y + 1),
            Self::Down => (x, y - 1),
            Self::Left => (x + 1, y),
            Self::Right => (x - 1, y),
        }
    }
}
fn read() -> (Vec<Vec<Cell>>, (usize, usize)) {
    let lines = read_lines("day6.input").unwrap();
    let mut gx = None;
    let mut gy = None;

    (
        lines
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .enumerate()
            .map(|(y, line)| {
                line.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '.' => Cell::Open,
                        '#' => Cell::Obstacle,
                        '^' => {
                            gx = Some(x + 1);
                            gy = Some(y + 1);
                            Cell::Guard
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        (gx.unwrap(), gy.unwrap()),
    )
}

pub fn a() {
    let (mut board, mut pos) = read();
    let mut dir = Direction::Up;
    pos = dir.step(pos);
    while let Some(cell) = board.get_mut(pos.1 - 1).and_then(|s| s.get_mut(pos.0 - 1)) {
        match cell {
            Cell::Open | Cell::Guard => {
                *cell = Cell::Guard;
            }
            Cell::Obstacle => {
                pos = dir.unstep(pos);
                dir = dir.rotate_right();
            }
        }
        pos = dir.step(pos);
    }
    let s = board
        .iter()
        .flat_map(|s| s.iter())
        .filter(|&c| c == &Cell::Guard)
        .count();
    println!("{s}");
}
pub fn b() {
    let (mut board, mut pos) = read();
    let guard_startpos = pos;
    let mut dir = Direction::Up;
    let mut states = HashSet::new();

    while let Some(cell) = board.get_mut(pos.1 - 1).and_then(|s| s.get_mut(pos.0 - 1)) {
        if cell == &Cell::Obstacle {
            pos = dir.unstep(pos);
            dir = dir.rotate_right();
        } else {
            states.insert(pos);
        }
        pos = dir.step(pos);
    }
    let s = states
        .iter()
        .filter(|&&obst_pos| {
            let mut pos = guard_startpos;
            let mut dir = Direction::Up;
            board[obst_pos.1 - 1][obst_pos.0 - 1] = Cell::Obstacle;
            let mut states = HashSet::new();

            while 0 < pos.0 && 0 < pos.1 {
                let Some(cell) = board.get(pos.1 - 1).and_then(|s| s.get(pos.0 - 1)) else {
                    break;
                };

                if cell == &Cell::Obstacle {
                    pos = dir.unstep(pos);
                    dir = dir.rotate_right();
                } else {
                    states.insert((pos, dir));
                }

                pos = dir.step(pos);
                if states.contains(&(pos, dir)) {
                    board[obst_pos.1 - 1][obst_pos.0 - 1] = Cell::Open;
                    return true;
                }
            }

            board[obst_pos.1 - 1][obst_pos.0 - 1] = Cell::Open;
            false
        })
        .count();
    println!("{s}");
}
