//! Ed25519 cryptographic signing and verification.
//!
//! Upgrades the structural witness (`@sign` filter) with real cryptographic
//! signatures using the SSH signing protocol (`sshsig`). The `@sign` pipeline
//! filter remains unchanged for backward compatibility. This module provides:
//!
//! - `sign_oid()` — sign a content-addressed OID with a private key
//! - `verify_oid()` — verify a signature against an OID with a public key
//! - `SigningKey` — resolved private key for signing
//! - CLI integration: `mirror compile --sign` and `mirror verify`

#[cfg(feature = "git")]
use ssh_key::{HashAlg, PrivateKey, PublicKey, SshSig};

use crate::Oid;

/// The namespace used for mirror signatures (sshsig protocol requires one).
const MIRROR_NAMESPACE: &str = "mirror";

/// Error type for signing/verification operations.
#[derive(Debug, Clone)]
pub struct SignError {
    pub message: String,
}

impl std::fmt::Display for SignError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for SignError {}

// ---------------------------------------------------------------------------
// Signing — produce an Ed25519 signature over a content OID
// ---------------------------------------------------------------------------

/// Sign a content-addressed OID with an Ed25519 private key.
///
/// Returns the signature as PEM-encoded `SshSig` (the standard `ssh-keygen -Y sign` format).
#[cfg(feature = "git")]
pub fn sign_oid(private_key: &PrivateKey, oid: &Oid) -> Result<String, SignError> {
    let msg = oid.as_ref().as_bytes();
    let sig = private_key
        .sign(MIRROR_NAMESPACE, HashAlg::Sha512, msg)
        .map_err(|e| SignError {
            message: format!("signing failed: {}", e),
        })?;
    sig.to_pem(ssh_key::LineEnding::LF).map_err(|e| SignError {
        message: format!("PEM encoding failed: {}", e),
    })
}

/// Verify a PEM-encoded signature against a content OID using a public key.
///
/// Returns `Ok(())` if valid, `Err(SignError)` if invalid or tampered.
#[cfg(feature = "git")]
pub fn verify_oid(public_key: &PublicKey, oid: &Oid, signature_pem: &str) -> Result<(), SignError> {
    let sig = SshSig::from_pem(signature_pem).map_err(|e| SignError {
        message: format!("invalid signature PEM: {}", e),
    })?;
    let msg = oid.as_ref().as_bytes();
    public_key
        .verify(MIRROR_NAMESPACE, msg, &sig)
        .map_err(|e| SignError {
            message: format!("verification failed: {}", e),
        })
}

// ---------------------------------------------------------------------------
// Key loading — resolve private/public keys from env or filesystem
// ---------------------------------------------------------------------------

/// Load a private key from a PEM string (OpenSSH format).
#[cfg(feature = "git")]
pub fn load_private_key(content: &str) -> Result<PrivateKey, SignError> {
    PrivateKey::from_openssh(content.trim()).map_err(|e| SignError {
        message: format!("invalid private key: {}", e),
    })
}

/// Load a public key from an OpenSSH authorized_keys line (e.g., `ssh-ed25519 AAAA... comment`).
#[cfg(feature = "git")]
pub fn load_public_key(content: &str) -> Result<PublicKey, SignError> {
    content.trim().parse::<PublicKey>().map_err(|e| SignError {
        message: format!("invalid public key: {}", e),
    })
}

/// Resolve a private key for signing from environment variables.
///
/// Resolution order:
/// 1. `MIRROR_CI_SIGN_KEY` — if it contains a private key (file path or base64)
/// 2. `CONVERSATION_KEYS_PRIVATE` — path to a specific private key file
/// 3. First private key file in `CONVERSATION_KEYS` directory
/// 4. First private key file in `~/.ssh`
#[cfg(feature = "git")]
pub fn resolve_private_key() -> Result<PrivateKey, SignError> {
    // 1. MIRROR_CI_SIGN_KEY
    if let Ok(val) = std::env::var("MIRROR_CI_SIGN_KEY") {
        if let Some(content) = resolve_key_content(&val) {
            if let Ok(key) = load_private_key(&content) {
                return Ok(key);
            }
        }
    }

    // 2. CONVERSATION_KEYS_PRIVATE
    if let Ok(path) = std::env::var("CONVERSATION_KEYS_PRIVATE") {
        let content = std::fs::read_to_string(&path).map_err(|_| SignError {
            message: format!("cannot read private key: {}", path),
        })?;
        return load_private_key(&content);
    }

    // 3. First private key in CONVERSATION_KEYS dir
    // 4. First private key in ~/.ssh
    let keys_dir = std::env::var("CONVERSATION_KEYS").unwrap_or_else(|_| {
        std::env::var("HOME")
            .map(|h| format!("{}/.ssh", h))
            .unwrap_or_else(|_| "~/.ssh".into())
    });

    find_first_private_key(&keys_dir)
}

