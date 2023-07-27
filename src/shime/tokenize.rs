/// structure that contains the command keyword and the arguments.
#[derive(Debug)]
pub struct Command {
    /// A string containing a keyword.
    pub keyword: String,
    /// Vector containing all the command arguments.
    pub args: String,
}

/// Implementation, which takes away the input and gives the structure
/// of `Command`.
impl Command {
    /// The function split commands to keyword and args
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