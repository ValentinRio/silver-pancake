use crate::common::{fatal_error, syntax_error, PeekableIterator};

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

#[derive(Debug, PartialEq)]
enum TokenMod {
    TOKENMOD_NONE,
    TOKENMOD_HEX,
    TOKENMOD_BIN,
    TOKENMOD_OCT,
    TOKENMOD_CHAR,
}

#[derive(Debug, PartialEq)]
enum TokenVal<'a> {
    Int(u64),
    Float(f64),
    Char(char),
    Name(&'a str)
}

#[derive(Debug, PartialEq)]
struct Token<'a> {
    token_kind: TokenKind,
    token_mod: TokenMod,
    val: TokenVal<'a>
}

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

fn scan_int<'a, I>(chars: &mut I) -> Token<'a>
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

fn scan_float<'a, I>(chars: &mut I) -> Token<'a>
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
    let mut val: f64 = val_str.parse().unwrap();
    Token {
        token_kind: TokenKind::FLOAT,
        token_mod: TokenMod::TOKENMOD_NONE,
        val: TokenVal::Float(val)
    }
}

fn escape_to_char(c: char) -> char {
    match c {
        'n' => '\n',
        'r' => '\r',
        _ => '0'
    }
}

fn scan_char<'a, I>(chars: &mut I) -> Token<'a>
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
        token_mod: TokenMod::TOKENMOD_NONE,
        val: TokenVal::Char(val)
    }
}

fn scan_str() {}

fn tokenize<'a>(s: &'a mut &str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut iter = s.chars().peekable();

    while let Some(c) = iter.peek() {
        match *c as u8 {
            b'0'..=b'9' => {
                let token = scan_int(&mut iter);
                tokens.push(token)
            },
            _ => {}
        }
    }
    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_char_simple() {
        let test_case = "'a'";
        let mut iter = test_case.chars().peekable();
        let token = scan_char(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::CHAR);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Char('a'));
    }

    #[test]
    fn test_scan_char_simple_escaped() {
        let test_case = "'\\n'";
        let mut iter = test_case.chars().peekable();
        let token = scan_char(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::CHAR);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
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