use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("src/main.msq")?;
    let reader = BufReader::new(file);
    let mut string = String::new();
    for line in reader.lines() {
        string.push_str(&line?);
    }
    println!("{}", string);
    Ok(())
}