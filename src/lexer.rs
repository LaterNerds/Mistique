#[derive(Debug)]
pub enum TYPE {
    // Operations
    Plus,
    PlusEquals,
    Minus,
    MinusEquals,
    Divide,
    DivideEquals,
    Multiply,
    MultiplyEquals,
    Mod,
    Bigger,
    BiggerEquals,
    Smaller,
    SmallerEquals,
    EqualTo,

    // Single Character
    Dot,
    Comma,
    Space,
    Equal,
    LBracket,
    RBracket,
    LBrace,
    RBrace,
    LParenthesis,
    RParenthesis,
    Colon,
    SemiColon,

    // Type Declaration
    StringType,
    IntType,
    FloatType,
    VarDec,

    //Varrying values
    Identifier
}

pub struct TOKEN {
    pub kind: TYPE,
    pub value: String,
}

/// Retrieves the character at the specified index in the given string.
fn get_char(string: &str, i: usize) -> char {
    string.as_bytes()[i] as char
}

/// Extracts a substring from the given string, starting from the 'start' index to the 'end' index.
fn get_str(string: &str, start: usize, end: usize) -> String {
    string[start..=end].to_string()
}

/// Tokenizes the input code string into a vector of TOKENs based on predefined character mappings.
pub fn tokenize(code: &str) -> Vec<TOKEN> {
    let mut value: String = String::new(); 
    let mut counter: usize = 0;
    let mut tokens: Vec<TOKEN> = Vec::new();
    let mut id_counter: usize = 0;
    // Iterate over the characters of the code string
    while counter < code.len() {
        let c: char = get_char(code, counter);

        match c {
            '+' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "+= " {
                    tokens.push(TOKEN {
                        kind: TYPE::PlusEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Plus,
                        value: c.to_string(),
                    });
                }
            },
            '-' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "-= " {
                    tokens.push(TOKEN {
                        kind: TYPE::MinusEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Minus,
                        value: c.to_string(),
                    });
                }
            },
            '/' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "/= " {
                    tokens.push(TOKEN {
                        kind: TYPE::DivideEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Divide,
                        value: c.to_string(),
                    });
                }
            },
            '*' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "*= " {
                    tokens.push(TOKEN {
                        kind: TYPE::MultiplyEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Multiply,
                        value: c.to_string(),
                    });
                }
            },
            '>' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == ">= " {
                    tokens.push(TOKEN {
                        kind: TYPE::BiggerEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Bigger,
                        value: c.to_string(),
                    });
                }
            },
            '<' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "<= " {
                    tokens.push(TOKEN {
                        kind: TYPE::SmallerEquals,
                        value: next_str,
                    });
                    counter += 1;
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Smaller,
                        value: c.to_string(),
                    });
                }
            },
            '.' => tokens.push(TOKEN {
                kind: TYPE::Dot,
                value: c.to_string(),
            }),
            ',' => tokens.push(TOKEN {
                kind: TYPE::Comma,
                value: c.to_string(),
            }),
            ' ' => tokens.push(TOKEN {
                kind: TYPE::Space,
                value: c.to_string(),
            }),
            's' => {
                let next_str = get_str(code, counter, counter + 2);
                if next_str == "str " {
                    tokens.push(TOKEN {
                        kind: TYPE::StringType,
                        value: next_str,
                    });
                    counter += 2;
                }
            },
            '%' => tokens.push(TOKEN {
                kind: TYPE::Mod,
                value: c.to_string(),
            }),
            '=' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "== " {
                    tokens.push(TOKEN {
                        kind: TYPE::EqualTo,
                        value: next_str,
                    });
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Equal,
                        value: "=".to_string(),
                    });
                }
            },
            '[' => tokens.push(TOKEN {
                kind: TYPE::LBracket,
                value: c.to_string(),
            }),
            ']' => tokens.push(TOKEN {
                kind: TYPE::RBracket,
                value: c.to_string(),
            }),
            '{' => tokens.push(TOKEN {
                kind: TYPE::LBrace,
                value: c.to_string(),
            }),
            '}' => tokens.push(TOKEN {
                kind: TYPE::RBrace,
                value: c.to_string(),
            }),
            '(' => tokens.push(TOKEN {
                kind: TYPE::LParenthesis,
                value: c.to_string(),
            }),
            ')' => tokens.push(TOKEN {
                kind: TYPE::RParenthesis,
                value: c.to_string(),
            }),
            ':' => tokens.push(TOKEN {
                kind: TYPE::Colon,
                value: c.to_string(),
            }),
            ';' => tokens.push(TOKEN {
                kind: TYPE::SemiColon,
                value: c.to_string(),
            }),
            'i' => {
                let next_str = get_str(code, counter, counter + 2);
                if next_str == "int " {
                    tokens.push(TOKEN {
                        kind: TYPE::IntType,
                        value: next_str,
                    });
                    counter += 2;
                }
            }
            'f' => {
                let next_str = get_str(code, counter, counter + 2);
                if next_str == "flt " {
                    tokens.push(TOKEN {
                        kind: TYPE::FloatType,
                        value: next_str,
                    });
                    counter += 2;
                }
            }
            'h' => {
                let next_str = get_str(code, counter, counter + 2);
                if next_str == "hld" {
                    tokens.push(TOKEN {
                        kind: TYPE::VarDec,
                        value: next_str,
                    });
                    counter += 2;
                }
            }
            _ => {
                if tokens[counter - 1].value == " " || tokens[counter - 1].value == "." || tokens[counter - 1].value == "," || tokens[counter - 1].value == "{" || tokens[counter - 1].value == "}" || tokens[counter - 1].value == "[" || tokens[counter - 1].value == "]" || tokens[counter - 1].value == "(" || tokens[counter - 1].value == ")" || tokens[counter - 1].value == ";" {
                    while get_char(code, counter + id_counter) != '.' && get_char(code, counter + id_counter) != ',' && get_char(code, counter + id_counter) != '}' && get_char(code, counter + id_counter) != '{' && get_char(code, counter + id_counter) != '[' && get_char(code, counter + id_counter) != ']' && get_char(code, counter + id_counter) != '(' && get_char(code, counter + id_counter) != ')' && get_char(code, counter + id_counter) != ';' && get_char(code, counter + id_counter) != ':' && get_char(code, counter + id_counter) != '/' {
                        value.push(get_char(code, counter + id_counter));
                        id_counter += 1;
                    }
                    tokens.push(TOKEN {
                        kind: TYPE::Identifier,
                        value: value.clone(),
                    });
                    counter += id_counter;
                    id_counter = 0;
                }
                
            }
        }
        counter += 1;
    }
    return tokens
}