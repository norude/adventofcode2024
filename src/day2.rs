use std::{
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

fn read() -> impl Iterator<Item = Vec<i32>> {
    read_lines("day2.input").unwrap().filter_map(|s| {
        let s = s.ok()?;
        if s.is_empty() {
            return None;
        };
        Some(
            s.split(' ')
                .map(str::parse)
                .map(Result::unwrap)
                .collect::<Vec<i32>>(),
        )
    })
}

pub fn a() {
    let s = read()
        .filter(|v| {
            v.iter()
                .scan(0, |s, &x| {
                    let r = *s - x;

                    *s = x;
                    Some(r)
                })
                .skip(1)
                .scan(None, |s, x| {
                    Some(
                        x * if let Some(sign) = s {
                            *sign
                        } else {
                            let sign = if x > 0 { 1 } else { -1 };
                            *s = Some(sign);
                            sign
                        },
                    )
                })
                .all(|x| x > 0 && x < 4)
        })
        .count();
    println!("{s}");
}

pub fn b() {
    let s = read()
        .filter(|v| {
            v.iter()
                .enumerate()
                .map(|(idx, &_)| {
                    v.iter()
                        .enumerate()
                        .filter_map(move |(jdx, j)| if idx == jdx { None } else { Some(*j) })
                })
                .any(|v| {
                    v.scan(0, |s, x| {
                        let r = *s - x;

                        *s = x;
                        Some(r)
                    })
                    .skip(1)
                    .scan(None, |s, x| {
                        Some(
                            x * if let Some(sign) = s {
                                *sign
                            } else {
                                let sign = if x > 0 { 1 } else { -1 };
                                *s = Some(sign);
                                sign
                            },
                        )
                    })
                    .all(|x| x > 0 && x < 4)
                })
        })
        .count();
    println!("{s}");
}
