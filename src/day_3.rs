use std::{collections::HashSet, env::args, fs};

pub fn main() {
    const LOW_A: u32 = 'a' as u32;
    let mut a = 0;
    let text = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
        .unwrap()
        .replace('\r', "");
    let lines = text.split('\n').collect::<Vec<_>>();
    for s in &lines {
        let (x, y) = s.split_at(s.len() / 2);
        let xs = x.chars().collect::<HashSet<char>>();
        let ys = y.chars().collect::<HashSet<char>>();
        let intersect = xs.intersection(&ys).collect::<Vec<_>>();
        let mut sum = 0;
        for is in intersect {
            if *is as u32 >= LOW_A {
                sum += *is as u32 - LOW_A + 1;
            } else {
                sum += *is as u32 - 'A' as u32 + 27;
            }
        }
        a += sum;
    }
    let mut b = 0;
    for group in lines.chunks(3) {
        let x = group[0].chars().collect::<HashSet<_>>();
        let y = group[1].chars().collect::<HashSet<_>>();
        let z = group[2].chars().collect::<HashSet<_>>();
        let first_intersect = x.intersection(&y).cloned().collect::<HashSet<_>>();
        let badge = first_intersect.intersection(&z).nth(0).unwrap();
        let mut sum = 0;
        if *badge as u32 >= LOW_A {
            sum += *badge as u32 - LOW_A + 1;
        } else {
            sum += *badge as u32 - 'A' as u32 + 27;
        }
        b += sum;
    }
    println!("A: {}\nB: {}", a, b);
}