/// Resolve a public key for verification from environment variables.
///
/// Resolution order:
/// 1. `MIRROR_CI_SIGN_KEY` — if it contains a public key (file path or base64)
/// 2. `CONVERSATION_KEYS_PUBLIC` — path to a specific public key file
/// 3. First `.pub` file in `CONVERSATION_KEYS` directory
/// 4. First `.pub` file in `~/.ssh`
#[cfg(feature = "git")]
pub fn resolve_public_key() -> Result<PublicKey, SignError> {
    // 1. MIRROR_CI_SIGN_KEY (might be a public key)
    if let Ok(val) = std::env::var("MIRROR_CI_SIGN_KEY") {
        if let Some(content) = resolve_key_content(&val) {
            if let Ok(key) = load_public_key(&content) {
                return Ok(key);
            }
        }
    }

    // 2. CONVERSATION_KEYS_PUBLIC
    if let Ok(path) = std::env::var("CONVERSATION_KEYS_PUBLIC") {
        let content = std::fs::read_to_string(&path).map_err(|_| SignError {
            message: format!("cannot read public key: {}", path),
        })?;
        return load_public_key(&content);
    }

    // 3/4. First .pub key in dir
    let keys_dir = std::env::var("CONVERSATION_KEYS").unwrap_or_else(|_| {
        std::env::var("HOME")
            .map(|h| format!("{}/.ssh", h))
            .unwrap_or_else(|_| "~/.ssh".into())
    });

    find_first_pub_key(&keys_dir)
}

/// Resolve key content from a value that is either a file path or base64 content.
#[cfg(feature = "git")]
fn resolve_key_content(value: &str) -> Option<String> {
    let path = std::path::Path::new(value);
    if path.exists() {
        return std::fs::read_to_string(path).ok();
    }
    use base64::Engine;
    base64::engine::general_purpose::STANDARD
        .decode(value)
        .ok()
        .and_then(|bytes| String::from_utf8(bytes).ok())
}

/// Find the first private key file in a directory (alphabetically).
/// Skips `.pub` files and known non-key files.
#[cfg(feature = "git")]
fn find_first_private_key(dir: &str) -> Result<PrivateKey, SignError> {
    let skip = ["known_hosts", "config", "authorized_keys", "environment"];
    let path = std::path::Path::new(dir);
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|_| no_sign_keys_error())?
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

    for entry in entries {
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if let Ok(key) = load_private_key(&content) {
            return Ok(key);
        }
    }
    Err(no_sign_keys_error())
}

/// Find the first `.pub` key file in a directory and parse as PublicKey.
#[cfg(feature = "git")]
fn find_first_pub_key(dir: &str) -> Result<PublicKey, SignError> {
    let path = std::path::Path::new(dir);
    let mut entries: Vec<_> = std::fs::read_dir(path)
        .map_err(|_| no_sign_keys_error())?
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext == "pub")
                .unwrap_or(false)
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    for entry in entries {
        let content = match std::fs::read_to_string(entry.path()) {
            Ok(c) => c,
            Err(_) => continue,
        };
        if let Ok(key) = load_public_key(&content) {
            return Ok(key);
        }
    }
    Err(no_sign_keys_error())
}

