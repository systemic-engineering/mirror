//! Git-integrated mirror store.
//!
//! Uses `NamespacedGitStore` from `fragmentation-git` to store
//! mirror crystals inside `.git/mirror/`.

use std::path::Path;

use fragmentation::fragment::Fractal;
use fragmentation_git::namespaced::{NamespacedGitStore, NamespacedStoreError};

/// Mirror's git-integrated store.
///
/// Wraps a `NamespacedGitStore` with namespace `"mirror"`, providing
/// mirror-specific operations on top of the content-addressed store.
pub struct MirrorGitStore {
    inner: NamespacedGitStore,
}

impl MirrorGitStore {
    /// Open or create the mirror store inside a git repository.
    /// Creates `.git/mirror/` if it doesn't exist.
    pub fn open(repo_path: &Path) -> Result<Self, NamespacedStoreError> {
        let inner = NamespacedGitStore::open(repo_path, "mirror")?;
        Ok(MirrorGitStore { inner })
    }

    /// The store directory path.
    pub fn path(&self) -> &Path {
        self.inner.path()
    }

    /// Store a compiled crystal by its OID.
    pub fn store_crystal(&self, oid: &str, crystal: Fractal<String>, size_bytes: usize) {
        self.inner
            .insert_persistent(oid.to_string(), crystal, size_bytes);
    }

    /// Retrieve a crystal by OID.
    pub fn get_crystal(&self, oid: &str) -> Option<Fractal<String>> {
        self.inner.get_persistent(oid)
    }

    /// Set the current branch's crystal ref.
    pub fn set_head(&self, oid: &str) -> Result<(), fragmentation::frgmnt_store::Error> {
        self.inner.set_ref("HEAD", oid)
    }

    /// Get the current branch's crystal ref.
    pub fn head(&self) -> Option<String> {
        self.inner.get_ref("HEAD")
    }

    /// Set a named branch ref.
    /// Creates the `heads/` subdirectory under refs/ if needed.
    pub fn set_branch(
        &self,
        branch: &str,
        oid: &str,
    ) -> Result<(), fragmentation::frgmnt_store::Error> {
        let heads_dir = self.inner.path().join("refs").join("heads");
        let _ = std::fs::create_dir_all(&heads_dir);
        self.inner.set_ref(&format!("heads/{}", branch), oid)
    }

    /// Get a named branch ref.
    pub fn get_branch(&self, branch: &str) -> Option<String> {
        self.inner.get_ref(&format!("heads/{}", branch))
    }

    /// Store a compiled .shatter artifact by its OID.
    ///
    /// The content is encoded as a Fractal<String> and stored persistently
    /// in the namespaced git store under the given OID key.
    pub fn store_shatter(&self, oid: &str, content: &str) {
        let fragment = fragmentation::encoding::encode(content);
        let size = content.len();
        self.inner
            .insert_persistent(oid.to_string(), fragment, size);
    }

    /// Set a file-path → OID mapping in the refs index.
    ///
    /// Hashes the path to a short key under `files/` so arbitrary filesystem
    /// paths (including slashes) can be stored as ref names.
    pub fn set_file_ref(
        &self,
        path: &str,
        oid: &str,
    ) -> Result<(), fragmentation::frgmnt_store::Error> {
        let files_dir = self.inner.path().join("refs").join("files");
        let _ = std::fs::create_dir_all(&files_dir);
        let key = format!("files/{}", path_hash(path));
        self.inner.set_ref(&key, oid)
    }

    /// Get the OID for a file path, if one was previously stored.
    pub fn get_file_ref(&self, path: &str) -> Option<String> {
        let key = format!("files/{}", path_hash(path));
        self.inner.get_ref(&key)
    }

    /// Flush cached entries to disk.
    pub fn flush(&self) {
        self.inner.flush();
    }

    /// Number of cached entries.
    pub fn cached_len(&self) -> usize {
        self.inner.cached_len()
    }

    /// Access the underlying namespaced store.
    pub fn inner(&self) -> &NamespacedGitStore {
        &self.inner
    }
}

/// Deterministic short key for a filesystem path.
///
/// SHA-256 of the path bytes, hex-encoded, first 16 chars.
/// Stable — same path always maps to the same key.
fn path_hash(path: &str) -> String {
    use fragmentation::sha::HashAlg;
    let hash = fragmentation::sha::Sha::hash(path.as_bytes());
    hash.as_str()[..16].to_string()
}

