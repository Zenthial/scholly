use logos::Logos;
use num_derive::{FromPrimitive, ToPrimitive};

#[derive(
    Debug, Copy, Clone, PartialEq, Logos, FromPrimitive, ToPrimitive, Hash, Ord, PartialOrd, Eq,
)]
pub(crate) enum SyntaxKind {
    Root,

    BinaryExpr,

    PrefixExpr,

    #[regex(" +")]
    Whitespace,

    #[token("fn")]
    FnKw,

    #[token("let")]
    LetKw,

    #[regex("[A-Za-z][A-Za-z0-9]*")]
    Ident,

    #[regex("[0-9]+")]
    Number,

    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("*")]
    Star,

    #[token("/")]
    Slash,

    #[token("=")]
    Equals,

    #[token("{")]
    LBrace,

    #[token("}")]
    RBrace,

    #[token("(")]
    LParen,

    #[token(")")]
    RParen,
}

pub(crate) struct Lexer<'a> {
    inner: logos::Lexer<'a, SyntaxKind>,
}

impl<'a> Lexer<'a> {
    pub(crate) fn new(input: &'a str) -> Self {
        Self {
            inner: SyntaxKind::lexer(input),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Lexeme<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let kind = if let Ok(k) = self.inner.next()? {
            k
        } else {
            return None;
        };

        let text = self.inner.slice();

        Some(Self::Item { kind, text })
    }
}

#[derive(Debug, PartialEq)]
pub(crate) struct Lexeme<'a> {
    pub(crate) kind: SyntaxKind,
    pub(crate) text: &'a str,
}

#[cfg(test)]
mod lexer_tests {
    use super::*;

    fn generic_test(input: &str, kind: SyntaxKind) {
        let mut lexer = Lexer::new(input);
        assert_eq!(lexer.next(), Some(Lexeme { kind, text: input }))
    }

    #[test]
    fn lex_spaces() {
        generic_test("   ", SyntaxKind::Whitespace);
    }

    #[test]
    fn lex_fn_keyword() {
        generic_test("fn", SyntaxKind::FnKw);
    }

    #[test]
    fn lex_let_keyword() {
        generic_test("let", SyntaxKind::LetKw);
    }

    #[test]
    fn lex_alphabetic_identifier() {
        generic_test("abcd", SyntaxKind::Ident);
    }

    #[test]
    fn lex_single_identifier() {
        generic_test("x", SyntaxKind::Ident);
    }

    #[test]
    fn lex_alphanumeric_identifier() {
        generic_test("ab123cde456", SyntaxKind::Ident);
    }

    #[test]
    fn lex_mixed_case_identifier() {
        generic_test("ABCdef", SyntaxKind::Ident);
    }

    #[test]
    fn lex_number() {
        generic_test("123456", SyntaxKind::Number);
    }

    #[test]
    fn lex_plus() {
        generic_test("+", SyntaxKind::Plus);
    }

    #[test]
    fn lex_minus() {
        generic_test("-", SyntaxKind::Minus);
    }

    #[test]
    fn lex_star() {
        generic_test("*", SyntaxKind::Star);
    }

    #[test]
    fn lex_slash() {
        generic_test("/", SyntaxKind::Slash);
    }

    #[test]
    fn lex_equals() {
        generic_test("=", SyntaxKind::Equals);
    }

    #[test]
    fn lex_left_brace() {
        generic_test("{", SyntaxKind::LBrace);
    }

    #[test]
    fn lex_right_brace() {
        generic_test("}", SyntaxKind::RBrace);
    }

    #[test]
    fn lex_left_paren() {
        generic_test("(", SyntaxKind::LParen);
    }

    #[test]
    fn lex_right_paren() {
        generic_test(")", SyntaxKind::RParen);
    }
}
