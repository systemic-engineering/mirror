//! Git-integrated mirror store.
//!
//! Behind the `git` feature flag. Uses `NamespacedGitStore` from
//! `fragmentation-git` to store mirror crystals inside `.git/mirror/`.

use std::path::{Path, PathBuf};

use fragmentation::fragment::{Fractal, Fragmentable};
use fragmentation_git::namespaced::{NamespacedGitStore, NamespacedStoreError};

/// Mirror's git-integrated store. TODO: implement.
pub struct MirrorGitStore {
    // stub
}

/// The backend for mirror's store.
pub enum MirrorStoreBackend {
    /// Git-integrated store living inside `.git/mirror/`.
    Git(MirrorGitStore),
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
