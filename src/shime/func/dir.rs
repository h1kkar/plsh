use std::env;

extern crate dirs;

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

pub fn cd(path: &String) {
    let path_split: Vec<String> = path.split_inclusive('/').map(String::from).collect();

    for i in path_split {
        match &i[..] {
            "../" | ".." => {
                back()
            },
            _ => next(&vec![i])
        }
    }
}

fn back() {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let mut dir_split: Vec<String> = cur_dir.split_inclusive('/').map(String::from).collect();
    let _ = dir_split.remove(dir_split.len()-1);

    let dirs = dir_split.concat();
    let _ = match env::set_current_dir(&dirs) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}

fn next(path: &Vec<String>) {
    let cur_dir = env::current_dir().unwrap().display().to_string();

    let dir = cur_dir + "/" + &path.concat()[..];

    let _ = match env::set_current_dir(&dir) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}

pub fn home_open() {
    let _ = match env::set_current_dir(home_dir()) {
        Ok(()) => print!(""),
        Err(error) => println!("{0}", error),
    };
}