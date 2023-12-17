//#![warn(missing_docs)]
//! apache-synapse evaluator
use std::{collections::HashSet, str::FromStr};

use anyhow::Result;
use tree_sitter::{Node, Tree, TreeCursor};

/// The main object that is used to evaluate apache-synapse programs.
pub struct Evaluator<'a> {
    tree_cursor: TreeCursor<'a>,
    properties: HashSet<String>,
    text: &'a str,
}

/// The different types of diagnostics that can be returned by the evaluator.
pub enum Diagnostic {
    Error,
    Warning,
}

pub enum Mediators {
    Log,
    Property,
}

struct PreOrderTraversal<'a> {
    tree_cursor: Option<TreeCursor<'a>>,
}

impl<'a> Evaluator<'a> {
    pub fn new(tree: &'a Tree, text: &'a str) -> Result<Self> {
        if tree.language() != tree_sitter_apachesynapse::language() {
            return Err(anyhow::anyhow!(
                "Invalid language tree, expected an apache-synapse tree"
            ));
        }
        Ok(Evaluator {
            tree_cursor: tree.walk(),
            properties: HashSet::new(),
            text,
        })
    }

    pub fn eval(&mut self) -> Result<Vec<Diagnostic>> {
        let mut _diagnostics: Vec<Diagnostic> = Vec::new();
        let traversal_cursor = PreOrderTraversal {
            tree_cursor: Some(self.tree_cursor.clone()),
        };

        traversal_cursor
            .filter(|node| node.is_named())
            .for_each(|node| match node.kind() {
                "mediator" => {
                    let _ = self.parse_mediator(node);
                }
                "ERROR" => {
                    //not sure how to handle this yet
                    for child in node.children(&mut node.walk()) {
                        println!("Error: {:?}", child.kind());
                    }
                }
                _ => {}
            });
        Ok(_diagnostics)
    }

    fn parse_mediator(&mut self, node: Node<'a>) -> Result<()> {
        let mut cursor = node.walk();
        let children = node.children(&mut cursor);
        for child in children {
            if let Ok(mediator) = Mediators::from_str(child.kind()) {
                match mediator {
                    Mediators::Log => {
                        self.parse_log_mediator(child)?;
                    }
                    Mediators::Property => {
                        self.parse_property_mediator(child)?;
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_log_mediator(&mut self, _node: Node<'a>) -> Result<()> {
        todo!()
    }

    fn parse_property_mediator(&mut self, node: Node<'_>) -> Result<()> {
        //<property name="message" value="Hello, world!"/>
        /*
        (property
            (name
             (identifier)
            )
            (value
            ))
             */
        let mut cursor = node.walk();

        let mut children = node.named_children(&mut cursor);

        let name_field = children.find(|child| child.kind() == "name").unwrap();
        let mut name_cursor = name_field.walk();
        let mut children = name_field.children(&mut name_cursor);

        let identifier = children
            .find(|child| child.kind() == "identifier")
            .ok_or(anyhow::anyhow!("Missing identifier"))?
            .utf8_text(self.text.as_bytes())?;

        self.properties.insert(identifier.to_string());

        Ok(())
    }
}

impl<'a> Iterator for PreOrderTraversal<'a> {
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

impl FromStr for Mediators {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "log" => Ok(Mediators::Log),
            "property" => Ok(Mediators::Property),
            _ => Err(anyhow::anyhow!("Invalid mediator")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::PreOrderTraversal;

    use super::Evaluator;

    #[test]
    fn validate_tree_language() {
        let input = "foo";
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let evaluator = Evaluator::new(&tree, input);
        assert!(evaluator.is_ok());

        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_rust::language())
            .expect("Error loading rust language");
        let tree = parser.parse(input, None).unwrap();
        let _traversal = PreOrderTraversal {
            tree_cursor: Some(tree.walk()),
        };
        let evaluator = Evaluator::new(&tree, input);
        assert!(evaluator.is_err());
    }

    #[test]
    fn iterate_tree() {
        let input = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <sequence name="main">
                <property name="message" value="Hello, world!"/>
            </sequence>
        "#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let mut evaluator = Evaluator::new(&tree, input).unwrap();
        evaluator.eval().unwrap();

        assert!(true)
    }
}
