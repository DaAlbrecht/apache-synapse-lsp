#![warn(missing_docs)]
//! apache-synapse evaluator
use anyhow::Result;
use tree_sitter::{Tree, TreeCursor};

/// The main object that is used to evaluate apache-synapse programs.
pub struct Evaluator<'a> {
    tree_cursor: TreeCursor<'a>,
}

/// The different types of diagnostics that can be returned by the evaluator.
pub enum Diagnostic {
    Error,
    Warning,
}

struct TraversalCursor<'a> {
    tree_cursor: Option<TreeCursor<'a>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tree: &'a Tree) -> Result<Self> {
        if tree.language() != tree_sitter_apachesynapse::language() {
            return Err(anyhow::anyhow!(
                "Invalid language tree, expected an apache-synapse tree"
            ));
        }
        Ok(Evaluator {
            tree_cursor: tree.walk(),
        })
    }

    pub fn eval(self) -> Result<Vec<Diagnostic>> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let cursor = self.tree_cursor;
        let mut traversal_cursor = TraversalCursor {
            tree_cursor: Some(cursor),
        };

        unreachable!()
    }
}

impl<'a> Iterator for TraversalCursor<'a> {
    type Item = tree_sitter::Node<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let cursor = match self.tree_cursor.as_mut() {
            Some(c) => c,
            None => return None,
        };

        let node = cursor.node();
        if cursor.goto_first_child() || cursor.goto_next_sibling() {
            Some(node)
        } else {
            while !cursor.goto_next_sibling() {
                if !cursor.goto_parent() {
                    self.tree_cursor = None;
                    return Some(node);
                }
            }
            Some(node)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::TraversalCursor;

    use super::Evaluator;

    #[test]
    fn validate_tree_language() {
        let input = "foo";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let evaluator = Evaluator::new(&tree);
        assert!(evaluator.is_ok());

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading rust language");
        let tree = parser.parse(input, None).unwrap();
        let evaluator = Evaluator::new(&tree);
        assert!(evaluator.is_err());
    }

    #[test]
    fn iterate_tree() {
        let input = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <sequence name="main">
                <log level="custom">
                    <property name="message" value="Hello, world!"/>
                </log>
            </sequence>
        "#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let cursor = tree.walk();
        let traversal_cursor = TraversalCursor {
            tree_cursor: Some(cursor),
        };
        traversal_cursor.for_each(|node| {
            println!("Node: {}", node.kind());
        });
        assert!(false)
    }
}
