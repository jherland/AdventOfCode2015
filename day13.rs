use std::collections::HashMap;
use std::io;

use itertools::Itertools;

fn find_best_arrangement(guests: &HashMap<String, HashMap<String, i32>>) -> i32 {
    let mut scores: Vec<i32> = Vec::new();
    for perm in guests.keys().permutations(guests.len()) {
        let mut table = perm.clone();
        table.push(perm[0]);
        table.push(perm[1]);
        let score = table
            .windows(3)
            .map(|t| {
                if let &[o, p, q] = t {
                    let guest = guests.get(p).unwrap();
                    guest.get(o).unwrap() + guest.get(q).unwrap()
                } else {
                    unreachable!("Failed to walk around table");
                }
            })
            .sum();
        scores.push(score);
    }
    *scores.iter().max().unwrap()
}

fn main() {
    let mut guests: HashMap<String, HashMap<String, i32>> = HashMap::new();
    for line in io::stdin().lines().map(Result::unwrap) {
        let words: Vec<&str> = line.split(' ').collect();
        if let [p1, "would", change, num, "happiness", "units", "by", "sitting", "next", "to", p2] =
            words[..]
        {
            let score = match change {
                "gain" => num.parse::<i32>().unwrap(),
                "lose" => -num.parse::<i32>().unwrap(),
                _ => unreachable!("Parse error: {change:?}"),
            };
            let p2 = p2.strip_suffix('.').unwrap();
            if !guests.contains_key(p1) {
                guests.insert(p1.to_owned(), HashMap::new());
            }
            let guest = guests.get_mut(p1).unwrap();
            assert!(!guest.contains_key(p2));
            guest.insert(p2.to_owned(), score);
        } else {
            unreachable!("Parse error: {line:?}");
        }
    }

    println!("Part 1: {}", find_best_arrangement(&guests));

    // Add myself to guests
    let mut myself = HashMap::new();
    for guest in guests.keys() {
        myself.insert(guest.to_owned(), 0);
    }
    for guest in myself.keys() {
        guests
            .get_mut(guest)
            .unwrap()
            .insert("myself".to_string(), 0);
    }
    guests.insert("myself".to_string(), myself);

    println!("Part 2: {}", find_best_arrangement(&guests));
}
