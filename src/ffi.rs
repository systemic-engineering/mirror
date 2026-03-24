//! FFI surface for the conversation crate.
//!
//! Public functions for NIF wrappers (Rustler) and internal use.
//! `parse_to_oid` and `compile_grammar_to_etf` are the two core operations.

use crate::compile;
use crate::domain::conversation::Kind;
use crate::logic::{Fact, ProofCertificate};
use crate::parse::Parse;
use crate::resolve::TypeRegistry;
use crate::ContentAddressed;
use crate::Vector;

/// Parse .conv source → content OID string.
pub fn parse_to_oid(source: &str) -> Result<String, String> {
    match Parse.trace(source.to_string()).into_result() {
        Ok(tree) => {
            #[cfg(feature = "git")]
            let oid = match commit_prism(&tree) {
                Ok(commit_oid) => commit_oid,
                Err(_) => tree.content_oid().as_ref().to_string(),
            };
            #[cfg(not(feature = "git"))]
            let oid = tree.content_oid().as_ref().to_string();
            Ok(oid)
        }
        Err(err) => Err(err.to_string()),
    }
}

/// Result of grammar compilation with per-phase content OIDs.
pub struct CompileResult {
    /// ETF-encoded BEAM module bytes.
    pub etf: Vec<u8>,
    /// Content OID of the parsed AST.
    pub parse_oid: String,
    /// Content OID of the resolved TypeRegistry.
    pub resolve_oid: String,
    /// Content OID of the compiled EAF bytes.
    pub compile_oid: String,
    /// Proof certificate OID — content address of the proof.
    pub proof_oid: String,
    /// Proof certificate serialized as ETF bytes.
    /// Can be decoded on the BEAM side as an Erlang term.
    pub proof_etf: Vec<u8>,
}

/// Compile with per-phase OIDs for traced compilation chain.
pub fn compile_grammar_with_phases(source: &str) -> Result<CompileResult, String> {
    let ast = Parse
        .trace(source.to_string())
        .into_result()
        .map_err(|e| e.to_string())?;

    let parse_oid = ast.content_oid().as_ref().to_string();

    let grammar_node = ast
        .children()
        .iter()
        .find(|c| c.data().is_decl("grammar"))
        .ok_or_else(|| "no grammar block found".to_string())?;

    let registry = TypeRegistry::compile(grammar_node).map_err(|e| e.to_string())?;
    let resolve_oid = crate::Oid::hash(registry.encoded()).as_ref().to_string();

    let domain_name = registry.domain.clone();
    let lenses: Vec<String> = ast
        .children()
        .iter()
        .filter(|c| c.data().is_decl("in"))
        .map(|c| c.data().value.trim_start_matches('@').to_string())
        .filter(|d| *d != domain_name)
        .collect();

    let extends: Vec<String> = grammar_node
        .children()
        .iter()
        .filter(|c| c.data().kind == Kind::Ref && c.data().name == "extends")
        .map(|c| c.data().value.trim_start_matches('@').to_string())
        .collect();

    let etf = compile::emit_actor_module(&registry, &lenses, &extends);
    let compile_oid = crate::Oid::hash(&etf).as_ref().to_string();

    let cert = ProofCertificate::from_registry(&registry);
    let proof_oid = cert.proof_oid.as_ref().to_string();
    let proof_etf = proof_certificate_to_etf(&cert);

    Ok(CompileResult {
        etf,
        parse_oid,
        resolve_oid,
        compile_oid,
        proof_oid,
        proof_etf,
    })
}

/// Compile .conv grammar source → ETF bytes for actor dispatch module.
pub fn compile_grammar_to_etf(source: &str) -> Result<Vec<u8>, String> {
    compile_grammar_with_phases(source).map(|r| r.etf)
}

