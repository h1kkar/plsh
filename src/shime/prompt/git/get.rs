use std::{
    process::Command,
    str::from_utf8,
};
use ansi_colors::ColouredStr;

pub fn branch() -> String {
    let branch = super::config().branch;
    let mut b = ColouredStr::new(&branch);
    b.magenta();
    b.bold();
    let s = String::from(" on ") + &b.to_string() + "";
    return s
}

pub fn dirty() -> String {
    let mut c = ColouredStr::new("* ");
    c.magenta();
    c.bold();
    if super::config().dirty {
        return c.to_string()
    } else {
        String::from(" ")
    }
}
    
pub fn commit() -> String {
    let commit = super::config().lastcommit;
    let mut c = ColouredStr::new(&commit);
    c.light_blue();
    c.bold();
    let mut s = String::new();
    if commit == "".to_string() {
        s.push_str(&c.to_string());
    } else {
        let f = c.to_string() + " ";
        s.push_str(&f);
    }
    return s
}

pub fn status() -> bool {
    let g = match Command::new("git")
        .arg("branch")
        .output() {
            Ok(p) => {
                Some(from_utf8(p.stdout.as_slice()).unwrap().to_string())
            },
            Err(_) => None
        };

    let g = match g {
        Some(h) => h,
        None => "".to_string()
    };

    if g != "".to_string() {
        return true
    } else {
        return false
    }
}