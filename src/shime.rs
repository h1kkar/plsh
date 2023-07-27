/// A function that controls all the main shell processes.
/// 
/// This function simply processes prompt, input, withdrawal and manages to which command to launch.
pub fn start() {
    use crate::{
        cmd::*,
        exec,
    };
    use read::*;
    use tokenize::*;
    use func::say::hi;

    use ansi_colors::*;

    println!("{}", hi());

    loop {
        prompt::exec('❯');
        let cmd = read_cmd();
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

/// Module responsible for processing prompt.
pub mod prompt;

/// The module responsible for entering commands from the keyboard.
pub mod read;

/// The module responsible for the processing of input.
pub mod tokenize;

/// A module, which is a combination of all internal functions, such as `cd`.
pub mod func;