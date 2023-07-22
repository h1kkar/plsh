/// A function that controls all the main shell processes.
/// 
/// This function simply processes prompt, input, withdrawal and manages to which command to launch.
pub fn start() {
    use read::*;
    use tokenize::*;
    use func::dir::{
        self,
        cd,
    };
    use std::process::Command as Cmds;

    loop {
        prompt::exec('%');
        let cmd = read_cmd();
        let cmds = Command::start(cmd);

        match &cmds.keyword[..] {
            "cd" => {
                if cmds.args.len() == 1 {
                    cd(&cmds.args[0])
                } else if cmds.args.len() == 0 {
                    dir::go_home()
                } else {
                    println!("many args")
                }
            },
            "exit" => break,
            "" => print!(""),
            _ => {
                match Cmds::new(cmds.keyword)
                    .args(cmds.args)
                    .current_dir(std::env::current_dir().unwrap())
                    .spawn() {
                    Ok(mut child) => {
                        child.wait().unwrap();
                    },
                    Err(err) => println!("{err}"),
                }
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