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
    WhiteSpace,
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
    Identifier,
    Float,
    Int
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
    let mut is_float: bool = false;
    // Iterate over the characters of the code string
    while counter < code.len() {
        let c: char = get_char(code, counter);

        match c {
            '+' => {
                let next_str = get_str(code, counter, counter + 1);
                if next_str == "+=" {
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
                if next_str == "-=" {
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
                if next_str == "/=" {
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
                if next_str == "*=" {
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
                if next_str == ">=" {
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
                if next_str == "<=" {
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
            ' ' => {
                while get_char(code, counter + 1) == ' ' {
                    counter += 1;
                }
                tokens.push(TOKEN {
                    kind: TYPE::WhiteSpace,
                    value: c.to_string(),
                });
            },
            's' => {
                let next_str = get_str(code, counter, counter + 2);
                if next_str == "str" {
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
                if next_str == "==" {
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
                if next_str == "int" {
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
            '0'..='9'=> {
                while get_char(code, counter + id_counter) != ',' && get_char(code, counter + id_counter) != '}' && get_char(code, counter + id_counter) != '{' && get_char(code, counter + id_counter) != '[' && get_char(code, counter + id_counter) != ']' && get_char(code, counter + id_counter) != '(' && get_char(code, counter + id_counter) != ')' && get_char(code, counter + id_counter) != ';' && get_char(code, counter + id_counter) != ':' && get_char(code, counter + id_counter) != '/' && get_char(code, counter + id_counter) != ' ' {
                    value.push(get_char(code, counter + id_counter));
                    if get_char(code, counter + id_counter) == '.' {
                        is_float = true;
                    }
                    id_counter += 1;
                }
                if is_float {
                    tokens.push(TOKEN {
                        kind: TYPE::Float,
                        value: value,
                    });
                } else {
                    tokens.push(TOKEN {
                        kind: TYPE::Int,
                        value: value,
                    });
                }
                counter += id_counter - 1;
                is_float = false;
                value = String::new();
                id_counter = 0;
            }
            _ => {

                    if counter != 0 && get_char(code, counter - 1).is_alphabetic() {
                        while counter != 0 && get_char(code, counter - 1).is_alphabetic() {
                            counter -= 1;
                        } 
                       tokens.pop();
                    }
                    while get_char(code, counter + id_counter) != '.' && get_char(code, counter + id_counter) != ',' && get_char(code, counter + id_counter) != '}' && get_char(code, counter + id_counter) != '{' && get_char(code, counter + id_counter) != '[' && get_char(code, counter + id_counter) != ']' && get_char(code, counter + id_counter) != '(' && get_char(code, counter + id_counter) != ')' && get_char(code, counter + id_counter) != ';' && get_char(code, counter + id_counter) != ':' && get_char(code, counter + id_counter) != '/' && get_char(code, counter + id_counter) != ' ' {
                        value.push(get_char(code, counter + id_counter));
                        id_counter += 1;
                    }
                    tokens.push(TOKEN {
                        kind: TYPE::Identifier,
                        value: value.clone(),
                    });
                    counter += id_counter - 1;
                    id_counter = 0;
                    value = String::new();
                }
        }
        counter += 1;
    }
    return tokens
}