//#![warn(missing_docs)]
//! apache-synapse evaluator
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use tree_sitter::{Node, Point, Query, QueryCursor, Tree, TreeCursor};

/// The main object that is used to evaluate apache-synapse programs.
pub struct Evaluator<'a> {
    tree_cursor: TreeCursor<'a>,
    properties: HashSet<String>,
    text: &'a str,
}

/// The different types of diagnostics that can be returned by the evaluator.
#[derive(Debug)]
pub enum Diagnostic {
    Error(Error),
    Warning(Warning),
}

#[derive(Debug)]
struct Error {
    message: String,
    position: Point,
}

#[derive(Debug)]
struct Warning {
    message: String,
    position: Point,
}

pub enum Mediators {
    Log,
    Property,
}

struct PreOrderTraversal<'a> {
    tree_cursor: Option<TreeCursor<'a>>,
}

#[derive(Debug)]
struct CaptureDetails {
    value: String,
    end_position: Point,
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
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let mut cursor = self.tree_cursor.clone();

        //TODO: not hardcore to sequence
        let mut children = self
            .tree_cursor
            .node()
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "sequence_definition");

        let mut cursor = children.next().unwrap().walk();

        let children = cursor
            .node()
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "mediator");

        for child in children {
            match self.parse_mediator(child) {
                Ok(Some(child_diagnostics)) => {
                    diagnostics.extend(child_diagnostics);
                }
                Ok(None) => {}
                Err(err) => {
                    anyhow::bail!(err);
                }
            }
        }

        Ok(diagnostics)
    }

    fn parse_mediator(&mut self, node: Node<'a>) -> Result<Option<Vec<Diagnostic>>> {
        let mut cursor = node.walk();
        let next_child = node.named_children(&mut cursor).next();
        match next_child {
            Some(next_child) => match Mediators::from_str(next_child.kind()) {
                Ok(mediator) => match mediator {
                    Mediators::Log => self.parse_log_mediator(next_child),
                    Mediators::Property => self.parse_property_mediator(next_child),
                },
                Err(mediator_err) => {
                    Err(anyhow::anyhow!("Error parsing mediator: {}", mediator_err))
                }
            },
            None => Err(anyhow::anyhow!("Invalid node")),
        }
    }

    fn parse_log_mediator(&mut self, node: Node<'a>) -> Result<Option<Vec<Diagnostic>>> {
        let mut diagnostics = Vec::new();
        let mediators = node
            .named_children(&mut self.tree_cursor)
            .filter(|node| node.kind() == "mediator")
            .filter_map(|node| node.child(0))
            .collect::<Vec<_>>();

        for mediator in mediators {
            let kind = mediator.kind();
            if kind != "property" {
                diagnostics.push(Diagnostic::Error(Error {
                    message: format!("Invalid mediator {}", kind),
                    position: mediator.start_position(),
                }));
                continue;
            }
            let _ = self.parse_property_mediator(mediator);
        }

        match diagnostics.is_empty() {
            true => Ok(None),
            false => Ok(Some(diagnostics)),
        }
    }

    fn parse_property_mediator(&mut self, node: Node<'_>) -> Result<Option<Vec<Diagnostic>>> {
        let query_string = r#"[
           (_
               (name
                   (identifier) @name
                   )
               (value
               )
            )
            (_
             (name
              (identifier)@name)
             (expression
              (_)*
              )@expression
             )

        ]"#;

        let props = self.query_capture(query_string, node);

        //if the expression contains a $ctx: then we need to make sure that the property is defined
        if let Some(expression) = props.get("expression").map(|c| &c.value) {
            if expression.contains("$ctx:") {
                let ctx_prop = expression
                    .split_once("$ctx:")
                    .ok_or_else(|| anyhow::anyhow!("Invalid expression"))?
                    .1
                    .strip_suffix('"')
                    .ok_or_else(|| anyhow::anyhow!("Invalid expression"))?;
                if !self.properties.contains(ctx_prop) {
                    println!("Property {} is not defined", ctx_prop);
                    return Err(anyhow::anyhow!("Property {} is not defined", ctx_prop));
                }
            }
        }

        if let Some(name) = props.get("name") {
            self.properties.insert(name.value.clone());
        }
        Ok(None)
    }

    fn query_capture(&mut self, query_string: &str, node: Node) -> HashMap<String, CaptureDetails> {
        let query = Query::new(tree_sitter_apachesynapse::language(), query_string)
            .unwrap_or_else(|e| panic!("Error creating query: {}", e));

        let mut query_cursor = QueryCursor::new();

        let capture_names = query.capture_names();

        let matches = query_cursor.matches(&query, node, self.text.as_bytes());
        matches
            .into_iter()
            .flat_map(|m| m.captures)
            .fold(HashMap::new(), |mut acc, capture| {
                let key = capture_names[capture.index as usize].to_owned();
                let value = if let Ok(capture_value) = capture.node.utf8_text(self.text.as_bytes())
                {
                    capture_value.to_owned()
                } else {
                    eprintln!("Error getting capture value");
                    "".to_owned()
                };
                let end_position = capture.node.end_position();

                acc.insert(
                    key,
                    CaptureDetails {
                        value,
                        end_position,
                    },
                );

                acc
            })
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
    fn property_mediator() {
        let input = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <sequence name="main">
                <property name="message" value="Hello, world!"/>
                <property name="message" value="Hello, world!"/>
                <property name="foo" value="Hello, world!"/>
                <property name="baz" expression="$ctx:foo" />
            </sequence>
        "#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let mut evaluator = Evaluator::new(&tree, input).unwrap();
        evaluator.eval().unwrap();

        println!("{:?}", evaluator.properties);

        assert!(evaluator.properties.len() == 3);
        assert!(evaluator.properties.contains("message"));
        assert!(evaluator.properties.contains("foo"));
        assert!(evaluator.properties.contains("baz"));
    }

    #[test]
    fn log_mediator() {
        let input = r#"
        <?xml version="1.0" encoding="UTF-8"?>
            <sequence name="main">
                <log level="custom">
                    <property name="message" value="Hello, world!"/>
                    <property name="foo" expression="$ctx:message" />
                </log>
            </sequence>
        "#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let mut evaluator = Evaluator::new(&tree, input).unwrap();
        let _ = evaluator.eval().unwrap();
        assert!(evaluator.properties.len() == 2);
        assert!(evaluator.properties.contains("message"));
    }
    /*
        #[test]
        fn log_mediator_wrong_level() {
            let input = r#"
            <?xml version="1.0" encoding="UTF-8"?>
                <sequence name="main">
                    <log level="foo">
                        <property name="message" value="Hello, world!"/>
                        <property name="foo" expression="$ctx:message" />
                    </log>
                </sequence>
            "#;
            let mut parser = tree_sitter::Parser::new();
            parser
                .set_language(tree_sitter_apachesynapse::language())
                .expect("Error loading apache-synapse language");
            let tree = parser.parse(input, None).unwrap();
            let mut evaluator = Evaluator::new(&tree, input).unwrap();
            evaluator.eval().unwrap();
            assert!(evaluator.properties.len() == 2);
            assert!(evaluator.properties.contains("message"));
            assert!(false)
        }
    */
}
