use tree_sitter::{Parser, Tree};

pub struct ParseProduct {
    pub tree: Tree,
}

pub fn parse(src: &str) -> Result<ParseProduct, &'static str> {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_apachesynapse::language())
        .expect("Error loading apache synapse grammar");

    let tree = parser
        .parse(src, None)
        .ok_or_else(|| "Error parsing source")?;
    Ok(ParseProduct { tree })
}
