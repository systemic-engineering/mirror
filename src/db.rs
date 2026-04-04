//! db — git-backed storage for compiled conversation grammars.
//!
//! Stores `.conv` source text in a git repository, indexed by domain name
//! and content OID. Loading a domain recompiles from stored source — the
//! source IS the canonical form, compilation is cheap and deterministic.
//!
//! This is the conversation-native side of spectral-db. spectral-db depends
//! on conversation; this module lets conversation store domains without
//! creating a circular dependency.
//!
//! ```sh
//! conversation db init /tmp/mydb schema.conv
//! conversation db insert /tmp/mydb person '{"name":"alex"}'
//! conversation db query /tmp/mydb person
//! conversation db status /tmp/mydb
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::model::Mirror;
use crate::parse::Parse;
use crate::ContentAddressed;
use crate::Vector;

/// Errors from db operations.
#[derive(Debug)]
pub enum DbError {
    /// Git operation failed.
    Git(git2::Error),
    /// IO error.
    Io(std::io::Error),
    /// Schema compilation failed.
    Schema(String),
    /// Mirror not found.
    NotFound(String),
    /// Validation error.
    Validation(String),
}

impl From<git2::Error> for DbError {
    fn from(e: git2::Error) -> Self {
        DbError::Git(e)
    }
}

impl From<std::io::Error> for DbError {
    fn from(e: std::io::Error) -> Self {
        DbError::Io(e)
    }
}

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DbError::Git(e) => write!(f, "git: {}", e),
            DbError::Io(e) => write!(f, "io: {}", e),
            DbError::Schema(s) => write!(f, "schema: {}", s),
            DbError::NotFound(s) => write!(f, "not found: {}", s),
            DbError::Validation(s) => write!(f, "validation: {}", s),
        }
    }
}

/// A conversation database: git-backed grammar storage.
///
/// Each database has a schema grammar that defines valid node types.
/// Domains are stored as `.conv` source text in git blobs.
/// An index file maps domain names to git blob OIDs.
pub struct ConversationDb {
    repo: git2::Repository,
    schema: Mirror,
    schema_source: String,
    index: HashMap<String, String>,
    path: PathBuf,
}

/// Stats for a conversation database.
#[derive(Debug, Clone)]
pub struct DbStats {
    pub domain_count: usize,
    pub schema_name: String,
    pub schema_types: Vec<String>,
    pub path: String,
}

impl ConversationDb {
    /// Initialize a new conversation database at `path` with the given schema source.
    ///
    /// Creates a git repository and stores the schema as the first commit.
    pub fn init(path: impl AsRef<Path>, schema_source: &str) -> Result<Self, DbError> {
        let path = path.as_ref().to_path_buf();
        std::fs::create_dir_all(&path)?;

        let repo = git2::Repository::init(&path)?;
        let schema = compile_schema(schema_source)?;

        // Store schema source as a blob and commit it.
        // Scope the treebuilder borrow so repo can be moved into the struct.
        {
            let blob_oid = repo.blob(schema_source.as_bytes())?;
            let mut builder = repo.treebuilder(None)?;
            builder.insert("schema.conv", blob_oid, 0o100644)?;

            let index_oid = repo.blob(b"")?;
            builder.insert("index", index_oid, 0o100644)?;

            let tree_oid = builder.write()?;
            let tree = repo.find_tree(tree_oid)?;
            let sig = git2::Signature::now("conversation", "conversation@systemic.engineer")?;
            repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                "init: conversation db",
                &tree,
                &[],
            )?;
        }

