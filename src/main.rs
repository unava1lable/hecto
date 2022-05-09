use std::io::{self, Read};

fn main() {
    for b in io::stdin().bytes() {
        let b = b.unwrap() as char;
        println!("{}", b);
    }
}
