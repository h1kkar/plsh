#[derive(Debug)]
pub struct Command {
    pub keyword: String,
    pub args: String,
}

impl Command {
    pub fn start(c: &String) -> Command {
        match &c[..] {
            "" => {
                Command {
                    keyword: "".to_string(),
                    args: "".to_string(),
                }
            },
            _ => {
                let mut cmd: Vec<String> = c.split_inclusive(' ').map(|s| s.to_string()).collect();
                
                let mut keyword = cmd.remove(0);
                if keyword.ends_with(' ') {
                    let _ = keyword.remove(keyword.len()-1);
                };

                let args = cmd.concat();

                Command {
                    keyword,
                    args,
                }
            },
        }
    }
}