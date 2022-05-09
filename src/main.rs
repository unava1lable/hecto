use std::io::{self, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        let b = b.unwrap();
        let c = b as char;
        if c.is_control() {
            println!("{:?}\r", b);
        } else {
            println!("{:?} ({})\r", b, c);
        }
        if b == to_ctrl_byte('q') {break;}
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let b = c as u8;
    b & 0b00011111
}