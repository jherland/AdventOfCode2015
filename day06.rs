use std::cmp::{min, max};
use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Debug)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

impl FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "turn on" => Ok(Self::TurnOn),
            "turn off" => Ok(Self::TurnOff),
            "toggle" => Ok(Self::Toggle),
            _ => Err(anyhow!("Failed to parse {s:?}")),
        }
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl FromStr for Point {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(',')
            .ok_or(anyhow!("missing','"))?;
        Ok(Point { x: x.parse()?, y: y.parse()? })
    }
}

#[derive(Debug)]
struct Area {
    tl: Point,
    br: Point,
}

impl Area {
    fn new(tl: Point, br: Point) -> Self {
        let ret = Area { tl, br };
        assert!(ret.size() > 0);
        ret
    }

    fn from_tuples(tl: (u32, u32), br: (u32, u32)) -> Self {
        Self::new(Point::new(tl.0, tl.1), Point::new(br.0, br.1))
    }

    fn size(&self) -> u32 {
        (self.br.x - self.tl.x) * (self.br.y - self.tl.y)
    }

    fn contains(&self, p: &Point) -> bool {
        self.tl.x <= p.x && p.x < self.br.x && self.tl.y <= p.y && p.y < self.br.y
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        let tl = Point::new(
            max(self.tl.x, other.tl.x),
            max(self.tl.y, other.tl.y),
        );
        let br = Point::new(
            min(self.br.x, other.br.x),
            min(self.br.y, other.br.y),
        );
        if br.x > tl.x && br.y > tl.y {
            Some(Area::new(tl, br))
        } else {
            None
        }
    }

    fn overlap(&self, other: &Self) -> bool {
        self.intersect(other).is_some()
    }
}

pub fn main() {
    loop {
        let mut line = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line");
        let line = line.trim();
        if line.len() == 0 {
            break;
        }
        let words: Vec<&str> = line.rsplitn(4, " ").collect();
        let instruction = words[3].parse::<Instruction>().unwrap();
        let upper_left = words[2].parse::<Point>().unwrap();
        assert!(words[1] == "through");
        let bottom_right = words[0].parse::<Point>().unwrap();
        let area = Area::new(
            upper_left,
            bottom_right + Point::new(1, 1),
        );
        println!(">>> {:?}: {:?} [{:?}]", instruction, area, area.size());

        // match words[3] {
        //     "toggle" | "turn on" | "turn off" => {
        //         println!(">>> {words:?}");
        //     }
        //     _ => {
        //         panic!("Failed to parse {words:?}");
        //     }
        // }

        // if line.len() == 0 { // stop on first empty line
        //     break;
        // }
        // if part1_predicates.iter().all(|pred| pred(line)) {
        //     part1 += 1;
        // }
        // if part2_predicates.iter().all(|pred| pred(line)) {
        //     part2 += 1;
        // }
    }
    // println!("{part1}");
    // println!("{part2}");
}

#[cfg(test)]
mod test {
    use super::{Area, Point};

    #[test]
    fn inside() {
        let a = Area::new(Point::new(1, 1), Point::new(7, 7));
        assert!(!a.contains(&Point::new(0, 0)));
        assert!(!a.contains(&Point::new(0, 1)));
        assert!(!a.contains(&Point::new(1, 0)));
        assert!(a.contains(&Point::new(1, 1)));
        assert!(a.contains(&Point::new(4, 4)));
        assert!(a.contains(&Point::new(6, 6)));
        assert!(!a.contains(&Point::new(6, 7)));
        assert!(!a.contains(&Point::new(7, 6)));
        assert!(!a.contains(&Point::new(7, 7)));
        assert!(!a.contains(&Point::new(99, 99)));
    }

    #[test]
    fn does_not_overlap() {
        let a = Area::new(Point::new(2, 2), Point::new(7, 7));
        assert!(!a.overlap(&Area::from_tuples((0, 0), (2, 2)))); // tl/br corners touch
        assert!(!a.overlap(&Area::from_tuples((2, 0), (7, 2)))); // common t/b edge
        assert!(!a.overlap(&Area::from_tuples((7, 0), (8, 5)))); // partial r/l edge
        assert!(!a.overlap(&Area::from_tuples((0, 2), (2, 7)))); // common l/r edge
        assert!(!a.overlap(&Area::from_tuples((7, 2), (9, 7)))); // common r/l edge
        assert!(!a.overlap(&Area::from_tuples((0, 7), (1, 90)))); // away from bl/tr corners
        assert!(!a.overlap(&Area::from_tuples((0, 7), (2, 90)))); // bl/tr corners touch
        assert!(!a.overlap(&Area::from_tuples((2, 7), (7, 8)))); // common b/t edge
        assert!(!a.overlap(&Area::from_tuples((7, 7), (90, 90)))); // br/tl corners touch
    }

    #[test]
    fn does_overlap() {
        let a = Area::new(Point::new(2, 2), Point::new(7, 7));
        assert!(a.overlap(&Area::from_tuples((0, 0), (3, 3)))); // tl/br corners overlap
        assert!(a.overlap(&Area::from_tuples((3, 0), (5, 6)))); // t/b edge overlaps
        assert!(a.overlap(&Area::from_tuples((6, 0), (8, 3)))); // tr/bl corners overlap
        assert!(a.overlap(&Area::from_tuples((0, 3), (3, 5)))); // l/r edge overlaps
        assert!(a.overlap(&Area::from_tuples((4, 4), (5, 5)))); // fully envelope
        assert!(a.overlap(&Area::from_tuples((5, 5), (8, 6)))); // r/l edge overlaps
        assert!(a.overlap(&Area::from_tuples((6, 6), (90, 90)))); // br/tl corners overlap
        assert!(a.overlap(&Area::from_tuples((2, 2), (7, 7)))); // exact match
        assert!(a.overlap(&Area::from_tuples((1, 1), (8, 8)))); // fully inside
        assert!(a.overlap(&Area::from_tuples((3, 1), (6, 8)))); // common center
    }
}
