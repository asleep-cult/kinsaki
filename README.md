## kinsaki
An experimental/concept programming language.

```
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

when <condition>:
    -- Test condition
;

when:
    <condition 1>:
        -- Test condition 1
        fallthrough  -- go to the next block
    ;

    <condition 2>:
        -- Test condition 2
    ;
;

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
```

TODO: Loops?<br>
TODO: Effect handlers?<br>
