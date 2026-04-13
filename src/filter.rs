//! Filter domains. `Vector<Value, Value>` pipeline stages.
//!
//! A filter transforms a Value — it doesn't read a tree from external state.
//! Filters compose in pipelines: `slug | @sha`, `article | @html | @sign`.
//!
//! Four filters:
//! - `@sha` — SHA-512 of the JSON string representation
//! - `@hash` — broader hashing (SHA-512 default)
//! - `@sign` — wraps value in a signed envelope
//! - `@encrypt` — encrypts value with age (SSH key support)

use std::io::Write;
use std::path::PathBuf;

use serde_json::Value;

use crate::{ContentAddressed, Oid, Trace, Vector};

// ---------------------------------------------------------------------------
// FilterError — local error type (formerly from resolve.rs)
// ---------------------------------------------------------------------------

/// An error from filter resolution or application.
#[derive(Debug, Clone)]
pub struct ResolveError {
    pub message: String,
    pub span: Option<(usize, usize)>,
    pub hints: Vec<String>,
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ResolveError {}

// ---------------------------------------------------------------------------
// Sha — content hash as pipeline operator
// ---------------------------------------------------------------------------

/// SHA-512 of the Value's JSON string representation.
/// Same hash as `ContentAddressed for Value`, surfaced as a filter.
pub struct Sha;

impl Vector<Value, Value> for Sha {
    type Error = std::convert::Infallible;

    fn trace(&self, source: Value) -> Trace<Value, Self::Error> {
        let hash = Oid::hash(source.to_string().as_bytes());
        let result = Value::String(hash.as_ref().to_string());
        let oid = result.content_oid();
        Trace::success(result, oid.into(), None)
    }
}

// ---------------------------------------------------------------------------
// Hash — broader hashing (SHA-512 default, future: configurable)
// ---------------------------------------------------------------------------

/// Hash filter. SHA-512.
pub struct Hash;

impl Vector<Value, Value> for Hash {
    type Error = std::convert::Infallible;

    fn trace(&self, source: Value) -> Trace<Value, Self::Error> {
        Sha.trace(source)
    }
}

// ---------------------------------------------------------------------------
// SignFilter — structural witness as pipeline operator
// ---------------------------------------------------------------------------

/// Wraps a value in a signed envelope.
/// Output: `{ "signer": name, "signature": hex, "value": original, "oid": content_hash }`
pub struct SignFilter {
    pub signer: String,
    pub signature: Vec<u8>,
}

impl SignFilter {
    pub fn new(signer: impl Into<String>, signature: Vec<u8>) -> Self {
        SignFilter {
            signer: signer.into(),
            signature,
        }
    }

    /// Load signing identity from a keys directory.
    /// Reads the first `.pub` file (alphabetically).
    pub fn from_keys_dir(path: &std::path::Path) -> Option<Self> {
        let mut entries: Vec<_> = std::fs::read_dir(path)
            .ok()?
            .filter_map(|e| e.ok())
            .filter(|e| {
                e.path()
                    .extension()
                    .map(|ext| ext == "pub")
                    .unwrap_or(false)
            })
            .collect();
        entries.sort_by_key(|e| e.file_name());
        let entry = entries.first()?;
        Self::from_pub_file(&entry.path())
    }

    /// Load signing identity from a specific public key file.
    pub fn from_pub_file(path: &std::path::Path) -> Option<Self> {
        let content = std::fs::read_to_string(path).ok()?;
        let content = content.trim();
        let parts: Vec<&str> = content.splitn(3, ' ').collect();
        let signer = if parts.len() >= 3 {
            parts[2].to_string()
        } else {
            path.file_stem()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string()
        };
        Some(SignFilter {
            signer,
            signature: content.as_bytes().to_vec(),
        })
    }

    /// Load signing identity from environment.
    ///
    /// Resolution order:
    /// 1. `CONVERSATION_KEYS_PUBLIC` — path to a specific `.pub` file
    /// 2. `CONVERSATION_KEYS` — path to a directory containing `.pub` files
    /// 3. `~/.ssh` — default keys directory
    ///
    /// `CONVERSATION_KEYS_PRIVATE` is recognized but reserved for future
    /// cryptographic signing (GPG/SSH). `@sign` currently uses the public
    /// key as a structural witness, not a cryptographic signature.
    pub fn from_env() -> Option<Self> {
        if let Ok(pub_path) = std::env::var("CONVERSATION_KEYS_PUBLIC") {
            return Self::from_pub_file(std::path::Path::new(&pub_path));
        }
        Self::from_keys_dir(std::path::Path::new(&resolve_keys_dir()))
    }
}

impl Vector<Value, Value> for SignFilter {
    type Error = std::convert::Infallible;

