#[allow(unused_imports)]
use crate::common::{fatal_error, syntax_error, PeekableIterator};

#[allow(non_camel_case_types)]
#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TokenKind {
    EOF,
    INT,
    FLOAT,
    STR,
    CHAR,
    NAME,
    LSHIFT,
    RSHIFT,
    EQ,
    NOTEQ,
    LTEQ,
    GTEQ,
    AND,
    OR,
    INC,
    DEC,
    COLON_ASSIGN,
    ADD_ASSIGN,
    SUB_ASSIGN,
    OR_ASSIGN,
    AND_ASSIGN,
    XOR_ASSIGN,
    LSHIFT_ASSIGN,
    RSHIFT_ASSIGN,
    MUL_ASSIGN,
    DIV_ASSIGN,
    MOD_ASSIGN,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
enum TokenMod {
    TOKENMOD_NONE,
    TOKENMOD_HEX,
    TOKENMOD_BIN,
    TOKENMOD_OCT,
    TOKENMOD_CHAR,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
enum TokenVal {
    Int(u64),
    Float(f64),
    Char(char),
    Str(String)
}

#[derive(Debug, PartialEq)]
struct Token {
    token_kind: TokenKind,
    token_mod: TokenMod,
    val: TokenVal
}

#[allow(dead_code)]
fn char_to_digit(c: &char) -> u64 {
    match c {
        '0' => 0,
        '1' => 1,
        '2' => 2,
        '3' => 3,
        '4' => 4,
        '5' => 5,
        '6' => 6,
        '7' => 7,
        '8' => 8,
        '9' => 9,
        'a' => 10, 'A' => 10,
        'b' => 11, 'B' => 11,
        'c' => 12, 'C' => 12,
        'd' => 13, 'D' => 13,
        'e' => 14, 'E' => 14,
        'f' => 15, 'F' => 15,
        _ => 0,
    }
}

#[allow(dead_code)]
fn scan_int<I>(chars: &mut I) -> Token
where 
    I: PeekableIterator<Item = char>,
{
    let mut base = 10;
    let mut token_mod = TokenMod::TOKENMOD_NONE;
    while let Some(c) = chars.peek() {
        match c {
            '0' => {
                chars.next();
                match chars.peek() {
                    Some(c) => {
                        if c.to_ascii_lowercase() == 'x' {
                            chars.next();
                            token_mod = TokenMod::TOKENMOD_HEX;
                            base = 16;
                            break;
                        } else if c.to_ascii_lowercase() == 'b' {
                            chars.next();
                            token_mod = TokenMod::TOKENMOD_BIN;
                            base = 2;
                            break;
                        } else if c.to_ascii_lowercase() == 'o' {
                            chars.next();
                            token_mod = TokenMod::TOKENMOD_OCT;
                            base = 8;
                            break;
                        }
                    },
                    None => {
                        break;
                    }
                }
            },
            _ => {
                break;
            }
        }
    }
    let mut val = 0;
    while let Some(c) = chars.peek() {
        let mut digit = char_to_digit(c);
        if digit == 0 && c.to_ascii_lowercase() != '0' {
            break;
        }
        if digit >= base {
            syntax_error("Digit out of range", Some(c));
            digit = 0;
        }
        if val > (u64::max_value() - digit) / base {
            syntax_error("Integer literal overflow", Some(c));
            while let Some(c) = chars.peek() {
                if c.is_digit(10) {
                    chars.next();
                }
            }
            val = 0;
        }
        val = val * base + digit;
        chars.next();
    }
    Token {
        token_kind: TokenKind::INT,
        token_mod: token_mod,
        val: TokenVal::Int(val)
    }
}

#[allow(dead_code)]
fn scan_float<I>(chars: &mut I) -> Token
where
    I: PeekableIterator<Item = char>,
{
    let mut val_str = String::from("");
    while let Some(c) = chars.peek() {
        if c.is_digit(10) || c.to_ascii_lowercase() == '.' {
            val_str.push(*c);
            chars.next();
        } else {
            break;
        }
    }
    while let Some(c) = chars.peek() {
        if c.to_ascii_lowercase() == 'e' {
            val_str.push(*c);
            chars.next();
            while let Some(c1) = chars.peek() {
                match c1 {
                    '+' | '-' => {
                        val_str.push(*c1);
                        chars.next();
                    },
                    _ => {
                        while let Some(c2) = chars.peek() {
                            if c2.is_digit(10) {
                                val_str.push(*c2);
                                chars.next();
                            } else {
                                syntax_error("Expected digit after float literal exponent, found: ", Some(c2));
                                break;
                            }
                        }
                        break;
                    }
                }
            }
        } else {
            break;
        }
    }
    println!("val_str {}", val_str);
    let val: f64 = val_str.parse().unwrap();
    Token {
        token_kind: TokenKind::FLOAT,
        token_mod: TokenMod::TOKENMOD_NONE,
        val: TokenVal::Float(val)
    }
}

#[allow(dead_code)]
fn escape_to_char(c: char) -> char {
    match c {
        'n' => '\n',
        'r' => '\r',
        _ => '0'
    }
}

#[allow(dead_code)]
fn scan_char<I>(chars: &mut I) -> Token
where
    I: PeekableIterator<Item= char>,
{
    let mut val = ' ';
    chars.next();
    while let Some(c) = chars.peek() {
        match c {
            '\'' =>  {
                syntax_error("Char literal cannot be empty", None);
                chars.next();
                break;
            }
            '\n' => {
                syntax_error("Char literal cannot contain newline", None);
                break;
            }
            '\\' => {
                chars.next();
                while let Some(c1) = chars.peek() {
                    val = escape_to_char(*c1);
                    if val == '0' {
                        syntax_error("Invalid char literal escape,", Some(c1));
                    }
                    chars.next();
                    break;
                }
                break;
            }
            _ => {
                val = *c;
                chars.next();
                break;
            }
        }
    }

    while let Some(c) = chars.peek() {
        if *c != '\'' {
            syntax_error("Expected closing char quote,", Some(c));
            break;
        } else {
            chars.next();
            break;
        }
    }
    Token {
        token_kind: TokenKind::CHAR,
        token_mod: TokenMod::TOKENMOD_CHAR,
        val: TokenVal::Char(val)
    }
}

#[allow(dead_code)]
fn scan_str<I>(chars: &mut I) -> Token
where
    I: PeekableIterator<Item = char>, 
{
    chars.next();
    let mut str = String::from("");
    while let Some(c) = chars.peek() {
        if *c != '"' {
            let mut val = *c;
            if val == '\n' {
                syntax_error("String literal cannot contain newline", None)
            } else if val == '\\' {
                if let Some(c) = chars.next() {
                    val = escape_to_char(c);
                    if val == '0' {
                        syntax_error("Invalid string literal escape, ", Some(&c));
                    }
                }
            }
            str.push(val);
            chars.next();
        } else {
            break;
        }
    }
    if let Some(c) = chars.peek() {
        if *c == '"' {
            chars.next();
        } else {
            syntax_error("Unexpected end of file within string literal", None);
        }
    } else {
        syntax_error("Unexpected end of file within string literal", None);
    }
    Token {
        token_kind: TokenKind::STR,
        token_mod: TokenMod::TOKENMOD_NONE,
        val: TokenVal::Str(str)
    }
}

#[allow(dead_code)]
fn tokenize(s: &mut &str) -> Vec<Token> {
    let mut tokens = vec![];
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.peek() {
        match *c as u8 {
            b' ' | b'\\' | b'\r' => {
                iter.next();
            }
            b'\'' => {
                tokens.push(scan_char(&mut iter));
            }
            b'"' => {
                tokens.push(scan_str(&mut iter));
            }
            b'.' => {
                tokens.push(scan_float(&mut iter));
            }
            b'0'..=b'9' => {
                let mut clone = iter.clone();
                while let Some(c) = clone.peek() {
                    if c.is_digit(10) {
                        clone.next();
                        continue;
                    }
                    if c.to_ascii_lowercase() == '.' || c.to_ascii_lowercase() == 'e' {
                        tokens.push(scan_float(&mut iter));
                    } else {
                        tokens.push(scan_int(&mut iter));
                    }
                    break;
                }
            }
            b'A'..=b'z' => {
                let mut name = String::from("");
                while let Some(c) = iter.peek() {
                    if c.is_alphabetic() || c.is_digit(10) {
                        name.push(*c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token {
                    token_kind: TokenKind::NAME,
                    token_mod: TokenMod::TOKENMOD_NONE,
                    val: TokenVal::Str(name)
                });
            }
            _ => {}
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let mut test_case = "\"foo\" toto 12.56 0x123F '\\n'";
        let tokens = tokenize(&mut test_case);
        println!("{:?}", tokens);
        assert!(tokens[0].token_kind == TokenKind::STR);
        assert!(tokens[0].token_mod == TokenMod::TOKENMOD_NONE);
        assert!(tokens[0].val == TokenVal::Str(String::from("foo")));
        assert!(tokens[1].token_kind == TokenKind::NAME);
        assert!(tokens[1].token_mod == TokenMod::TOKENMOD_NONE);
        assert!(tokens[1].val == TokenVal::Str(String::from("toto")));
        assert!(tokens[2].token_kind == TokenKind::FLOAT);
        assert!(tokens[2].token_mod == TokenMod::TOKENMOD_NONE);
        assert!(tokens[2].val == TokenVal::Float(12.56));
        assert!(tokens[3].token_kind == TokenKind::INT);
        assert!(tokens[3].token_mod == TokenMod::TOKENMOD_HEX);
        assert!(tokens[3].val == TokenVal::Int(4671));
        assert!(tokens[4].token_kind == TokenKind::CHAR);
        assert!(tokens[4].token_mod == TokenMod::TOKENMOD_CHAR);
        assert!(tokens[4].val == TokenVal::Char('\n'));
    }

    #[test]
    fn test_scan_str_simple() {
        let test_case = "\"foo\"";
        let mut iter = test_case.chars().peekable();
        let token = scan_str(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::STR);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Str(String::from("foo")));
    }

    #[test]
    fn test_scan_str_escaped() {
        let test_case = "\"a\nb\"";
        let mut iter = test_case.chars().peekable();
        let token = scan_str(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::STR);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Str(String::from("a\nb")));
    }

    #[test]
    fn test_scan_char_simple() {
        let test_case = "'a'";
        let mut iter = test_case.chars().peekable();
        let token = scan_char(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::CHAR);
        assert!(token.token_mod == TokenMod::TOKENMOD_CHAR);
        assert!(token.val == TokenVal::Char('a'));
    }

    #[test]
    fn test_scan_char_simple_escaped() {
        let test_case = "'\\n'";
        let mut iter = test_case.chars().peekable();
        let token = scan_char(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::CHAR);
        assert!(token.token_mod == TokenMod::TOKENMOD_CHAR);
        assert!(token.val == TokenVal::Char('\n'));
    }

    #[test]
    fn test_scan_float_simple() {
        let test_case = "1.56";
        let mut iter = test_case.chars().peekable();
        let token = scan_float(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::FLOAT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Float(1.56));
    }

    #[test]
    fn test_scan_float_simple2() {
        let test_case = ".34";
        let mut iter = test_case.chars().peekable();
        let token = scan_float(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::FLOAT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Float(0.34));
    }

    #[test]
    fn test_scan_float_simple3() {
        let test_case = "45.";
        let mut iter = test_case.chars().peekable();
        let token = scan_float(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::FLOAT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Float(45.));
    }

    #[test]
    fn test_scan_float_negative_power_of() {
        let test_case = "2.5e-2";
        let mut iter = test_case.chars().peekable();
        let token = scan_float(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::FLOAT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Float(0.025));
    }

    #[test]
    fn test_scan_float_positive_power_of() {
        let test_case = "2e2";
        let mut iter = test_case.chars().peekable();
        let token = scan_float(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::FLOAT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Float(200.0));
    }

    #[test]
    fn test_scan_int_dec() {
        let test_case = "1234";
        let mut iter = test_case.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Int(1234));
    }

    #[test]
    fn test_scan_int_hexa() {
        let test_case = "0x123F";
        let mut iter = test_case.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_HEX);
        assert!(token.val == TokenVal::Int(4671));
    }

    #[test]
    fn test_scan_int_bin() {
        let test_case = "0b0111001";
        let mut iter = test_case.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_BIN);
        assert!(token.val == TokenVal::Int(57));
    }

    #[test]
    fn test_scan_int_oct() {
        let test_case = "0o756";
        let mut iter = test_case.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_OCT);
        assert!(token.val == TokenVal::Int(494));
    }
}