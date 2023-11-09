use ::tree_sitter::{Point, Tree};
use dashmap::DashMap;
use ropey::Rope;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};
use tree_sitter::parse;
mod apache_synapse;
mod tree_sitter;

#[derive(Debug)]
struct Backend {
    client: Client,
    document_map: DashMap<String, Rope>,
    tree_map: DashMap<String, Tree>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["<".to_string()]),
                    ..Default::default()
                }),
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                ..ServerCapabilities::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        apache_synapse::init_apache_synapse_mediators();
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = params.text_document_position_params.text_document.uri;
        let rope = self
            .document_map
            .get(&uri.to_string())
            .expect("File not found");
        let tree = self.tree_map.get(&uri.to_string()).expect("Tree not found");
        let position = params.text_document_position_params.position;
        let mut tree_cursor = tree.root_node().walk();
        let point = Point::new(position.line as usize, position.character as usize);
        tree_cursor.goto_first_child_for_point(point);
        let hovered_node = tree_cursor
            .node()
            .children(&mut tree_cursor)
            .find(|child| child.start_position() <= point && child.end_position() >= point);

        match hovered_node {
            Some(node) => match node.kind() {
                "mediator" => {
                    let mediator_name = node.named_child(0).expect("mediator name not found");
                    let hover = Hover {
                        contents: HoverContents::Scalar(MarkedString::String(format!(
                            "Mediator: {}",
                            mediator_name.kind()
                        ))),
                        range: None,
                    };
                    Ok(Some(hover))
                }
                _ => Ok(None),
            },
            None => todo!(),
        }
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file opened!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: params.text_document.text,
            version: params.text_document.version,
        })
        .await
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        self.client
            .log_message(MessageType::INFO, "file changed!")
            .await;
        self.on_change(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        })
        .await
    }

    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        let completion_list = CompletionList {
            is_incomplete: false,
            items: apache_synapse::APACHE_SYNAPSE_MEDIATORS
                .get()
                .unwrap()
                .clone(),
        };

        Ok(Some(CompletionResponse::List(completion_list)))
    }
}
struct TextDocumentItem {
    uri: Url,
    text: String,
    version: i32,
}
impl Backend {
    async fn on_change(&self, params: TextDocumentItem) {
        self.client
            .log_message(MessageType::INFO, format!("file changed: {}", params.uri))
            .await;
        let rope = ropey::Rope::from_str(&params.text);
        self.document_map.insert(params.uri.to_string(), rope);
        match parse(&params.text) {
            Ok(tree) => {
                self.tree_map.insert(params.uri.to_string(), tree.tree);
            }
            Err(e) => {
                self.client
                    .log_message(MessageType::ERROR, format!("error: {}", e))
                    .await;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| Backend {
        client,
        document_map: DashMap::new(),
        tree_map: DashMap::new(),
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}
