use std::fmt;
use std::io::{self, Read};

use itertools::Itertools;

#[derive(Clone, Debug)]
struct Password(Vec<u8>);

impl Password {
    fn parse(bytes: impl Iterator<Item = u8>) -> Self {
        Self(
            bytes
                .filter(|b| b.is_ascii_lowercase())
                .map(|b| b - b'a')
                .collect(),
        )
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = self.0.iter().map(|b| (b + b'a') as char).join("");
        write!(f, "{}", s)
    }
}

impl Iterator for Password {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (0..self.0.len()).rev() {
            self.0[i] += 1;
            let carry = self.0[i] >= 26;
            self.0[i] %= 26;
            if !carry {
                break;
            }
        }
        Some(self.clone())
    }
}

impl Password {
    fn has_increasing_straight_of_three(&self) -> bool {
        self.0.windows(3).any(|win| match win {
            &[a, b, c] => a + 1 == b && b + 1 == c,
            _ => false,
        })
    }

    fn has_confusing_letters(&self) -> bool {
        self.0
            .iter()
            .map(|b| b + b'0')
            .any(|b| b == b'i' || b == b'o' || b == b'l')
    }

    fn has_two_different_letter_pairs(&self) -> bool {
        self.0
            .windows(2)
            .filter(|win| match win {
                &[a, b] => a == b,
                _ => false,
            })
            .unique()
            .count()
            >= 2
    }

    fn is_valid(&self) -> bool {
        self.has_increasing_straight_of_three()
            && !self.has_confusing_letters()
            && self.has_two_different_letter_pairs()
    }
}

pub fn main() {
    let pwd = Password::parse(io::stdin().bytes().map(Result::unwrap));

    let mut pwdgen = pwd.into_iter().filter(|pw| pw.is_valid());
    println!("Part 1: {}", pwdgen.next().unwrap());
    println!("Part 2: {}", pwdgen.next().unwrap());
}