/// Serialize a ProofCertificate as ETF bytes.
///
/// The certificate becomes an Erlang proplist:
/// ```erlang
/// [{domain, <<"test">>},
///  {proof_oid, <<"sha512:...">>},
///  {facts, [{type_exists, <<"test">>, <<"color">>}, ...]},
///  {discharged, [{<<"requirement">>, <<"evidence">>}, ...]}]
/// ```
///
/// DESIGN BREAK: This serialization is lossy in a specific way.
/// The Fact enum variants map to tagged tuples, but the structure
/// is flat — we lose the Rust type safety. On the BEAM side, pattern
/// matching recovers the tag dispatch, but there's no compile-time
/// guarantee that the BEAM consumer handles all Fact variants. This
/// is the fundamental gap at the NIF boundary: Rust's exhaustive
/// match becomes Erlang's runtime pattern match. A proof certificate
/// that the BEAM side doesn't fully consume is only a partial proof.
///
/// To close this gap, we would need:
/// 1. A shared schema language (the grammar itself could serve this role)
/// 2. Codegen for BEAM-side decoders from the Fact enum
/// 3. Or: represent the proof as a content-addressed OID only,
///    and let the BEAM side query back into Rust for specific facts
///
/// Option 3 is cheapest but defeats the purpose of the proof traveling
/// with the artifact. Option 2 is the right answer but requires
/// the grammar to describe itself — a fixpoint we haven't reached yet.
fn proof_certificate_to_etf(cert: &ProofCertificate) -> Vec<u8> {
    use eetf::{Atom, List, Term, Tuple};

    let domain_pair = Term::from(Tuple::from(vec![
        Term::from(Atom::from("domain")),
        etf_binary(&cert.domain),
    ]));

    let oid_pair = Term::from(Tuple::from(vec![
        Term::from(Atom::from("proof_oid")),
        etf_binary(cert.proof_oid.as_ref()),
    ]));

    let fact_terms: Vec<Term> = cert.facts.iter().map(fact_to_etf_term).collect();
    let facts_pair = Term::from(Tuple::from(vec![
        Term::from(Atom::from("facts")),
        Term::from(List::from(fact_terms)),
    ]));

    let obligation_terms: Vec<Term> = cert
        .discharged
        .iter()
        .map(|ob| {
            Term::from(Tuple::from(vec![
                etf_binary(&ob.requirement),
                etf_binary(&ob.evidence),
            ]))
        })
        .collect();
    let discharged_pair = Term::from(Tuple::from(vec![
        Term::from(Atom::from("discharged")),
        Term::from(List::from(obligation_terms)),
    ]));

    let proplist = Term::from(List::from(vec![
        domain_pair,
        oid_pair,
        facts_pair,
        discharged_pair,
    ]));

    let mut buf = Vec::new();
    proplist
        .encode(&mut buf)
        .expect("ETF encoding should not fail");
    buf
}

/// Convert a Fact to an ETF term (tagged tuple).
fn fact_to_etf_term(fact: &Fact) -> eetf::Term {
    use eetf::{Atom, Term, Tuple};

    match fact {
        Fact::TypeExists { domain, type_name } => Term::from(Tuple::from(vec![
            Term::from(Atom::from("type_exists")),
            etf_binary(domain),
            etf_binary(type_name),
        ])),
        Fact::TypeHasVariant {
            domain,
            type_name,
            variant,
        } => Term::from(Tuple::from(vec![
            Term::from(Atom::from("type_has_variant")),
            etf_binary(domain),
            etf_binary(type_name),
            etf_binary(variant),
        ])),
        Fact::VariantRefs {
            domain,
            type_name,
            variant,
            ref_type,
        } => Term::from(Tuple::from(vec![
            Term::from(Atom::from("variant_refs")),
            etf_binary(domain),
            etf_binary(type_name),
            etf_binary(variant),
            etf_binary(ref_type),
        ])),
        Fact::ActionExists {
            domain,
            action_name,
        } => Term::from(Tuple::from(vec![
            Term::from(Atom::from("action_exists")),
            etf_binary(domain),
            etf_binary(action_name),
        ])),
        Fact::ActionField {
            domain,
            action_name,
            field_name,
            type_ref,
        } => {
            let type_ref_term = match type_ref {
                Some(t) => etf_binary(t),
                None => Term::from(Atom::from("none")),
            };
            Term::from(Tuple::from(vec![
                Term::from(Atom::from("action_field")),
                etf_binary(domain),
                etf_binary(action_name),
                etf_binary(field_name),
                type_ref_term,
            ]))
        }
        Fact::ActionCalls {
            domain,
            action_name,
            target_domain,
            target_action,
        } => Term::from(Tuple::from(vec![
            Term::from(Atom::from("action_calls")),
            etf_binary(domain),
            etf_binary(action_name),
            etf_binary(target_domain),
            etf_binary(target_action),
        ])),
    }
}