        Ok(ConversationDb {
            repo,
            schema,
            schema_source: schema_source.to_string(),
            index: HashMap::new(),
            path,
        })
    }

    /// Open an existing conversation database.
    pub fn open(path: impl AsRef<Path>) -> Result<Self, DbError> {
        let path = path.as_ref().to_path_buf();
        let repo = git2::Repository::open(&path)?;

        // Read schema and index from HEAD. Scope the borrows so repo
        // can be moved into the struct afterward.
        let (schema_source, schema, index) = {
            let head = repo.head()?;
            let commit = head.peel_to_commit()?;
            let tree = commit.tree()?;

            let schema_entry = tree
                .get_name("schema.conv")
                .ok_or_else(|| DbError::Schema("no schema.conv in repository".to_string()))?;
            let schema_blob = repo.find_blob(schema_entry.id())?;
            let source = String::from_utf8_lossy(schema_blob.content()).to_string();
            let schema = compile_schema(&source)?;

            let idx = match tree.get_name("index") {
                Some(entry) => {
                    let blob = repo.find_blob(entry.id())?;
                    parse_index(blob.content())
                }
                None => HashMap::new(),
            };

            (source, schema, idx)
        };

        Ok(ConversationDb {
            repo,
            schema,
            schema_source,
            index,
            path,
        })
    }

    /// Store a compiled domain. Returns its content OID.
    ///
    /// The domain's source is stored as a git blob. The index is updated
    /// to map the domain name to its OID.
    pub fn store_domain(&mut self, source: &str) -> Result<String, DbError> {
        let domain = compile_schema(source)?;
        let domain_name = domain.domain_name().to_string();
        let oid = domain.content_oid();
        let oid_str = oid.as_ref().to_string();

        // Store source as a blob
        let blob_oid = self.repo.blob(source.as_bytes())?;

        // Update index
        self.index.insert(domain_name.clone(), oid_str.clone());

        // Commit: update tree with new domain blob and index
        let head = self.repo.head()?;
        let parent = head.peel_to_commit()?;
        let parent_tree = parent.tree()?;

        let mut builder = self.repo.treebuilder(Some(&parent_tree))?;
        // Tree entries are flat — no path separators. Use "domain.<name>" prefix.
        builder.insert(format!("domain.{}.conv", domain_name), blob_oid, 0o100644)?;

        // Write updated index
        let index_bytes = serialize_index(&self.index);
        let index_oid = self.repo.blob(&index_bytes)?;
        builder.insert("index", index_oid, 0o100644)?;

        let tree_oid = builder.write()?;
        let tree = self.repo.find_tree(tree_oid)?;
        let sig = git2::Signature::now("conversation", "conversation@systemic.engineer")?;
        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("store: @{}", domain_name),
            &tree,
            &[&parent],
        )?;

        Ok(oid_str)
    }

    /// Load a domain by name. Recompiles from stored source.
    pub fn load_domain(&self, name: &str) -> Result<Mirror, DbError> {
        let _oid = self
            .index
            .get(name)
            .ok_or_else(|| DbError::NotFound(name.to_string()))?;

        // Read source from git tree
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        let tree = commit.tree()?;

        let entry_name = format!("domain.{}.conv", name);
        let entry = tree
            .get_name(&entry_name)
            .ok_or_else(|| DbError::NotFound(format!("domain.{}.conv", name)))?;
        let blob = self.repo.find_blob(entry.id())?;
        let source = String::from_utf8_lossy(blob.content()).to_string();

        compile_schema(&source)
    }

    /// Load a domain by its content OID.
    pub fn load_domain_by_oid(&self, target_oid: &str) -> Result<Mirror, DbError> {
        let name = self
            .index
            .iter()
            .find(|(_, oid)| oid.as_str() == target_oid)
            .map(|(name, _)| name.clone())
            .ok_or_else(|| DbError::NotFound(target_oid.to_string()))?;
        self.load_domain(&name)
    }

    /// Insert a typed node into the database. Returns its content OID.
    ///
    /// The node type must exist in the schema.
    pub fn insert(&mut self, node_type: &str, data: &str) -> Result<String, DbError> {
        // Validate type against schema
        let valid = self
            .schema
            .types
            .iter()
            .any(|t| t.variants.iter().any(|v| v.name.as_str() == node_type));
        if !valid {
            return Err(DbError::Validation(format!(
                "type '{}' not in schema @{}",
                node_type,
                self.schema.domain_name()
            )));
        }

        // Content: "type:data"
        let content = format!("{}:{}", node_type, data);
        let blob_oid = self.repo.blob(content.as_bytes())?;
        let oid_str = blob_oid.to_string();

        // Update index
        let key = format!("node:{}", oid_str);
        self.index.insert(key, node_type.to_string());

        // Commit
        let head = self.repo.head()?;
        let parent = head.peel_to_commit()?;
        let parent_tree = parent.tree()?;

        let mut builder = self.repo.treebuilder(Some(&parent_tree))?;
        builder.insert(format!("node.{}", oid_str), blob_oid, 0o100644)?;

        let index_bytes = serialize_index(&self.index);
        let index_oid = self.repo.blob(&index_bytes)?;
        builder.insert("index", index_oid, 0o100644)?;

        let tree_oid = builder.write()?;
        let tree = self.repo.find_tree(tree_oid)?;
        let sig = git2::Signature::now("conversation", "conversation@systemic.engineer")?;
        self.repo.commit(
            Some("HEAD"),
            &sig,
            &sig,
            &format!("insert: {} node", node_type),
            &tree,
            &[&parent],
        )?;

        Ok(oid_str)
    }

    /// Query nodes by type. Returns (oid, data) pairs.
    pub fn query(&self, node_type: &str) -> Result<Vec<(String, String)>, DbError> {
        let head = self.repo.head()?;
        let commit = head.peel_to_commit()?;
        let tree = commit.tree()?;

        let mut results = Vec::new();
        for (key, val) in &self.index {
            if key.starts_with("node:") && val == node_type {
                let oid = key.strip_prefix("node:").unwrap();
                if let Some(entry) = tree.get_name(&format!("node.{}", oid)) {
                    if let Ok(blob) = self.repo.find_blob(entry.id()) {
                        let content = String::from_utf8_lossy(blob.content()).to_string();
                        // Parse "type:data" format
                        if let Some(pos) = content.find(':') {
                            let data = &content[pos + 1..];
                            results.push((oid.to_string(), data.to_string()));
                        }
                    }
                }
            }
        }

        Ok(results)
    }

    /// List all stored domain names.
    pub fn domain_names(&self) -> Vec<String> {
        self.index
            .iter()
            .filter(|(k, _)| !k.starts_with("node:"))
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// Get database stats.
    pub fn stats(&self) -> DbStats {
        let domain_count = self
            .index
            .iter()
            .filter(|(k, _)| !k.starts_with("node:"))
            .count();
        let schema_types: Vec<String> = self
            .schema
            .types
            .iter()
            .flat_map(|t| t.variants.iter().map(|v| v.name.as_str().to_string()))
            .collect();

        DbStats {
            domain_count,
            schema_name: self.schema.domain_name().to_string(),
            schema_types,
            path: self.path.display().to_string(),
        }
    }

    /// The schema domain.
    pub fn schema(&self) -> &Mirror {
        &self.schema
    }

    /// The schema source text.
    pub fn schema_source(&self) -> &str {
        &self.schema_source
    }
}

