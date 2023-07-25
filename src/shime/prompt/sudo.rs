use std::env;

//use super::super::func::dir;

use ansi_colors::*;

pub fn root() {
    if var() == "root" {
        let mut r = ColouredStr::new("root");
        r.red();
        r.bold();
        print!("{0} in ", r)
    }
}

pub fn var() -> String {
    let key = "user".to_uppercase();

    match env::var(key) {
        Ok(usr) => return usr,
        Err(err) => {
            println!("{0}", err);
            return String::from("");
        },
    }
}