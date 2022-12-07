use std::io;
use std::iter::Iterator;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};

#[derive(Clone, Debug, PartialEq)]
enum Instruction {
    TurnOn,
    TurnOff,
    Toggle,
}

use Instruction::*;

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

#[derive(Clone, Debug, PartialEq, PartialOrd)]
struct Point {
    x: u32,
    y: u32,
}

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
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
        let (x, y) = s.split_once(',').ok_or(anyhow!("missing','"))?;
        Ok(Point {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}

#[derive(Clone, Debug, PartialEq)]
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

//     fn from_tuples(tl: (u32, u32), br: (u32, u32)) -> Self {
//         Self::new(Point::new(tl.0, tl.1), Point::new(br.0, br.1))
//     }

    fn size(&self) -> u32 {
        (self.br.x - self.tl.x) * (self.br.y - self.tl.y)
    }

    fn points<F>(&mut self, f: F)
    where
        F: FnMut(&Point) -> (),
    {
        for x in self.tl.x..self.br.x {
            for y in self.tl.y..self.br.y {
                f(&Point { x, y });
            }
        }
    }

//     fn contains(&self, p: &Point) -> bool {
//         self.tl.x <= p.x && p.x < self.br.x && self.tl.y <= p.y && p.y < self.br.y
//     }

//     fn intersect(&self, other: &Self) -> Option<Self> {
//         let tl = Point::new(max(self.tl.x, other.tl.x), max(self.tl.y, other.tl.y));
//         let br = Point::new(min(self.br.x, other.br.x), min(self.br.y, other.br.y));
//         if br.x > tl.x && br.y > tl.y {
//             Some(Area::new(tl, br))
//         } else {
//             None
//         }
//     }

//     fn overlap(&self, other: &Self) -> bool {
//         self.intersect(other).is_some()
//     }

//     fn interact(self, instruction: Instruction, other: Self) -> (std::iter::Once<Self>, Option<(Instruction, Self)>) {
//         match self.intersect(&other) {
//             None => {
//                 // No overlap between this area and other
//                 (std::iter::once(self), Some((instruction, other)))
//             }
//             Some(i) => {
//                 match (instruction, other) {
//                     (TurnOn, o) if o == i => {
//                         // other is fully contained within self, and hence already turned on
//                         (std::iter::once(self), None)
//                     }
//                     (_, o) if o == i => {
//                         // self is fully contained within other, and turned off by it
//                         (std::iter::once(self), Some((instruction, other)))
// //                        (std::iter::empty(), Some((instruction, other)))
//                     }
//                     _ => panic!("Found intersection: {i:?}"),
//                 }
//             }
//         }
//     }
}

fn parse(line: &str) -> (Instruction, Area) {
    let words: Vec<&str> = line.rsplitn(4, " ").collect();
    let instruction = words[3].parse::<Instruction>().unwrap();
    let upper_left = words[2].parse::<Point>().unwrap();
    assert!(words[1] == "through");
    let bottom_right = words[0].parse::<Point>().unwrap();
    let area = Area::new(upper_left, bottom_right + Point {x: 1, y: 1});
    (instruction, area)
}

pub fn main() {
    let input: Vec<_> = io::stdin().lines().map(Result::unwrap).map(|s| parse(&s)).collect();

    let mut bitmap = [[0u8; 1000]; 1000];
    for (instruction, area) in input.iter() {
        println!(">>> {:?}: {:?}", instruction, area);  // TODO: REMOVE
        area.points(|p| {
            assert!(p.x < 1000 && p.y < 1000);  // TODO: FIXME
            bitmap[p.x][p.y] = match instruction {
                TurnOn => 1,
                TurnOff => 0,
                Toggle => 1 - bitmap[p.x][p.y],
            }
        });
    }
    let mut sum = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            sum += bitmap[x][y];
        }
    }
    println!("Part 1: {}", sum);
}

// #[cfg(test)]
// mod test {
//     use super::{Area, Instruction, Instruction::*, Point};

//     #[test]
//     fn inside() {
//         let a = Area::new(Point::new(1, 1), Point::new(7, 7));
//         assert!(!a.contains(&Point::new(0, 0)));
//         assert!(!a.contains(&Point::new(0, 1)));
//         assert!(!a.contains(&Point::new(1, 0)));
//         assert!(a.contains(&Point::new(1, 1)));
//         assert!(a.contains(&Point::new(4, 4)));
//         assert!(a.contains(&Point::new(6, 6)));
//         assert!(!a.contains(&Point::new(6, 7)));
//         assert!(!a.contains(&Point::new(7, 6)));
//         assert!(!a.contains(&Point::new(7, 7)));
//         assert!(!a.contains(&Point::new(99, 99)));
//     }

