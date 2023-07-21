use std::io;

pub fn read_cmd() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd)
        .expect("failed to read in cmd");
    
    cmd.trim().to_string()
}