use std::{
    io::*,
    env,
};

use super::func::dir::home_dir;

use colorized::*;
use rand::Rng;

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
    let cur_dir = env::current_dir().unwrap().display().to_string();
    let home = home_dir() + "/";
    let d: Vec<String> = cur_dir.split(&home).map(String::from).collect();
    if cur_dir == home_dir() {
        println!("{0}", "~".color(Colors::BrightYellowFg))
    } else {
        if d.len() == 1 {
            println!("{0}", d[0].color(Colors::RedFg))
        } else {
            println!("{0}", d.concat().color(Colors::BrightCyanFg))
        }
    }

    let clr = match n() {
        0 => Colors::RedFg,
        1 => Colors::GreenFg,
        2 => Colors::BrightGreenFg,
        3 => Colors::BrightYellowFg,
        4 => Colors::BlueFg,
        5 => Colors::BrightBlueFg,
        6 => Colors::MagentaFg,
        7 => Colors::BrightCyanFg,
        _ => Colors::GreenFg,
    };

    print!("{0} ", c.to_string().color(clr));
    stdout().flush().unwrap();    
}

#[doc(hidden)]
fn n() -> i8 {
    rand::thread_rng().gen_range(0..8)
}