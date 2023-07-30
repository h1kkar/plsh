use std::{
    fs,
    process::Command,
    str::from_utf8,
};

use tsu;
use ansi_colors::*;

pub fn pkg_ver() -> String {
    let file = match fs::read_to_string("Cargo.toml") {
        Ok(v) => v,
        Err(_) => "".to_string(),
    };
    
    if file != "".to_string() {
        let cargo_toml = tsu::toml_from_str(file);
        let package = cargo_toml.get("package").unwrap();
        let version = package.get("version").unwrap();
        let s: String = version.to_string();
        let s: Vec<String> = s.split('"').map(String::from).collect();
        let mut ver = ColouredStr::new(&s[1]);
        ver.yellow();
        ver.bold();
        let v = "is 📦 ".to_string() + &ver.to_string() + " ";
        return v
    } else {
        return "".to_string()
    }
}

pub fn rust_ver() -> String {
    let out = Command::new("rustc")
        .arg("-V")
        .output()
        .unwrap().stdout;
    let v = from_utf8(&out).unwrap().to_string();
    let v: Vec<String> = v.split(' ').map(String::from).collect();
    let v = &v[1];
    let mut ver = ColouredStr::new(v);
    ver.red();
    ver.bold();
    let version = "via 🦀 ".to_string() + &ver.to_string()+ " ";
    version
    //print!("{v}")
}