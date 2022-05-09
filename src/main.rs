use core::panic;
use std::io::{self, Read};
use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap();
    for b in io::stdin().bytes() {
        match b {
            Ok(b) => {
                let c = b as char;
                if c.is_control() {
                    println!("{:?}\r", b);
                } else {
                    println!("{:?} ({})", b, c);
                }
                if b == to_ctrl_byte('q') {break;}
            }
            Err(e) => die(e),
        }
    }
}

fn to_ctrl_byte(c: char) -> u8 {
    let b = c as u8;
    b & 0b00011111
}

fn die(e: io::Error) {
    panic!("{}", e);
}