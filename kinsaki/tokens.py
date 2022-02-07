import enum
import typing

import attr

__all__ = ('TokenType', 'IdentifierToken')


class TokenType(enum.Enum):
    EOF = enum.auto()
    INVALID = enum.auto()
    OPEN_PAREN = enum.auto()
    CLOSE_PAREN = enum.auto()
    DOT = enum.auto()
    COLON = enum.auto()
    SEMICOLON = enum.auto()
    COMMA = enum.auto()
    PLUS = enum.auto()
    MINUS = enum.auto()
    STAR = enum.auto()
    SLASH = enum.auto()
    PIPE = enum.auto()
    AMPERSAND = enum.auto()
    TILDE = enum.auto()
    EQUAL = enum.auto()
    GTHAN = enum.auto()
    GTHAN_EQUAL = enum.auto()
    LTHAN = enum.auto()
    LTHAN_EQUAL = enum.auto()
    LTHAN_GTHAN = enum.auto()
    RARROW = enum.auto()

    IDENTIFIER = enum.auto()
    STRING = enum.auto()
    NUMBER = enum.auto()

    TYPE = enum.auto()
    WHEN = enum.auto()
    PROCEDURE = enum.auto()
    FALLTHROUGH = enum.auto()


@attr.s(kw_only=True)
class Token:
    type: TokenType = attr.ib()
    start: int = attr.ib()
    stop: int = attr.ib()


@attr.s(kw_only=True)
class IdentifierToken(Token):
    type: typing.Literal[TokenType.IDENTIFIER] = attr.ib(
        default=TokenType.IDENTIFIER, init=False
    )
    content: str = attr.ib()