    fn trace(&self, source: Value) -> Trace<Value, Self::Error> {
        let source_oid = source.content_oid();
        let mut envelope = serde_json::Map::new();
        envelope.insert("signer".into(), Value::String(self.signer.clone()));
        envelope.insert(
            "signature".into(),
            Value::String(hex::encode(&self.signature)),
        );
        envelope.insert("value".into(), source);
        envelope.insert("oid".into(), Value::String(source_oid.as_ref().to_string()));
        let result = Value::Object(envelope);
        let oid = result.content_oid();
        Trace::success(result, oid.into(), None)
    }
}

// ---------------------------------------------------------------------------
// Visibility — consent boundary for CI pipelines
// ---------------------------------------------------------------------------

/// Visibility level for CI pipeline output.
///
/// Read from `MIRROR_CI_VISIBILITY` env var. Defaults to `Public`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Visibility {
    Public,
    Protected,
    Private,
}

impl Visibility {
    /// Read visibility from `MIRROR_CI_VISIBILITY` env var.
    ///
    /// Case-insensitive. Unset or unrecognized values default to `Public`.
    pub fn from_env() -> Self {
        // TODO: implement — always returns Public for now
        Visibility::Public
    }
}

// ---------------------------------------------------------------------------
// CI key resolution helper
// ---------------------------------------------------------------------------

/// Resolve a CI key env var value to key content.
///
/// The value is either:
/// - A path to a key file (if a file exists at that path)
/// - Base64-encoded key content (otherwise)
fn resolve_ci_key(_value: &str) -> Option<String> {
    // TODO: implement
    None
}

// ---------------------------------------------------------------------------
// EncryptKey — argument union for @encrypt
// ---------------------------------------------------------------------------

/// Key selection for `@encrypt`. Arguments are a union type.
///
/// - `Public` — encrypt with public key from `CONVERSATION_KEYS` hierarchy
/// - `Private` — encrypt with private key from `CONVERSATION_KEYS_PRIVATE`
/// - `Key(path)` — explicit key file; `.pub` = public, else private
#[derive(Debug, Clone, PartialEq)]
pub enum EncryptKey {
    Public,
    Private,
    Key(PathBuf),
}

impl EncryptKey {
    /// Parse the raw parameter string from pipe syntax.
    ///
    /// `None` or `"public"` → `Public`; `"private"` → `Private`;
    /// `"key: /path/to/file"` → `Key(path)`.
    pub fn from_params(params: Option<&str>) -> Result<Self, ResolveError> {
        match params.map(|s| s.trim()) {
            None | Some("public") => Ok(EncryptKey::Public),
            Some("private") => Ok(EncryptKey::Private),
            Some(s) if s.starts_with("key:") => {
                let path = s.strip_prefix("key:").unwrap().trim();
                if path.is_empty() {
                    return Err(ResolveError {
                        message: "encrypt key: requires a path".into(),
                        span: None,
                        hints: vec!["@encrypt(key: /path/to/key)".into()],
                    });
                }
                Ok(EncryptKey::Key(PathBuf::from(path)))
            }
            Some(other) => Err(ResolveError {
                message: format!("unknown encrypt parameter: {}", other),
                span: None,
                hints: vec![
                    "@encrypt or @encrypt(public)".into(),
                    "@encrypt(private)".into(),
                    "@encrypt(key: /path/to/key)".into(),
                ],
            }),
        }
    }

    /// Resolve the key to SSH public key content for encryption.
    pub fn resolve(&self) -> Result<String, ResolveError> {
        match self {
            EncryptKey::Public => {
                // Same hierarchy as SignFilter: CONVERSATION_KEYS_PUBLIC → CONVERSATION_KEYS dir → ~/.ssh
                if let Ok(pub_path) = std::env::var("CONVERSATION_KEYS_PUBLIC") {
                    return read_key_file(&pub_path);
                }
                let keys_dir = resolve_keys_dir();
                read_first_pub_key(&keys_dir)
            }
            EncryptKey::Private => {
                if let Ok(priv_path) = std::env::var("CONVERSATION_KEYS_PRIVATE") {
                    return read_key_file(&priv_path);
                }
                let keys_dir = resolve_keys_dir();
                read_first_private_key(&keys_dir)
            }
            EncryptKey::Key(path) => read_key_file(&path.to_string_lossy()),
        }
    }
}

/// Resolve the keys directory from env.
fn resolve_keys_dir() -> String {
    std::env::var("CONVERSATION_KEYS").unwrap_or_else(|_| {
        std::env::var("HOME")
            .map(|h| format!("{}/.ssh", h))
            .unwrap_or_else(|_| "~/.ssh".into())
    })
}

