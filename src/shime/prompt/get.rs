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

pub fn ver() {
    print!("{0}",cargo::ver())
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