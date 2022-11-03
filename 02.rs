use std::fmt;
use std::io;
use std::num::ParseIntError;
use std::str::FromStr;

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

#[derive(Debug)]
struct ParseCuboidError {
    details: String,
}

impl ParseCuboidError {
    fn new(msg: &str) -> ParseCuboidError {
        ParseCuboidError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for ParseCuboidError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl From<ParseIntError> for ParseCuboidError {
    fn from(err: ParseIntError) -> Self {
        ParseCuboidError::new(format!("{}", err).as_ref())
    }
}

impl FromStr for Cuboid {
    type Err = ParseCuboidError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (l, rest) = s
            .split_once('x')
            .ok_or(ParseCuboidError::new("missing 1st 'x'"))?;
        let (w, h) = rest
            .split_once('x')
            .ok_or(ParseCuboidError::new("missing 2nd 'x'"))?;

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
    // part 1
    println!("{paper_needed}");

    // part 2
    println!("{ribbon_needed}");
}
