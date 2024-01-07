//#![warn(missing_docs)]
//! apache-synapse evaluator
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use anyhow::Result;
use tree_sitter::{Node, Query, QueryCursor, Tree, TreeCursor};

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
                    let tag_name = node
                        .utf8_text(self.text.as_bytes())
                        .expect("Error getting tag name");
                    println!("Error parsing tag {}", tag_name);
                }
                _ => {}
            });
        Ok(_diagnostics)
    }

    fn parse_mediator(&mut self, node: Node<'a>) -> Result<()> {
        let mut cursor = node.walk();
        let node_type = node.named_children(&mut cursor).next();
        match node_type {
            Some(mediator_typ) => match Mediators::from_str(mediator_typ.kind()) {
                Ok(mediator) => match mediator {
                    Mediators::Log => self.parse_log_mediator(node),
                    Mediators::Property => self.parse_property_mediator(node),
                },
                Err(mediator_err) => {
                    println!("Error parsing mediator: {}", mediator_err);
                    Err(anyhow::anyhow!("Error parsing mediator: {}", mediator_err))
                }
            },
            None => Err(anyhow::anyhow!("Invalid node")),
        }
    }

    fn parse_log_mediator(&mut self, node: Node<'a>) -> Result<()> {
        let query_string = r#"(_
        (mediator)@mediator
        )"#;

        let props = self.query_props(query_string, node);

        println!("{:?}", props);
        Ok(())
    }

    fn parse_property_mediator(&mut self, node: Node<'_>) -> Result<()> {
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

        let props = self.query_props(query_string, node);

        //if the expression contains a $ctx: then we need to make sure that the property is defined
        if let Some(expression) = props.get("expression") {
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
            self.properties.insert(name.to_owned());
        }
        Ok(())
    }

    fn query_props(&mut self, query_string: &str, node: Node) -> HashMap<String, String> {
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

                acc.insert(key, value);

                acc
            })
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
        evaluator.eval().unwrap();
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