/// Compile a .conv source into a Mirror.
fn compile_schema(source: &str) -> Result<Mirror, DbError> {
    let ast = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| DbError::Schema(e.message))?;

    for child in ast.children() {
        if child.data().is_decl("grammar") {
            return Mirror::from_grammar(child).map_err(DbError::Schema);
        }
    }

    Err(DbError::Schema(
        "no grammar block found in source".to_string(),
    ))
}

/// Serialize index as "key\tvalue\n" lines.
fn serialize_index(index: &HashMap<String, String>) -> Vec<u8> {
    let mut lines: Vec<String> = index.iter().map(|(k, v)| format!("{}\t{}", k, v)).collect();
    lines.sort(); // deterministic output
    lines.join("\n").into_bytes()
}

/// Parse index from "key\tvalue\n" lines.
fn parse_index(bytes: &[u8]) -> HashMap<String, String> {
    let text = String::from_utf8_lossy(bytes);
    let mut map = HashMap::new();
    for line in text.lines() {
        if let Some((key, val)) = line.split_once('\t') {
            map.insert(key.to_string(), val.to_string());
        }
    }
    map
}

// ---------------------------------------------------------------------------
// CLI entry point
// ---------------------------------------------------------------------------

/// Run the `conversation db` subcommand.
pub fn cli(args: &[String]) {
    let subcmd = args.first().map(|s| s.as_str());
    match subcmd {
        Some("init") => cli_init(&args[1..]),
        Some("insert") => cli_insert(&args[1..]),
        Some("query") => cli_query(&args[1..]),
        Some("status") => cli_status(&args[1..]),
        _ => {
            eprintln!("usage: conversation db <init|insert|query|status>");
            eprintln!();
            eprintln!("  init   <path> <schema.conv>  — create a conversation db");
            eprintln!("  insert <path> <type> <data>   — insert a typed node");
            eprintln!("  query  <path> <type>          — find nodes by type");
            eprintln!("  status <path>                 — show db stats");
            std::process::exit(1);
        }
    }
}