/// The backend for mirror's store.
///
/// Either standalone (`.mirror/`) or git-integrated (`.git/mirror/`).
pub enum MirrorStoreBackend {
    /// Git-integrated store living inside `.git/mirror/`.
    Git(MirrorGitStore),
}

impl MirrorStoreBackend {
    /// Open the git backend.
    pub fn git(repo_path: &Path) -> Result<Self, NamespacedStoreError> {
        Ok(MirrorStoreBackend::Git(MirrorGitStore::open(repo_path)?))
    }

    /// Get the path to the store directory.
    pub fn path(&self) -> &Path {
        match self {
            MirrorStoreBackend::Git(store) => store.path(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fragmentation::fragment::Fragmentable;
    use std::path::PathBuf;

    #[test]
    fn mirror_git_store_opens_in_git_repo() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();
        assert!(store.path().exists());
        assert!(store.path().join("objects").exists());
    }

    #[test]
    fn store_and_retrieve_crystal() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();

        let crystal = fragmentation::encoding::encode("test crystal");
        let oid = fragmentation::fragment::content_oid(&crystal);

        store.store_crystal(&oid, crystal.clone(), 100);
        let got = store.get_crystal(&oid);
        assert!(got.is_some());
        assert_eq!(got.unwrap().data(), crystal.data());
    }

    #[test]
    fn head_ref() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();
        assert!(store.head().is_none());

        store.set_head("abc123").unwrap();
        assert_eq!(store.head().as_deref(), Some("abc123"));
    }

    #[test]
    fn branch_refs() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();

        store.set_branch("main", "main-oid").unwrap();
        store.set_branch("feature", "feature-oid").unwrap();

        assert_eq!(store.get_branch("main").as_deref(), Some("main-oid"));
        assert_eq!(store.get_branch("feature").as_deref(), Some("feature-oid"));
    }

    #[test]
    fn backend_enum_git() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let backend = MirrorStoreBackend::git(dir.path()).unwrap();
        assert!(backend.path().exists());
    }

    #[test]
    fn not_a_git_repo() {
        let dir = tempfile::tempdir().unwrap();
        let result = MirrorGitStore::open(dir.path());
        assert!(result.is_err());
    }

    /// End-to-end: mirror can open a store in its own git repository.
    /// This test runs on the mirror repo itself — the crate IS the test subject.
    #[test]
    fn mirror_store_on_self() {
        let repo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        let store = MirrorGitStore::open(&repo_root).unwrap();

        // The store should exist inside .git/mirror/
        assert!(store.path().exists());
        assert!(store.path().join("objects").exists());
        assert!(store.path().join("refs").exists());

        // Write a crystal and read it back
        let crystal = fragmentation::encoding::encode("mirror self-test crystal");
        let oid = fragmentation::fragment::content_oid(&crystal);
        store.store_crystal(&oid, crystal.clone(), 100);
        let got = store.get_crystal(&oid);
        assert!(got.is_some());
        assert_eq!(got.unwrap().data(), crystal.data());

        // Set HEAD and read it back
        store.set_head(&oid).unwrap();
        assert_eq!(store.head().as_deref(), Some(oid.as_str()));
    }

    #[test]
    fn store_shatter_roundtrip() {
        use fragmentation::fragment::Fragmentable;

        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();
        let content = "---\noid: test123\nluminosity: light\n---\n\ntype color = red | blue\n";
        store.store_shatter("test123", content);

        let got = store.get_crystal("test123");
        assert!(
            got.is_some(),
            "shatter artifact should be retrievable by oid"
        );
        assert_eq!(got.unwrap().data(), content);
    }

    #[test]
    fn store_file_ref_and_lookup() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();
        store.set_file_ref("src/main.mirror", "abc123").unwrap();
        assert_eq!(
            store.get_file_ref("src/main.mirror").as_deref(),
            Some("abc123")
        );
    }

    #[test]
    fn file_ref_different_paths_independent() {
        let dir = tempfile::tempdir().unwrap();
        git2::Repository::init(dir.path()).unwrap();

        let store = MirrorGitStore::open(dir.path()).unwrap();
        store.set_file_ref("src/foo.mirror", "oid-foo").unwrap();
        store.set_file_ref("src/bar.mirror", "oid-bar").unwrap();

        assert_eq!(
            store.get_file_ref("src/foo.mirror").as_deref(),
            Some("oid-foo")
        );
        assert_eq!(
            store.get_file_ref("src/bar.mirror").as_deref(),
            Some("oid-bar")
        );
        assert_eq!(store.get_file_ref("src/unknown.mirror"), None);
    }
}
