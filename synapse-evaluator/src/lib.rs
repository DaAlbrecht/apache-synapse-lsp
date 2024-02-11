//#![warn(missing_docs)]
//! apache-synapse evaluator
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use tree_sitter::{Node, Point, Query, QueryCursor, Tree};

use tower_lsp::lsp_types;

/// The main object that is used to evaluate apache-synapse programs.
pub struct Evaluator {
    tree: Tree,
    properties: HashSet<String>,
    text: String,
}

/// The different types of diagnostics that can be returned by the evaluator.
#[derive(Debug)]
pub enum Diagnostic {
    Error(Error),
    Warning(Warning),
}

#[derive(Debug)]
pub struct Error {
    message: String,
    position: Point,
}

#[derive(Debug)]
pub struct Warning {
    message: String,
    position: Point,
}

pub enum Mediators {
    Log,
    Property,
}

#[derive(Debug)]
pub struct CaptureDetails {
    value: String,
    end_position: Point,
}

impl Evaluator {
    pub fn new(tree: Tree, text: String) -> Result<Self> {
        Ok(Evaluator {
            tree,
            properties: HashSet::new(),
            text,
        })
    }

    pub fn eval(&mut self) -> Result<Vec<Diagnostic>> {
        let mut diagnostics: Vec<Diagnostic> = Vec::new();
        let tree = self.tree.clone();
        let root = tree.root_node();
        let mut cursor = tree.walk();

        if root.has_error() {
            self.parse_error_node(root);
        }

        let mut children = cursor
            .node()
            .named_children(&mut cursor)
            .filter(|node| node.kind() == "sequence_definition");

        let seuqnce_definition = children
            .next()
            .ok_or_else(|| anyhow::anyhow!("No sequence definition found in the root node"))?;

        if seuqnce_definition.has_error() {
            self.parse_error_node(seuqnce_definition);
        }

        let mut cursor = seuqnce_definition.walk();

        cursor.goto_first_child();

        while cursor.goto_next_sibling() {
            let node = cursor.node();

            if node.has_error() {
                println!("node: {:?}", node);
                self.parse_error_node(node);
            }

            match node.kind() {
                "mediator" => {
                    let mediator_diagnostics = self.parse_mediator(node);
                    if let Ok(Some(child_diagnostics)) = mediator_diagnostics {
                        diagnostics.extend(child_diagnostics);
                    }
                }
                _ => {}
            }
        }

        Ok(diagnostics)
    }

    fn parse_mediator(&mut self, node: Node) -> Result<Option<Vec<Diagnostic>>> {
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

    fn parse_log_mediator(&mut self, node: Node) -> Result<Option<Vec<Diagnostic>>> {
        let mut diagnostics = Vec::new();
        let mut cursor = node.walk();
        let mediators = node
            .named_children(&mut cursor)
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
            let property_diagnostics = self.parse_property_mediator(mediator);

            if let Ok(Some(child_diagnostics)) = property_diagnostics {
                diagnostics.extend(child_diagnostics);
            }
        }

        match diagnostics.is_empty() {
            true => Ok(None),
            false => Ok(Some(diagnostics)),
        }
    }

    fn parse_property_mediator(&mut self, node: Node<'_>) -> Result<Option<Vec<Diagnostic>>> {
        let mut diagnostics = Vec::new();
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
                    diagnostics.push(Diagnostic::Error(Error {
                        message: format!("Property {} is not defined", ctx_prop),
                        position: props.get("expression").unwrap().end_position,
                    }));
                    println!("diagnostics: {:?}", diagnostics);
                }
            }
        }

        if let Some(name) = props.get("name") {
            self.properties.insert(name.value.clone());
        }
        match diagnostics.is_empty() {
            true => Ok(None),
            false => Ok(Some(diagnostics)),
        }
    }

    fn parse_error_node(&mut self, node: Node) {}

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
    use super::Evaluator;

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
        let mut evaluator = Evaluator::new(tree, input.to_string()).unwrap();
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
        let mut evaluator = Evaluator::new(tree, input.to_string()).unwrap();
        let _ = evaluator.eval().unwrap();
        assert!(evaluator.properties.len() == 2);
        assert!(evaluator.properties.contains("message"));
    }
    #[test]
    fn log_mediator_wrong_level() {
        let input = r#"
            <?xml version="1.0" encoding="UTF-8"?>
                <sequence name="main">
                    <log level="foo">
                    </log>
                </sequence>
            "#;
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(tree_sitter_apachesynapse::language())
            .expect("Error loading apache-synapse language");
        let tree = parser.parse(input, None).unwrap();
        let mut evaluator = Evaluator::new(tree, input.to_string()).unwrap();
        let _ = evaluator.eval().unwrap();
        assert!(false)
    }
}

impl Diagnostic {
    pub fn into_lsp_type(&self) -> Result<lsp_types::Diagnostic> {
        match self {
            Diagnostic::Error(err) => Ok(lsp_types::Diagnostic {
                range: lsp_types::Range {
                    start: lsp_types::Position {
                        line: err.position.row as u32,
                        character: err.position.column as u32,
                    },
                    end: lsp_types::Position {
                        line: err.position.row as u32,
                        character: err.position.column as u32,
                    },
                },
                severity: Some(lsp_types::DiagnosticSeverity::ERROR),
                message: err.message.clone(),
                ..Default::default()
            }),
            Diagnostic::Warning(warning) => Ok(lsp_types::Diagnostic {
                range: lsp_types::Range {
                    start: lsp_types::Position {
                        line: warning.position.row as u32,
                        character: warning.position.column as u32,
                    },
                    end: lsp_types::Position {
                        line: warning.position.row as u32,
                        character: warning.position.column as u32,
                    },
                },
                severity: Some(lsp_types::DiagnosticSeverity::WARNING),
                message: warning.message.clone(),
                ..Default::default()
            }),
        }
    }
}
