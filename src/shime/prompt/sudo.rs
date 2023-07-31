use std::env;

use ansi_colors::*;

pub fn root() -> String {
    if var() == "root" {
        let mut r = ColouredStr::new("root");
        r.red();
        r.bold();
        let s = r.to_string() + " in ";
        return s.to_string();
    } else {
        "".to_string()
    }
}

pub fn var() -> String {
    let key = "user".to_uppercase();

    match env::var(key) {
        Ok(usr) => return usr,
        Err(error) => {
            let err = error.to_string();
            let mut err = ColouredStr::new(&err);
            err.back_red();
            err.bold();
            println!("{err}");
            return String::from("");
        },
    }
}
