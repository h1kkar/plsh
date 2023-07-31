use std::process::Command;

use ansi_colors::*;

pub fn start(cmds: &String) {
    match Command::new("sh")
        .arg("-c")
        .arg(cmds)
        .current_dir(std::env::current_dir().unwrap())
        .spawn() {
            Ok(mut child) => {
                match child.wait() {
                    Ok(_) => println!(""),
                    Err(error) => {
                        let err = error.to_string();
                        let mut err = ColouredStr::new(&err);
                        err.back_red();
                        err.bold();
                        println!("{err}")
                    }
                }
            },
            Err(error) => {
                let err = error.to_string();
                let mut err = ColouredStr::new(&err);
                err.back_red();
                err.bold();
                println!("{err}")
            }
    }
}