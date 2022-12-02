use std::{env::args, fs};

pub fn main() {
    println!(
        "A: {}\nB: {}",
        fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
            .unwrap()
            .replace('\r', "")
            .split('\n')
            .map(|it| {
                let it = it.split(' ').collect::<Vec<_>>();
                let a = it[0].chars().next().unwrap();
                let b = it[1].chars().next().unwrap();
                match a {
                    'A' => match b {
                        'X' => return 1 + 3,
                        'Y' => return 2 + 6,
                        'Z' => return 3 + 0,
                        _ => unreachable!(),
                    },
                    'B' => match b {
                        'X' => return 1 + 0,
                        'Y' => return 2 + 3,
                        'Z' => return 3 + 6,
                        _ => unreachable!(),
                    },
                    'C' => match b {
                        'X' => return 1 + 6,
                        'Y' => return 2 + 0,
                        'Z' => return 3 + 3,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>(),
        fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
            .unwrap()
            .replace('\r', "")
            .split('\n')
            .map(|it| {
                let it = it.split(' ').collect::<Vec<_>>();
                let a = it[0].chars().next().unwrap();
                let b = it[1].chars().next().unwrap();
                match a {
                    'A' => match b {
                        'X' => return 3 + 0,
                        'Y' => return 1 + 3,
                        'Z' => return 2 + 6,
                        _ => unreachable!(),
                    },
                    'B' => match b {
                        'X' => return 1 + 0,
                        'Y' => return 2 + 3,
                        'Z' => return 3 + 6,
                        _ => unreachable!(),
                    },
                    'C' => match b {
                        'X' => return 2 + 0,
                        'Y' => return 3 + 3,
                        'Z' => return 1 + 6,
                        _ => unreachable!(),
                    },
                    _ => unreachable!(),
                }
            })
            .collect::<Vec<u32>>()
            .iter()
            .sum::<u32>()
    );
}
