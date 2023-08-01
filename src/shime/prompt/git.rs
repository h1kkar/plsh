pub struct Conf {
    pub branch: String,
    pub dirty: bool,
    pub lastcommit: String,
}

pub fn config() -> Conf {
    let info = git_info::get();
    Conf {
        branch: info.current_branch.unwrap_or(alt_conf::branch()),
        dirty: info.dirty.unwrap_or(false),
        lastcommit: info.head.last_commit_hash_short.unwrap_or("".to_string())
    }
}

pub mod get;
pub mod alt_conf;