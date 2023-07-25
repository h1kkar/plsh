/// A function that controls all the main shell processes.
/// 
/// This function simply processes prompt, input, withdrawal and manages to which command to launch.
pub fn start() {
    use crate::cmd::*;
    use read::*;
    use tokenize::*;
    use func::say::hi;

    println!("{}", hi());

    loop {
        prompt::exec('❯');
        let cmd = read_cmd();
        let cmds = Command::start(cmd);

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
            "" => print!(""),
            _ => {
                exec(cmds)
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