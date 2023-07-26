use crate::shime::{
    tokenize::*,
    //ttoken::*,
    func::{
        dir::{
            self,
            go,
        },
        say,
    }
};

use ansi_colors::*;

/// The function responsible for the transition between the directors and the withdrawal of relevant information.
pub fn cd(cmds: Command) {
    //if cmds.len() == 1 {
        if cmds.args.len() == 1 {
            go(&cmds.args[0])
        } else if cmds.args.len() == 0 {
            match dir::go_home() {
                Ok(_) => {
                    let mut m = ColouredStr::new("moved");
                    m.cyan();
                    let mut h = ColouredStr::new("home");
                    h.magenta();
                    println!("you {0} to the {1} dir", m, h)
                    },
                Err(error) => println!("{0}", error),
            }
        } else {
            println!("many args")
        }
    /*} else {
        println!("error")
    }*/

    
}

#[doc(hidden)]
pub fn clr() {
    // bl  r  g  y  b  m  c  w
    // bbl br bg by bb bm bc bw
    let s = "██";
    let mut bl = ColouredStr::new(s);
    bl.black();
    let mut r = ColouredStr::new(s);
    r.red();
    let mut g = ColouredStr::new(s);
    g.green();
    let mut y = ColouredStr::new(s);
    y.yellow();
    let mut b = ColouredStr::new(s);
    b.blue();
    let mut m = ColouredStr::new(s);
    m.magenta();
    let mut c = ColouredStr::new(s);
    c.cyan();
    let mut w = ColouredStr::new(s);
    w.gray();
    let mut bbl = ColouredStr::new(s);
    bbl.dark_gray();
    let mut br = ColouredStr::new(s);
    br.light_red();
    let mut bg = ColouredStr::new(s);
    bg.light_green();
    let mut by = ColouredStr::new(s);
    by.light_yellow();
    let mut bb = ColouredStr::new(s);
    bb.light_blue();
    let mut bm = ColouredStr::new(s);
    bm.pink();
    let mut bc = ColouredStr::new(s);
    bc.light_cyan();
    let mut bw = ColouredStr::new(s);
    bw.white();
    println!("{} {} {} {} {} {} {} {}", bl, r, g, y, b, m, c, w);
    println!("{} {} {} {} {} {} {} {}", bbl, br, bg, by, bb, bm, bc, bw);
}

/// The function responsible for the exit from `shime` and the withdrawal of relevant information.
pub fn exit() {
    let bye = say::bye();
    println!("{0}", bye);
}