use std::io;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn find_md5_prefix(base_md5: Md5, prefix: &str) -> i32 {
    let mut n = 1;
    loop {
        let mut md5 = base_md5;
        md5.input_str(&n.to_string());
        if md5.result_str()[0..prefix.len()].eq(prefix) {
            return n;
        }
        n += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read stdin!");
    let mut base_md5 = Md5::new();
    base_md5.input_str(&input);
    println!("Part 1: {}", find_md5_prefix(base_md5, "00000"));
    println!("Part 2: {}", find_md5_prefix(base_md5, "000000"));
}
