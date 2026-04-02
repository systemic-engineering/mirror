//! Language Server Protocol support for `.conv` files.
//!
//! Provides diagnostics, hover, and text synchronization via tower-lsp.

pub mod analysis;
pub mod position;

use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer, LspService, Server};

use crate::model::Domain;
use crate::packages::PackageRegistry;
use crate::resolve::Namespace;

/// In-memory state for a single open document.
struct Document {
    source: String,
    domains: Vec<Domain>,
}

/// The conversation language server.
struct ConversationLsp {
    client: Client,
    documents: RwLock<HashMap<Url, Document>>,
    namespace: Arc<Namespace>,
}

#[tower_lsp::async_trait]
impl LanguageServer for ConversationLsp {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::FULL,
                )),
                hover_provider: Some(HoverProviderCapability::Simple(true)),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "conversation-lsp".into(),
                version: Some(env!("CARGO_PKG_VERSION").into()),
            }),
        })
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let source = params.text_document.text;
        self.update_document(uri, source).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        // Full sync: last change contains the entire document.
        if let Some(change) = params.content_changes.into_iter().last() {
            self.update_document(uri, change.text).await;
        }
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.clone();
        {
            let mut docs = self.documents.write().unwrap();
            docs.remove(&uri);
        }
        // Clear diagnostics for the closed file.
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn hover(&self, params: HoverParams) -> Result<Option<Hover>> {
        let uri = &params.text_document_position_params.text_document.uri;
        let position = params.text_document_position_params.position;

        let docs = self.documents.read().unwrap();
        let doc = match docs.get(uri) {
            Some(d) => d,
            None => return Ok(None),
        };

        let result = analysis::hover_at(
            &doc.source,
            position,
            &self.namespace,
            &doc.domains,
        );

        Ok(result.map(|text| Hover {
            contents: HoverContents::Markup(MarkupContent {
                kind: MarkupKind::Markdown,
                value: text,
            }),
            range: None,
        }))
    }
}

impl ConversationLsp {
    async fn update_document(&self, uri: Url, source: String) {
        let result = analysis::analyze(&source, &self.namespace);

        self.client
            .publish_diagnostics(uri.clone(), result.diagnostics, None)
            .await;

        let mut docs = self.documents.write().unwrap();
        docs.insert(
            uri,
            Document {
                source,
                domains: result.domains,
            },
        );
    }
}

/// Load the garden namespace from `~/.conversation/` (or `$CONVERSATION_PACKAGES`).
fn load_namespace() -> Namespace {
    let home = PackageRegistry::packages_dir();
    let roots = PackageRegistry::package_roots(&home);
    if roots.is_empty() {
        return Namespace::new();
    }
    match PackageRegistry::discover_ordered(&roots) {
        Ok(registry) => match registry.to_namespace() {
            Ok(ns) => ns,
            Err(e) => {
                eprintln!("conversation-lsp: packages: {}", e);
                Namespace::new()
            }
        },
        Err(e) => {
            eprintln!("conversation-lsp: packages: {}", e);
            Namespace::new()
        }
    }
}

/// Start the LSP server on stdin/stdout.
pub async fn run() {
    let namespace = Arc::new(load_namespace());

    let (service, socket) = LspService::new(|client| ConversationLsp {
        client,
        documents: RwLock::new(HashMap::new()),
        namespace,
    });

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();
    Server::new(stdin, stdout, socket).serve(service).await;
}
