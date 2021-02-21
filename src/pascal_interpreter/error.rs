use super::token::{
    Token,
    TokenType
};

#[derive(Clone, Debug, PartialEq)]
pub enum RuntimeError {
    UnexpectedToken(Token, TokenType),
    MissingArgument,
    UnexpectedChar(char),
    UndefinedVariable(String),
    UnhandledBinaryOp(Token),
    UnhandledUnaryOp(Token),
    DivisionByZero,
    IllformedVarExpr,
    UnknownType(String),
    VariableRedefinition(String),
    StackUnderflow,
    UnsupportedArgumentTypeByOp(Token),
    MissingProcedure
}