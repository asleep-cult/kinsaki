use std::str::Chars;

const EOF_CHAR: char = '\0';

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub begin: usize,
    pub end: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TokenKind {
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
    RightArrow,
    LessThan,
    LessThanOrEqualTo,
    LessThanOrGreaterThan,
    GreaterThan,
    GreaterThanOrEqualTo,

    Comment,
    WhiteSpace,

    Ident,
    Str { terminated: bool },
    Int { radix: Radix, empty: bool },
    Float { radix: Radix, empty: bool },
}

#[derive(Debug, PartialEq, Eq)]
pub enum Radix {
    Binary,
    Octal,
    Decimal,
    Hexadecimal,
}

fn is_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_ident_part(c: char) -> bool {
    is_ident_start(c) || matches!(c, '0'..='9')
}

fn is_whitespace(c: char) -> bool {
    matches!(
        c,
        '\r'
        | '\n'
        | '\t'
        | '\u{000B}' // vertical tab
        | '\u{000C}' // form feed
        | '\u{0020}' // space
    )
}

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    source_length: usize,
}

impl<'a> Tokenizer<'a> {
    pub fn new(source: &str) -> Tokenizer {
        Tokenizer {
            chars: source.chars(),
            source_length: source.len(),
        }
    }

    pub fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek_char(&self) -> char {
        self.chars.clone().next().unwrap_or(EOF_CHAR)
    }

    fn consume_char(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn consume_while(&mut self, mut predicate: impl FnMut(char) -> bool) {
        while predicate(self.peek_char()) && !self.is_eof() {
            self.chars.next();
        }
    }

    fn length_consumed(&self) -> usize {
        self.source_length - self.chars.as_str().len()
    }

    fn whitespace(&mut self) -> TokenKind {
        self.consume_while(is_whitespace);
        TokenKind::WhiteSpace
    }

    fn identifier(&mut self) -> TokenKind {
        self.consume_while(is_ident_part);
        TokenKind::Ident
    }

    fn comment(&mut self) -> TokenKind {
        self.consume_while(|c| c != '\n');
        TokenKind::Comment
    }

    fn string(&mut self) -> bool {
        loop {
            match self.peek_char() {
                '\\' => {
                    self.consume_char();
                    self.consume_char();
                }
                '"' => {
                    self.consume_char();
                    return true;
                }
                '\n' => break,
                EOF_CHAR if self.is_eof() => break,
                _ => {
                    self.consume_char();
                }
            }
        }

        false
    }

    fn number(&mut self, first: char) -> TokenKind {
        let (radix, empty) = if first == '0' {
            match self.peek_char() {
                'b' => {
                    self.consume_char();
                    (Radix::Binary, self.consume_binary())
                }
                'o' => {
                    self.consume_char();
                    (Radix::Octal, self.consume_octal())
                }
                'x' => {
                    self.consume_char();
                    (Radix::Hexadecimal, self.consume_hexadecimal())
                }
                _ => {
                    self.consume_decimal();
                    (Radix::Decimal, false)
                }
            }
        } else {
            self.consume_decimal();
            (Radix::Decimal, false)
        };

        return TokenKind::Int { radix: radix, empty: empty };
    }

    fn consume_binary(&mut self) -> bool {
        let mut empty = true;
        loop {
            match self.peek_char() {
                '0' | '1' => {
                    self.consume_char();
                    empty = false;
                }
                _ => break,
            }
        }
        empty
    }

    fn consume_octal(&mut self) -> bool {
        let mut empty = true;
        loop {
            match self.peek_char() {
                '0'..='7' => {
                    self.consume_char();
                    empty = false;
                }
                _ => break,
            }
        }
        empty
    }

    fn consume_hexadecimal(&mut self) -> bool {
        let mut empty = true;
        loop {
            match self.peek_char() {
                '0'..='9' | 'a'..='f' => {
                    self.consume_char();
                    empty = false;
                }
                _ => break,
            }
        }
        empty
    }

    fn consume_decimal(&mut self) -> bool {
        let mut empty = true;
        loop {
            match self.peek_char() {
                '0'..='9' => {
                    self.consume_char();
                    empty = false;
                }
                _ => break,
            }
        }
        empty
    }

    pub fn next_token(&mut self) -> Token {
        let begin = self.length_consumed();

        let token_kind = match self.consume_char().unwrap() {
            c if is_whitespace(c) => self.whitespace(),
            c if is_ident_start(c) => self.identifier(),

            '"' => TokenKind::Str { terminated: self.string() },
            c @ '0'..='9' => self.number(c),

            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::CloseParenthesis,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,
            '+' => TokenKind::Plus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '|' => TokenKind::Pipe,
            '&' => TokenKind::Ampersand,
            '~' => TokenKind::Tilde,
            '=' => TokenKind::Equal,

            '-' => match self.peek_char() {
                '-' => self.comment(),
                '>' => {
                    self.consume_char();
                    TokenKind::RightArrow
                }
                _ => TokenKind::Minus,
            },

            '<' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    TokenKind::LessThanOrEqualTo
                }
                '>' => {
                    self.consume_char();
                    TokenKind::LessThanOrGreaterThan
                }
                _ => TokenKind::LessThan,
            },

            '>' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    TokenKind::GreaterThanOrEqualTo
                },
                _ => TokenKind::GreaterThan,
            },

            EOF_CHAR if self.is_eof() => TokenKind::EndOfFile,
            _ => TokenKind::Invalid,
        };

        Token { kind: token_kind, begin: begin, end: self.length_consumed() }
    }
}
