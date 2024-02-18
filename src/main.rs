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
            lexer::TYPE::Identifier => println!("IDENTIFIER"),
            lexer::TYPE::Dot => println!("DOT"),
            lexer::TYPE::Comma => println!("COMMA"),
            lexer::TYPE::Space => println!("SPACE"),
            lexer::TYPE::Plus => println!("PLUS"),
            lexer::TYPE::PlusEquals => println!("PLUS_EQUALS"),
            lexer::TYPE::Minus => println!("MINUS"),
            lexer::TYPE::MinusEquals => println!("MINUS_EQUALS"),
            lexer::TYPE::Divide => println!("DIVIDE"),
            lexer::TYPE::DivideEquals => println!("DIVIDE_EQUALS"),
            lexer::TYPE::Multiply => println!("MULTIPLY"),
            lexer::TYPE::MultiplyEquals => println!("MULTIPLY_EQUALS"),
            lexer::TYPE::Mod => println!("MOD"),
            lexer::TYPE::Bigger => println!("BIGGER"),
            lexer::TYPE::BiggerEquals => println!("BIGGER_EQUALS"),
            lexer::TYPE::Smaller => println!("SMALLER"),
            lexer::TYPE::SmallerEquals => println!("SMALLER_EQUALS"),
            lexer::TYPE::Equal => println!("EQUAL"),
            lexer::TYPE::LBracket => println!("LBRACKET"),
            lexer::TYPE::RBracket => println!("RBRACKET"),
            lexer::TYPE::LBrace => println!("LBRACE"),
            lexer::TYPE::RBrace => println!("RBRACE"),
            lexer::TYPE::LParenthesis => println!("LPARENTHESIS"),
            lexer::TYPE::RParenthesis => println!("RPARENTHESIS"),
            lexer::TYPE::StringType => println!("STRING_TYPE"),
            lexer::TYPE::IntType => println!("INT_TYPE"),
            lexer::TYPE::FloatType => println!("FLOAT_TYPE"),
            lexer::TYPE::VarDec => println!("VAR_DEC"),
            lexer::TYPE::EqualTo => println!("EQUAL_TO"),
            lexer::TYPE::Colon => println!("COLON"),
            lexer::TYPE::SemiColon => println!("SEMICOLON")
        };
        println!("VALUE: \"{}\"", token.value);
    }
}
