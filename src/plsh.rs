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
        let cmds = Command::tokenize_arg(cmd);

        match &cmds.keyword[..] {
            "cd" => {
                if cmds.args.len() == 1 {
                    cd(&cmds.args[0])
                } else if cmds.args.len() == 0 {
                    dir::home_open()
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

mod prompt;
mod read;
mod tokenize;
mod func;