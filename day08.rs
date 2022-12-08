use std::io;

pub fn main() {
    let lines: Vec<_> = io::stdin().lines().map(Result::unwrap).collect();
    let mut escape = 0;
    let mut total = 0;
    let mut decoded = 0;
    let mut encoded = lines.len() * 2; // quotes to delimit every line
    for c in lines.iter().flat_map(|line| line.bytes()) {
        assert!(!c.is_ascii_whitespace());
        decoded += match (escape, c) {
            (0, b'"') => 0, // quote (start or end)
            (0, b'\\') => {
                escape = 1;
                0
            } // backslash (start of escape)
            (0, _) => 1,    // vanilla char
            (1, b'\\') => {
                escape = 0;
                1
            } // double backslah
            (1, b'"') => {
                escape = 0;
                1
            } // escaped quote char
            (1, b'x') => {
                escape = 2;
                1
            } // hex escape => 1 char
            (_, _) => {
                escape -= 1;
                0
            } // later parts of hex escape
        };
        encoded += match c {
            b'"' | b'\\' => 2, // prepend backslash to quote and backslash
            _ => 1,            // otherwise pass as-is
        };
        total += 1;
    }
    println!("Part 1: {}", total - decoded);
    println!("Part 2: {}", encoded - total);
}
