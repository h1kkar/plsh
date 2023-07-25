use std::{
    env,
    process::Command,
    str::from_utf8,
};

use super::super::func::dir::home_dir;
use ansi_colors::*;

/// The function responsible for whether the name of the repository
/// or directory will be displayed.
pub fn main() -> String {
    let g = match Command::new("git")
        .arg("branch")
        .output() {
            Ok(p) => {
                from_utf8(p.stdout.as_slice()).unwrap().to_string()},
            Err(_) => "".to_string()
        };

    if g != "".to_string() {
        git()
    } else {
        local()
    }
}

/// a function responsible for the type of directory output.
fn local() -> String {
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let home = home_dir() + "/";
    let d: Vec<String> = cur_dir.split(&home).map(String::from).collect();
    if cur_dir == home_dir() {
        let mut h = ColouredStr::new("home");
        h.magenta();
        h.bold();
        h.to_string()
    } else {
        if d.len() == 1 {
            let c = d[0].clone();
            let mut r = ColouredStr::new(&c[..]);
            r.red();
            r.bold();
            r.to_string()
        } else {
            let c = d.concat();
            let mut dir = ColouredStr::new(&c);
            dir.cyan();
            dir.bold();
            dir.to_string()
        }
    }
}

/// The function responsible for the conclusion of the name of the
/// repository.
fn git() -> String {
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let mut cur_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let name = cur_split.remove(cur_split.len()-1);
    let mut pr = ColouredStr::new(&name[..]);
    pr.light_green();
    pr.bold();
    pr.to_string()
}