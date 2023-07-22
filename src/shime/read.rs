use std::io;

/// A function that takes values from the keyboard and returns them
/// in the form of a string with the type of `String`
///
/// # Example
///
/// ```no_run
/// % echo "hi"
/// ```
///
/// ```no_run
/// let cmd = read::read_cmd();
/// assert_eq!("echo \"hi\"", cmd());
/// ```
pub fn read_cmd() -> String {
    let mut cmd = String::new();
    io::stdin().read_line(&mut cmd)
        .expect("failed to read in cmd");
    
    cmd.trim().to_string()
}