mod shell;

use std::io::prelude::*;

fn prompt() {
    print!("jouet$ ");
    std::io::stdout().flush().ok().expect("Could not flush stdout");
}

fn main() {
    prompt();
    let stdin = std::io::BufReader::new(std::io::stdin());
    for line in stdin.lines() {
        let result = shell::eval_ln(line.unwrap());
        println!("{:?}", result);
        prompt();
    }
}
