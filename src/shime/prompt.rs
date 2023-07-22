use std::io::*;

/// A function that simply prints prompt, nothing special.
///
/// # Example
///
/// ```no_run
/// exec('%');
/// println!("hi\n");
///
/// exec('$');
/// println!("hi")
/// ```
///
/// ```no_run
/// % hi
///
/// $ hi
/// ```
pub fn exec(c: char) {
    print!("{c} ");
    stdout().flush().unwrap();    
}