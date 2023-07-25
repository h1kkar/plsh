use dye::clr;

use std::io::*;

use crate::shime::prompt::sudo::root;

/// A function that simply prints prompt, nothing special.
///
/// # Example
///
/// ```no_run
/// exec('❯');
/// println!("hi\n");
///
/// exec('❯');
/// println!("hi")
/// ```
///
/// ```no_run
/// ~
/// ❯ hi
///
/// ~
/// ❯ hi
/// ```
pub fn exec(c: char) {
    print!("\n");

    line();
    print!("{0} ", clr(c));
    stdout().flush().unwrap();    
    
}

fn line(/*ch: Vec<String>*/) {
    root();
    print!("{0}\n", dir::main())
}

pub mod dye;
pub mod dir;
pub mod git;
pub mod lang;
pub mod sudo;
pub mod time;