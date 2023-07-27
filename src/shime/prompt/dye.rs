use rand::Rng;
use ansi_colors::*;

pub fn clr(c: char) -> String {
    match n() {
        0 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.red();
            s.to_string()
        },
        1 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.green();
            s.to_string()
        },
        2 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.light_green();
            s.to_string()
        },
        3 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.light_yellow();
            s.to_string()
        },
        4 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.light_blue();
            s.to_string()
        },
        5 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.magenta();
            s.to_string()
        },
        6 => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.light_cyan();
            s.to_string()
        },
        _ => {
            let c = &c.to_string()[..];
            let mut s = ColouredStr::new(c);
            s.green();
            s.to_string()
        },
    }
}

#[doc(hidden)]
fn n() -> i8 {
    rand::thread_rng().gen_range(0..7)
}
