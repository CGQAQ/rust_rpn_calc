fn main() {
    println!("Hello, world!");
}

#[derive(Debug, Eq, PartialEq)]
enum Op_t {
    Plus,
    Minus,
    Multi,
    Div,
    Pow,
    Mod,
    Factorial,
    Par_left,  // (
    Par_right, // )
}

#[derive(Debug, PartialEq, Eq)]
enum Func_t {
    Sum,
    Average,
    Sqrt,
}

#[derive(Debug, PartialEq, Eq)]
//['+', '-', '*', '/', '^', '%', '!', '(', ')']
enum Token {
    Op(Op_t),
    Num(i64),
    Func(Func_t),
}

#[derive(Debug, Eq, PartialEq)]
enum TokenState {
    CHAR,
    NUM,
    SYM,
    EMPTY,
}

#[test]
fn test_tokenizer() {
    let actual = tokenizer("12+3").unwrap();
    assert_eq!(
        vec![Token::Num(12), Token::Op(Op_t::Plus), Token::Num(3)],
        actual
    );
    let actual = tokenizer("12*3+5/2").unwrap();
    assert_eq!(
        vec![
            Token::Num(12),
            Token::Op(Op_t::Multi),
            Token::Num(3),
            Token::Op(Op_t::Plus),
            Token::Num(5),
            Token::Op(Op_t::Div),
            Token::Num(2)
        ],
        actual
    );
    let actual = tokenizer("(2+2)! % 5^2").unwrap();
    assert_eq!(
        vec![
            Token::Op(Op_t::Par_left),
            Token::Num(2),
            Token::Op(Op_t::Plus),
            Token::Num(2),
            Token::Op(Op_t::Par_right),
            Token::Op(Op_t::Factorial),
            Token::Op(Op_t::Mod),
            Token::Num(5),
            Token::Op(Op_t::Pow),
            Token::Num(2)
        ],
        actual
    );
}

fn to_token(cw: TokenState, s: String) -> Result<Token, ()> {
    match cw {
        TokenState::CHAR => {
            if s == "sum".to_owned() {
                Ok(Token::Func(Func_t::Sum))
            } else if s == "average".to_owned() {
                Ok(Token::Func(Func_t::Average))
            } else if s == "sqrt".to_owned() {
                Ok(Token::Func(Func_t::Sqrt))
            } else {
                Err(())
            }
        }
        TokenState::NUM => match s.parse::<i64>() {
            Ok(n) => Ok(Token::Num(n)),
            Err(_) => Err(()),
        },

        //     enum Op_t {
        // Plus,
        // Minus,
        // Multi,
        // Div,
        // Pow,
        // Mod,
        // Factorial,
        // Par_left,  // (
        // Par_right, // )
        TokenState::SYM => match &s as &str {
            "+" => Ok(Token::Op(Op_t::Plus)),
            "-" => Ok(Token::Op(Op_t::Minus)),
            "*" => Ok(Token::Op(Op_t::Multi)),
            "/" => Ok(Token::Op(Op_t::Div)),
            "^" => Ok(Token::Op(Op_t::Pow)),
            "%" => Ok(Token::Op(Op_t::Mod)),
            "!" => Ok(Token::Op(Op_t::Factorial)),
            "(" => Ok(Token::Op(Op_t::Par_left)),
            ")" => Ok(Token::Op(Op_t::Par_right)),
            _ => Err(()),
        },
        TokenState::EMPTY => Err(()),
    }
}

fn tokenizer(s: &str) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = vec![];
    let mut buf: String = String::new();
    let mut currentState = TokenState::EMPTY;
    for ch in s.chars() {
        match ch {
            '0'..='9' => {
                if currentState != TokenState::NUM
                    && (!&buf.is_empty() || currentState == TokenState::EMPTY)
                {
                    if currentState != TokenState::EMPTY {
                        result.push(to_token(currentState, buf.clone()).unwrap());
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
                    result.push(to_token(currentState, buf.clone()).unwrap());
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
                        result.push(to_token(currentState, buf.clone()).unwrap());
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
        result.push(to_token(currentState, buf).unwrap())
    }
    Ok(result)
}

#[test]
fn test_generate_rpn() {}

fn generate_rpn(tokens: Vec<String>) {}
