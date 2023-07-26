use std::process::Command;

#[doc(hidden)]
pub fn start(cmds: String) {
    match Command::new("sh")
        .arg("-c")
        .arg(cmds)
        .spawn() {
        Ok(mut child) => {
            child.wait().unwrap();
        },
        Err(error) => println!("{error}")
    }
}