//     #[test]
//     fn does_not_overlap() {
//         let a = Area::new(Point::new(2, 2), Point::new(7, 7));
//         assert!(!a.overlap(&Area::from_tuples((0, 0), (2, 2)))); // tl/br corners touch
//         assert!(!a.overlap(&Area::from_tuples((2, 0), (7, 2)))); // common t/b edge
//         assert!(!a.overlap(&Area::from_tuples((7, 0), (8, 5)))); // partial r/l edge
//         assert!(!a.overlap(&Area::from_tuples((0, 2), (2, 7)))); // common l/r edge
//         assert!(!a.overlap(&Area::from_tuples((7, 2), (9, 7)))); // common r/l edge
//         assert!(!a.overlap(&Area::from_tuples((0, 7), (1, 90)))); // away from bl/tr corners
//         assert!(!a.overlap(&Area::from_tuples((0, 7), (2, 90)))); // bl/tr corners touch
//         assert!(!a.overlap(&Area::from_tuples((2, 7), (7, 8)))); // common b/t edge
//         assert!(!a.overlap(&Area::from_tuples((7, 7), (90, 90)))); // br/tl corners touch
//     }

//     #[test]
//     fn does_overlap() {
//         let a = Area::new(Point::new(2, 2), Point::new(7, 7));
//         assert!(a.overlap(&Area::from_tuples((0, 0), (3, 3)))); // tl/br corners overlap
//         assert!(a.overlap(&Area::from_tuples((3, 0), (5, 6)))); // t/b edge overlaps
//         assert!(a.overlap(&Area::from_tuples((6, 0), (8, 3)))); // tr/bl corners overlap
//         assert!(a.overlap(&Area::from_tuples((0, 3), (3, 5)))); // l/r edge overlaps
//         assert!(a.overlap(&Area::from_tuples((4, 4), (5, 5)))); // fully envelope
//         assert!(a.overlap(&Area::from_tuples((5, 5), (8, 6)))); // r/l edge overlaps
//         assert!(a.overlap(&Area::from_tuples((6, 6), (90, 90)))); // br/tl corners overlap
//         assert!(a.overlap(&Area::from_tuples((2, 2), (7, 7)))); // exact match
//         assert!(a.overlap(&Area::from_tuples((1, 1), (8, 8)))); // fully inside
//         assert!(a.overlap(&Area::from_tuples((3, 1), (6, 8)))); // common center
//     }

//     fn verify_interaction(area: Area, apply: (Instruction, Area), expect_iter: Vec<Area>, expect_apply: Option<(Instruction, Area)>) {
//         let (iter, rest) = area.interact(apply.0, apply.1);
//         assert!(iter.collect::<Vec<_>>() == expect_iter);
//         assert!(rest == expect_apply);
//     }

//     fn verify_zero_interaction(area: Area, apply: (Instruction, Area)) {
//         let unchanged_iter = vec![area.clone()];
//         let unchanged_apply = Some(apply.clone());
//         verify_interaction(area, apply, unchanged_iter, unchanged_apply);
//     }

//     #[test]
//     fn interact_no_overlap() { // The applied area has no interaction with existing
//         let a = Area::from_tuples((1, 1), (3, 3));
//         verify_zero_interaction(a.clone(), (TurnOn, Area::from_tuples((0, 0), (1, 1))));
//         verify_zero_interaction(a.clone(), (TurnOff, Area::from_tuples((3, 3), (5, 5))));
//         verify_zero_interaction(a.clone(), (Toggle, Area::from_tuples((0, 1), (1, 5))));
//     }

//     fn verify_already_on_interaction(area: Area, apply: (Instruction, Area)) {
//         let unchanged_iter = vec![area.clone()];
//         let swallow_apply = None;
//         verify_interaction(area, apply, unchanged_iter, swallow_apply);
//     }

//     #[test]
//     fn interact_swallow() { // The applied area is contained within the existing, and does not change it
//         let a = Area::from_tuples((1, 1), (4, 4));
//         verify_already_on_interaction(a.clone(), (TurnOn, Area::from_tuples((1, 1), (4, 2)))); // top edge overlap
//         verify_already_on_interaction(a.clone(), (TurnOn, Area::from_tuples((2, 2), (3, 3)))); // overlap in middle
//         verify_already_on_interaction(a.clone(), (TurnOn, Area::from_tuples((1, 1), (4, 4)))); // full overlap
//     }

//     fn verify_remove_all_interaction(area: Area, apply: (Instruction, Area)) {
//         let empty_iter = vec![];
//         let unchanged_apply = Some(apply.clone());
//         verify_interaction(area, apply, empty_iter, unchanged_apply);
//     }

//     #[test]
//     fn interact_remove() { // The applied area removes the entire existing area
//         let a = Area::from_tuples((1, 1), (4, 4));
//         verify_remove_all_interaction(a.clone(), (TurnOff, Area::from_tuples((1, 1), (4, 4)))); // turn off exactly
//         verify_remove_all_interaction(a.clone(), (TurnOff, Area::from_tuples((0, 0), (5, 5)))); // turn off superset
//         verify_remove_all_interaction(a.clone(), (Toggle, Area::from_tuples((0, 0), (5, 5)))); // toggle superset
//     }
// }
