use std::{
    env,
    io,
};

use dirs;
use ansi_colors::*;

/// A function returning the home directory.
pub fn home_dir() -> String {
    match dirs::home_dir() {
        Some(path) => {
            path.display().to_string()
        },
        None => {
            String::from("/")
        },
    }
}

/// a function responsible for the transition to a particular directory.
pub fn go(path: &String) {
    let path_split: Vec<String> = path.split_inclusive('/').map(String::from).collect();

    if path_split[0].starts_with("/") {
        match absolute_path(&path_split) {
            Ok(_) => {
                let mut m = ColouredStr::new("moved");
                m.cyan();
                let mut p = ColouredStr::new(path);
                p.magenta();
                println!("you {0} to the {1} dir", m, p)
            },
            Err(error) => println!("{0}", error),
        }
    } else {
        let mut i = 0;
        let mut cond = false;
        while i < path_split.len() {
            match &path_split[i][..] {
                "../" | ".." => {
                    cond = match back() {
                        Ok(c) => c,
                        Err(error) => {
                            println!("{0}", error);
                            false
                        }
                    };
                },
                _ => {
                    let arg = path_split[i].clone();
                    cond = match next(&vec![arg]){
                        Ok(c) => c,
                        Err(error) => {
                            println!("{0}", error);
                            false
                        }
                    };
                }
            }
            i += 1;
        }

        let cur_dir: String = env::current_dir().unwrap().display().to_string();
        let home = home_dir() + "/";
        let dir: Vec<String> = cur_dir.split(&home).map(String::from).collect();
        if cond {
            if cur_dir == home_dir() {
                let mut m = ColouredStr::new("moved");
                m.cyan();
                let mut h = ColouredStr::new("home");
                h.magenta();
                println!("you {0} to the {1} dir", m, h)
            } else {
                if dir.len() == 1 {
                    let mut m = ColouredStr::new("moved");
                    m.cyan();
                    let mut r = ColouredStr::new(&dir[0]);
                    r.red();
                    println!("you {0} to the {1} dir", m, r)
                } else{
                    let mut m = ColouredStr::new("moved");
                    m.cyan();
                    let d = &dir.concat()[..];
                    let mut d = ColouredStr::new(d);
                    d.magenta();
                    println!("you {0} to the {1} dir", m, d)
                }
            }
        }
    }
}

/// The function that transition to the directory is back (accompanies the `cd` ).
pub fn back() -> Result<bool, io::Error> {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let mut dir_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let _ = dir_split.remove(dir_split.len()-1);

    let dirs = dir_split.concat();
    let _ = match env::set_current_dir(&dirs) {
        Ok(()) => return Ok(true),
        Err(error) => return Err(error),
    };
}

/// The function that transition to a specific directory is lower in the directory tree (accompanies the `cd`).
pub fn next(path: &Vec<String>) -> Result<bool, io::Error> {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let dir = cur_dir + "/" + &path.concat()[..];

    let _ = match env::set_current_dir(&dir) {
        Ok(()) => return Ok(true),
        Err(error) => return Err(error),
    };
}

/// The function that helps in working with the absolute paths
pub fn absolute_path(path: &Vec<String>) -> Result<Option<String>, io::Error> {
    let dir = path.concat();
    let _ = match env::set_current_dir(&dir) {
        Ok(()) => return Ok(None),
        Err(error) => return Err(error),
    };
}

/// The function that transition to a home directory
pub fn go_home() -> Result<Option<String>, io::Error> {
    let _ = match env::set_current_dir(home_dir()) {
        Ok(()) => return Ok(None),
        Err(error) => return Err(error),
    };
}