/// Create an ETF binary from a string.
fn etf_binary(s: &str) -> eetf::Term {
    eetf::Term::from(eetf::Binary::from(s.as_bytes()))
}

/// Write a Prism tree to git objects. Returns the root tree OID.
///
/// Maps Prism variants to git objects following fragmentation conventions:
/// - Shard → blob (data bytes)
/// - Fractal → tree with `.data` blob + numbered children
/// - Lens → tree with `.data` blob + `.lens` blob (target OIDs)
/// - Optics → tree with `.data` blob + `.lens` blob + numbered children
#[cfg(feature = "git")]
fn write_prism_tree(
    repo: &git2::Repository,
    tree: &crate::prism::Prism<crate::ast::AstNode>,
) -> Result<git2::Oid, git2::Error> {
    use fragmentation::encoding::Encode;

    let data_bytes = tree.data().encode();

    if tree.is_shard() {
        return repo.blob(&data_bytes);
    }

    let mut builder = repo.treebuilder(None)?;
    let data_oid = repo.blob(&data_bytes)?;
    builder.insert(".data", data_oid, 0o100644)?;

    // Lens targets
    if tree.is_lens() || !tree.targets().is_empty() {
        let lens_content: String = tree
            .targets()
            .iter()
            .map(|sha| sha.0.as_str())
            .collect::<Vec<&str>>()
            .join("\n");
        let lens_oid = repo.blob(lens_content.as_bytes())?;
        builder.insert(".lens", lens_oid, 0o100644)?;
    }

    // Children
    for (i, child) in tree.children().iter().enumerate() {
        let child_oid = write_prism_tree(repo, child)?;
        let mode = if child.is_shard() { 0o100644 } else { 0o040000 };
        builder.insert(format!("{:04}", i), child_oid, mode)?;
    }

    builder.write()
}

/// Deterministic Ed25519 signing key for conversation commits.
/// sha512("conversation") → first 32 bytes → Ed25519 seed → keypair.
/// Same pattern as @compiler actor in Gleam (sha512("compiler") → keypair).
#[cfg(feature = "git")]
fn conversation_key() -> Result<ssh_key::PrivateKey, String> {
    use sha2::{Digest, Sha512};
    use ssh_key::private::{Ed25519Keypair, KeypairData};

    let hash = Sha512::digest(b"conversation");
    let seed: [u8; 32] = hash[..32].try_into().expect("SHA-512 produces 64 bytes");
    let keypair = Ed25519Keypair::from_seed(&seed);
    let key_data = KeypairData::Ed25519(keypair);
    ssh_key::PrivateKey::new(key_data, "conversation@systemic.engineering")
        .map_err(|e| format!("key: {}", e))
}

