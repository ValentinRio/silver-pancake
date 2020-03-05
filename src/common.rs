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

pub fn fatal_error(err: &str, c: Option<&char>) {
    if c != None {
        panic!("Fatal error: {} caused by char: {}", err, c.unwrap());
    } else {
        panic!("Fatal error: {}", err);
    }
}

pub fn syntax_error(err: &str, c: Option<&char>) {
    if c != None {
        eprintln!("Syntax error: {} caused by char: {}", err, c.unwrap());
    } else {
        eprintln!("Syntax error: {}", err);
    }
}

pub trait PeekableIterator: std::iter::Iterator {
    fn peek(&mut self) -> Option<&Self::Item>;
}

impl<I: std::iter::Iterator> PeekableIterator for std::iter::Peekable<I> {
    fn peek(&mut self) -> Option<&Self::Item> {
        std::iter::Peekable::peek(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}