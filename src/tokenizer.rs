const EOF_CHAR: char = '\0';

pub enum Token<'a> {
    EndOfFile,
    Invalid,
    
    OpenParenthesis,
    CloseParenthesis,
    Dot,
    Colon,
    SemiColon,
    Comma,
    Plus,
    Minus,
    Star,
    Slash,
    Pipe,
    Ampersand,
    Tilde,
    Equal,
    GreaterThan,
    GreaterThanOrEqualTo,
    LessThan,
    LessThanOrEqualTo,
    LessThanOrGreaterThan,
    RightArrow,

    Comment(&'a str),
    Identifier(&'a str),
    String(&'a str),
    Number(&'a str),
}

struct Tokenizer<'a> {
    chars: Chars<'a>,
}

impl <'a> Tokenizer<'a> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            chars: source.chars(),
        }
    }

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn consume_char(&self) -> Option<char> {
        self.chars.next()
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate() && !self.is_eof() {
            self.chars.next();
        }
    }
}