/// Read a key file's content.
fn read_key_file(path: &str) -> Result<String, ResolveError> {
    std::fs::read_to_string(path)
        .map(|s| s.trim().to_string())
        .map_err(|_| ResolveError {
            message: format!("cannot read key file: {}", path),
            span: None,
            hints: vec![],
        })
}

/// Find the first `.pub` file in a directory (alphabetically).
fn read_first_pub_key(dir: &str) -> Result<String, ResolveError> {
    let path = std::path::Path::new(dir);
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|_| no_keys_error())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "pub")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    let entry = entries.first().ok_or_else(no_keys_error)?;
    read_key_file(&entry.path().to_string_lossy())
}

/// Find the first non-`.pub` key file in a directory (alphabetically).
/// Skips `known_hosts`, `config`, and `authorized_keys`.
fn read_first_private_key(dir: &str) -> Result<String, ResolveError> {
    let skip = ["known_hosts", "config", "authorized_keys", "environment"];
    let path = std::path::Path::new(dir);
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|_| no_keys_error())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            let ext_is_pub = p.extension().map(|ext| ext == "pub").unwrap_or(false);
            let name = p
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            !ext_is_pub && p.is_file() && !skip.contains(&name.as_str())
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());
    let entry = entries.first().ok_or_else(no_keys_error)?;
    read_key_file(&entry.path().to_string_lossy())
}

fn no_keys_error() -> ResolveError {
    ResolveError {
        message: "no encryption keys found (set CONVERSATION_KEYS or add keys to ~/.ssh)".into(),
        span: None,
        hints: vec![
            "CONVERSATION_KEYS=/path/to/keys".into(),
            "CONVERSATION_KEYS_PUBLIC=/path/to/key.pub".into(),
            "CONVERSATION_KEYS_PRIVATE=/path/to/key".into(),
        ],
    }
}

// ---------------------------------------------------------------------------
// EncryptFilter — age encryption as pipeline operator
// ---------------------------------------------------------------------------

/// Encrypts a value using age with an SSH key.
/// Output: `{ "encrypted": "<age ciphertext>", "oid": "<content hash of original>" }`
pub struct EncryptFilter {
    pub recipient_key: String,
}

impl EncryptFilter {
    pub fn new(recipient_key: impl Into<String>) -> Self {
        EncryptFilter {
            recipient_key: recipient_key.into(),
        }
    }

    /// Build from parsed filter params, resolving keys from env.
    pub fn from_params(params: Option<&str>) -> Result<Self, ResolveError> {
        let key = EncryptKey::from_params(params)?;
        let key_content = key.resolve()?;

        // For private keys, we need to derive the public key / recipient.
        // age::ssh can parse both. For now, store the key content as-is
        // and let the Vector impl handle recipient parsing.
        Ok(EncryptFilter {
            recipient_key: key_content,
        })
    }
}

/// Encrypt plaintext bytes to an SSH recipient key, returning base64-encoded ciphertext.
fn age_encrypt(recipient_key: &str, plaintext: &[u8]) -> Result<String, String> {
    let recipient = recipient_key
        .parse::<age::ssh::Recipient>()
        .map_err(|e| format!("invalid SSH public key for encryption: {:?}", e))?;

    // These operations cannot fail: with_recipients has a valid recipient,
    // and wrap_output/write_all/finish target a Vec<u8> (infallible I/O).
    let recipients: Vec<&dyn age::Recipient> = vec![&recipient];
    let encryptor = age::Encryptor::with_recipients(recipients.into_iter())
        .expect("single valid recipient should not fail");
    let mut encrypted = vec![];
    let mut writer = encryptor
        .wrap_output(&mut encrypted)
        .expect("wrap_output to Vec should not fail");
    writer
        .write_all(plaintext)
        .expect("write to Vec should not fail");
    writer.finish().expect("finish to Vec should not fail");

    use base64::Engine;
    Ok(base64::engine::general_purpose::STANDARD.encode(&encrypted))
}

impl Vector<Value, Value> for EncryptFilter {
    type Error = ResolveError;

    fn trace(&self, source: Value) -> Trace<Value, Self::Error> {
        let source_oid = source.content_oid();
        let plaintext = source.to_string();

        match age_encrypt(&self.recipient_key, plaintext.as_bytes()) {
            Ok(encoded) => {
                let mut envelope = serde_json::Map::new();
                envelope.insert("encrypted".into(), Value::String(encoded));
                envelope.insert("oid".into(), Value::String(source_oid.as_ref().to_string()));
                let result = Value::Object(envelope);
                let oid = result.content_oid();
                Trace::success(result, oid.into(), None)
            }
            Err(msg) => {
                let err = ResolveError {
                    message: msg,
                    span: None,
                    hints: vec![],
                };
                Trace::failure(err, source_oid.into(), None)
            }
        }
    }
}

// ---------------------------------------------------------------------------
// apply_filter — lookup by name, apply
// ---------------------------------------------------------------------------

