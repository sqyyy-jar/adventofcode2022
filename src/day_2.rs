use std::{env::args, fs};

pub fn main() {
    let mut ar = 0;
    let mut br = 0;
    let a_lookup = [[4, 8, 3], [1, 5, 9], [7, 2, 6]];
    let b_lookup = [[3, 4, 8], [1, 5, 9], [2, 6, 7]];
    fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
        .unwrap()
        .replace('\r', "")
        .split('\n')
        .for_each(|it| {
            let it = it.split(' ').collect::<Vec<_>>();
            let a = it[0].chars().next().unwrap() as u32 - 'A' as u32;
            let b = it[1].chars().next().unwrap() as u32 - 'X' as u32;
            ar += a_lookup[a as usize][b as usize];
            br += b_lookup[a as usize][b as usize];
        });
    println!("A: {}\nB: {}", ar, br);
}
