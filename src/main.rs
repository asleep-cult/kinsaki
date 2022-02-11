mod tokenizer;

fn main() {
    let mut toks = tokenizer::Tokenizer::new(
        r"y: Integer  -- declare y as an integer
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
                fallthrough;  -- go to the next block
        
            abc = AbcState::Disabled:
                -- Test condition 2
                ();
        ;
        
        type Span (Integer, Integer)  -- type with anonymous fields
        
        type User (  -- type with named fields
            id: Integer,
            messages: Integer,
        ):
            -- implementation
        ;
        
        proc product (Integer, Integer) -> (Integer):
            in.0 * in.1 -> out.0;  -- gives the product of 2 Integers"
    );

    while !toks.is_eof() {
        let token = toks.next_token();

        match token.kind {
            tokenizer::TokenKind::WhiteSpace => { }
            _ => { println!("{:?}", token) }
        }
    }
}
