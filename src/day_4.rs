use std::{env::args, fs};

pub fn main() {
    let text = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
        .unwrap()
        .replace('\r', "");
    let mut a = 0;
    let mut b = 0;
    for s in text.split('\n') {
        let mut ranges = s.split(',');
        let mut x = ranges.next().unwrap().split('-');
        let mut y = ranges.next().unwrap().split('-');
        let first =
            x.next().unwrap().parse::<u32>().unwrap()..=x.next().unwrap().parse::<u32>().unwrap();
        let second =
            y.next().unwrap().parse::<u32>().unwrap()..=y.next().unwrap().parse::<u32>().unwrap();
        if first.clone().all(|it| second.contains(&it))
            || second.clone().all(|it| first.contains(&it))
        {
            a += 1;
        }
        if first.clone().any(|it| second.contains(&it))
            || second.clone().any(|it| first.contains(&it))
        {
            b += 1;
        }
    }
    println!("A: {}\nB: {}", a, b);
}
