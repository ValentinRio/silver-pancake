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
    Char(&'a char),
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
                        } else if c.is_digit(10) {
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
    Token {
        token_kind: TokenKind::FLOAT,
        token_mod: TokenMod::TOKENMOD_NONE,
        val: TokenVal::Float(0.0)
    }
}

fn escape_to_char(c: char) -> char {
    match c {
        'n' => '\n',
        'r' => '\r',
        _ => 0 as char
    }
}

fn scan_char() {}

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
    fn test_scan_int_dec() {
        let str = "1234";
        let mut iter = str.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_NONE);
        assert!(token.val == TokenVal::Int(1234));
    }

    #[test]
    fn test_scan_int_hexa() {
        let str = "0x123F";
        let mut iter = str.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_HEX);
        assert!(token.val == TokenVal::Int(4671));
    }

    #[test]
    fn test_scan_int_bin() {
        let str = "0b0111001";
        let mut iter = str.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_BIN);
        assert!(token.val == TokenVal::Int(57));
    }

    #[test]
    fn test_scan_int_oct() {
        let str = "0756";
        let mut iter = str.chars().peekable();
        let token = scan_int(&mut iter);
        println!("{:?}", token);
        assert!(token.token_kind == TokenKind::INT);
        assert!(token.token_mod == TokenMod::TOKENMOD_OCT);
        assert!(token.val == TokenVal::Int(494));
    }
}