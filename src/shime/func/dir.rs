use std::env;

extern crate dirs;

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
            String::from("")
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
        absolute_path(&path_split)
    } else {
        for i in path_split {
            match &i[..] {
                "../" | ".." => {
                    back()
                },
                _ => next(&vec![i])
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
/// back()
/// // Current directories is "shime"
/// ```
pub fn back() {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let mut dir_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let _ = dir_split.remove(dir_split.len()-1);

    let dirs = dir_split.concat();
    let _ = match env::set_current_dir(&dirs) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}

/// The function that transition to a specific directory is lower in the directory tree (accompanies the [cd function][cd]).
///
/// # Usage
///
/// ```no_run
/// // Current directories is "shime"
/// let path: String = String::from("src");
/// next(&vec![path]);
/// // Current directories is "shime/src"
/// ```
pub fn next(path: &Vec<String>) {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let dir = cur_dir + "/" + &path.concat()[..];

    let _ = match env::set_current_dir(&dir) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}

pub fn absolute_path(path: &Vec<String>) {
    let dir = path.concat();
    let _ = match env::set_current_dir(&dir) {
        Ok(()) => print!(""),
        Err(err) => println!("{0}", err),
    };
}

/// The function that transition to a home directory
///
/// # Usage
///
/// ```no_run
/// go_home()
/// // Current directories is "/home/user"
/// ```
pub fn go_home() {
    let _ = match env::set_current_dir(home_dir()) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}