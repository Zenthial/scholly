mod event;
mod expr;
mod sink;
mod source;

use crate::lexer::{Lexeme, Lexer, SyntaxKind};
use crate::syntax::SyntaxNode;
use event::Event;
use expr::expr;
use rowan::GreenNode;
use sink::Sink;

use self::source::Source;

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
    source: Source<'l, 'input>,
    events: Vec<Event>,
}

impl<'l, 'input> Parser<'l, 'input> {
    fn new(lexemes: &'l [Lexeme<'input>]) -> Self {
        Self {
            source: Source::new(lexemes),
            events: Vec::new(),
        }
    }

    fn parse(mut self) -> Vec<Event> {
        self.start_node(SyntaxKind::Root);
        expr(&mut self);
        self.finish_node();

        self.events
    }

    fn bump(&mut self) {
        let Lexeme { kind, text } = self.source.next_lexeme().unwrap();

        self.events.push(Event::AddToken {
            kind: *kind,
            text: (*text).into(),
        })
    }

    fn peek(&mut self) -> Option<SyntaxKind> {
        self.source.peek_kind()
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

    #[test]
    fn parse_comment() {
        check(
            "# hello!",
            expect![[r##"
Root@0..8
  Comment@0..8 "# hello!""##]],
        );
    }

    #[test]
    fn parse_binary_expression_interspersed_with_comments() {
        check(
            "
1
  + 1 # Add one
  + 10 # Add ten",
            expect![[r##"
Root@0..35
  Whitespace@0..1 "\n"
  BinaryExpr@1..35
    BinaryExpr@1..21
      Number@1..2 "1"
      Whitespace@2..5 "\n  "
      Plus@5..6 "+"
      Whitespace@6..7 " "
      Number@7..8 "1"
      Whitespace@8..9 " "
      Comment@9..18 "# Add one"
      Whitespace@18..21 "\n  "
    Plus@21..22 "+"
    Whitespace@22..23 " "
    Number@23..25 "10"
    Whitespace@25..26 " "
    Comment@26..35 "# Add ten""##]],
        );
    }
}
