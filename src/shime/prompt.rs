use dirs;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

pub fn exec(c: &str) -> String {
    let c = dye::clr(c);
    
    let mut rl = DefaultEditor::new().unwrap();
    
    if rl.load_history(&get::history()).is_err() {
        println!("history file not found")
    }
    
    let mut s = String::new();
    
        print!("\n");
        line();
        
        let readline = rl.readline(&c);
        
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str()).unwrap();
                s.push_str(&line);
            },
            Err(ReadlineError::Interrupted) => {},
            Err(ReadlineError::Eof) => {},
            Err(err) => {
                println!("Error: {:?}", err);
            }
        }
    
    rl.save_history(&get::history()).unwrap();
    return s;
}

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
    
    pub fn history() -> String {
        match dirs::config_dir() {
        Some(path) => {
            let mut path = path.display().to_string();
            path.push_str("/shime/shime_history");
            path
        },
        None => "".to_string()
    }
    }
}

pub mod dye;
pub mod dir;
pub mod git;
pub mod cargo;
pub mod sudo;