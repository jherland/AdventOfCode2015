use std::io;

fn at_least_three_vowels(line: &str) -> bool {
    line.chars().filter(|&c| "aeiou".contains(c)).count() >= 3
}

fn has_repeated_letter(line: &str) -> bool {
    let mut prev: u8 = 0;
    for cur in line.bytes() {
        if cur == prev {
            return true;
        }
        prev = cur;
    }
    false
}

fn has_no_special_substrings(line: &str) -> bool {
    for special in ["ab", "cd", "pq", "xy"] {
        if line.contains(special) {
            return false;
        }
    }
    true
}

fn has_repeated_letter_pair(line: &str) -> bool {
    for i in 0..(line.as_bytes().len() - 2) {
        if line[i + 2..].contains(&line[i..i + 2]) {
            return true;
        }
    }
    false
}

fn has_repeated_letter_with_one_in_between(line: &str) -> bool {
    let mut prev1: u8 = 0;
    let mut prev2: u8 = 0;
    for cur in line.bytes() {
        if cur == prev2 {
            return true;
        }
        prev2 = prev1;
        prev1 = cur;
    }
    false
}

pub fn main() {
    let part1_predicates = [
        at_least_three_vowels,
        has_repeated_letter,
        has_no_special_substrings,
    ];
    let part2_predicates = [
        has_repeated_letter_pair,
        has_repeated_letter_with_one_in_between,
    ];

    let mut part1 = 0;
    let mut part2 = 0;
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();
        if line.is_empty() {
            break; // stop on first empty line
        }
        if part1_predicates.iter().all(|pred| pred(line)) {
            part1 += 1;
        }
        if part2_predicates.iter().all(|pred| pred(line)) {
            part2 += 1;
        }
    }
    println!("{part1}");
    println!("{part2}");
}
