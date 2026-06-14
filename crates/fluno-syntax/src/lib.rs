pub mod ast;
pub mod diagnostics;
pub mod lexer;
pub mod parser;
pub mod source_map;

pub use ast::node::{Expression as AstExpr, Program as AstModule};
pub use lexer::{tokenize, LexError, Lexer, Token, TokenKind};
pub use parser::{ParseError, Parser};

#[derive(Debug, Clone, thiserror::Error)]
pub enum SyntaxError {
    #[error(transparent)]
    Lex(#[from] LexError),
    #[error(transparent)]
    Parse(#[from] ParseError),
}

pub fn parse_module(source: &str) -> Result<AstModule, SyntaxError> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?;
    Ok(parser.parse_program()?)
}

pub fn parse_expr(source: &str) -> Result<AstExpr, SyntaxError> {
    let lexer = Lexer::new(source);
    let mut parser = Parser::new(lexer)?;
    Ok(parser.parse_expression()?)
}

#[cfg(test)]
mod public_api_tests {
    use super::*;

    #[test]
    fn tokenize_public_api_accepts_lowercase_identifiers() {
        let tokens = tokenize("let value = 1;").expect("tokenize should succeed");
        assert_eq!(tokens[1].kind, TokenKind::Identifier);
        assert_eq!(tokens[1].text(), Some("value"));
    }

    #[test]
    fn parse_module_public_api_accepts_function() {
        let module = parse_module("fn main() { let value = 1; }").expect("module parses");
        assert_eq!(module.items.len(), 1);
    }

    #[test]
    fn parse_expr_public_api_accepts_arithmetic() {
        let expr = parse_expr("1 + 2 * 3").expect("expression parses");
        assert!(matches!(expr, AstExpr::Binary { .. }));
    }
}