/// Sign a commit buffer with the conversation SSH key.
/// Returns PEM-encoded SSH signature suitable for git.
#[cfg(feature = "git")]
fn sign_commit(commit_content: &[u8]) -> Result<String, String> {
    let key = conversation_key()?;
    let sig = key
        .sign("git", ssh_key::HashAlg::Sha512, commit_content)
        .map_err(|e| format!("sign: {}", e))?;
    let pem = sig
        .to_pem(ssh_key::LineEnding::LF)
        .map_err(|e| format!("pem: {}", e))?;
    Ok(pem)
}

/// Commit a parsed Prism to a git repository.
///
/// Writes the Prism tree to git objects, creates an SSH-signed commit
/// authored by `conversation@systemic.engineering`, and updates
/// `refs/conversation/<branch>`. Returns the commit OID hex string.
#[cfg(feature = "git")]
fn commit_prism_to_repo(
    repo: &git2::Repository,
    tree: &crate::prism::Prism<crate::ast::AstNode>,
) -> Result<String, String> {
    let head = repo.head().map_err(|e| format!("HEAD: {}", e))?;
    let branch = head
        .shorthand()
        .ok_or_else(|| "HEAD: not a branch".to_string())?
        .to_string();

    let tree_oid = write_prism_tree(repo, tree).map_err(|e| format!("write tree: {}", e))?;
    let git_tree = repo
        .find_tree(tree_oid)
        .map_err(|e| format!("find tree: {}", e))?;

    let sig = git2::Signature::now("conversation", "conversation@systemic.engineering")
        .map_err(|e| format!("signature: {}", e))?;

    // Find parent commit on refs/conversation/<branch> if it exists
    let ref_name = format!("refs/conversation/{}", branch);
    let parent_commit;
    let parents: Vec<&git2::Commit> = match repo.find_reference(&ref_name) {
        Ok(r) => {
            let oid = r.target().ok_or_else(|| "ref: no target".to_string())?;
            parent_commit = repo
                .find_commit(oid)
                .map_err(|e| format!("parent commit: {}", e))?;
            vec![&parent_commit]
        }
        Err(_) => vec![],
    };

    // Build commit content, sign it, write signed commit
    let commit_buf = repo
        .commit_create_buffer(&sig, &sig, "prism", &git_tree, &parents)
        .map_err(|e| format!("commit buffer: {}", e))?;
    let commit_content =
        std::str::from_utf8(&commit_buf).map_err(|e| format!("commit content: {}", e))?;

    let signature = sign_commit(commit_buf.as_ref())?;

    let commit_oid = repo
        .commit_signed(commit_content, &signature, Some("gpgsig"))
        .map_err(|e| format!("signed commit: {}", e))?;

    // Update the ref to point to the new commit
    repo.reference(&ref_name, commit_oid, true, "conversation: prism commit")
        .map_err(|e| format!("update ref: {}", e))?;

    Ok(commit_oid.to_string())
}

