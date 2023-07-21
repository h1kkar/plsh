use std::io::*;

pub fn exec(c: char) {
    print!("{c} ");
    stdout().flush().unwrap();    
}