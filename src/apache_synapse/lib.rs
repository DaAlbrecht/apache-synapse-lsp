use std::{collections::HashMap, sync::OnceLock};

use tower_lsp::lsp_types::{CompletionItem, CompletionItemKind};

pub static APACHE_SYNAPSE_MEDIATORS: OnceLock<Vec<CompletionItem>> = OnceLock::new();
pub static TEXT_STORE: OnceLock<HashMap<String, String>> = OnceLock::new();

pub fn init_stores() {
    init_text_store();
    init_apache_synapse_mediators();
}

fn init_text_store() {
    _ = TEXT_STORE.set(
        vec![
            ("NTLM", include_str!("./mediators/ntlm-mediator.md")),
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
            ("callout", include_str!("./mediators/callout-mediator.md")),
            ("class", include_str!("./mediators/class-mediator.md")),
            ("clone", include_str!("./mediators/clone-mediator.md")),
            ("dss", include_str!("./mediators/dss-mediator.md")),
            (
                "data-mapper-json-schema",
                include_str!("./mediators/data-mapper-json-schema-specification.md"),
            ),
            (
                "datamapper",
                include_str!("./mediators/data-mapper-mediator.md"),
            ),
            ("dblookup", include_str!("./mediators/dblookup-mediator.md")),
            (
                "dbreport",
                include_str!("./mediators/db-report-mediator.md"),
            ),
            ("drop", include_str!("./mediators/drop-mediator.md")),
            ("ejb", include_str!("./mediators/ejb-mediator.md")),
            ("enrich", include_str!("./mediators/enrich-mediator.md")),
            (
                "entitlementService",
                include_str!("./mediators/entitlement-mediator.md"),
            ),
            ("fastXSLT", include_str!("./mediators/fastxslt-mediator.md")),
            ("filter", include_str!("./mediators/filter-mediator.md")),
            ("foreach", include_str!("./mediators/foreach-mediator.md")),
            ("header", include_str!("./mediators/header-mediator.md")),
            ("iterate", include_str!("./mediators/iterate-mediator.md")),
            (
                "jsontransform",
                include_str!("./mediators/json-transform-mediator.md"),
            ),
            ("log", include_str!("./mediators/log-mediator.md")),
            ("loopback", include_str!("./mediators/loopback-mediator.md")),
            ("makefault", include_str!("./mediators/fault-mediator.md")),
            (
                "oauthService",
                include_str!("./mediators/oauth-mediator.md"),
            ),
            ("property", include_str!("./mediators/property-mediator.md")),
            (
                "property-group",
                include_str!("./mediators/property-group-mediator.md"),
            ),
            ("respond", include_str!("./mediators/respond-mediator.md")),
            ("script", include_str!("./mediators/script-mediator.md")),
            ("send", include_str!("./mediators/send-mediator.md")),
            ("sequence", include_str!("./mediators/sequence-mediator.md")),
            ("smooks", include_str!("./mediators/smooks-mediator.md")),
            ("store", include_str!("./mediators/store-mediator.md")),
            ("switch", include_str!("./mediators/switch-mediator.md")),
            ("throttle", include_str!("./mediators/throttle-mediator.md")),
            (
                "transaction",
                include_str!("./mediators/transaction-mediator.md"),
            ),
            (
                "urlRewrite",
                include_str!("./mediators/urlRewrite-mediator.md"),
            ),
            ("validate", include_str!("./mediators/validate-mediator.md")),
            ("xquery", include_str!("./mediators/xquery-mediator.md")),
            ("xslt", include_str!("./mediators/xslt-mediator.md")),
        ]
        .into_iter()
        .map(|(name, doc)| (name.to_string(), doc.to_string()))
        .collect(),
    )
}

fn init_apache_synapse_mediators() {
    _ = APACHE_SYNAPSE_MEDIATORS.set(to_apache_synapse_mediator_completion());
}

fn to_apache_synapse_mediator_completion() -> Vec<CompletionItem> {
    TEXT_STORE
        .get()
        .expect("Text store not initialized")
        .iter()
        .map(|(name, doc)| CompletionItem {
            label: name.to_string(),
            detail: Some("Apache Synapse Mediator".to_string()),
            documentation: Some(tower_lsp::lsp_types::Documentation::MarkupContent(
                tower_lsp::lsp_types::MarkupContent {
                    kind: tower_lsp::lsp_types::MarkupKind::Markdown,
                    value: doc.to_string(),
                },
            )),
            kind: Some(CompletionItemKind::KEYWORD),
            ..CompletionItem::default()
        })
        .collect()
}
