use::std::io;
use std::io::Read;

use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap(); //binding raw mode (refer to Ownership rules)

    for b in io::stdin().bytes() {      //wrapper class for answer and err
        match b {
            Ok (b) => {
                let c = b as char;

                if c.is_control(){
                    println!("{:?}", b);
                }
                else {
                    println!("{:?} ({})\r", b, c);
                }
                if b == to_control_byte('q') {
                    break;
                }
            }

            Err (err) => die(err)
        }

    }
}

fn to_control_byte(c: char) -> u8 {
    let byte = c as u8;
    byte & 0b0001_1111
}

fn die(e: std::io::Error){
    panic!("{}", e);
}