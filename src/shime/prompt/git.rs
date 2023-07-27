
//use ansi_colors::*;

pub struct Conf {
    pub branch: String,
    pub username: String,
    pub noncommited: bool,
    pub lastcommit: String,
}

pub fn config() -> Conf {
    let info = git_info::get();
    Conf {
        branch: info.current_branch.unwrap_or(alt_conf::branch()),
        username: info.user_name.unwrap_or("".to_string()),
        noncommited: info.dirty.unwrap_or(false),
        lastcommit: info.head.last_commit_hash_short.unwrap_or("".to_string())
    }
}

pub mod alt_conf {
    use std::{process::Command, str::from_utf8};

    pub fn branch() -> String {
        let head = Command::new("cat")
            .arg(".git/HEAD")
            .output()
            .expect("faild");
        from_utf8(head.stdout.as_slice()).unwrap().to_string()
    }
}
