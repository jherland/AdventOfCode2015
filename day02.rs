use std::fmt;
use std::io;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

struct Cuboid {
    l: u32,
    w: u32,
    h: u32,
}

impl Cuboid {
    fn volume(&self) -> u32 {
        self.l * self.w * self.h
    }

    fn sides(&self) -> Vec<u32> {
        [self.l * self.w, self.w * self.h, self.h * self.l].to_vec()
    }

    fn perimeters(&self) -> Vec<u32> {
        [self.l + self.w, self.w + self.h, self.h + self.l]
            .iter()
            .map(|l| 2 * l)
            .collect()
    }

    fn paper_needed(&self) -> u32 {
        let sides = self.sides();
        let smallest = sides.iter().min().unwrap();
        let surface_area: u32 = sides.iter().map(|s| 2 * s).sum();
        surface_area + smallest
    }

    fn ribbon_needed(&self) -> u32 {
        self.perimeters().iter().min().unwrap() + self.volume()
    }
}

impl FromStr for Cuboid {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, rest) = s
            .split_once('x')
            .ok_or_else(|| anyhow!("missing 1st 'x'"))?;
        let (w, h) = rest
            .split_once('x')
            .ok_or_else(|| anyhow!("missing 2nd 'x'"))?;

        Ok(Cuboid {
            l: l.parse()?,
            w: w.parse()?,
            h: h.parse()?,
        })
    }
}

impl fmt::Display for Cuboid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}x{}x{}", self.l, self.w, self.h)
    }
}

fn main() {
    let mut paper_needed = 0;
    let mut ribbon_needed = 0;
    loop {
        let mut line = String::new();

        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");

        match line.trim().parse::<Cuboid>() {
            Ok(b) => {
                // println!("{b}");
                paper_needed += b.paper_needed();
                ribbon_needed += b.ribbon_needed();
            }
            _ => break,
        }
    }
    println!("Part 1: {paper_needed}");
    println!("Part 2: {ribbon_needed}");
}
