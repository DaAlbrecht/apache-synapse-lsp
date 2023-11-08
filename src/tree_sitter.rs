use anyhow::Result;
use tree_sitter::{Parser, Tree};

pub struct ParseProduct {
    pub tree: Tree,
}

pub fn parse(src: &str) -> Result<ParseProduct> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_apachesynapse::language())
        .expect("Error loading apache synapse grammar");

    let tree = parser
        .parse(src, None)
        .ok_or_else(|| anyhow::anyhow!("Error parsing source"))?;
    Ok(ParseProduct { tree })
}
