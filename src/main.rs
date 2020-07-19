fn main() {
    println!("Hello, world!");
}

enum Op {}

#[derive(Debug, Eq, PartialEq)]
enum TokenState {
    CHAR,
    NUM,
    SYM,
    EMPTY,
}

fn tokenizer(s: &str) -> Result<Vec<String>, String> {
    let mut result: Vec<String> = vec![];
    let mut buf: String = String::new();
    let mut currentState = TokenState::EMPTY;
    for ch in s.chars() {
        match ch {
            '0'..='9' => {
                if currentState != TokenState::NUM
                    && (!&buf.is_empty() || currentState == TokenState::EMPTY)
                {
                    if currentState != TokenState::EMPTY {
                        result.push(buf.clone());
                    }
                    buf.clear();
                    buf.push(ch);
                } else if currentState == TokenState::NUM {
                    buf.push(ch);
                } else if currentState != TokenState::EMPTY {
                    return Err("currentState is not NUM but buf is empty".to_string());
                }
                currentState = TokenState::NUM;
            }
            symb if ['+', '-', '*', '/', '^', '%', '!', '(', ')'].contains(&ch) => {
                if !buf.is_empty() {
                    result.push(buf.clone());
                    buf.clear();
                }
                buf.push(symb);

                currentState = TokenState::SYM;
            }
            c if ch.is_ascii_alphabetic() => {
                if currentState != TokenState::CHAR
                    && (!buf.is_empty() || currentState == TokenState::EMPTY)
                {
                    if currentState != TokenState::EMPTY {
                        result.push(buf.clone());
                    }
                    buf.clear();
                    buf.push(ch);
                } else if currentState == TokenState::CHAR {
                    buf.push(c.to_ascii_lowercase())
                } else if currentState != TokenState::EMPTY {
                    return Err("currentState is no CHAR but buf is empty".to_string());
                }
                currentState = TokenState::CHAR;
            }
            ' ' => {}
            _ => return Err("unrecognized token".to_string()),
        }
    }
    if !buf.is_empty() {
        result.push(buf)
    }
    Ok(result)
}

#[test]
fn test_tokenizer() {
    let actual = tokenizer("12+3").unwrap();
    assert_eq!(
        vec!["12".to_string(), "+".to_string(), "3".to_string()],
        actual
    );
    let actual = tokenizer("12*3+5/2").unwrap();
    assert_eq!(
        vec![
            "12".to_string(),
            "*".to_string(),
            "3".to_string(),
            "+".to_string(),
            "5".to_string(),
            "/".to_string(),
            "2".to_string()
        ],
        actual
    );
    let actual = tokenizer("(2+2)! % 5^2").unwrap();
    assert_eq!(
        vec![
            "(".to_string(),
            "2".to_string(),
            "+".to_string(),
            "2".to_string(),
            ")".to_string(),
            "!".to_string(),
            "%".to_string(),
            "5".to_string(),
            "^".to_string(),
            "2".to_string()
        ],
        actual
    );
}
