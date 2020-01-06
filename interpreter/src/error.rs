use crate::runtime_value::Value;
use crate::token::Token;

#[derive(Debug, Clone, Display)]
pub enum ErrorType {
    #[display(fmt = "String not closed")]
    StringNotClosed,
    #[display(fmt = "Unexpected character")]
    UnexpectedCharacter,
    #[display(fmt = "Unparsable expression")]
    UnparsableExpression,
    #[display(fmt = "Expected open parenthesis")]
    ExpectedOpenParenthesis,
    #[display(fmt = "Expected close parenthesis")]
    ExpectedCloseParenthesis,
    #[display(fmt = "Expected close bar")]
    ExpectedCloseBar,
    #[display(fmt = "Unclosed parenthesis")]
    UnclosedParenthesis,
    #[display(fmt = "Expected operator")]
    ExpectedOperator,
    #[display(fmt = "Expected unary operator")]
    ExpectedUnaryOperator,
    #[display(fmt = "Expected semicolon")]
    ExpectedSemicolon,
    #[display(fmt = "Unexpected type mismatch")]
    WrongType,
    #[display(fmt = "Expected variable to have an identifier")]
    ExpectedIdentifier,
    #[display(fmt = "Expected assign after identifier")]
    ExpectedAssign,
    #[display(fmt = "Variable is undefined")]
    UndefinedVariable,
    #[display(fmt = "Invalid Assignment")]
    InvalidAssignment,
    #[display(fmt = "Expected close brace at the end of the block")]
    ExpectedBlockEnd,
    #[display(fmt = "Expected open brace at the start of the block")]
    ExpectedBlockStart,
    #[display(fmt = "Expected if statement to have an else block")]
    ExpectedElseStatement,
    #[display(fmt = "This keyword needs can't be used outside of the loops")]
    NotAllowedOutsideLoop,
    #[display(fmt = "Maximum number of the arguments is 255")]
    MaximumArguments,
    #[display(fmt = "This value is not callable")]
    ValueNotCallable,
    #[display(fmt = "Invalid number of arguments")]
    InvalidNumberOfArguments,
}

#[derive(Debug, Clone)]
pub struct Error {
    pub token: Token,
    pub error_type: ErrorType,
}

pub fn error(token: &Token, error_type: ErrorType) -> Result<Value, Error> {
    Err(Error {
        token: token.clone(),
        error_type,
    })
}
