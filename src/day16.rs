use core::panic;
use std::{
    collections::{hash_map::Entry, BinaryHeap, HashMap, HashSet, VecDeque},
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

fn read() -> (Vec<Vec<bool>>, Pos, Pos) {
    let lines = read_lines("day16.input").unwrap();
    let mut pos = None;
    let mut pos2 = None;
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
                            pos = Some((x, y));
                            false
                        }
                        'E' => {
                            pos2 = Some((x, y));
                            false
                        }
                        _ => panic!(),
                    })
                    .collect()
            })
            .collect(),
        pos.unwrap(),
        pos2.unwrap(),
    )
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash, PartialOrd, Ord)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}
type Pos = (usize, usize);
impl Dir {
    const fn step(self, (x, y): Pos) -> Pos {
        match self {
            Self::Up => (x, y.wrapping_sub(1)),
            Self::Down => (x, y.wrapping_add(1)),
            Self::Left => (x.wrapping_sub(1), y),
            Self::Right => (x.wrapping_add(1), y),
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
    const fn rotate_left(self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Left => Self::Down,
            Self::Down => Self::Right,
            Self::Right => Self::Up,
        }
    }
}
#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct State(usize, Pos, Dir);

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .0
            .cmp(&self.0)
            .then_with(|| self.1.cmp(&other.1))
            .then_with(|| self.2.cmp(&other.2))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
pub fn a() {
    let (grid, start, end) = read();
    let mut minmap: HashMap<(Pos, Dir), usize> = HashMap::new();
    let mut heap = BinaryHeap::new();
    minmap.insert((start, Dir::Right), 0);
    heap.push(State(0, start, Dir::Right));
    let cost = loop {
        let State(cost, pos, dir) = heap.pop().expect("a way out ");
        if pos == end {
            break cost;
        }
        [
            (cost + 1000, pos, dir.rotate_left()),
            (cost + 1000, pos, dir.rotate_right()),
        ]
        .into_iter()
        .chain(
            (grid
                .get(dir.step(pos).1)
                .and_then(|row| row.get(dir.step(pos).0))
                == Some(&false))
            .then_some((cost + 1, dir.step(pos), dir)),
        )
        .for_each(|(cost, pos, dir)| match minmap.entry((pos, dir)) {
            Entry::Occupied(mut occupied_entry) => {
                if &cost < occupied_entry.get() {
                    occupied_entry.insert(cost);
                    heap.push(State(cost, pos, dir));
                }
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(cost);
                heap.push(State(cost, pos, dir));
            }
        });
    };

    println!("{cost}");
}


pub fn b() {
    let (grid, start, end) = read();
    let mut minmap = HashMap::new();
    let mut heap = BinaryHeap::new();
    minmap.insert((start, Dir::Right), (0, vec![]));
    heap.push(State(0, start, Dir::Right));
    while let Some(State(cost, opos, odir)) = heap.pop() {
        [
            (cost + 1000, opos, odir.rotate_left()),
            (cost + 1000, opos, odir.rotate_right()),
        ]
        .into_iter()
        .chain(
            (grid
                .get(odir.step(opos).1)
                .and_then(|row| row.get(odir.step(opos).0))
                == Some(&false))
            .then_some((cost + 1, odir.step(opos), odir)),
        )
        .for_each(|(cost, pos, dir)| match minmap.entry((pos, dir)) {
            Entry::Occupied(mut occupied_entry) => {
                if cost < occupied_entry.get().0 {
                    occupied_entry.insert((cost, vec![(opos, odir)]));
                    heap.push(State(cost, pos, dir));
                }
                if cost == occupied_entry.get().0 {
                    occupied_entry.get_mut().1.push((opos, odir));
                }
            }
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert((cost, vec![(opos, odir)]));
                heap.push(State(cost, pos, dir));
            }
        });
    }

    let mut on_paths = HashSet::new();
    let mut prev = VecDeque::new();
    let ends = [
        (end, Dir::Left),
        (end, Dir::Right),
        (end, Dir::Up),
        (end, Dir::Down),
    ];
    let min_cost = ends.iter().map(|s|minmap[s].0).min().unwrap();
    prev.extend(ends.iter().filter(|s|minmap[s].0==min_cost));


    while let Some((pos, dir)) = prev.pop_front() {
        let Some((_, path)) = minmap.get(&(pos, dir)) else {
            continue;
        };
        on_paths.extend(path.iter().map(|x| x.0));
        prev.extend(path);
    }

    println!("{}", on_paths.len()+1);
}
