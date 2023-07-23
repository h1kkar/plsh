/// A function that controls all the main shell processes.
/// 
/// This function simply processes prompt, input, withdrawal and manages to which command to launch.
pub fn start() {
    use colorized::*;
    use read::*;
    use tokenize::*;
    use func::{
        dir::{
            self,
            cd,
        },
        bye,
    };
    use std::process::Command as Cmds;

    loop {
        prompt::exec('❯');
        let cmd = read_cmd();
        let cmds = Command::start(cmd);

        match &cmds.keyword[..] {
            "cd" => {
                if cmds.args.len() == 1 {
                    cd(&cmds.args[0])
                } else if cmds.args.len() == 0 {
                    match dir::go_home() {
                        Ok(_) => println!("you {0} to the {1} dir", "moved".color(Colors::CyanFg), "home".color(Colors::MagentaFg)),
                        Err(error) => println!("{0}", error),
                    }
                } else {
                    println!("many args")
                }
            },
            "clr" => {
                println!("{} {} {} {} {} {} {} {}", "██".color(Colors::BlackFg), "██".color(Colors::RedFg), "██".color(Colors::GreenFg), "██".color(Colors::YellowFg), "██".color(Colors::BlueFg), "██".color(Colors::MagentaFg), "██".color(Colors::CyanFg), "██".color(Colors::WhiteFg));
                println!("{} {} {} {} {} {} {} {}", "██".color(Colors::BrightBlackFg), "██".color(Colors::BrightRedFg), "██".color(Colors::BrightGreenFg), "██".color(Colors::BrightYellowFg), "██".color(Colors::BrightBlueFg), "██".color(Colors::BrightMagentaFg), "██".color(Colors::BrightCyanFg), "██".color(Colors::BrightWhiteFg));
            },
            "exit" => {
                let (bye, clr) = bye::say();
                println!("{0}", bye.color(clr));
                break;
            },
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