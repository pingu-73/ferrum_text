use::std::io;
use std::io::Read;
fn main() {
    for b in io::stdin().bytes(){
        let c = b.unwrap()as char;
        println!("{}", c);

        if c == 'q'{
            break;
        }
    }
}
