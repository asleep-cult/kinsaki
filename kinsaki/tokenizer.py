import string
from io import TextIOBase

from .tokens import IdentifierToken, Token, TokenType

__all__ = ('Tokenizer',)

EOF = '\0'


class Tokenizer:
    def __init__(self, source: TextIOBase) -> None:
        self.source = source.read()
        self.position = 0

        self._token_map = {
            '(': TokenType.OPEN_PAREN,
            ')': TokenType.CLOSE_PAREN,
            '.': TokenType.DOT,
            ':': TokenType.COLON,
            ';': TokenType.SEMICOLON,
            ',': TokenType.COMMA,
            '*': TokenType.STAR,
            '/': TokenType.SLASH,
            '|': TokenType.PIPE,
            '&': TokenType.AMPERSAND,
            '~': TokenType.TILDE,
            '=': TokenType.EQUAL,
        }

    def eof(self) -> bool:
        return len(self.source) <= self.position

    def char(self, offset: int = 0) -> str:
        if len(self.source) > self.position + offset:
            return self.source[self.position + offset]

        return EOF

    def advance(self, amount: int = 1) -> None:
        self.position += amount

    def token(self) -> TokenType:
        char = self.char()

        type = self._token_map.get(char)
        if type is not None:
            self.advance()
            return type

        if char == '>':
            self.advance()

            if self.char() == '=':
                self.advance()
                return TokenType.GTHAN_EQUAL

            return TokenType.GTHAN

        elif char == '<':
            self.advance()

            if self.char() == '=':
                self.advance()
                return TokenType.LTHAN_EQUAL

            elif self.char() == '>':
                self.advance()
                return TokenType.LTHAN_GTHAN

            return TokenType.LTHAN

        elif char == '-':
            self.advance()

            if self.char() == '>':
                self.advance()
                return TokenType.RARROW

            return TokenType.MINUS

        while not self.eof():
            char = self.char()

            if char in string.whitespace:
                break

            self.advance()

        return TokenType.INVALID

    def identifier(self) -> str:
        content = ''

        while not self.eof():
            char = self.char()
            if not char.isidentifier():
                break

            content += char
            self.advance()

        return content

    def whitespace(self) -> None:
        while self.char() in string.whitespace:
            self.advance()

    def comment(self) -> bool:
        if '-' == self.char() == self.char(1):
            self.advance(2)

            position = self.source.find('\n', self.position)
            self.position = (
                position if position != -1 else len(self.source)
            )

            return True

        return False

    def scan(self) -> Token:
        while True:
            self.whitespace()

            if not self.comment():
                break

        start = self.position

        if self.eof():
            return Token(type=TokenType.EOF, start=start, stop=self.position)

        char = self.char()
        if char.isidentifier():
            content = self.identifier()

            if content == 'type':
                return Token(
                    type=TokenType.TYPE, start=start, stop=self.position
                )

            elif content == 'when':
                return Token(
                    type=TokenType.WHEN, start=start, stop=self.position
                )

            elif content == 'procedure':
                return Token(
                    type=TokenType.PROCEDURE, start=start, stop=self.position
                )

            elif content == 'fallthrough':
                return Token(
                    type=TokenType.FALLTHROUGH, start=start, stop=self.position
                )

            return IdentifierToken(
                start=start, stop=self.position, content=content
            )

        type = self.token()
        return Token(type=type, start=start, stop=self.position)
