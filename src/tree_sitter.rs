use tree_sitter::Parser;

pub fn parse(src: &str) -> tree_sitter::Tree {
    let mut parser = Parser::new();
    parser
        .set_language(tree_sitter_apachesynapse::language())
        .expect("Error loading apache synapse grammar");

    parser.parse(src, None).unwrap()
}
