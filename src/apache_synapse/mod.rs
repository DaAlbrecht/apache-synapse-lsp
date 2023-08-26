use std::sync::OnceLock;

use tower_lsp::lsp_types::CompletionItem;

pub static APACHE_SYNAPSE_MEDIATORS: OnceLock<Vec<CompletionItem>> = OnceLock::new();

pub fn init_apache_synapse_mediators() {
    APACHE_SYNAPSE_MEDIATORS.set(to_apache_synapse_mediator_completion(vec![
        (
            "aggregate",
            include_str!("./mediators/aggregate-mediator.md"),
        ),
        ("builder", include_str!("./mediators/builder-mediator.md")),
        ("cache", include_str!("./mediators/cache-mediator.md")),
        ("call", include_str!("./mediators/call-mediator.md")),
        (
            "call-template",
            include_str!("./mediators/call-template-mediator.md"),
        ),
        "callout",
        include_str!("./mediators/callout-mediator.md"),
        "class",
        include_str!("./mediators/class-mediator.md"),
        "clone",
        include_str!("./mediators/clone-mediator.md"),
        "data-mapper-json",
        include_str!("./mediators/data-mapper-json-mediator.md"),
        "data-mapper",
        include_str!("./mediators/data-mapper-mediator.md"),
        "dbreport",
        include_str!("./mediators/dbreport-mediator.md"),
        "dblookup",
        include_str!("./mediators/dblookup-mediator.md"),
        "dss",
        include_str!("./mediators/dss-mediator.md"),
    ]));
}

fn to_apache_synapse_mediator_completion(vec: Vec<(&str, &str)>) -> Vec<CompletionItem> {
    vec.iter()
        .map(|(name, doc)| CompletionItem {
            label: name.to_string(),
            detail: Some("Apache Synapse Mediator".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: doc.to_string(),
                },
            )),
            ..CompletionItem::default()
        })
        .collect()
}
