static mut COUNTER: usize = 0;
pub static mut TOKENS: Vec<Types> = Vec::new();

#[derive(Debug)] pub enum Types {
    // Single character tokens
    LeftParen, RightParen, LeftBrace, RightBrace, Comma, Dot, Semicolon, Comment, Quote,

    // Operators
    Plus, Minus, Times, Divide, Bigger, Smaller, And, Not, Or, Equal, EqualTo,
    PlusEqual, MinusEqual, TimesEqual, DivideEqual, BiggerEqual, SmallerEqual,

    // Literals
    Identifier, String, Int, Float, Binary, Hex, If, Else, Class, Struct, Print, Return, True, False, While, Signed, Unsigned, Space
}

fn add_token(token_type: Types) {
    unsafe {
        TOKENS.push(token_type);
    }
}

fn advance(code: &str) -> Option<char> {
    unsafe {
        COUNTER += 1;
        return code.chars().nth(COUNTER - 1);
    }
}

pub fn tokenize(code: &str) {
    let mut commented = false;
    while let Some(c) = advance(code) {
        if commented == false {
            match c {
                '{' => add_token(Types::LeftBrace),
                '}' => add_token(Types::RightBrace),
                '#' => {
                    add_token(Types::Comment);
                    commented = true;
                }
                ')' => add_token(Types::RightParen),
                '(' => add_token(Types::LeftParen),
                ',' => add_token(Types::Comma),
                '.' => add_token(Types::Dot),
                ';' => add_token(Types::Semicolon),
                '"' => add_token(Types::Quote),
                '+' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::PlusEqual);
                    } else {
                        add_token(Types::Plus);
                    }
                }
                '-' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::MinusEqual);
                    } else {
                        add_token(Types::Minus);
                    }
                }
                '*' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::TimesEqual);
                    } else {
                        add_token(Types::Times);
                    }
                }
                '/' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::DivideEqual);
                    } else {
                        add_token(Types::Divide);
                    }
                }
                '>' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::BiggerEqual);
                    } else {
                        add_token(Types::Bigger);
                    }
                }
                '<' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::SmallerEqual);
                    } else {
                        add_token(Types::Smaller);
                    }
                }
                '&' => {
                    if let Some('&') = advance(code) {
                        add_token(Types::And);
                    }
                }
                '!' => add_token(Types::Not),
                '|' => {
                    if let Some('|') = advance(code) {
                        add_token(Types::Or);
                    }
                }
                '=' => {
                    if let Some('=') = advance(code) {
                        add_token(Types::EqualTo);
                    } else {
                        add_token(Types::Equal);
                    }
                }
                _ => {}
            }
        } else {
            match c {
                '{' => add_token(Types::LeftBrace),
                '}' => add_token(Types::RightBrace),
                '#' => {
                    add_token(Types::Comment);
                    commented = false;
                }
                _ => {}
            }
        }
    }
}