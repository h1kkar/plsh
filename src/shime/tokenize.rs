pub struct Command {
    pub keyword: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn tokenize_arg(c: String) -> Command {
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
        assert_eq!("", Command::tokenize_arg("".to_string()).keyword)
    }

    #[test]
    fn test_keyword() {
        assert_eq!("test", Command::tokenize_arg("test".to_string()).keyword)
    }

    #[test]
    fn no_arg() {
        assert_eq!(0, Command::tokenize_arg("test".to_string()).args.len())
    }

    #[test]
    fn one_arg() {
        assert_eq!(1, Command::tokenize_arg("test one".to_string()).args.len())
    }

    #[test]
    fn multi_args() {
        assert_eq!(3, Command::tokenize_arg("test one two three".to_string()).args.len())
    }

    #[test]
    fn quotes () {
        assert_eq!(2, Command::tokenize_arg("test \"one two\" three".trim().to_string()).args.len())
    }
}