use std::collections::VecDeque;
use std::io::{self, Read};

fn look_and_say(mut input: VecDeque<u8>) -> VecDeque<u8> {
    let mut ret = VecDeque::new();
    while !input.is_empty() {
        let item = input.pop_front().unwrap();
        let mut count = 1;
        while input.front() == Some(&item) {
            input.pop_front();
            count += 1;
        }
        ret.push_back(count);
        ret.push_back(item);
    }
    ret
}

pub fn main() {
    let mut digits: VecDeque<u8> = io::stdin()
        .bytes()
        .map(Result::unwrap)
        .filter(|b| b.is_ascii_digit())
        .map(|b| b - b'0')
        .collect();

    for _ in 0..40 {
        digits = look_and_say(digits);
    }
    println!("Part 1: {:?}", digits.len());

    for _ in 40..50 {
        digits = look_and_say(digits);
    }
    println!("Part 2: {:?}", digits.len());
}
