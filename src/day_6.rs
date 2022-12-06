#![allow(unused_assignments)]

use std::{collections::VecDeque, env::args, fs};

pub fn main() {
    let text = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap()).unwrap();
    let mut chars_a: Vec<_> = text.trim_end().chars().collect();
    chars_a.reverse();
    let mut chars_b = chars_a.clone();
    let mut a = VecDeque::new();
    let mut b = VecDeque::new();
    let mut ai = -1;
    let mut bi = -1;
    for _ in 0..4 {
        let c = chars_a.pop().unwrap();
        a.push_back(c);
    }
    for _ in 0..14 {
        let c = chars_b.pop().unwrap();
        b.push_back(c);
    }
    let mut i = 4;
    'outer: loop {
        if chars_a.len() <= 0 {
            panic!();
        }
        'inner: loop {
            let mut dq = Vec::new();
            for ac in &a {
                if dq.contains(ac) {
                    break 'inner;
                }
                dq.push(ac.clone());
            }
            ai = i as isize;
            break 'outer;
        }
        let c = chars_a.pop().unwrap();
        a.pop_front().unwrap();
        a.push_back(c.clone());
        i += 1;
    } 
    i = 14;
    'outer: loop {
        if chars_b.len() <= 0 {
            panic!();
        }
        'inner: loop {
            let mut dq = Vec::new();
            for bc in &b {
                if dq.contains(bc) {
                    break 'inner;
                }
                dq.push(bc.clone());
            }
            bi = i as isize;
            break 'outer;
        }
        let c = chars_b.pop().unwrap();
        b.pop_front().unwrap();
        b.push_back(c.clone());
        i += 1;
    }
    println!("A: {}\nB: {}", ai, bi);
}
