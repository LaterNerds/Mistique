use std::fs::File;
use std::io::{self, Read};

mod lexer;

fn main() -> io::Result<()> {
    // Open the file
    let file_path = "tester.msq";
    let mut file = File::open(file_path)?;

    // Read the contents of the file into a String
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    lexer::tokenize(contents.as_str());
    unsafe {
        for token in &lexer::TOKENS {
            println!("{:?}", token);
        }
    }
    Ok(())
}