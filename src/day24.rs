use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead},
};

use cached::{proc_macro::cached, Cached};
use itertools::Itertools;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<fs::File>>>
where
    P: AsRef<std::path::Path>,
{
    let file = fs::File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, PartialOrd, Ord)]
struct Idf(char, char, char);
impl std::fmt::Debug for Idf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}
impl std::fmt::Display for Idf {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum Gate {
    And,
    Or,
    Xor,
}

type Inputs = HashMap<Idf, bool>;

type Gates = HashMap<Idf, (Idf, Gate, Idf)>;

fn read() -> (Inputs, Gates) {
    fn idf(mut a: impl Iterator<Item = char>) -> Idf {
        Idf(a.next().unwrap(), a.next().unwrap(), a.next().unwrap())
    }
    let mut lines = read_lines("day24.input").unwrap().map(Result::unwrap);
    (
        lines
            .by_ref()
            .take_while(|line| !line.is_empty())
            .map(|line| (idf(line.chars()), line.ends_with('1')))
            .collect(),
        lines
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut s = line.split(' ');
                (
                    idf(s.next().unwrap().chars()),
                    match s.next().unwrap() {
                        "AND" => Gate::And,
                        "OR" => Gate::Or,
                        "XOR" => Gate::Xor,
                        _ => unreachable!(),
                    },
                    idf(s.next().unwrap().chars()),
                    idf(s.nth(1).unwrap().chars()),
                )
            })
            .map(|(a, b, c, d)| (d, (a, b, c)))
            .collect(),
    )
}

#[cached(key = "Idf", convert = "{idf}")]
fn eval(idf: Idf, gts: &Gates, inps: &Inputs) -> bool {
    if let Some(&b) = inps.get(&idf) {
        return b;
    }
    let (a, b, c) = gts[&idf];
    match b {
        Gate::And => eval(a, gts, inps) && eval(c, gts, inps),
        Gate::Or => eval(a, gts, inps) || eval(c, gts, inps),
        Gate::Xor => eval(a, gts, inps) ^ eval(c, gts, inps),
    }
}

pub fn a() {
    let (inps, gts) = read();
    let res = eval_z(&inps, &gts);
    println!("{res:?}");
}
const fn bit(x: u64, idx: u32) -> bool {
    1 == (x >> idx) & 1
}

fn inps_from_x_y(x: u64, y: u64) -> Inputs {
    assert_eq!(x >> 45, 0);
    assert_eq!(y >> 45, 0);
    (0..=44)
        .flat_map(|idx| [(idf('x', idx), bit(x, idx)), (idf('y', idx), bit(y, idx))])
        .collect()
}

const fn idf(r: char, idx: u32) -> Idf {
    Idf(
        r,
        char::from_digit(idx / 10, 10).unwrap(),
        char::from_digit(idx % 10, 10).unwrap(),
    )
}

fn eval_z(inps: &Inputs, gts: &Gates) -> u64 {
    EVAL.lock().unwrap().cache_clear();
    (0..=45)
        .map(|idx| u64::from(eval(idf('z', idx), gts, inps)) << idx)
        .fold(0, std::ops::BitOr::bitor)
}

#[allow(clippy::too_many_lines)]
pub fn b() {
    let (_, mut gts) = read();
    #[allow(clippy::many_single_char_names)]
    let desired_gts: Gates = (0..=44)
        .flat_map(|idx| {
            let x = idf('x', idx);
            let y = idf('y', idx);
            let z = idf('z', idx);
            let c = idf('c', idx);
            if idx == 0 {
                vec![(z, (x, Gate::Xor, y)), (c, (x, Gate::And, y))].into_iter()
            } else {
                let and = idf('a', idx);
                let xor = idf('r', idx);
                let int = idf('i', idx);
                let prev_c = idf('c', idx - 1);

                vec![
                    (and, (x, Gate::And, y)),
                    (xor, (x, Gate::Xor, y)),
                    (z, (prev_c, Gate::Xor, xor)),
                    (int, (prev_c, Gate::And, xor)),
                    (
                        if idx == 44 { idf('z', 45) } else { c },
                        (int, Gate::Or, and),
                    ),
                ]
                .into_iter()
            }
        })
        .collect();

    assert!((0..1000).all(|_| {
        let a = rand::random::<u64>() & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        let b = rand::random::<u64>() & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        let inps = inps_from_x_y(a, b);
        (a + b) == eval_z(&inps, &desired_gts)
    }));

    let mut dict: HashMap<Idf, Idf> = (0..=44)
        .flat_map(|idx| {
            [
                (idf('x', idx), idf('x', idx)),
                (idf('y', idx), idf('y', idx)),
            ]
        })
        .chain(std::iter::once((idf('z', 45), idf('z', 45))))
        .collect();

    let mut swapped = vec![];
    let mut cool = true;
    'cool: while cool {
        macro_rules! swap {
            ($a:ident,$b:ident) => {
                let (r, f) = ($a, $b);
                unsafe {
                    std::ptr::swap(gts.get_mut(&r).unwrap(), gts.get_mut(&f).unwrap());
                }
                cool = true;
                dict.remove(&r);
                dict.remove(&f);
                swapped.push(r);
                swapped.push(f);
                continue 'cool;
            };
        }
        cool = false;
        for (&r, &(a, g1, b)) in &gts.clone() {
            if !dict.contains_key(&r) && dict.contains_key(&a) && dict.contains_key(&b) {
                for (&f, &(c, g2, d)) in &desired_gts {
                    if g1 == g2
                        && (dict[&a] == c && dict[&b] == d || dict[&a] == d && dict[&b] == c)
                    {
                        if r.0 != 'z' && f.0 == 'z' {
                            swap!(r, f);
                        }
                        dict.insert(r, f);
                        cool = true;
                    } else if g1 == g2 && dict[&a] == c {
                        let dd = *dict.iter().find(|(_, &v)| v == d).unwrap().0;
                        swap!(b, dd);
                    }
                }
            }
        }
    }

    assert!((0..1000).all(|_| {
        let a = rand::random::<u64>() & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        let b = rand::random::<u64>() & 0b1111_1111_1111_1111_1111_1111_1111_1111_1111_1111_1111;
        let inps = inps_from_x_y(a, b);
        (a + b) == eval_z(&inps, &desired_gts)
    }));

    println!("{}", swapped.iter().sorted().join(","));
}
