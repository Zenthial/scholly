mod event;
mod expr;
mod sink;

use crate::lexer::{Lexeme, Lexer, SyntaxKind};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;

pub fn parse(input: &str) -> Parse {
    let lexemes: Vec<_> = Lexer::new(input).collect();
    let parser = Parser::new(&lexemes);
    let events = parser.parse();
    let sink = Sink::new(&lexemes, events);

    Parse {
        green_node: sink.finish(),
    }
}

struct Parser<'l, 'input> {
    lexemes: &'l [Lexeme<'input>],
    cursor: usize,
    events: Vec<Event>,
}

impl<'l, 'input> Parser<'l, 'input> {
    fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            lexemes,
            cursor: 0,
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root);
        expr(&mut self);
        self.finish_node();

        self.events
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.eat_whitespace();
        self.peek_raw()
    }

    fn eat_whitespace(&mut self) {
        while self.peek_raw() == Some(SyntaxKind::Whitespace) {
            self.cursor += 1;
        }
    }

    fn peek_raw(&mut self) -> Option<SyntaxKind> {
        self.lexemes
            .get(self.cursor)
            .map(|Lexeme { kind, .. }| *kind)
    }

    fn bump(&mut self) {
        let Lexeme { kind, text } = self.lexemes[self.cursor];

        self.cursor += 1;
        self.events.push(Event::AddToken {
            kind,
            text: text.into(),
        })
    }

    fn start_node_at(&mut self, checkpoint: usize, kind: SyntaxKind) {
        self.events.push(Event::StartNodeAt { kind, checkpoint })
    }

    fn checkpoint(&self) -> usize {
        self.events.len()
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.events.push(Event::StartNode { kind })
    }

    fn finish_node(&mut self) {
        self.events.push(Event::FinishNode)
    }
}

pub struct Parse {
    green_node: GreenNode,
}

impl Parse {
    pub fn debug_tree(&self) -> String {
        let syntax_node = SyntaxNode::new_root(self.green_node.clone());
        let formatted = format!("{:#?}", syntax_node);
        formatted[0..formatted.len() - 1].to_string()
    }
}

#[cfg(test)]
fn check(input: &str, expected_tree: expect_test::Expect) {
    let parse = parse(input);
    expected_tree.assert_eq(&parse.debug_tree());
}

#[cfg(test)]
mod parser_tests {
    use super::*;
    use expect_test::expect;

    #[test]
    fn parse_nothing() {
        check("", expect![[r#"Root@0..0"#]]);
    }

    #[test]
    fn parse_whitespace() {
        check(
            "   ",
            expect![[r#"
Root@0..3
  Whitespace@0..3 "   ""#]],
        );
    }
}