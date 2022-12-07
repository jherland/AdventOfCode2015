use std::collections::{HashSet, LinkedList};
use std::io;

type Pos = (i32, i32);

fn next(cur: Pos, dir: char) -> Pos {
    // println!("next({cur:?}, {dir})");
    match dir {
        '<' => (cur.0 - 1, cur.1),
        '>' => (cur.0 + 1, cur.1),
        '^' => (cur.0, cur.1 + 1),
        'v' => (cur.0, cur.1 - 1),
        _ => panic!("Invalid input!"),
    }
}

fn santa_delivery(num_santas: u8, instructions: &str) -> usize {
    let mut visited = HashSet::new();
    visited.insert((0, 0));

    let mut santas = LinkedList::new();
    for _ in 0..num_santas {
        santas.push_back((0, 0))
    }

    for c in instructions.chars() {
        let mut santa = santas.pop_front().unwrap();
        santa = next(santa, c);
        visited.insert(santa);
        santas.push_back(santa);
    }
    visited.len()
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // part 1
    let num_visited = santa_delivery(1, input.trim());
    println!("{num_visited}");

    // part 2
    let num_visited = santa_delivery(2, input.trim());
    println!("{num_visited}");
}
