use std::sync::OnceLock;

use tower_lsp::lsp_types::CompletionItem;

pub static APACHE_SYNAPSE_MEDIATORS: OnceLock<Vec<CompletionItem>> = OnceLock::new();

pub fn init_apache_synapse_mediators() {
    APACHE_SYNAPSE_MEDIATORS.set(to_apache_synapse_mediator_completion(vec![
        ("log", include_str!("./mediators/log-mediator.md")),
        ("call", include_str!("./mediators/call-mediator.md")),
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