fn cli_init(args: &[String]) {
    if args.len() < 2 {
        eprintln!("usage: conversation db init <path> <schema.conv>");
        std::process::exit(1);
    }
    let path = &args[0];
    let schema_path = &args[1];
    let source = match std::fs::read_to_string(schema_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("conversation db init: {}: {}", schema_path, e);
            std::process::exit(1);
        }
    };
    match ConversationDb::init(path, &source) {
        Ok(db) => {
            println!("initialized conversation db at {}", path);
            println!("schema: @{}", db.schema().domain_name());
        }
        Err(e) => {
            eprintln!("conversation db init: {}", e);
            std::process::exit(1);
        }
    }
}

fn cli_insert(args: &[String]) {
    if args.len() < 3 {
        eprintln!("usage: conversation db insert <path> <type> <data>");
        std::process::exit(1);
    }
    let path = &args[0];
    let node_type = &args[1];
    let data = &args[2];
    let mut db = match ConversationDb::open(path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("conversation db insert: {}", e);
            std::process::exit(1);
        }
    };
    match db.insert(node_type, data) {
        Ok(oid) => println!("{}", oid),
        Err(e) => {
            eprintln!("conversation db insert: {}", e);
            std::process::exit(1);
        }
    }
}

fn cli_query(args: &[String]) {
    if args.len() < 2 {
        eprintln!("usage: conversation db query <path> <type>");
        std::process::exit(1);
    }
    let path = &args[0];
    let node_type = &args[1];
    let db = match ConversationDb::open(path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("conversation db query: {}", e);
            std::process::exit(1);
        }
    };
    match db.query(node_type) {
        Ok(results) => {
            for (oid, data) in &results {
                println!("{}\t{}", oid, data);
            }
            if results.is_empty() {
                println!("no {} nodes found", node_type);
            }
        }
        Err(e) => {
            eprintln!("conversation db query: {}", e);
            std::process::exit(1);
        }
    }
}

