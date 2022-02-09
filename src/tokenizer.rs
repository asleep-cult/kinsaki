use std::str::Chars;

const EOF_CHAR: char = '\0';

pub struct Token {
    pub kind: TokenKind,
    pub begin: usize,
    pub end: usize,
}

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
    EqualGreaterThan,
    LessThan,
    LessThanOrEqualTo,
    LessThanOrGreaterThan,
    GreaterThan,
    GreaterThanOrEqualTo,

    Type,
    Procedure,
    In,
    Out,
    When,
    Fallthrough,

    Comment,
    WhiteSpace,

    Ident,
    Str,
    Num,
}

fn is_ident_start(c: char) -> bool {
    matches!(c, 'a'..='z' | 'A'..='Z' | '_')
}

fn is_ident_part(c: char) -> bool {
    is_ident_start(c) || matches!(c, '0'..='9')
}

struct Tokenizer<'a> {
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

    fn is_eof(&self) -> bool {
        self.chars.as_str().is_empty()
    }

    fn peek_char(&self) -> char {
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

    fn length_consumed(&self) -> usize {
        self.source_length - self.chars.as_str().len()
    }

    fn identifier(&mut self) -> TokenKind {
        self.consume_while(is_ident_part);
        TokenKind::Ident
    }

    fn next_token(&mut self) -> Token {
        let begin = self.length_consumed();

        let token_kind = match self.consume_char().unwrap() {
            c if is_ident_start(c) => self.identifier(),

            '(' => TokenKind::OpenParenthesis,
            ')' => TokenKind::CloseParenthesis,
            '.' => TokenKind::Dot,
            ':' => TokenKind::Colon,
            ';' => TokenKind::SemiColon,
            ',' => TokenKind::Comma,
            '+' => TokenKind::Plus,
            '-' => TokenKind::Minus,
            '*' => TokenKind::Star,
            '/' => TokenKind::Slash,
            '|' => TokenKind::Pipe,
            '&' => TokenKind::Ampersand,
            '~' => TokenKind::Tilde,

            '=' => match self.peek_char() {
                '>' => {
                    self.consume_char();
                    TokenKind::EqualGreaterThan
                }
                _ => TokenKind::Equal,
            },

            '<' => match self.peek_char() {
                '=' => {
                    self.consume_char();
                    TokenKind::LessThanOrEqualTo
                }
                '>' => {
                    self.consume_char()
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

            _ => TokenKind::Invalid,
        }

        return Token { kind: token_kind, begin: begin, end: self.length_consumed() }
    }
}
