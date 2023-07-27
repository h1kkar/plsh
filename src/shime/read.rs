use std::io;

use ansi_colors::*;

/// A function that takes values from the keyboard and returns them
/// in the form of a string with the type of `String`.
pub fn read_cmd() -> String {
    let mut cmd = String::new();
    match io::stdin().read_line(&mut cmd) {
        Ok(_) => println!(""),
        Err(error) => {
            let err = error.to_string();
            let mut err = ColouredStr::new(&err);
            err.back_red();
            err.bold();
            println!("{err}")
        }
    };
    
    cmd.trim().to_string()
}