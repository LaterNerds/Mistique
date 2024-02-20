use std::fs::File;
use std::io::{BufRead, BufReader};
mod lexer;

fn main() -> std::io::Result<()> {
    // Open the file specified (assuming "src/main.msq") and create a buffered reader.
    let file = File::open("src/main.msq")?;
    let reader = BufReader::new(file);

    // Read the lines from the file and concatenate them into a single string.
    let mut string = String::new();
    for line in reader.lines() {
        string.push_str(&line?);
    }

    // Tokenize the string using the lexer module and print the resulting tokens.
    print_struct(lexer::tokenize(&string));
    Ok(())
}

/// Prints the information about each token in the vector.
fn print_struct(tokens: Vec<lexer::TOKEN>) {
    for token in tokens {
        print!("TYPE: ");
        match token.kind {
            lexer::TYPE::Float => println!("Float"),
            lexer::TYPE::Int => println!("Int"),
            lexer::TYPE::Identifier => println!("IDENTIFIER"),
            lexer::TYPE::Dot => println!("Dot"),
            lexer::TYPE::Comma => println!("Comma"),
            lexer::TYPE::WhiteSpace => println!("WhiteSpace"),
            lexer::TYPE::Plus => println!("Plus"),
            lexer::TYPE::PlusEquals => println!("PlusEquals"),
            lexer::TYPE::Minus => println!("Minus"),
            lexer::TYPE::MinusEquals => println!("MinusEquals"),
            lexer::TYPE::Divide => println!("Divide"),
            lexer::TYPE::DivideEquals => println!("DivideEquals"),
            lexer::TYPE::Multiply => println!("Multiply"),
            lexer::TYPE::MultiplyEquals => println!("MultiplyEquals"),
            lexer::TYPE::Mod => println!("Mod"),
            lexer::TYPE::Bigger => println!("Bigger"),
            lexer::TYPE::BiggerEquals => println!("BiggerEquals"),
            lexer::TYPE::Smaller => println!("Smaller"),
            lexer::TYPE::SmallerEquals => println!("SmallerEquals"),
            lexer::TYPE::Equal => println!("Equal"),
            lexer::TYPE::LBracket => println!("LBracket"),
            lexer::TYPE::RBracket => println!("RBracket"),
            lexer::TYPE::LBrace => println!("LBrace"),
            lexer::TYPE::RBrace => println!("RBrace"),
            lexer::TYPE::LParenthesis => println!("LParenthesis"),
            lexer::TYPE::RParenthesis => println!("RParenthesis"),
            lexer::TYPE::StringType => println!("StringType"),
            lexer::TYPE::IntType => println!("IntType"),
            lexer::TYPE::FloatType => println!("FloatType"),
            lexer::TYPE::VarDec => println!("VarDec"),
            lexer::TYPE::EqualTo => println!("EqualTo"),
            lexer::TYPE::Colon => println!("Colon"),
            lexer::TYPE::SemiColon => println!("SemiColon")
        };
        println!("VALUE: \"{}\"", token.value);
    }
}
