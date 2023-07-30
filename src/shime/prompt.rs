use std::io::*;

use dye::clr;

/// A function that simply prints prompt, nothing special.
pub fn exec(c: char) {
    print!("\n");

    line();
    print!("{0} ", clr(c));
    stdout().flush().unwrap();
}

/// The function responsible for the output of the useful line of prompt.
fn line() {
    get::root();
    get::dir();
    get::branch();
    get::delim();
    get::commit();
    get::pkgver();
    
    get::rustver();
    print!("\n")
}

pub mod get {
    use super::*;
    
    pub fn root() {
        print!("{0}", sudo::root())
    }
    
    pub fn dir() {
        print!("{0}", dir::main())
    }
    
    pub fn branch() {
        print!("{0}{1}", git::get::branch(), git::get::dirty())
    }
    
    pub fn commit() {
        print!("{0}", git::get::commit())
    }
    
    pub fn delim() {
        print!("❘ ")
    }
    
    pub fn pkgver() {
        print!("{0}",super::cargo::pkg_ver())
    }
    
    pub fn rustver() {
        print!("{}", super::cargo::rust_ver())
    }
}

pub mod dye;
pub mod dir;
pub mod git;
pub mod cargo;
pub mod sudo;
pub mod time;