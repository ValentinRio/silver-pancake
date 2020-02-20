use std::fmt;
use std::ptr;

struct InternStr<'a> {
    len: usize,
    str: &'a str,
}

fn str_intern_range<'a>(interns: &mut Vec<InternStr<'a>>, str: &'a str) -> &'a str {
    let len = str.len();
    for i in 0..interns.len() {
        if (interns[i].len == len) & (interns[i].str == str) {
            return &interns[i].str;
        }
    }
    let intern = InternStr {
        len: str.len(),
        str: &str,
    };
    interns.push(intern);
    &str
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Int(u64),
    Name(&'a str),
    Other(char),
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn tokenize<'a>(s: &'a mut &str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut iter = s.chars().enumerate().peekable();

    while let Some((i, c)) = iter.next() {
        match c as u8 {
            b'0'..=b'9' => {
                let mut val = 0;
                val *= 10;
                val += c as u64 - '0' as u64;
                while let Some((_j, n)) = iter.peek() {
                    if n.is_digit(10) {
                        val *= 10;
                        val = val + (*n as u64 - '0' as u64);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Int(val));
            }
            b'A'..=b'z' => {
                let mut k = i.clone() + 1;
                while let Some((_j, a)) = iter.peek() {
                    if a.is_alphabetic() | a.is_digit(10) {
                        k += 1;
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Name(&s[i as usize..k as usize]));
            }
            _ => {
                tokens.push(Token::Other(c));
            }
        }
    }
    tokens
}

fn parse_expr3<'a, I>(tokens: &mut I, token: &Token) -> i32
where
    I: Iterator<Item = &'a Token<'a>>,
{
    match token {
        Token::Int(n) => *n as i32,
        Token::Other('(') => {
            let next_token = tokens.next().unwrap();
            let val = parse_expr(tokens, next_token);
            match next_token {
                Token::Other(')') => {
                    tokens.next();
                }
                _ => {}
            }
            val
        }
        _ => panic!("Expected integer or ("),
    }
}

fn parse_expr2<'a, I>(tokens: &mut I, token: &Token) -> i32
where
    I: Iterator<Item = &'a Token<'a>>,
{
    match token {
        Token::Other('+') => {
            let next_token = tokens.next().unwrap();
            parse_expr2(tokens, next_token)
        }
        Token::Other('-') => {
            let next_token = tokens.next().unwrap();
            -parse_expr2(tokens, next_token)
        }
        _ => parse_expr3(tokens, token),
    }
}

fn parse_expr1<'a, I>(tokens: &mut I, token: &Token) -> i32
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut val = parse_expr2(tokens, token);
    while let Some(token) = tokens.peekable().peek() {
        match token {
            Token::Other('*') => {
                let next_token = tokens.next().unwrap();
                let rval = parse_expr2(tokens, next_token);
                val *= rval;
            }
            Token::Other('/') => {
                let next_token = tokens.next().unwrap();
                let rval = parse_expr2(tokens, next_token);
                val /= rval;
            }
            _ => {
                break;
            }
        }
    }
    val
}

fn parse_expr0<'a, I>(tokens: &mut I, token: &Token) -> i32
where
    I: Iterator<Item = &'a Token<'a>>,
{
    let mut val = parse_expr1(tokens, token);
    while let Some(token) = tokens.peekable().peek() {
        match token {
            Token::Other('+') => {
                let next_token = tokens.next().unwrap();
                let rval = parse_expr1(tokens, next_token);
                val += rval;
            }
            Token::Other('-') => {
                let next_token = tokens.next().unwrap();
                let rval = parse_expr1(tokens, next_token);
                val -= rval;
            }
            _ => {
                break;
            }
        }
    }
    val
}

fn parse_expr<'a, I>(tokens: &mut I, token: &Token) -> i32
where
    I: Iterator<Item = &'a Token<'a>>,
{
    parse_expr0(tokens, token)
}

fn parse_expr_str(expr: &str) -> i32 {
    let mut stream = expr;
    let tokens = tokenize(&mut stream);
    let mut iter = tokens.iter();
    let mut token = iter.clone();
    let token1 = token.next().unwrap();
    parse_expr(&mut iter, token1)
}

fn main() {
    println!("Result: {}", parse_expr_str("5-7"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut stream = "+()_A1,23+!FOO!994 / a25*t1 1 + 1";
        let tokens = tokenize(&mut stream);
        println!("{:?}", tokens);
        assert!(tokens.len() == 23);
    }

    #[test]
    fn test_string_interning() {
        let x = "hello";
        let y = "hello";
        assert!(!ptr::eq(&x, &y));
        let mut interns: Vec<InternStr> = Vec::new();
        let px = str_intern_range(&mut interns, x);
        let py = str_intern_range(&mut interns, y);
        assert!(ptr::eq(px, py));
        let z = "hello!";
        let pz = str_intern_range(&mut interns, z);
        assert!(!ptr::eq(pz, px));
    }

    macro_rules! test_expr {
        ($x:expr) => {
            assert!(parse_expr_str(stringify!($x)) == $x);
        };
    }

    #[test]
    fn test_parser() {
        test_expr!(1 + 1);
    }
}
