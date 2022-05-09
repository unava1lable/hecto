use std::io::{self, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap() as char;
        println!("{}", b);
        if b == 'q' {break;}
    }
}
