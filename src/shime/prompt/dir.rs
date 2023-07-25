use std::{
    env,
    process::Command,
    str::from_utf8,
};

use super::super::func::dir::home_dir;
//use super::git;
use ansi_colors::*;
//use git_info;

pub fn main() -> String {
    /*let cur_dir = env::current_dir().unwrap().display().to_string();
    let dir_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    println!("{0} : {1}", dir_split[&dir_split.len()-1], git::config().branch);
    if git::config().branch == dir_split[&dir_split.len()-1] {
        let b = &git::config().branch[..];
        let mut b = ColouredStr::new(b);
        b.magenta();
        b.bold();
        b.to_string()*/

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

    /*if info.branches.unwrap_or(vec![]).len() == 0 {
        local()
    } else {
        git::config().branch
    }*/
}

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

fn git() -> String {
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let mut cur_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let name = cur_split.remove(cur_split.len()-1);
    let mut pr = ColouredStr::new(&name[..]);
    pr.light_green();
    pr.bold();
    pr.to_string()
}