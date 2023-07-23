use std::{
    env,
    io,
};

use dirs;
use colorized::*;

/// A function returning the home directory.
///
/// # Usage
/// 
/// ```no_run
/// println!(home_dir());
/// ```
/// Here's what will turn out in the end, instead of `user`, there should be the
/// name of your current user.
///
/// ```no_run
/// "/home/user"
/// ```
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
///
/// Equal to the `cd` function in Unix- systems.
///
/// # Usage
///
/// ```no_run
/// // Current directories is "/home/user/"
/// let path: String = String::from("project/git/shime");
/// cd(&path);
/// // Current directories is "/home/user/project/git/shime"
/// ```
pub fn cd(path: &String) {
    let path_split: Vec<String> = path.split_inclusive('/').map(String::from).collect();

    if path_split[0].starts_with("/") {
        match absolute_path(&path_split) {
            Ok(_) => println!("you {0} to the {1} dir", "moved".color(Colors::CyanFg), path.color(Colors::MagentaFg)),
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
        //let s = String::from("~/") + &dir[1][..];
        if cond {
            if cur_dir == home_dir() {
                println!("you {0} to the {1} dir", "moved".color(Colors::CyanFg), "home".color(Colors::MagentaFg))
            } else {
                if dir.len() == 1 {
                    println!("you {0} to the {1} dir", "moved".color(Colors::CyanFg), dir[0].color(Colors::RedFg))
                } else{
                    println!("you {0} to the {1} dir", "moved".color(Colors::CyanFg), dir.concat().color(Colors::MagentaFg))
                }
            }
        }
    }
}

/// The function that transition to the directory is back (accompanies the [cd function][cd]).
///
/// # Usage
///
/// ```no_run
/// // Current directories is "shime/src"
/// let _ = back();
/// // Current directories is "shime"
/// ```
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

/// The function that transition to a specific directory is lower in the directory tree (accompanies the [cd function][cd]).
///
/// # Usage
///
/// ```no_run
/// // Current directories is "shime"
/// let path: String = String::from("src");
/// let _ = next(&vec![path]);
/// // Current directories is "shime/src"
/// ```
pub fn next(path: &Vec<String>) -> Result<bool, io::Error> {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let dir = cur_dir + "/" + &path.concat()[..];

    let _ = match env::set_current_dir(&dir) {
        Ok(()) => return Ok(true),
        Err(error) => return Err(error),
    };
}

/// The function that helps in working with the absolute paths
///
/// # Usage
///
/// ```no_run
/// let path: Vec<String> = !vec[String::from("/home/"), String::from("user/")];
/// let _ = absolute_path(&path);
/// // Current directories is "/home/user"
/// ```
pub fn absolute_path(path: &Vec<String>) -> Result<Option<String>, io::Error> {
    let dir = path.concat();
    let _ = match env::set_current_dir(&dir) {
        Ok(()) => return Ok(None),
        Err(error) => return Err(error),
    };
}

/// The function that transition to a home directory
///
/// # Usage
///
/// ```no_run
/// let _ = go_home()
/// // Current directories is "/home/user"
/// ```
pub fn go_home() -> Result<Option<String>, io::Error> {
    let _ = match env::set_current_dir(home_dir()) {
        Ok(()) => return Ok(None),
        Err(error) => return Err(error),
    };
}