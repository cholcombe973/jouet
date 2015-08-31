extern crate regex;
use std::process::Command;

enum TYPE {
    STRING,
}

struct Token {
    tag : TYPE,
    data : String,
    children : Option<Vec<Token>>
}

impl Token {
    fn replace_children(&mut self, children : Option<Vec<Token>>) {
        self.children = children;
    }
}

fn parse_ln(input : String) -> Token {
    let whitespace_exp = regex::Regex::new(r"\s+").unwrap();
    let mut tokens = Vec::new();
    for word in whitespace_exp.split(&input) {
        // later we can use regular expressions to tag tokens
        // for now they're all going to be "string" :)
        // Further, we're not building an AST right now; but will
        // need to later in order to handle things like variable assignment
        let mut index = tokens.len();
        if index > 0 {
            index = index - 1;
        }
        tokens.insert(
            index,
            Token{tag: TYPE::STRING, data: word.to_string(), children: None}
        );
    }
    let mut root = tokens.pop().unwrap();
    if tokens.len() > 0 {
        root.replace_children(Some(tokens));
    }
    return root;
}

fn eval_cmd(code : Token) -> Token {
    let mut result = Command::new(code.data);
    match code.children {
        None => {},
        _  => {
            let mut children = code.children.unwrap();
            for _ in 0..children.len() {
                result.arg(children.pop().unwrap().data);
            }
        },
    }
    let stdout = String::from_utf8(result.output().unwrap().stdout).unwrap();
    return Token{tag: TYPE::STRING, data: stdout, children: None}
}

fn eval(code : Token) -> Token {
    match code.tag {
        TYPE::STRING => {
            return eval_cmd(code);
        }
    }
}

/**
 * Parse a string, then eval the resulting Token.
 **/
pub fn eval_ln(input : String) {
    let code = parse_ln(input);
    let result = eval(code);
    println!("{}", result.data);
}
