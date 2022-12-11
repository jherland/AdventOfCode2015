use std::collections::{HashMap, HashSet};
use std::io;

use itertools::Itertools;

#[derive(Debug)]
struct DistMap {
    dmap: HashMap<String, HashMap<String, usize>>,
}

impl DistMap {
    fn add_route(&mut self, a: &str, b: &str, dist: usize) {
        if !self.dmap.contains_key(a) {
            self.dmap.insert(a.to_owned(), HashMap::new());
        }
        let a_conns = self.dmap.get_mut(a).unwrap();
        assert!(!a_conns.contains_key(b));
        a_conns.insert(b.to_owned(), dist);
    }

    fn parse<I>(lines: I) -> Self
    where
        I: Iterator<Item = String>,
    {
        let mut ret = DistMap {
            dmap: HashMap::new(),
        };
        for line in lines {
            let words: Vec<_> = line.split(' ').collect();
            assert!(words.len() == 5 && words[1] == "to" && words[3] == "=");
            let a = words[0];
            let b = words[2];
            let d = words[4].parse::<usize>().unwrap();
            ret.add_route(a, b, d);
            ret.add_route(b, a, d);
        }
        ret
    }

    fn get(&self, a: &str, b: &str) -> Option<usize> {
        match self.dmap.get(a) {
            None => None,
            Some(a_conns) => a_conns.get(b).copied(),
        }
    }

    fn travel_cost(&self, route: &[&String]) -> Option<usize> {
        route
            .windows(2)
            .map(|cities| match cities {
                [a, b] => self.get(a, b),
                _ => unreachable!("NOPE"),
            })
            .sum()
    }
}

fn main() {
    let dmap = DistMap::parse(io::stdin().lines().map(Result::unwrap));
    let cities: HashSet<String> = dmap.dmap.keys().cloned().collect();

    println!(
        "Part 1: {}",
        cities
            .iter()
            .permutations(cities.len())
            .map(|r| dmap.travel_cost(&r).unwrap())
            .min()
            .unwrap()
    );
    println!(
        "Part 2: {}",
        cities
            .iter()
            .permutations(cities.len())
            .map(|r| dmap.travel_cost(&r).unwrap())
            .max()
            .unwrap()
    );
}
