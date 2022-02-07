## kinsaki
An experimental/concept programming language.

```hs
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

when x / 2 > y:
    -- Test condition
;

when:
    abc = AbcState::Enabled:
        -- Test condition 1
        fallthrough  -- go to the next block
    ;

    abc = AbcState::Disabled:
        -- Test condition 2
    ;
;

procedure product(a: Integer, b: Integer) -> (c: Integer):
    a * b -> c
;  -- accepts 2 Integers and stores their product in result

type Span (Integer, Integer)  -- type with anonymous fields

type User (  -- type with named fields and implementation
    id: Integer,
    messages: Integer,
):
    procedure qualified_for_discount(self) -> (qual: Boolean):
        self.messages > 50 -> qual
    ;
;
```

TODO: Loops?<br>
TODO: Effect handlers?<br>