#[cfg(feature = "git")]
fn no_sign_keys_error() -> SignError {
    SignError {
        message: "no signing keys found (set CONVERSATION_KEYS_PRIVATE or add keys to ~/.ssh)"
            .into(),
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
#[cfg(feature = "git")]
mod tests {
    use super::*;

    /// Generate a fresh Ed25519 keypair for testing.
    fn test_keypair() -> PrivateKey {
        PrivateKey::random(&mut ssh_key::rand_core::OsRng, ssh_key::Algorithm::Ed25519)
            .expect("keygen should succeed")
    }

    /// Write a private key to a temp file, returning the path content.
    fn write_test_key(dir: &std::path::Path, name: &str, key: &PrivateKey) -> std::path::PathBuf {
        let path = dir.join(name);
        let pem = key
            .to_openssh(ssh_key::LineEnding::LF)
            .expect("to_openssh should succeed");
        let pem_bytes: &[u8] = pem.as_ref();
        std::fs::write(&path, pem_bytes).unwrap();
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600)).unwrap();
        }
        path
    }

    // -- sign_oid / verify_oid --

    #[test]
    fn sign_and_verify_round_trip() {
        let key = test_keypair();
        let oid = Oid::hash(b"test content");

        let sig_pem = sign_oid(&key, &oid).expect("signing should succeed");
        assert!(sig_pem.contains("BEGIN SSH SIGNATURE"));

        let pub_key = key.public_key().clone();
        verify_oid(&pub_key, &oid, &sig_pem).expect("verification should succeed");
    }

    #[test]
    fn verify_fails_on_tampered_oid() {
        let key = test_keypair();
        let oid = Oid::hash(b"original");
        let tampered_oid = Oid::hash(b"tampered");

        let sig_pem = sign_oid(&key, &oid).expect("signing should succeed");

        let result = verify_oid(&key.public_key().clone(), &tampered_oid, &sig_pem);
        assert!(result.is_err(), "tampered OID must fail verification");
        assert!(
            result.unwrap_err().message.contains("verification failed"),
            "error should indicate verification failure"
        );
    }

    #[test]
    fn verify_fails_with_wrong_key() {
        let signer = test_keypair();
        let other = test_keypair();
        let oid = Oid::hash(b"content");

        let sig_pem = sign_oid(&signer, &oid).expect("signing should succeed");

        let result = verify_oid(&other.public_key().clone(), &oid, &sig_pem);
        assert!(result.is_err(), "wrong key must fail verification");
    }

    #[test]
    fn verify_fails_on_invalid_pem() {
        let key = test_keypair();
        let oid = Oid::hash(b"content");

        let result = verify_oid(&key.public_key().clone(), &oid, "not a pem");
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("invalid signature PEM"));
    }

    #[test]
    fn sign_error_display() {
        let e = SignError {
            message: "test error".into(),
        };
        assert_eq!(format!("{}", e), "test error");
        // Also test Error trait
        let _: &dyn std::error::Error = &e;
    }

    // -- load_private_key / load_public_key --

    #[test]
    fn load_private_key_from_openssh() {
        let key = test_keypair();
        let pem = key.to_openssh(ssh_key::LineEnding::LF).expect("to_openssh");
        let loaded = load_private_key(pem.as_ref()).expect("should load");
        assert_eq!(loaded.public_key().to_bytes(), key.public_key().to_bytes());
    }

    #[test]
    fn load_private_key_invalid() {
        let result = load_private_key("not a key");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("invalid private key"));
    }

    #[test]
    fn load_public_key_from_authorized_keys_line() {
        let key = test_keypair();
        let pub_key = key.public_key();
        let line = pub_key.to_openssh().expect("to_openssh");
        let loaded = load_public_key(&line).expect("should load");
        assert_eq!(loaded.to_bytes(), pub_key.to_bytes());
    }

    #[test]
    fn load_public_key_invalid() {
        let result = load_public_key("not a key");
        assert!(result.is_err());
        assert!(result.unwrap_err().message.contains("invalid public key"));
    }

    // -- resolve key from env --

    #[test]
    fn resolve_keys_from_env() {
        // All env-dependent tests in one function to avoid parallel races.
        let dir = tempfile::tempdir().unwrap();
        let key = test_keypair();

        // Write private key
        let priv_path = write_test_key(dir.path(), "id_ed25519", &key);

        // Write public key
        let pub_line = key.public_key().to_openssh().unwrap();
        let pub_path = dir.path().join("id_ed25519.pub");
        std::fs::write(&pub_path, &pub_line).unwrap();

        // Clear all relevant env vars
        std::env::remove_var("MIRROR_CI_SIGN_KEY");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS");

        // 1. CONVERSATION_KEYS_PRIVATE
        std::env::set_var("CONVERSATION_KEYS_PRIVATE", priv_path.as_os_str());
        let resolved =
            resolve_private_key().expect("should resolve from CONVERSATION_KEYS_PRIVATE");
        assert_eq!(
            resolved.public_key().to_bytes(),
            key.public_key().to_bytes()
        );
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");

        // 2. CONVERSATION_KEYS_PUBLIC
        std::env::set_var("CONVERSATION_KEYS_PUBLIC", pub_path.as_os_str());
        let resolved = resolve_public_key().expect("should resolve from CONVERSATION_KEYS_PUBLIC");
        assert_eq!(resolved.to_bytes(), key.public_key().to_bytes());
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");

        // 3. CONVERSATION_KEYS dir (auto-discovery)
        std::env::set_var("CONVERSATION_KEYS", dir.path().as_os_str());
        let resolved = resolve_private_key().expect("should resolve from CONVERSATION_KEYS dir");
        assert_eq!(
            resolved.public_key().to_bytes(),
            key.public_key().to_bytes()
        );

        let resolved = resolve_public_key().expect("should resolve pub from CONVERSATION_KEYS dir");
        assert_eq!(resolved.to_bytes(), key.public_key().to_bytes());
        std::env::remove_var("CONVERSATION_KEYS");

        // 4. MIRROR_CI_SIGN_KEY as file path (private key)
        std::env::set_var("MIRROR_CI_SIGN_KEY", priv_path.to_str().unwrap());
        let resolved = resolve_private_key().expect("should resolve from MIRROR_CI_SIGN_KEY file");
        assert_eq!(
            resolved.public_key().to_bytes(),
            key.public_key().to_bytes()
        );
        std::env::remove_var("MIRROR_CI_SIGN_KEY");

        // 5. No keys → error
        std::env::remove_var("CONVERSATION_KEYS");
        std::env::remove_var("HOME");
        let result = resolve_private_key();
        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .message
            .contains("no signing keys found"));

        // Clean up
        std::env::remove_var("MIRROR_CI_SIGN_KEY");
        std::env::remove_var("CONVERSATION_KEYS_PRIVATE");
        std::env::remove_var("CONVERSATION_KEYS_PUBLIC");
        std::env::remove_var("CONVERSATION_KEYS");
    }

    // -- end-to-end: compile, sign, tamper, verify --

    #[test]
    fn end_to_end_compile_sign_verify() {
        let key = test_keypair();

        // "Compile" some content and get its OID
        let content = b"grammar @test { type greeting }";
        let oid = Oid::hash(content);

        // Sign
        let sig_pem = sign_oid(&key, &oid).expect("sign should succeed");

        // Write .shatter and .shatter.sig
        let dir = tempfile::tempdir().unwrap();
        let shatter_path = dir.path().join("test.shatter");
        let sig_path = dir.path().join("test.shatter.sig");
        std::fs::write(&shatter_path, content).unwrap();
        std::fs::write(&sig_path, &sig_pem).unwrap();

        // Verify succeeds
        let loaded_sig = std::fs::read_to_string(&sig_path).unwrap();
        verify_oid(&key.public_key().clone(), &oid, &loaded_sig)
            .expect("verification should succeed for untampered content");

        // Tamper with the shatter
        std::fs::write(&shatter_path, b"grammar @test { type TAMPERED }").unwrap();
        let tampered_oid = Oid::hash(b"grammar @test { type TAMPERED }");

        // Verify fails (tampered OID != signed OID)
        let result = verify_oid(&key.public_key().clone(), &tampered_oid, &loaded_sig);
        assert!(result.is_err(), "tampered content must fail verification");
    }

    // -- resolve_key_content --

    #[test]
    fn resolve_key_content_from_file() {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("test.key");
        std::fs::write(&path, "key content").unwrap();
        let content = resolve_key_content(path.to_str().unwrap());
        assert_eq!(content.unwrap(), "key content");
    }

    #[test]
    fn resolve_key_content_from_base64() {
        use base64::Engine;
        let encoded = base64::engine::general_purpose::STANDARD.encode("key content");
        let content = resolve_key_content(&encoded);
        assert_eq!(content.unwrap(), "key content");
    }

    #[test]
    fn resolve_key_content_invalid() {
        // Not a file, not valid base64
        let content = resolve_key_content("!!!not-base64-or-file!!!");
        assert!(content.is_none());
    }

    // -- find_first_private_key edge cases --

    #[test]
    fn find_first_private_key_skips_non_keys() {
        let dir = tempfile::tempdir().unwrap();
        // Write non-key files that should be skipped
        std::fs::write(dir.path().join("known_hosts"), "skip").unwrap();
        std::fs::write(dir.path().join("config"), "skip").unwrap();
        // No valid private keys → error
        let result = find_first_private_key(&dir.path().to_string_lossy());
        assert!(result.is_err());
    }

    #[test]
    fn find_first_private_key_nonexistent_dir() {
        let result = find_first_private_key("/nonexistent/dir");
        assert!(result.is_err());
    }

    #[test]
    fn find_first_pub_key_nonexistent_dir() {
        let result = find_first_pub_key("/nonexistent/dir");
        assert!(result.is_err());
    }

    #[test]
    fn find_first_pub_key_empty_dir() {
        let dir = tempfile::tempdir().unwrap();
        let result = find_first_pub_key(&dir.path().to_string_lossy());
        assert!(result.is_err());
    }

    #[test]
    fn find_first_pub_key_invalid_content() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::write(dir.path().join("bad.pub"), "not a real key").unwrap();
        let result = find_first_pub_key(&dir.path().to_string_lossy());
        assert!(result.is_err());
    }
}
