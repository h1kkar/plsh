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

pub mod get;
pub mod dye;
pub mod dir;
pub mod git;
pub mod cargo;
pub mod sudo;