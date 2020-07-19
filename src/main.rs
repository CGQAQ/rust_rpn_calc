fn main() {
    println!("1+1={}", calculate("1+1"));
    println!("(1+2)^2={}", calculate("(1+2)^2"));
    println!("5+3!={}", calculate("5+3!"));
    println!("7+7*2={}", calculate("7+7*2"));
    println!("(3+3)/3={}", calculate("(3+3)/3"));
    println!("4!={}", calculate("4!"));
}

pub fn calculate(expr: &str) -> i64 {
    if let Ok(tokens) = tokenizer(expr) {
        return calc(generate_rpn(tokens));
    }
    -1
}

#[derive(Debug, Eq, PartialEq, Clone)]
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

fn get_op_precedence(op: &Token) -> i32 {
    if let Token::Op(op) = op {
        return match op {
            Op_t::Plus => 2,
            Op_t::Minus => 2,
            Op_t::Multi => 3,
            Op_t::Div => 3,
            Op_t::Pow => 4,
            Op_t::Mod => 3,
            Op_t::Factorial => 6,
            Op_t::Par_left => 5,
            Op_t::Par_right => 5,
        };
    }
    return -1;
}

#[derive(Debug, Eq, PartialEq, Clone)]
enum Assoc {
    Left,
    Right,
    Invalid,
}

fn get_op_associativity(op: &Token) -> Assoc {
    if let Token::Op(op) = op {
        return match op {
            Op_t::Plus => Assoc::Left,
            Op_t::Minus => Assoc::Left,
            Op_t::Multi => Assoc::Left,
            Op_t::Div => Assoc::Left,
            Op_t::Pow => Assoc::Right,
            Op_t::Mod => Assoc::Left,
            Op_t::Factorial => Assoc::Invalid,
            Op_t::Par_left => Assoc::Invalid,
            Op_t::Par_right => Assoc::Invalid,
        };
    }
    return Assoc::Invalid;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Func_t {
    Sum,
    Average,
    Sqrt,
}

#[derive(Debug, PartialEq, Eq, Clone)]
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
            _ => return Err(format!("unrecognized token: {}", ch)),
        }
    }
    if !buf.is_empty() {
        result.push(to_token(currentState, buf).unwrap())
    }
    Ok(result)
}

fn rpn_to_string(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for t in tokens {
        match t {
            Token::Op(op) => match op {
                Op_t::Plus => result += "+ ",
                Op_t::Minus => result += "- ",
                Op_t::Multi => result += "* ",
                Op_t::Div => result += "/ ",
                Op_t::Pow => result += "^ ",
                Op_t::Mod => result += "% ",
                Op_t::Factorial => result += "! ",
                Op_t::Par_left => result += "( ",
                Op_t::Par_right => result += ") ",
            },
            Token::Num(n) => result += &format!("{} ", n),
            Token::Func(f) => result += &format!("{} ", stringify!(f)),
        }
    }
    result
}

#[test]
fn test_generate_rpn() {
    let actual = generate_rpn(tokenizer("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3").unwrap());
    assert_eq!(
        vec![
            Token::Num(3),
            Token::Num(4),
            Token::Num(2),
            Token::Op(Op_t::Multi),
            Token::Num(1),
            Token::Num(5),
            Token::Op(Op_t::Minus),
            Token::Num(2),
            Token::Num(3),
            Token::Op(Op_t::Pow),
            Token::Op(Op_t::Pow),
            Token::Op(Op_t::Div),
            Token::Op(Op_t::Plus)
        ],
        actual
    );
    // println!("{:?}", generate_rpn(tokenizer("(8/2) - 3 * 2").unwrap()))
    // println!(
    //     "{}",
    //     rpn_to_string(generate_rpn(tokenizer("(8/2)+6-4+(6*6)").unwrap()))
    // );
}

fn generate_rpn(tokens: Vec<Token>) -> Vec<Token> {
    let mut output: Vec<Token> = vec![];
    let mut stack: Vec<Token> = vec![];

    for token in tokens {
        match &token {
            n @ Token::Num(_) => {
                output.push(n.clone());
            }
            f @ Token::Func(_) => {
                stack.push(f.clone());
            }
            lp @ Token::Op(Op_t::Par_left) => {
                stack.push(lp.clone());
            }
            Token::Op(Op_t::Par_right) => {
                while stack.last().unwrap().clone() != Token::Op(Op_t::Par_left) {
                    if let Some(v) = stack.pop() {
                        output.push(v);
                    }
                }
                if stack.last().unwrap().clone() == Token::Op(Op_t::Par_left) {
                    stack.pop();
                }
            }
            o @ Token::Op(_) => {
                //while ((there is a operator at the top of the operator stack)
                //   and ((the operator at the top of the operator stack has greater precedence)
                //    or (the operator at the top of the operator stack has equal precedence and the token is left associative))
                //   and (the operator at the top of the operator stack is not a left parenthesis)):
                while (!stack.is_empty())
                    && ((get_op_precedence(stack.last().unwrap()) > get_op_precedence(o))
                        || ((get_op_precedence(stack.last().unwrap()) == get_op_precedence(o))
                            && get_op_associativity(stack.last().unwrap()) == Assoc::Left))
                    && *stack.last().unwrap() != Token::Op(Op_t::Par_left)
                {
                    if let Some(v) = stack.pop() {
                        output.push(v);
                    }
                }
                stack.push(o.clone());
            }
        }
    }
    while !stack.is_empty() {
        output.push(stack.pop().unwrap());
    }
    return output;
}

fn factorial(n: i64) -> i64 {
    if n < 2 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn calc(rpn: Vec<Token>) -> i64 {
    let mut stack: Vec<i64> = vec![];
    for t in rpn {
        if let Token::Num(n) = t {
            stack.push(n);
        } else {
            match t {
                Token::Op(op) => match op {
                    Op_t::Plus => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l + r);
                    }
                    Op_t::Minus => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l + r);
                    }
                    Op_t::Multi => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l * r);
                    }
                    Op_t::Div => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l / r);
                    }
                    Op_t::Pow => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l.pow(r as u32));
                    }
                    Op_t::Mod => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l % r);
                    }
                    Op_t::Factorial => {
                        let r = stack.pop().unwrap();
                        stack.push(factorial(r));
                    }
                    Op_t::Par_left => {}
                    Op_t::Par_right => {}
                },
                Token::Func(_) => {}
                Token::Num(_) => {}
            }
        }
    }
    return stack.last().unwrap().clone();
}
