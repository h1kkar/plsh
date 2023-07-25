use std::io;

/// A function that takes values from the keyboard and returns them
/// in the form of a string with the type of `String`.
pub fn read_cmd() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd)
        .expect("failed to read in cmd");
    
    cmd.trim().to_string()
}