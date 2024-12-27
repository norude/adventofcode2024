use std::{
    cmp::Ordering,
    fs,
    io::{self, stdin, stdout, BufRead, Read, Write},
};

use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn read() -> Vec<((i32, i32), (i32, i32))> {
    let lines = read_lines("day14.input").unwrap();
    lines
        .map(Result::unwrap)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.strip_prefix("p=")
                .unwrap()
                .split(" v=")
                .map(|x| {
                    x.split(',')
                        .map(str::parse)
                        .map(Result::unwrap)
                        .collect_tuple()
                        .unwrap()
                })
                .collect_tuple()
                .unwrap()
        })
        .collect()
}

pub fn a() {
    let robots = read();
    let (height, width) = (103, 101);
    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for ((px, py), (vx, vy)) in robots {
        use Ordering as Ord;
        let px = (px + vx * 100).rem_euclid(width);
        let py = (py + vy * 100).rem_euclid(height);
        match ((px * 2).cmp(&(width - 1)), (py * 2).cmp(&(height - 1))) {
            (Ord::Less, Ord::Less) => a += 1,
            (Ord::Less, Ord::Greater) => c += 1,
            (Ord::Greater, Ord::Less) => b += 1,
            (Ord::Greater, Ord::Greater) => d += 1,
            (_, Ord::Equal) | (Ord::Equal, _) => (),
        };
        // println!("{px} {py}: {r}");
    }
    println!("{}", a * b * c * d);
}

const HUMAN: bool = false;

pub fn b() {
    const HEIGHT: usize = 103;
    const WIDTH: usize = 101;
    let mut skipped = 0;
    let robots = read();
    'l: for steps in 0.. {
        let mut grid = [[false; WIDTH]; HEIGHT];

        #[allow(clippy::cast_possible_wrap, clippy::cast_possible_truncation)]
        for &((px, py), (vx, vy)) in &robots {
            grid[(py + vy * steps).rem_euclid(HEIGHT as i32) as usize]
                [(px + vx * steps).rem_euclid(WIDTH as i32) as usize] = true;
        }

        let intresting = grid
            .into_iter()
            .map(|x| {
                x.into_iter()
                    .tuple_windows()
                    .any(|(a, b, c, d)| b && a && c && d)
            })
            .tuple_windows()
            .any(|(a, b)| a && b);

        if intresting {
            if HUMAN {
                for row in grid {
                    println!("{}", row.map(|x| if x { '█' } else { ' ' }).iter().join(""));
                }
                {
                    let msg: &String =
                    &format!("Press Enter to continue, <any key>+Enter to stop, {steps} steps already passed:");
                    let mut stdout = stdout();
                    stdout.write_all(msg.as_bytes()).unwrap();
                    stdout.flush().unwrap();
                    let r = stdin().read(&mut [0, 0]).unwrap();
                    if r == 2 {
                        break 'l;
                    }
                };
            } else {
                if skipped == 1 {
                    println!("{steps}");
                    break 'l;
                }
                skipped += 1;
            }
        }
    }
}
