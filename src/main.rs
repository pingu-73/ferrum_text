use::std::io;
use std::io::Read;

use termion::raw::IntoRawMode;

fn main() {
    let _stdout = io::stdout().into_raw_mode().unwrap(); //binding raw mode (refer to Ownership rules)

    for b in io::stdin().bytes(){
        let c = b.unwrap()as char;
        println!("{}", c);

        if c == 'q'{
            break;
        }
    }
}
