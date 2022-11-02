use std::io;

fn main() {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // part 1
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => break,
        }
    }
    println!("{floor}");

    // part 2
    let mut pos = 1;
    let mut floor = 0;
    for c in input.chars() {
        match c {
            '(' => floor += 1,
            ')' => floor -= 1,
            _ => break,
        };
        if floor < 0 { break };
        pos += 1;
    }
    println!("{pos}");
}
