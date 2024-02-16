use std::fs::File;
use std::io::{BufRead, BufReader};
mod lexer;

fn main() -> std::io::Result<()> {
    let file = File::open("src/main.msq")?;
    let reader = BufReader::new(file);
    let mut string = String::new();
    for line in reader.lines() {
        string.push_str(&line?);
    }
    println!("{}\n\n", string);
    
    println!("{}", lexer::tokenize(string).len());
    Ok(())
}

fn print_struct(token: lexer::TOKEN) {
    print!("TYPE: ");
    match token.kind {
        lexer::TYPE::DOT => println!("DOT"),
        lexer::TYPE::COMMA => println!("COMMA"),
        lexer::TYPE::SPACE => println!("SPACE"),
        lexer::TYPE::NONE => println!("NONE")
    };
    println!("VALUE: {}", token.value)
}