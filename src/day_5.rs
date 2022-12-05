use std::{env::args, fs};

pub fn main() {
    let text = fs::read_to_string(args().collect::<Vec<String>>().get(1).unwrap())
        .unwrap()
        .trim_end()
        .replace('\r', "");
    let parts: Vec<_> = text.split("\n\n").collect();
    // lines of the crate stacks
    let mut crate_stacks: Vec<_> = parts[0].lines().collect();
    // pop crate numbers
    crate_stacks.pop();
    let moves = parts[1].lines();
    let mut a_stacks = Vec::new();
    for line in crate_stacks {
        let mut index = 0;
        let mut length = line.len();
        while length > 3 {
            if a_stacks.len() <= index {
                a_stacks.push(Vec::new());
            }
            let element = &line[index * 4..index * 4 + 4];
            match element {
                "    " => {}
                s => {
                    a_stacks[index].push(s[1..2].chars().nth(0).unwrap());
                }
            }
            index += 1;
            length -= 4;
        }
        if a_stacks.len() <= index {
            a_stacks.push(Vec::new());
        }
        let element = &line[index * 4..index * 4 + 3];
        match element {
            "   " => continue,
            s => {
                a_stacks[index].push(s[1..2].chars().nth(0).unwrap());
            }
        }
    }
    // reverse stacks to actually be stacks
    for stack in &mut a_stacks {
        stack.reverse();
    }
    let mut b_stacks = a_stacks.clone();
    for step in moves {
        let split: Vec<_> = step.split(' ').collect();
        let from: usize = split[3].parse().unwrap();
        let to: usize = split[5].parse().unwrap();
        let mut temp = Vec::new();
        for _ in 0..split[1].parse().unwrap() {
            let ac = a_stacks[from - 1].pop().unwrap();
            let bc = b_stacks[from - 1].pop().unwrap();
            a_stacks[to - 1].push(ac);
            temp.push(bc);
        }
        temp.reverse();
        for tmp in temp {
            b_stacks[to - 1].push(tmp);
        }
    }
    let mut a = String::new();
    for stack in &mut a_stacks {
        a.push(stack.pop().unwrap());
    }
    let mut b = String::new();
    for stack in &mut b_stacks {
        b.push(stack.pop().unwrap());
    }
    println!("A: {}\nB: {}", a, b);
}