fn cli_status(args: &[String]) {
    if args.is_empty() {
        eprintln!("usage: conversation db status <path>");
        std::process::exit(1);
    }
    let path = &args[0];
    let db = match ConversationDb::open(path) {
        Ok(db) => db,
        Err(e) => {
            eprintln!("conversation db status: {}", e);
            std::process::exit(1);
        }
    };
    let stats = db.stats();
    println!("conversation db: {}", stats.path);
    println!("schema: @{}", stats.schema_name);
    println!("types: {}", stats.schema_types.join(", "));
    println!("domains: {}", stats.domain_count);
    let total_nodes: usize = stats
        .schema_types
        .iter()
        .filter_map(|t| db.query(t).ok())
        .map(|r| r.len())
        .sum();
    println!("nodes: {}", total_nodes);
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    const SCHEMA: &str = "grammar @people {\n  type = person | team\n}";

    #[test]
    fn init_creates_repo() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let db = ConversationDb::init(&db_path, SCHEMA).unwrap();
        assert_eq!(db.schema().domain_name(), "people");
        assert!(db_path.join(".git").exists());
    }

    #[test]
    fn open_reads_schema() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        ConversationDb::init(&db_path, SCHEMA).unwrap();

        let db = ConversationDb::open(&db_path).unwrap();
        assert_eq!(db.schema().domain_name(), "people");
    }

    #[test]
    fn store_and_load_domain_roundtrip() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        let domain_source = "grammar @roles {\n  type = engineer | designer | manager\n}";
        let oid = db.store_domain(domain_source).unwrap();
        assert!(!oid.is_empty());

        let loaded = db.load_domain("roles").unwrap();
        assert_eq!(loaded.domain_name(), "roles");
        assert!(loaded.types[0]
            .variants
            .iter()
            .any(|v| v.name.as_str() == "engineer"));
        assert!(loaded.types[0]
            .variants
            .iter()
            .any(|v| v.name.as_str() == "designer"));
        assert!(loaded.types[0]
            .variants
            .iter()
            .any(|v| v.name.as_str() == "manager"));
    }

    #[test]
    fn store_domain_persists_across_reopen() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        {
            let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();
            db.store_domain("grammar @tools {\n  type = hammer | wrench\n}")
                .unwrap();
        }
        // Reopen
        let db = ConversationDb::open(&db_path).unwrap();
        let loaded = db.load_domain("tools").unwrap();
        assert_eq!(loaded.domain_name(), "tools");
    }

    #[test]
    fn load_domain_by_oid() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        let source = "grammar @colors {\n  type = red | blue | green\n}";
        let oid = db.store_domain(source).unwrap();

        let loaded = db.load_domain_by_oid(&oid).unwrap();
        assert_eq!(loaded.domain_name(), "colors");
    }

    #[test]
    fn insert_and_query() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        db.insert("person", "alex").unwrap();
        db.insert("person", "reed").unwrap();
        db.insert("team", "systemic").unwrap();

        let people = db.query("person").unwrap();
        assert_eq!(people.len(), 2);

        let teams = db.query("team").unwrap();
        assert_eq!(teams.len(), 1);
        assert_eq!(teams[0].1, "systemic");
    }

    #[test]
    fn insert_rejects_invalid_type() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        let err = db.insert("invalid", "data").unwrap_err();
        assert!(err.to_string().contains("not in schema"));
    }

    #[test]
    fn stats_reports_counts() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        db.store_domain("grammar @tools {\n  type = hammer | saw\n}")
            .unwrap();
        db.insert("person", "alex").unwrap();

        let stats = db.stats();
        assert_eq!(stats.domain_count, 1);
        assert_eq!(stats.schema_name, "people");
        assert!(stats.schema_types.contains(&"person".to_string()));
        assert!(stats.schema_types.contains(&"team".to_string()));
    }

    #[test]
    fn domain_names_lists_stored() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let mut db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        db.store_domain("grammar @a {\n  type = x | y\n}").unwrap();
        db.store_domain("grammar @b {\n  type = m | n\n}").unwrap();

        let mut names = db.domain_names();
        names.sort();
        assert_eq!(names, vec!["a", "b"]);
    }

    #[test]
    fn load_missing_domain_returns_not_found() {
        let dir = tempfile::tempdir().unwrap();
        let db_path = dir.path().join("testdb");
        let db = ConversationDb::init(&db_path, SCHEMA).unwrap();

        let err = db.load_domain("nonexistent").unwrap_err();
        assert!(err.to_string().contains("not found"));
    }

    #[test]
    fn serialize_deserialize_index() {
        let mut index = HashMap::new();
        index.insert("alpha".to_string(), "one".to_string());
        index.insert("beta".to_string(), "two".to_string());

        let bytes = serialize_index(&index);
        let restored = parse_index(&bytes);

        assert_eq!(restored.get("alpha").unwrap(), "one");
        assert_eq!(restored.get("beta").unwrap(), "two");
    }

    #[test]
    fn empty_index_roundtrip() {
        let index: HashMap<String, String> = HashMap::new();
        let bytes = serialize_index(&index);
        let restored = parse_index(&bytes);
        assert!(restored.is_empty());
    }
}
