#[derive(Debug)]
pub enum TYPE {
    DOT,
    COMMA,
    SPACE,

    NONE
}

pub struct TOKEN {
    pub kind:TYPE,
    pub value: String
}

fn get_char(string:String, i: usize) -> char {
    return string.as_bytes()[i] as char;
}

pub fn tokenize(code: String) -> Vec<TOKEN> {
    let mut counter: usize = 0;
    let mut tokens: Vec<TOKEN> = Vec::new();
    while counter < code.len() {
        println!("{}", counter);
        let c: char = get_char(code.clone(), counter);
        match c {
            '.' => tokens.push(TOKEN {
                kind:TYPE::DOT,
                value: c.to_string()
            }),
            ',' => tokens.push(TOKEN {
                kind:TYPE::COMMA,
                value: c.to_string()
            }),
            ' ' => tokens.push(TOKEN {
                kind:TYPE::SPACE,
                value: c.to_string()
            }),
            _ => tokens.push(TOKEN {
                kind: TYPE::NONE,
                value: c.to_string()
            })
        }
        counter += 1;
    }
    return tokens;
}