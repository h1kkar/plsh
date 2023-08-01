use std::{process::Command, str::from_utf8};

use ansi_colors::*;

pub fn branch() -> String {
    let mut err = ColouredStr::new("failed find git branch");
    err.back_red();
    err.bold();

    let head = Command::new("cat")
        .arg(".git/HEAD")
        .output()
        .expect(&err.to_string());
    let branch = from_utf8(head.stdout.as_slice()).unwrap().trim().to_string();
    let br: Vec<String> = branch.split('/').map(String::from).collect();
    let b = &br[br.len()-1];
    b.to_string()
}