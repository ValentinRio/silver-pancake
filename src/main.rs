use std::fmt;
use std::ptr;

struct InternStr<'a> {
    len: usize,
    str: &'a str
}

fn str_intern_range<'a>(interns: & mut Vec<InternStr<'a>>, str: &'a str) -> &'a str {
    let len = str.len();
    for i in 0..interns.len() {
        if (interns[i].len == len) & (interns[i].str == str) {
            return &interns[i].str;
        }
    }
    let intern = InternStr {
        len: str.len(),
        str: &str
    };
    interns.push(intern);
    &str
}

#[derive(Debug, PartialEq)]
enum Token<'a> {
    Int(u64),
    Name(&'a str),
    Other(char)
}

impl fmt::Display for Token<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

fn tokenize<'a>(stream: &'a mut &str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut bytes = stream.as_bytes().iter().enumerate();
    while let Some((i, &byte)) = bytes.next() {
        match byte {
            b'0'..=b'9' => {
                let mut val = 0;
                val *= 10;
                val += byte as u64 - '0' as u64;
                while let Some((j, &num)) = bytes.next() {
                    if (num as char).is_digit(10) {
                        val *= 10;
                        val = val + (num as u64 - '0' as u64);
                        if j == stream.len() - 1 {
                            tokens.push(Token::Int(val));
                            break;
                        }
                        if !&stream.chars().nth(j + 1).unwrap().is_digit(10) {
                            tokens.push(Token::Int(val));
                            break;
                        }
                    } else {
                        tokens.push(Token::Int(val));
                        break;
                    }
                }
            },
            b'A'..=b'z' => {
                while let Some((j, &alpha)) = bytes.next() {
                    if (alpha as char).is_alphabetic() | (alpha as char).is_digit(10) {
                        if j == stream.len() - 1 {
                            if &stream.chars().nth(j).unwrap().is_alphabetic() | &stream.chars().nth(j).unwrap().is_digit(10) {
                                tokens.push(Token::Name(&stream[i..j + 1]));
                            }
                            break;
                        }
                        if !&stream.chars().nth(j + 1).unwrap().is_alphabetic() & !&stream.chars().nth(j + 1).unwrap().is_digit(10) {
                            tokens.push(Token::Name(&stream[i..j + 1]));
                            break;
                        }
                    } else {
                        tokens.push(Token::Name(&stream[i..j]));
                        break;
                    }
                }
            },
            _ => {
                tokens.push(Token::Other(byte as char))
            }
        }
    }
    tokens
}

fn parse_expr_str(expr: &str) -> i32 {
    println!("{}", expr);
    0
}

fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer() {
        let mut stream = "+()_A1,23+!FOO!994/a25*t1";
        let tokens = tokenize(&mut stream);
        assert!(tokens.len() == 15);
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
        }
    }

    #[test]
    fn test_parser() {
        test_expr!(1 + 1);
    }
}