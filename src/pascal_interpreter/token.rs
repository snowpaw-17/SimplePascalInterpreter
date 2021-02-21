use crate::pascal_interpreter::literal::Literal;

#[derive(PartialEq, Eq, Debug, Copy, Clone)]
pub enum TokenType
{
   IntegerConst,
   FloatConst, 
   Plus,
   Minus,
   Multiply,
   Division,
   Modulus,
   Lparen,
   Rparen,
   Identifier,
   Assignment,
   Semi,
   Begin,
   End,
   Program,
   Dot,
   Var,
   IntegerDivision,
   Colon,
   Comma,
   IntegerType,
   FloatType,
   Procedure,
   EOF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
        token_type: TokenType,
        literal: Literal,
        line : u32,
        col: u32,
    }

    impl Token {
        pub fn new(token_type: TokenType, literal: Literal, line: u32, col: u32) -> Token {
            Token {
                token_type: token_type,
                literal: literal,
                line: line, 
                col: col,
            }
        }

        pub fn token_type(&self) -> TokenType {
            self.token_type
        }

        pub fn literal(&self) -> &Literal {
           &self.literal
        }
    }

