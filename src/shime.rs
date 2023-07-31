pub fn start() {
    use crate::{
        cmd::*,
        exec,
    };
    use tokenize::*;
    use func::say::hi;
    
    use ansi_colors::*;

    println!("{}", hi());

    loop {
        let cmd = prompt::exec("❯ ");
        let cmds = Command::start(&cmd);

        match &cmds.keyword[..] {
            "cd" => {
                cd(cmds)
            },
            "clr" => {
                clr()
            },
            "exit" => {
                exit();
                break;
            },
            "config" | "conf" | "c" => {
                let mut path = func::dir::home_dir();
                path.push_str("/.config");
                cd(Command { keyword: "cd".to_string(), args: path })
            },
            "home" | "h" => {
                match func::dir::go_home() {
                    Ok(_) => {
                        let mut m = ColouredStr::new("moved");
                        m.cyan();
                        let mut h = ColouredStr::new("home");
                        h.magenta();
                        println!("you {0} to the {1} dir", m, h)
                        },
                    Err(error) => {
                        let err = error.to_string();
                        let mut err = ColouredStr::new(&err);
                        err.back_red();
                        err.bold();
                        println!("{0}", err)
                    },
                }
            }
            "" => print!(""),
            _ => {
                exec::start(&cmd)
            },
        }
    }
}

pub mod prompt;
pub mod tokenize;
pub mod func;