/// structure that contains the command keyword and the arguments.
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
    pub fn start(c: String) -> Command {
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
    ///
    /// # Example
    ///
    /// ```no_run
    /// let cmd: Vec<String> = vec![String::from("echo"), String::from("\"hi"), String::from("bro\"")];
    /// search_quotes(cmd);
    /// ```
    ///
    /// This is what happens at the exit:
    ///
    /// ```no_run
    /// Command {
    ///     keyword: "echo",
    ///     args: ["hi bro"],
    /// }
    /// ```
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


#[cfg(test)]
mod tokenize_test {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!("", Command::start("".to_string()).keyword)
    }

    #[test]
    fn test_keyword() {
        assert_eq!("test", Command::start("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
        assert_eq!(0, Command::start("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
        assert_eq!(1, Command::start("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
        assert_eq!(3, Command::start("test one two three".to_string()).args.len())
    }

    #[test]
    fn quotes () {
        assert_eq!(2, Command::start("test \"one two\" three".trim().to_string()).args.len())
    }
}