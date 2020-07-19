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
enum Op {
    Plus,
    Minus,
    Multi,
    Div,
    Pow,
    Mod,
    Factorial,
    ParLeft,  // (
    ParRight, // )
}

fn get_op_precedence(op: &Token) -> i32 {
    if let Token::Op(op) = op {
        return match op {
            Op::Plus => 2,
            Op::Minus => 2,
            Op::Multi => 3,
            Op::Div => 3,
            Op::Pow => 4,
            Op::Mod => 3,
            Op::Factorial => 6,
            Op::ParLeft => 5,
            Op::ParRight => 5,
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
            Op::Plus => Assoc::Left,
            Op::Minus => Assoc::Left,
            Op::Multi => Assoc::Left,
            Op::Div => Assoc::Left,
            Op::Pow => Assoc::Right,
            Op::Mod => Assoc::Left,
            Op::Factorial => Assoc::Invalid,
            Op::ParLeft => Assoc::Invalid,
            Op::ParRight => Assoc::Invalid,
        };
    }
    return Assoc::Invalid;
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Func {
    Sum,
    Average,
    Sqrt,
}

#[derive(Debug, PartialEq, Eq, Clone)]
//['+', '-', '*', '/', '^', '%', '!', '(', ')']
enum Token {
    Op(Op),
    Num(i64),
    Func(Func),
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
        vec![Token::Num(12), Token::Op(Op::Plus), Token::Num(3)],
        actual
    );
    let actual = tokenizer("12*3+5/2").unwrap();
    assert_eq!(
        vec![
            Token::Num(12),
            Token::Op(Op::Multi),
            Token::Num(3),
            Token::Op(Op::Plus),
            Token::Num(5),
            Token::Op(Op::Div),
            Token::Num(2)
        ],
        actual
    );
    let actual = tokenizer("(2+2)! % 5^2").unwrap();
    assert_eq!(
        vec![
            Token::Op(Op::ParLeft),
            Token::Num(2),
            Token::Op(Op::Plus),
            Token::Num(2),
            Token::Op(Op::ParRight),
            Token::Op(Op::Factorial),
            Token::Op(Op::Mod),
            Token::Num(5),
            Token::Op(Op::Pow),
            Token::Num(2)
        ],
        actual
    );
}

fn to_token(cw: TokenState, s: String) -> Result<Token, ()> {
    match cw {
        TokenState::CHAR => {
            if s == "sum".to_owned() {
                Ok(Token::Func(Func::Sum))
            } else if s == "average".to_owned() {
                Ok(Token::Func(Func::Average))
            } else if s == "sqrt".to_owned() {
                Ok(Token::Func(Func::Sqrt))
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
            "+" => Ok(Token::Op(Op::Plus)),
            "-" => Ok(Token::Op(Op::Minus)),
            "*" => Ok(Token::Op(Op::Multi)),
            "/" => Ok(Token::Op(Op::Div)),
            "^" => Ok(Token::Op(Op::Pow)),
            "%" => Ok(Token::Op(Op::Mod)),
            "!" => Ok(Token::Op(Op::Factorial)),
            "(" => Ok(Token::Op(Op::ParLeft)),
            ")" => Ok(Token::Op(Op::ParRight)),
            _ => Err(()),
        },
        TokenState::EMPTY => Err(()),
    }
}

fn tokenizer(s: &str) -> Result<Vec<Token>, String> {
    let mut result: Vec<Token> = vec![];
    let mut buf: String = String::new();
    let mut current_state = TokenState::EMPTY;
    for ch in s.chars() {
        match ch {
            '0'..='9' => {
                if current_state != TokenState::NUM
                    && (!&buf.is_empty() || current_state == TokenState::EMPTY)
                {
                    if current_state != TokenState::EMPTY {
                        result.push(to_token(current_state, buf.clone()).unwrap());
                    }
                    buf.clear();
                    buf.push(ch);
                } else if current_state == TokenState::NUM {
                    buf.push(ch);
                } else if current_state != TokenState::EMPTY {
                    return Err("currentState is not NUM but buf is empty".to_string());
                }
                current_state = TokenState::NUM;
            }
            symb if ['+', '-', '*', '/', '^', '%', '!', '(', ')'].contains(&ch) => {
                if !buf.is_empty() {
                    result.push(to_token(current_state, buf.clone()).unwrap());
                    buf.clear();
                }
                buf.push(symb);

                current_state = TokenState::SYM;
            }
            c if ch.is_ascii_alphabetic() => {
                if current_state != TokenState::CHAR
                    && (!buf.is_empty() || current_state == TokenState::EMPTY)
                {
                    if current_state != TokenState::EMPTY {
                        result.push(to_token(current_state, buf.clone()).unwrap());
                    }
                    buf.clear();
                    buf.push(ch);
                } else if current_state == TokenState::CHAR {
                    buf.push(c.to_ascii_lowercase())
                } else if current_state != TokenState::EMPTY {
                    return Err("currentState is no CHAR but buf is empty".to_string());
                }
                current_state = TokenState::CHAR;
            }
            ' ' => {}
            _ => return Err(format!("unrecognized token: {}", ch)),
        }
    }
    if !buf.is_empty() {
        result.push(to_token(current_state, buf).unwrap())
    }
    Ok(result)
}

#[allow(dead_code)]
fn rpn_to_string(tokens: Vec<Token>) -> String {
    let mut result = String::new();
    for t in tokens {
        match t {
            Token::Op(op) => match op {
                Op::Plus => result += "+ ",
                Op::Minus => result += "- ",
                Op::Multi => result += "* ",
                Op::Div => result += "/ ",
                Op::Pow => result += "^ ",
                Op::Mod => result += "% ",
                Op::Factorial => result += "! ",
                Op::ParLeft => result += "( ",
                Op::ParRight => result += ") ",
            },
            Token::Num(n) => result += &format!("{} ", n),
            Token::Func(_) => result += &format!("{} ", stringify!(f)),
        }
    }
    result.trim_end().to_owned()
}

#[test]
fn test_generate_rpn() {
    let actual = generate_rpn(tokenizer("3 + 4 * 2 / ( 1 - 5 ) ^ 2 ^ 3").unwrap());
    assert_eq!("3 4 2 * 1 5 - 2 3 ^ ^ / +", rpn_to_string(actual));
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
            lp @ Token::Op(Op::ParLeft) => {
                stack.push(lp.clone());
            }
            Token::Op(Op::ParRight) => {
                while stack.last().unwrap().clone() != Token::Op(Op::ParLeft) {
                    if let Some(v) = stack.pop() {
                        output.push(v);
                    }
                }
                if stack.last().unwrap().clone() == Token::Op(Op::ParLeft) {
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
                    && *stack.last().unwrap() != Token::Op(Op::ParLeft)
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
                    Op::Plus => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l + r);
                    }
                    Op::Minus => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l + r);
                    }
                    Op::Multi => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l * r);
                    }
                    Op::Div => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l / r);
                    }
                    Op::Pow => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l.pow(r as u32));
                    }
                    Op::Mod => {
                        let r = stack.pop().unwrap();
                        let l = stack.pop().unwrap();
                        stack.push(l % r);
                    }
                    Op::Factorial => {
                        let r = stack.pop().unwrap();
                        stack.push(factorial(r));
                    }
                    Op::ParLeft => {}
                    Op::ParRight => {}
                },
                Token::Func(_) => {}
                Token::Num(_) => {}
            }
        }
    }
    return stack.last().unwrap().clone();
}
