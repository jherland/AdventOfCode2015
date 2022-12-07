use std::io;
use std::ops::Add;
use std::str::FromStr;

use anyhow::{anyhow, Error, Result};
use array2d::Array2D;

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
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Self {
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
        let (x, y) = s.split_once(',').ok_or_else(|| anyhow!("missing','"))?;
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
        assert!(tl.x < br.x && tl.y < br.y);
        Area { tl, br }
    }
}

fn parse(line: &str) -> (Instruction, Area) {
    let words: Vec<&str> = line.rsplitn(4, ' ').collect();
    let instruction = words[3].parse::<Instruction>().unwrap();
    let upper_left = words[2].parse::<Point>().unwrap();
    assert!(words[1] == "through");
    let bottom_right = words[0].parse::<Point>().unwrap();
    let area = Area::new(upper_left, bottom_right + Point { x: 1, y: 1 });
    (instruction, area)
}

pub fn main() {
    let input: Vec<_> = io::stdin()
        .lines()
        .map(Result::unwrap)
        .map(|s| parse(&s))
        .collect();

    // part 1
    let mut bitmap = Array2D::filled_with(0u8, 1000, 1000);
    for (instruction, area) in input.iter() {
        for x in area.tl.x..area.br.x {
            for y in area.tl.y..area.br.y {
                bitmap[(x, y)] = match instruction {
                    TurnOn => 1,
                    TurnOff => 0,
                    Toggle => 1 - bitmap[(x, y)],
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            sum += bitmap[(x, y)] as u32;
        }
    }
    println!("Part 1: {}", sum);

    // part 2
    let mut bitmap = Array2D::filled_with(0u8, 1000, 1000);
    for (instruction, area) in input.iter() {
        for x in area.tl.x..area.br.x {
            for y in area.tl.y..area.br.y {
                bitmap[(x, y)] = match instruction {
                    TurnOn => bitmap[(x, y)] + 1,
                    TurnOff => {
                        if bitmap[(x, y)] > 0 {
                            bitmap[(x, y)] - 1
                        } else {
                            0
                        }
                    }
                    Toggle => bitmap[(x, y)] + 2,
                }
            }
        }
    }
    let mut sum: u32 = 0;
    for x in 0..1000 {
        for y in 0..1000 {
            sum += bitmap[(x, y)] as u32;
        }
    }
    println!("Part 2: {}", sum);
}
