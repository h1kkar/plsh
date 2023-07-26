/// structure that contains the command keyword and the arguments.
#[derive(Debug)]
pub struct Command {
    /// A string containing a keyword.
    pub keyword: String,
    /// Vector containing all the command arguments.
    pub args: Vec<String>,
}

/// Implementation, which takes away the input and gives the structure
/// of `Command`.
impl Command {
    /// The main function that simply compares the command with `""` and
    /// performs the necessary actions.
    pub fn start(c: &String) -> Command {
        match &c[..] {
            "" => {
                Command {
                    keyword: "".to_string(),
                    args: [].to_vec(),
                }
            },
            _ => {
                let cmd: Vec<String> = c.split_whitespace().map(|s| s.to_string()).collect();

                Command::search_quotes(cmd)
                
            },
        }


    }

    /// The function that receives the vector searches brackets and
    /// returns the structure of the Command with already combined
    /// values depending on the obtained values.
    fn search_quotes(mut c: Vec<String>) -> Command {
        let mut i = 0;
        let mut has_quote = false;
        while i < c.len() {
            if has_quote {
                let s = c.remove(i);
                let s = if s.ends_with('"') {
                    has_quote = false;
                    &s[..s.len()-1]
                } else {
                    &s
                };
                c[i-1].push(' ');
                c[i-1].push_str(s);
            } else {
                if c[i].starts_with('"') {
                    c[i].replace_range(..1, "");
                    if c[i].ends_with('"') {
                        let quote = c[i].len()-1;
                        c[i].replace_range(quote.., "");
                    } else {
                        has_quote = true;
                    }
                }

                i += 1;
            }
        }
        
        if has_quote {
            println!("unmatched quotes")
        }

        Command {
            keyword: c.remove(0),
            args: c,
        }
    }
}