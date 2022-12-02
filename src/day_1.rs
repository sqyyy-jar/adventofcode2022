use std::{env::args, fs};

pub fn main() {
    println!(
        "A: {}\nB: {}",
        fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
            .unwrap()
            .replace('\r', "")
            .split("\n\n")
            .map(|it| {
                it.split("\n")
                    .map(|it| it.parse::<i64>().unwrap())
                    .sum::<i64>()
            })
            .max()
            .unwrap(),
        {
            let mut v = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
                .unwrap()
                .replace('\r', "")
                .split("\n\n")
                .map(|it| {
                    it.split("\n")
                        .map(|it| it.parse::<i64>().unwrap())
                        .sum::<i64>()
                })
                .collect::<Vec<i64>>();
            v.sort();
            v.drain(v.len() - 3..).sum::<i64>()
        }
    );
}
