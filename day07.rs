use std::collections::{HashMap, HashSet};
use std::io;

#[derive(Debug)]
enum Operation {
    Id(String),
    Not(String),
    RShift(String, u16),
    LShift(String, u16),
    And(String, String),
    Or(String, String),
}

use Operation::*;

impl Operation {
    fn parse(words: Vec<String>) -> Result<Self, String> {
        match words.len() {
            1 => Ok(Id(words[0].clone())),
            2 => {
                assert!(words[0] == "NOT");
                Ok(Not(words[1].clone()))
            }
            3 => match words[1].as_str() {
                "RSHIFT" => Ok(RShift(words[0].clone(), words[2].parse().unwrap())),
                "LSHIFT" => Ok(LShift(words[0].clone(), words[2].parse().unwrap())),
                "AND" => Ok(And(words[0].clone(), words[2].clone())),
                "OR" => Ok(Or(words[0].clone(), words[2].clone())),
                _ => unreachable!("Invalid operation! ({words:?})"),
            },
            _ => unreachable!("Invalid operation! ({words:?})"),
        }
    }
}

#[derive(Debug)]
struct Unresolved {
    op: Operation,
    deps: HashSet<String>,
}

fn insert_unless_num(set: &mut HashSet<String>, s: &str) {
    if s.parse::<u16>().is_err() {
        set.insert(s.to_owned());
    };
}

impl Unresolved {
    fn parse(words: Vec<String>) -> Result<Self, String> {
        let op = Operation::parse(words).unwrap();
        let mut deps: HashSet<String> = HashSet::new();
        match &op {
            Id(a) => insert_unless_num(&mut deps, a),
            Not(a) => insert_unless_num(&mut deps, a),
            RShift(a, _) => insert_unless_num(&mut deps, a),
            LShift(a, _) => insert_unless_num(&mut deps, a),
            And(a, b) => {
                insert_unless_num(&mut deps, a);
                insert_unless_num(&mut deps, b);
            }
            Or(a, b) => {
                insert_unless_num(&mut deps, a);
                insert_unless_num(&mut deps, b);
            }
        };
        Ok(Unresolved { op, deps })
    }
}

fn parse_or_resolve(s: &str, resolved: &HashMap<String, u16>) -> u16 {
    s.parse::<u16>()
        .unwrap_or_else(|_| *resolved.get(s).unwrap())
}

fn resolve(uop: &Unresolved, resolved: &HashMap<String, u16>) -> u16 {
    match &uop.op {
        Id(a) => parse_or_resolve(a, resolved),
        Not(a) => !parse_or_resolve(a, resolved),
        RShift(a, n) => parse_or_resolve(a, resolved) >> n,
        LShift(a, n) => parse_or_resolve(a, resolved) << n,
        And(a, b) => parse_or_resolve(a, resolved) & parse_or_resolve(b, resolved),
        Or(a, b) => parse_or_resolve(a, resolved) | parse_or_resolve(b, resolved),
    }
}

fn resolve_all(parts: &HashMap<String, Unresolved>) -> HashMap<String, u16> {
    let mut resolved = HashMap::new();
    while resolved.len() < parts.len() {
        let before = resolved.len();
        for (dst, uop) in parts {
            if resolved.contains_key(dst) {
                continue;
            }
            // println!("{}: {:?}", dst, uop);
            if uop
                .deps
                .difference(&resolved.keys().cloned().collect())
                .count()
                == 0
            {
                resolved.insert(dst.to_owned(), resolve(uop, &resolved));
            }
        }
        if resolved.len() == before {
            panic!("No progress!");
        }
    }
    resolved
}

pub fn main() {
    let mut parts: HashMap<String, Unresolved> = HashMap::new();
    for line in io::stdin().lines().map(Result::unwrap) {
        let mut words: Vec<String> = line.split(' ').map(|w| w.to_owned()).collect();
        assert!(words.len() >= 3);
        let dst = words.pop().unwrap();
        let arrow = words.pop().unwrap();
        assert!(arrow == "->");
        assert!(!parts.contains_key(&dst));
        parts.insert(dst.to_owned(), Unresolved::parse(words).unwrap());
    }

    let wire_a = *resolve_all(&parts).get("a").unwrap();
    println!("Part 1: {}", wire_a);

    parts.insert(
        "b".to_owned(),
        Unresolved {
            op: Id(wire_a.to_string()),
            deps: HashSet::new(),
        },
    );
    println!("Part 2: {}", resolve_all(&parts).get("a").unwrap());
}
