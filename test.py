import io

import kinsaki


program = io.StringIO("""
y: Integer  -- declare y as an integer
x -> y      -- x into y i.e. set y to x

~x      -- negate y
y = x   -- y is equivalent to x
y > x   -- y is greater than x
y >= x  -- y is greater than or equal to x
y < x   -- y is less than x
y <= x  -- y is less than or equal to x
y <> x  -- y is less than or greater than x

x = (y | z)  -- x is equivalent to y or z
x = (y & z)  -- x is equivalent to y and z
             -- can be combined with any of the other operators

(y | z)  -- x or y is equivalent to true
(x & y)  -- x and y are equivalent to true

procedure product(a: Integer, b: Integer) -> (result: Integer):
    a * b -> result
;  -- accepts 2 Integers and stores their product in result

type Span (Integer, Integer)  -- type with anonymous fields

type Span (  -- type with named fields
    start: Integer,
    stop: Integer,
)

type Username (value: String):  -- type with implementation
    procedure is_valid(self: Username) -> (valid: Boolean):
        self.value.length < 20 -> valid
    ;
;
""")

tokenizer = kinsaki.Tokenizer(program)

while True:
    token = tokenizer.scan()
    print(token)

    if token.type is kinsaki.TokenType.EOF:
        break
