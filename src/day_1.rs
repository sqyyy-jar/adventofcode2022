use std::{env::args, fs};

pub fn main() {
    let text = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
        .unwrap()
        .trim_end()
        .replace('\r', "");
    let mut sums: Vec<_> = text
        .split("\n\n")
        .map(|it| {
            it.split("\n")
                .map(|it| it.parse::<i64>().unwrap())
                .sum::<i64>()
        })
        .collect();
    sums.sort();
    println!(
        "A: {}\nB: {}",
        sums.iter().max().unwrap(),
        sums.iter().rev().take(3).sum::<i64>(),
    );
}