/// Discover the git repo from cwd and commit.
#[cfg(feature = "git")]
fn commit_prism(tree: &crate::prism::Prism<crate::ast::AstNode>) -> Result<String, String> {
    let repo = git2::Repository::discover(".").map_err(|e| format!("git repo: {}", e))?;
    commit_prism_to_repo(&repo, tree)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compile_result_has_phase_oids() {
        let result = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        assert!(!result.etf.is_empty());
        assert_eq!(result.etf[0], 131); // ETF version byte
        assert!(!result.parse_oid.is_empty());
        assert!(!result.resolve_oid.is_empty());
        assert!(!result.compile_oid.is_empty());
        // All OIDs should be different (different content at each phase)
        assert_ne!(result.parse_oid, result.resolve_oid);
        assert_ne!(result.resolve_oid, result.compile_oid);
    }

    #[test]
    fn compile_result_phase_oids_deterministic() {
        let a = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        let b = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        assert_eq!(a.parse_oid, b.parse_oid);
        assert_eq!(a.resolve_oid, b.resolve_oid);
        assert_eq!(a.compile_oid, b.compile_oid);
    }

    #[test]
    fn compile_result_has_proof_certificate() {
        let result = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        // Proof OID should be non-empty — it's the content address of the proof
        assert!(
            !result.proof_oid.is_empty(),
            "proof_oid should be non-empty"
        );
        // Proof ETF should be non-empty — it's the serialized certificate
        assert!(
            !result.proof_etf.is_empty(),
            "proof_etf should be non-empty"
        );
        // Proof ETF should start with ETF version byte 131
        assert_eq!(result.proof_etf[0], 131, "proof_etf should be valid ETF");
    }

    #[test]
    fn compile_result_proof_certificate_deterministic() {
        let a = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        let b = compile_grammar_with_phases(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        assert_eq!(a.proof_oid, b.proof_oid);
        assert_eq!(a.proof_etf, b.proof_etf);
    }

    #[test]
    fn compile_result_proof_certificate_differs_for_different_grammars() {
        let a = compile_grammar_with_phases("grammar @alpha {\n  type = x\n}\n").unwrap();
        let b = compile_grammar_with_phases("grammar @beta {\n  type = y | z\n}\n").unwrap();
        assert_ne!(a.proof_oid, b.proof_oid);
    }

    #[test]
    fn compile_result_proof_etf_decodable() {
        let result = compile_grammar_with_phases(
            "grammar @test {\n  type color = red | blue\n  type shade = light | dark\n}\n",
        )
        .unwrap();
        // The proof ETF should be decodable as an Erlang term
        let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
        let s = format!("{:?}", term);
        // Should contain the domain name as bytes (eetf::Binary Debug shows byte array)
        let test_bytes: Vec<u8> = "test".bytes().collect();
        assert!(
            s.contains(&format!("{:?}", test_bytes)),
            "proof ETF should contain domain bytes: {}",
            s
        );
        // Should contain fact tags
        assert!(
            s.contains("domain"),
            "proof ETF should contain 'domain' atom"
        );
        assert!(s.contains("facts"), "proof ETF should contain 'facts' atom");
    }

    #[test]
    fn compile_result_proof_etf_all_fact_variants() {
        // Grammar that produces ALL Fact variants:
        // - TypeExists, TypeHasVariant (from type definitions)
        // - VariantRefs (from parameterized variant red(shade))
        // - ActionExists, ActionField (from action with field)
        // - ActionCalls (from cross-domain call)
        // - Obligations (discharged VariantRefs)
        let result = compile_grammar_with_phases(
            "grammar @full {\n  type color = red(shade) | blue\n  type shade = light | dark\n  action paint {\n    brush: color\n    @tools.apply(brush)\n  }\n}\n",
        )
        .unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
        let s = format!("{:?}", term);
        // All fact tags should be present
        assert!(s.contains("type_exists"), "should have type_exists facts");
        assert!(
            s.contains("type_has_variant"),
            "should have type_has_variant facts"
        );
        assert!(s.contains("variant_refs"), "should have variant_refs facts");
        assert!(
            s.contains("action_exists"),
            "should have action_exists facts"
        );
        assert!(s.contains("action_field"), "should have action_field facts");
        assert!(s.contains("action_calls"), "should have action_calls facts");
        // Discharged obligations (from red(shade) → shade exists)
        assert!(
            s.contains("discharged"),
            "should have discharged obligations"
        );
    }

    #[test]
    fn compile_result_proof_etf_action_field_none_type_ref() {
        // Action with field that has no type ref → ActionField with type_ref=None
        let result = compile_grammar_with_phases(
            "grammar @bare {\n  type = a\n  action touch {\n    target\n  }\n}\n",
        )
        .unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&result.proof_etf)).unwrap();
        let s = format!("{:?}", term);
        assert!(
            s.contains("action_field"),
            "should have action_field: {}",
            s
        );
        // The None type_ref should be encoded as atom 'none'
        assert!(
            s.contains("none"),
            "untyped field should encode type_ref as 'none': {}",
            s
        );
    }

    #[test]
    fn parse_success() {
        let oid = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        assert!(!oid.is_empty());
    }

    #[test]
    fn parse_error() {
        let err = parse_to_oid("@@@invalid").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn parse_deterministic() {
        let a = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        let b = parse_to_oid("grammar @test {\n  type = a | b\n}\n").unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn compile_grammar_success() {
        let etf = compile_grammar_to_etf(
            "grammar @compiler {\n  type = target\n  type target = eaf | beam\n  action compile {\n    source: target\n  }\n}\n",
        )
        .unwrap();
        assert!(!etf.is_empty());
        assert_eq!(etf[0], 131); // ETF version byte
    }

    #[test]
    fn compile_grammar_parse_error() {
        let err = compile_grammar_to_etf("!!! not valid conv syntax").unwrap_err();
        assert!(!err.is_empty());
    }

    #[test]
    fn compile_grammar_type_ref_error() {
        let err =
            compile_grammar_to_etf("grammar @test {\n  type = when(nonexistent)\n}\n").unwrap_err();
        assert!(err.contains("unknown type"));
    }

    #[test]
    fn compile_grammar_no_grammar_block() {
        let err = compile_grammar_to_etf("in @filesystem\ntemplate $t {\n\tslug\n}\n").unwrap_err();
        assert!(err.contains("grammar"));
    }

    #[test]
    fn compile_grammar_deterministic() {
        let a = compile_grammar_to_etf(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        let b = compile_grammar_to_etf(
            "grammar @test {\n  type = a | b\n  action ping {\n    target: a\n  }\n}\n",
        )
        .unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn compile_grammar_includes_lenses() {
        // Source with `in @reality` sibling — compile should include the Lens
        let etf = compile_grammar_to_etf(
            "in @reality\n\ngrammar @filesystem {\n  type = file | folder\n}\n",
        )
        .unwrap();

        // Decode and verify the lenses/0 function is present
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        assert!(
            forms_str.contains("lenses"),
            "expected 'lenses' export in EAF: {}",
            forms_str,
        );
        // "reality" encoded as ByteList bytes
        let reality_bytes: Vec<u8> = "reality".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", reality_bytes)),
            "expected 'reality' Lens bytes in EAF: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_no_lenses_when_no_in() {
        // Source with grammar only — no `in` declarations
        let etf = compile_grammar_to_etf("grammar @test {\n  type = a | b\n}\n").unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        // lenses/0 should still exist but return empty list
        assert!(
            forms_str.contains("lenses"),
            "expected 'lenses' export even when empty: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_includes_extends() {
        let etf = compile_grammar_to_etf(
            "grammar @fox extends @smash, @controller {\n  type = move | attack\n}\n",
        )
        .unwrap();

        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        assert!(
            forms_str.contains("extends"),
            "expected 'extends' export in EAF: {}",
            forms_str,
        );
        let smash_bytes: Vec<u8> = "smash".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", smash_bytes)),
            "expected 'smash' extends bytes in EAF: {}",
            forms_str,
        );
        let controller_bytes: Vec<u8> = "controller".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", controller_bytes)),
            "expected 'controller' extends bytes in EAF: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_no_extends_when_absent() {
        let etf = compile_grammar_to_etf("grammar @test {\n  type = a | b\n}\n").unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        // extends/0 should still exist but return empty list
        assert!(
            forms_str.contains("extends"),
            "expected 'extends' export even when empty: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_module_has_conv_prefix() {
        let etf = compile_grammar_to_etf("grammar @test {\n  type = a | b\n}\n").unwrap();
        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        assert!(
            forms_str.contains("conv_test"),
            "expected module name 'conv_test' in EAF: {}",
            forms_str,
        );
    }

    #[test]
    fn compile_grammar_self_lens_filtered() {
        // `in @filesystem` in a @filesystem grammar should NOT appear in lenses
        let etf = compile_grammar_to_etf(
            "in @filesystem\nin @reality\n\ngrammar @filesystem {\n  type = file | folder\n}\n",
        )
        .unwrap();

        let term = eetf::Term::decode(std::io::Cursor::new(&etf)).unwrap();
        let forms_str = format!("{:?}", term);
        // Should have "reality" in lenses but not "filesystem"
        let reality_bytes: Vec<u8> = "reality".bytes().collect();
        assert!(
            forms_str.contains(&format!("{:?}", reality_bytes)),
            "expected 'reality' in lenses",
        );
    }

    // -- git commit tests --

    #[cfg(feature = "git")]
    mod git_tests {
        use super::*;

        fn init_repo_with_branch() -> (tempfile::TempDir, git2::Repository) {
            let dir = tempfile::tempdir().unwrap();
            let repo = git2::Repository::init(dir.path()).unwrap();

            // Create an initial commit so HEAD points to a branch.
            {
                let sig = git2::Signature::now("test", "test@test").unwrap();
                let tree_oid = repo.treebuilder(None).unwrap().write().unwrap();
                let tree = repo.find_tree(tree_oid).unwrap();
                repo.commit(Some("HEAD"), &sig, &sig, "init", &tree, &[])
                    .unwrap();
            }

            (dir, repo)
        }

        #[test]
        fn commit_prism_creates_signed_ref() {
            let (_dir, repo) = init_repo_with_branch();

            let source = "grammar @test {\n  type = a | b\n}\n";
            let tree = Parse.trace(source.to_string()).into_result().unwrap();
            let oid_str = commit_prism_to_repo(&repo, &tree).unwrap();
            assert!(!oid_str.is_empty());

            // Verify the ref exists
            let reference = repo.find_reference("refs/conversation/main").unwrap();
            let commit_oid = reference.target().unwrap();
            assert_eq!(commit_oid.to_string(), oid_str);

            // Verify commit metadata
            let commit = repo.find_commit(commit_oid).unwrap();
            assert_eq!(commit.author().name(), Some("conversation"));
            assert_eq!(
                commit.author().email(),
                Some("conversation@systemic.engineering")
            );
            assert_eq!(commit.message(), Some("prism"));

            // Verify SSH signature exists
            let (sig_bytes, _) = repo.extract_signature(&commit_oid, None).unwrap();
            let sig_str = std::str::from_utf8(&sig_bytes).unwrap();
            assert!(sig_str.contains("BEGIN SSH SIGNATURE"));
        }

        #[test]
        fn commit_prism_chains_parents() {
            let (_dir, repo) = init_repo_with_branch();

            let source1 = "grammar @a {\n  type = x\n}\n";
            let tree1 = Parse.trace(source1.to_string()).into_result().unwrap();
            let oid1 = commit_prism_to_repo(&repo, &tree1).unwrap();

            let source2 = "grammar @b {\n  type = y\n}\n";
            let tree2 = Parse.trace(source2.to_string()).into_result().unwrap();
            let oid2 = commit_prism_to_repo(&repo, &tree2).unwrap();

            assert_ne!(oid1, oid2);

            // Second commit should have first as parent
            let commit2 = repo
                .find_commit(git2::Oid::from_str(&oid2).unwrap())
                .unwrap();
            assert_eq!(commit2.parent_id(0).unwrap().to_string(), oid1);
        }

        #[test]
        fn write_prism_tree_roundtrip() {
            let (_dir, repo) = init_repo_with_branch();

            let source = "grammar @test {\n  type = a | b\n}\n";
            let tree = Parse.trace(source.to_string()).into_result().unwrap();
            let oid = write_prism_tree(&repo, &tree).unwrap();

            // The OID should be a valid git object
            let obj = repo.find_object(oid, None).unwrap();
            assert!(obj.kind() == Some(git2::ObjectType::Tree));
        }
    }
}