/// Apply a named filter to a Value.
/// The filter name is the `@domain` from the pipe syntax, without the `@`.
/// Supports parameterized filters: `@encrypt(public)` → name="encrypt", params=Some("public").
pub fn apply_filter(raw: &str, value: Value) -> Result<Value, ResolveError> {
    let raw = raw.strip_prefix('@').unwrap_or(raw);
    let (name, params) = match raw.find('(') {
        Some(i) => (&raw[..i], Some(raw[i + 1..].trim_end_matches(')'))),
        None => (raw, None),
    };
    match name {
        "sha" => Ok(Sha.trace(value).into_result().unwrap()),
        "hash" => Ok(Hash.trace(value).into_result().unwrap()),
        "sign" => {
            let filter = SignFilter::from_env().ok_or_else(|| ResolveError {
                message: "no signing keys found (set CONVERSATION_KEYS or add keys to ~/.ssh)"
                    .into(),
                span: None,
                hints: vec!["CONVERSATION_KEYS=/path/to/keys".into()],
            })?;
            Ok(filter.trace(value).into_result().unwrap())
        }
        "encrypt" => {
            let filter = EncryptFilter::from_params(params)?;
            filter.trace(value).into_result()
        }
        _ => Err(ResolveError {
            message: format!("unknown filter @{}", name),
            span: None,
            hints: Vec::new(),
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // -- Sha --

    #[test]
    fn sha_hashes_string_value() {
        let input = Value::String("hello".into());
        let result = Sha.trace(input).unwrap();
        // SHA-512 of the JSON string representation: "\"hello\""
        let expected = crate::Oid::hash(b"\"hello\"");
        assert_eq!(result, Value::String(expected.as_ref().to_string()));
    }

    #[test]
    fn sha_hashes_object_value() {
        let mut map = serde_json::Map::new();
        map.insert("key".into(), Value::String("value".into()));
        let input = Value::Object(map);
        let result = Sha.trace(input).unwrap();
        assert!(result.is_string());
        assert_eq!(result.as_str().unwrap().len(), 128); // hex SHA-512
    }

    #[test]
    fn sha_same_input_same_output() {
        let a = Value::String("deterministic".into());
        let b = Value::String("deterministic".into());
        assert_eq!(Sha.trace(a).unwrap(), Sha.trace(b).unwrap());
    }

    #[test]
    fn sha_matches_content_addressed() {
        let input = Value::String("hello".into());
        let oid = input.content_oid();
        let hash = Sha.trace(input).unwrap();
        assert_eq!(hash.as_str().unwrap(), oid.as_ref());
    }

    // -- Hash --

    #[test]
    fn hash_defaults_to_sha() {
        let input = Value::String("test".into());
        let sha_result = Sha.trace(input.clone()).unwrap();
        let hash_result = Hash.trace(input).unwrap();
        assert_eq!(sha_result, hash_result);
    }

    // -- SignFilter --

    #[test]
    fn sign_wraps_value() {
        let filter = SignFilter::new("Reed", vec![0xDE, 0xAD]);
        let input = Value::String("hello".into());
        let result = filter.trace(input.clone()).unwrap();

        assert!(result.is_object());
        assert_eq!(result["signer"], "Reed");
        assert_eq!(result["signature"], "dead");
        assert_eq!(result["value"], input);
        assert!(result["oid"].is_string());
    }

    #[test]
    fn sign_different_signer_different_output() {
        let reed = SignFilter::new("Reed", vec![0xDE, 0xAD]);
        let alex = SignFilter::new("Alex", vec![0xCA, 0xFE]);
        let input = Value::String("same".into());
        let reed_result = reed.trace(input.clone()).unwrap();
        let alex_result = alex.trace(input).unwrap();
        assert_ne!(reed_result, alex_result);
    }

    #[test]
    fn sign_preserves_original_value() {
        let filter = SignFilter::new("Reed", vec![0xDE, 0xAD]);
        let input = Value::String("preserved".into());
        let result = filter.trace(input.clone()).unwrap();
        assert_eq!(result["value"], input);
    }

    // -- apply_filter --

    #[test]
    fn apply_filter_sha() {
        let input = Value::String("test".into());
        let result = apply_filter("sha", input.clone()).unwrap();
        assert_eq!(result, Sha.trace(input).unwrap());
    }

    #[test]
    fn apply_filter_hash() {
        let input = Value::String("test".into());
        let result = apply_filter("hash", input.clone()).unwrap();
        assert_eq!(result, Hash.trace(input).unwrap());
    }

    #[test]
    fn apply_filter_strips_at_prefix() {
        let input = Value::String("test".into());
        let result = apply_filter("@sha", input.clone()).unwrap();
        assert_eq!(result, Sha.trace(input).unwrap());
    }

    #[test]
    fn apply_filter_extracts_params_from_name() {
        // Parameterized unknown filter: name is extracted, params stripped
        let input = Value::String("test".into());
        let err = apply_filter("@bogus(foo)", input).unwrap_err();
        assert!(err.message.contains("unknown filter @bogus"));
        assert!(!err.message.contains("(foo)"));
    }

    #[test]
    fn filter_env_var_scenarios() {
        // ALL env-var-dependent filter tests in one function to avoid parallel race.
        // Both @sign and @encrypt touch CONVERSATION_KEYS / HOME.

        // === @sign scenarios ===

        // Sign 1: CONVERSATION_KEYS points to a directory
        let dir = tempfile::tempdir().unwrap();
        let key_path = dir.path().join("id_test.pub");
        std::fs::write(&key_path, "ssh-ed25519 AAAA reed@test\n").unwrap();

        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        std::env::set_var("CONVERSATION_KEYS", dir.path().as_os_str());
        let input = Value::String("signed".into());
        let result = apply_filter("sign", input).unwrap();

        assert!(result.is_object());
        assert_eq!(result["signer"], "reed@test");
        assert_eq!(result["value"], "signed");

        // Sign 2: CONVERSATION_KEYS_PUBLIC overrides CONVERSATION_KEYS
        let override_dir = tempfile::tempdir().unwrap();
        let override_path = override_dir.path().join("custom.pub");
        std::fs::write(&override_path, "ssh-ed25519 BBBB custom@override\n").unwrap();

        std::env::set_var("CONVERSATION_KEYS_PUBLIC", override_path.as_os_str());
        let input = Value::String("override".into());
        let result = apply_filter("sign", input).unwrap();
        assert_eq!(result["signer"], "custom@override");

        // Sign 3: Falls back to HOME/.ssh
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS");
        let home_dir = tempfile::tempdir().unwrap();
        let ssh_dir = home_dir.path().join(".ssh");
        std::fs::create_dir(&ssh_dir).unwrap();
        std::fs::write(
            ssh_dir.join("id_ed25519.pub"),
            "ssh-ed25519 CCCC home@key\n",
        )
        .unwrap();
        std::env::set_var("HOME", home_dir.path().as_os_str());
        let input = Value::String("home".into());
        let result = apply_filter("sign", input).unwrap();
        assert_eq!(result["signer"], "home@key");

        // Sign 4: No keys anywhere → error
        let empty_home = tempfile::tempdir().unwrap();
        std::fs::create_dir(empty_home.path().join(".ssh")).unwrap();
        std::env::set_var("HOME", empty_home.path().as_os_str());
        let input = Value::String("fail".into());
        let err = apply_filter("sign", input).unwrap_err();
        assert!(err.message.contains("no signing keys found"));

        // Sign 5: HOME unset falls back to "~/.ssh"
        std::env::remove_var("HOME");
        assert!(SignFilter::from_env().is_none());

        // === @encrypt scenarios ===

        // Encrypt 1: Public key from CONVERSATION_KEYS directory
        let edir = tempfile::tempdir().unwrap();
        std::fs::write(edir.path().join("id_ed25519.pub"), TEST_SSH_PUB).unwrap();
        std::fs::write(edir.path().join("id_ed25519"), "PRIVATE_KEY_CONTENT").unwrap();

        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        std::env::set_var("CONVERSATION_KEYS", edir.path().as_os_str());

        let key_content = EncryptKey::Public.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);

        // Encrypt 2: Private key from CONVERSATION_KEYS directory (skips .pub)
        let key_content = EncryptKey::Private.resolve().unwrap();
        assert_eq!(key_content, "PRIVATE_KEY_CONTENT");

        // Encrypt 3: CONVERSATION_KEYS_PUBLIC overrides for Public
        let eoverride = tempfile::tempdir().unwrap();
        let pub_override = eoverride.path().join("custom.pub");
        std::fs::write(&pub_override, TEST_SSH_PUB).unwrap();
        std::env::set_var("CONVERSATION_KEYS_PUBLIC", pub_override.as_os_str());

        let key_content = EncryptKey::Public.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);

        // Encrypt 4: CONVERSATION_KEYS_PRIVATE overrides for Private
        let priv_override = eoverride.path().join("custom_priv");
        std::fs::write(&priv_override, "OVERRIDE_PRIVATE").unwrap();
        std::env::set_var("CONVERSATION_KEYS_PRIVATE", priv_override.as_os_str());

        let key_content = EncryptKey::Private.resolve().unwrap();
        assert_eq!(key_content, "OVERRIDE_PRIVATE");

        // Encrypt 5: Empty dir → no encryption keys error
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        let empty = tempfile::tempdir().unwrap();
        std::env::set_var("CONVERSATION_KEYS", empty.path().as_os_str());

        let err = EncryptKey::Public.resolve().unwrap_err();
        assert!(err.message.contains("no encryption keys found"));

        // Encrypt 6: apply_filter encrypt with public key from dir
        std::env::set_var("CONVERSATION_KEYS", edir.path().as_os_str());
        let input = Value::String("secret".into());
        let result = apply_filter("@encrypt(public)", input).unwrap();
        assert!(result["encrypted"].is_string());
        assert!(result["oid"].is_string());

        // Encrypt 7: Fallback to HOME/.ssh
        std::env::remove_var("CONVERSATION_KEYS");
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        let ehome = tempfile::tempdir().unwrap();
        let essh = ehome.path().join(".ssh");
        std::fs::create_dir(&essh).unwrap();
        std::fs::write(essh.join("id_ed25519.pub"), TEST_SSH_PUB).unwrap();
        std::env::set_var("HOME", ehome.path().as_os_str());

        let key_content = EncryptKey::Public.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);

        // Encrypt 8: HOME unset → "~/.ssh" literal
        std::env::remove_var("HOME");
        let err = EncryptKey::Public.resolve().unwrap_err();
        assert!(err.message.contains("no encryption keys found"));

        // Clean up
        std::env::remove_var("CONVERSATION_KEYS");
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
    }

    #[test]
    fn apply_filter_unknown_domain_errors() {
        let input = Value::String("test".into());
        let err = apply_filter("unknown", input).unwrap_err();
        assert!(err.message.contains("unknown filter"));
    }

    // -- EncryptKey --

    #[test]
    fn encrypt_key_default_is_public() {
        assert_eq!(EncryptKey::from_params(None).unwrap(), EncryptKey::Public);
    }

    #[test]
    fn encrypt_key_public_explicit() {
        assert_eq!(
            EncryptKey::from_params(Some("public")).unwrap(),
            EncryptKey::Public
        );
    }

    #[test]
    fn encrypt_key_private() {
        assert_eq!(
            EncryptKey::from_params(Some("private")).unwrap(),
            EncryptKey::Private
        );
    }

    #[test]
    fn encrypt_key_path() {
        assert_eq!(
            EncryptKey::from_params(Some("key: /path/to/key.pub")).unwrap(),
            EncryptKey::Key(PathBuf::from("/path/to/key.pub"))
        );
    }

    #[test]
    fn encrypt_key_empty_path_errors() {
        let err = EncryptKey::from_params(Some("key:")).unwrap_err();
        assert!(err.message.contains("requires a path"));
    }

    #[test]
    fn encrypt_key_unknown_param_errors() {
        let err = EncryptKey::from_params(Some("bogus")).unwrap_err();
        assert!(err.message.contains("unknown encrypt parameter"));
    }

    // -- EncryptFilter --

    const TEST_SSH_PUB: &str =
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIPHT8r0b+ggu9Uh+XvCtivXnStC8RdBDCS5kwEf7j4zF test@encrypt";

    #[test]
    fn encrypt_produces_envelope() {
        let filter = EncryptFilter::new(TEST_SSH_PUB);
        let input = Value::String("hello".into());
        let result = filter.trace(input).into_result().unwrap();
        assert!(result.is_object());
        assert!(result["encrypted"].is_string());
        assert!(result["oid"].is_string());
    }

    #[test]
    fn encrypt_oid_matches_original() {
        let filter = EncryptFilter::new(TEST_SSH_PUB);
        let input = Value::String("hello".into());
        let oid = input.content_oid();
        let result = filter.trace(input).into_result().unwrap();
        assert_eq!(result["oid"].as_str().unwrap(), oid.as_ref());
    }

    #[test]
    fn encrypt_different_input_different_output() {
        let filter = EncryptFilter::new(TEST_SSH_PUB);
        let a = filter
            .trace(Value::String("alpha".into()))
            .into_result()
            .unwrap();
        let b = filter
            .trace(Value::String("beta".into()))
            .into_result()
            .unwrap();
        assert_ne!(a["oid"], b["oid"]);
    }

    #[test]
    fn encrypt_invalid_key_errors() {
        let filter = EncryptFilter::new("not-a-key");
        let input = Value::String("hello".into());
        let err = filter.trace(input).into_result().unwrap_err();
        assert!(err.message.contains("invalid SSH public key"));
    }

    #[test]
    fn encrypt_resolve_key_path() {
        let dir = tempfile::tempdir().unwrap();
        let key_path = dir.path().join("my.pub");
        std::fs::write(&key_path, TEST_SSH_PUB).unwrap();

        let key = EncryptKey::Key(key_path);
        let key_content = key.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);
    }

    #[test]
    fn encrypt_resolve_missing_key_errors() {
        let key = EncryptKey::Key(PathBuf::from("/nonexistent/key.pub"));
        let err = key.resolve().unwrap_err();
        assert!(err.message.contains("cannot read key file"));
    }

    // -- read_first_private_key --

    #[test]
    fn private_key_missing_dir_errors() {
        let err = read_first_private_key("/nonexistent").unwrap_err();
        assert!(err.message.contains("no encryption keys found"));
    }

    #[test]
    fn private_key_empty_dir_errors() {
        let dir = tempfile::tempdir().unwrap();
        let err = read_first_private_key(&dir.path().to_string_lossy()).unwrap_err();
        assert!(err.message.contains("no encryption keys found"));
    }

    #[test]
    fn private_key_picks_first_alphabetically() {
        let dir = tempfile::tempdir().unwrap();
        // Add .pub files (should be skipped) and private keys
        std::fs::write(dir.path().join("id_ed25519.pub"), "pub").unwrap();
        std::fs::write(dir.path().join("b_key"), "SECOND_PRIVATE").unwrap();
        std::fs::write(dir.path().join("a_key"), "FIRST_PRIVATE").unwrap();

        let content = read_first_private_key(&dir.path().to_string_lossy()).unwrap();
        assert_eq!(content, "FIRST_PRIVATE");
    }

    #[test]
    fn private_key_skips_config_files() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("known_hosts"), "skip").unwrap();
        std::fs::write(dir.path().join("config"), "skip").unwrap();
        std::fs::write(dir.path().join("authorized_keys"), "skip").unwrap();
        std::fs::write(dir.path().join("id_ed25519"), "THE_KEY").unwrap();

        let content = read_first_private_key(&dir.path().to_string_lossy()).unwrap();
        assert_eq!(content, "THE_KEY");
    }

    // -- read_first_pub_key --

    #[test]
    fn pub_key_picks_first_alphabetically() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("b_key.pub"), "ssh-ed25519 BBB second@key").unwrap();
        std::fs::write(dir.path().join("a_key.pub"), "ssh-ed25519 AAA first@key").unwrap();

        let content = read_first_pub_key(&dir.path().to_string_lossy()).unwrap();
        assert_eq!(content, "ssh-ed25519 AAA first@key");
    }

    // -- from_keys_dir --

    #[test]
    fn from_keys_dir_reads_pub_file() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(
            dir.path().join("id_ed25519.pub"),
            "ssh-ed25519 AAAA alex@machine\n",
        )
        .unwrap();

        let filter = SignFilter::from_keys_dir(dir.path()).unwrap();
        assert_eq!(filter.signer, "alex@machine");
        assert_eq!(filter.signature, b"ssh-ed25519 AAAA alex@machine");
    }

    #[test]
    fn from_keys_dir_uses_filename_when_no_comment() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("mykey.pub"), "ssh-rsa AAAA\n").unwrap();

        let filter = SignFilter::from_keys_dir(dir.path()).unwrap();
        assert_eq!(filter.signer, "mykey");
    }

    #[test]
    fn from_keys_dir_returns_none_for_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        assert!(SignFilter::from_keys_dir(dir.path()).is_none());
    }

    #[test]
    fn from_keys_dir_returns_none_for_missing_dir() {
        assert!(SignFilter::from_keys_dir(std::path::Path::new("/nonexistent")).is_none());
    }

    #[test]
    fn from_pub_file_returns_none_for_missing_file() {
        assert!(SignFilter::from_pub_file(std::path::Path::new("/nonexistent/key.pub")).is_none());
    }

    #[test]
    fn from_keys_dir_picks_first_alphabetically() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("b_key.pub"), "ssh-ed25519 BBB second@key\n").unwrap();
        std::fs::write(dir.path().join("a_key.pub"), "ssh-ed25519 AAA first@key\n").unwrap();

        let filter = SignFilter::from_keys_dir(dir.path()).unwrap();
        assert_eq!(filter.signer, "first@key");
    }

    // -- Visibility --

    #[test]
    fn visibility_default_is_public() {
        std::env::remove_var("MIRROR_CI_VISIBILITY");
        assert_eq!(Visibility::from_env(), Visibility::Public);
    }

    #[test]
    fn visibility_from_env_public() {
        std::env::set_var("MIRROR_CI_VISIBILITY", "public");
        assert_eq!(Visibility::from_env(), Visibility::Public);
        std::env::remove_var("MIRROR_CI_VISIBILITY");
    }

    #[test]
    fn visibility_from_env_protected() {
        std::env::set_var("MIRROR_CI_VISIBILITY", "protected");
        assert_eq!(Visibility::from_env(), Visibility::Protected);
        std::env::remove_var("MIRROR_CI_VISIBILITY");
    }

    #[test]
    fn visibility_from_env_private() {
        std::env::set_var("MIRROR_CI_VISIBILITY", "private");
        assert_eq!(Visibility::from_env(), Visibility::Private);
        std::env::remove_var("MIRROR_CI_VISIBILITY");
    }

    #[test]
    fn visibility_from_env_case_insensitive() {
        std::env::set_var("MIRROR_CI_VISIBILITY", "Protected");
        assert_eq!(Visibility::from_env(), Visibility::Protected);
        std::env::set_var("MIRROR_CI_VISIBILITY", "PRIVATE");
        assert_eq!(Visibility::from_env(), Visibility::Private);
        std::env::remove_var("MIRROR_CI_VISIBILITY");
    }

    #[test]
    fn visibility_from_env_unknown_defaults_public() {
        std::env::set_var("MIRROR_CI_VISIBILITY", "bogus");
        assert_eq!(Visibility::from_env(), Visibility::Public);
        std::env::remove_var("MIRROR_CI_VISIBILITY");
    }

    // -- MIRROR_CI_SIGN_KEY / MIRROR_CI_ENCRYPT_KEY --

    #[test]
    fn mirror_ci_env_var_scenarios() {
        // ALL MIRROR_CI env-var tests in one function to avoid parallel race.

        // === MIRROR_CI_SIGN_KEY ===

        // CI Sign 1: MIRROR_CI_SIGN_KEY as file path takes priority
        let ci_dir = tempfile::tempdir().unwrap();
        let ci_key = ci_dir.path().join("ci.pub");
        std::fs::write(&ci_key, "ssh-ed25519 CICI ci@runner\n").unwrap();

        // Also set CONVERSATION_KEYS_PUBLIC to prove CI takes priority
        let fallback_dir = tempfile::tempdir().unwrap();
        let fallback_key = fallback_dir.path().join("fallback.pub");
        std::fs::write(&fallback_key, "ssh-ed25519 FALL fallback@key\n").unwrap();
        std::env::set_var("CONVERSATION_KEYS_PUBLIC", fallback_key.as_os_str());

        std::env::set_var("MIRROR_CI_SIGN_KEY", ci_key.to_str().unwrap());
        let filter = SignFilter::from_env().unwrap();
        assert_eq!(filter.signer, "ci@runner");

        // CI Sign 2: MIRROR_CI_SIGN_KEY as inline base64 content
        use base64::Engine;
        let key_content = "ssh-ed25519 AAAA inline@ci";
        let encoded = base64::engine::general_purpose::STANDARD.encode(key_content);
        std::env::set_var("MIRROR_CI_SIGN_KEY", &encoded);
        let filter = SignFilter::from_env().unwrap();
        assert_eq!(filter.signer, "inline@ci");

        // CI Sign 3: Unset MIRROR_CI_SIGN_KEY, falls back to CONVERSATION_KEYS_PUBLIC
        std::env::remove_var("MIRROR_CI_SIGN_KEY");
        let filter = SignFilter::from_env().unwrap();
        assert_eq!(filter.signer, "fallback@key");

        // === MIRROR_CI_ENCRYPT_KEY ===

        // CI Encrypt 1: MIRROR_CI_ENCRYPT_KEY as file path takes priority
        let ci_enc_dir = tempfile::tempdir().unwrap();
        let ci_enc_key = ci_enc_dir.path().join("ci_enc.pub");
        std::fs::write(&ci_enc_key, TEST_SSH_PUB).unwrap();

        std::env::set_var("MIRROR_CI_ENCRYPT_KEY", ci_enc_key.to_str().unwrap());
        let key_content = EncryptKey::Public.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);

        // CI Encrypt 2: MIRROR_CI_ENCRYPT_KEY as inline base64 content
        let encoded = base64::engine::general_purpose::STANDARD.encode(TEST_SSH_PUB);
        std::env::set_var("MIRROR_CI_ENCRYPT_KEY", &encoded);
        let key_content = EncryptKey::Public.resolve().unwrap();
        assert_eq!(key_content, TEST_SSH_PUB);

        // CI Encrypt 3: Unset MIRROR_CI_ENCRYPT_KEY, falls back to CONVERSATION_KEYS_PUBLIC
        std::env::remove_var("MIRROR_CI_ENCRYPT_KEY");
        // CONVERSATION_KEYS_PUBLIC is still set from sign tests
        let key_content = EncryptKey::Public.resolve().unwrap();
        // Should read from fallback_key (CONVERSATION_KEYS_PUBLIC)
        assert_eq!(key_content, "ssh-ed25519 FALL fallback@key");

        // Clean up
        std::env::remove_var("MIRROR_CI_SIGN_KEY");
        std::env::remove_var("MIRROR_CI_ENCRYPT_KEY");
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        std::env::remove_var("CONVERSATION_KEYS");
    }
}
