//! Resolve traceable. AST → validated program.
//!
//! The resolver IS a traceable: trace resolves.
//! Validates domain references, template references, output structure.
//! Errors carry spans and did-you-mean hints.

use std::collections::{HashMap, HashSet};
use std::marker::PhantomData;

use serde_json::Value;

use crate::ast::{AstNode, Span};
use crate::domain::{Addressable, Setting};
use crate::parse::ParseError;
use crate::prism::{self, Prism};
use crate::{ComposedError, Vector};

use fragmentation::fragment::{self, Fragmentable};
use fragmentation::ref_::Ref;
use fragmentation::repo::Repo;
use fragmentation::sha::{self, Sha};
use fragmentation::store::Store;

domain_oid!(/// Content address for resolved conversations.
pub ConversationOid);

/// Compiled grammar. Maps type names to valid variants.
///
/// ```text
/// grammar @conversation { type = in | out | ...  type op = gt | lt | ... }
/// ```
/// yields:
///   `""` → `{"in", "out", "template", ...}`    (anonymous/default type)
///   `"op"` → `{"gt", "lt", "gte", "lte", ...}` (named type)
#[derive(Clone, Debug)]
pub struct TypeRegistry {
    ref_: Ref,
    encoded: Vec<u8>,
    pub domain: String,
    types: HashMap<String, HashSet<String>>,
    #[allow(dead_code)] // read in tests; used by Phase 4 validation
    params: HashMap<(String, String), String>,
    acts: HashMap<String, Vec<(String, Option<String>)>>,
}

impl TypeRegistry {
    /// Compile a Grammar AST node into a validated TypeRegistry.
    ///
    /// Walks the Grammar's TypeDef children, extracts type names and variant
    /// names, records parameterized variant references, then validates that
    /// every TypeRef points to a declared type name.
    pub fn compile(grammar_node: &Prism<AstNode>) -> Result<TypeRegistry, ResolveError> {
        let raw = &grammar_node.data().value;
        let domain = raw.strip_prefix('@').unwrap_or(raw).to_string();

        let mut types: HashMap<String, HashSet<String>> = HashMap::new();
        let mut params: HashMap<(String, String), String> = HashMap::new();
        let mut acts: HashMap<String, Vec<(String, Option<String>)>> = HashMap::new();

        for child in grammar_node.children() {
            if child.data().is_form("type-def") {
                let type_name = child.data().value.clone();
                let mut variants = HashSet::new();

                for variant in child.children() {
                    if !variant.data().is_form("variant") {
                        continue;
                    }
                    let variant_name = variant.data().value.clone();

                    for sub in variant.children() {
                        if sub.data().is_ref("type-ref") {
                            params.insert(
                                (type_name.clone(), variant_name.clone()),
                                sub.data().value.clone(),
                            );
                        }
                    }

                    variants.insert(variant_name);
                }

                types.insert(type_name, variants);
            } else if child.data().is_form("action-def") {
                let act_name = child.data().value.clone();
                let mut fields = Vec::new();

                for field in child.children() {
                    if !field.data().is_atom("field") {
                        continue;
                    }
                    let field_name = field.data().value.clone();
                    let type_ref = field
                        .children()
                        .iter()
                        .find(|c| c.data().is_ref("type-ref"))
                        .map(|c| c.data().value.clone());
                    fields.push((field_name, type_ref));
                }

                acts.insert(act_name, fields);
            }
        }

        // Validate: every TypeRef must reference a declared type name
        let span = grammar_node.data().span;
        for ((parent_type, variant_name), ref_type) in &params {
            if !types.contains_key(ref_type) {
                return Err(Self::bad_type_ref(
                    ref_type,
                    &types,
                    &domain,
                    format!(
                        "variant \"{}\" in type \"{}\"",
                        variant_name,
                        if parent_type.is_empty() {
                            "<default>"
                        } else {
                            parent_type
                        },
                    ),
                    span,
                ));
            }
        }
        // Act field type-refs are semantic annotations, not validated references.
        // Unlike parameterized variants (which MUST reference a declared type name),
        // act fields can reference variants, external types, or undeclared names.

        Ok(Self::finalize(domain, types, params, acts))
    }

    /// Build a "unknown type reference" error with did-you-mean hints.
    fn bad_type_ref(
        ref_type: &str,
        types: &HashMap<String, HashSet<String>>,
        domain: &str,
        context: String,
        span: crate::ast::Span,
    ) -> ResolveError {
        let declared: Vec<&str> = types.keys().map(|s| s.as_str()).collect();
        let hints = hint_did_you_mean(ref_type, &declared, |s| format!("did you mean \"{}\"?", s));
        ResolveError {
            message: format!(
                "unknown type reference \"{}\" in grammar @{} ({})",
                ref_type, domain, context,
            ),
            span: Some(span),
            hints,
        }
    }

    /// Check if a named type exists in this registry.
    pub fn has_type(&self, name: &str) -> bool {
        self.types.contains_key(name)
    }

    /// Check if a variant exists under a given type name.
    pub fn has_variant(&self, type_name: &str, variant: &str) -> bool {
        self.types
            .get(type_name)
            .is_some_and(|vs| vs.contains(variant))
    }

    /// Validate that a type reference name exists. Returns error with did-you-mean if not.
    pub fn validate_type_ref(&self, ref_name: &str) -> Result<(), ResolveError> {
        if self.types.contains_key(ref_name) {
            Ok(())
        } else {
            let declared: Vec<&str> = self.types.keys().map(|s| s.as_str()).collect();
            let hints =
                hint_did_you_mean(ref_name, &declared, |s| format!("did you mean \"{}\"?", s));
            Err(ResolveError {
                message: format!("unknown type \"{}\" in grammar @{}", ref_name, self.domain),
                span: None,
                hints,
            })
        }
    }

    /// Check if a named action exists in this registry.
    pub fn has_action(&self, name: &str) -> bool {
        self.acts.contains_key(name)
    }

    /// Get the fields of a named action: (field_name, optional_type_ref).
    pub fn action_fields(&self, name: &str) -> Option<&[(String, Option<String>)]> {
        self.acts.get(name).map(|v| v.as_slice())
    }

    /// All type names declared in this grammar.
    pub fn type_names(&self) -> Vec<&str> {
        self.types.keys().map(|s| s.as_str()).collect()
    }

    /// All variants for a named type. Returns None if the type doesn't exist.
    pub fn variants(&self, type_name: &str) -> Option<Vec<&str>> {
        self.types
            .get(type_name)
            .map(|vs| vs.iter().map(|s| s.as_str()).collect())
    }

    /// The parameter type reference for a parameterized variant, if any.
    pub fn variant_param(&self, type_name: &str, variant: &str) -> Option<&str> {
        self.params
            .get(&(type_name.to_string(), variant.to_string()))
            .map(|s| s.as_str())
    }

    /// All act names declared in this grammar.
    pub fn act_names(&self) -> Vec<&str> {
        self.acts.keys().map(|s| s.as_str()).collect()
    }

    /// Test-only: build a registry with a parameterized variant whose type ref
    /// is NOT declared. This bypasses compile-time validation to exercise the
    /// `None => continue` defensive path in `generate::derive_type`.
    #[cfg(test)]
    pub(crate) fn with_dangling_param(
        domain: &str,
        type_name: &str,
        variant: &str,
        param_ref: &str,
    ) -> Self {
        let mut types = HashMap::new();
        let mut variants = HashSet::new();
        variants.insert(variant.to_string());
        types.insert(type_name.to_string(), variants);
        // param points to a type that does NOT exist in `types`
        let mut params = HashMap::new();
        params.insert(
            (type_name.to_string(), variant.to_string()),
            param_ref.to_string(),
        );
        Self::finalize(domain.to_string(), types, params, HashMap::new())
    }

    /// Build a finalized TypeRegistry from raw data.
    /// Computes the canonical encoding and content address.
    fn finalize(
        domain: String,
        types: HashMap<String, HashSet<String>>,
        params: HashMap<(String, String), String>,
        acts: HashMap<String, Vec<(String, Option<String>)>>,
    ) -> Self {
        let encoded = Self::encode_canonical(&domain, &types, &params, &acts);
        let sha = Sha(fragment::blob_oid_bytes(&encoded));
        let ref_ = Ref::new(sha, format!("grammar/{}", domain));
        TypeRegistry {
            ref_,
            encoded,
            domain,
            types,
            params,
            acts,
        }
    }

    /// Deterministic encoding of grammar data.
    /// Sorted keys ensure same grammar → same bytes → same OID.
    fn encode_canonical(
        domain: &str,
        types: &HashMap<String, HashSet<String>>,
        params: &HashMap<(String, String), String>,
        acts: &HashMap<String, Vec<(String, Option<String>)>>,
    ) -> Vec<u8> {
        let mut lines = Vec::new();
        lines.push(domain.to_string());

        // Types: sorted by name, variants sorted
        let mut type_keys: Vec<&String> = types.keys().collect();
        type_keys.sort();
        for name in type_keys {
            let mut variants: Vec<&String> = types[name].iter().collect();
            variants.sort();
            let vs: Vec<&str> = variants.iter().map(|s| s.as_str()).collect();
            lines.push(format!("type:{}={}", name, vs.join(",")));
        }

        // Params: sorted by (type, variant)
        let mut param_keys: Vec<&(String, String)> = params.keys().collect();
        param_keys.sort();
        for key in param_keys {
            lines.push(format!("param:{},{}={}", key.0, key.1, params[key]));
        }

        // Acts: sorted by name, fields in order
        let mut act_keys: Vec<&String> = acts.keys().collect();
        act_keys.sort();
        for name in act_keys {
            let fields: Vec<String> = acts[name]
                .iter()
                .map(|(f, t)| match t {
                    Some(tr) => format!("{}:{}", f, tr),
                    None => f.clone(),
                })
                .collect();
            lines.push(format!("act:{}={}", name, fields.join(",")));
        }

        lines.join("\n").into_bytes()
    }
}

impl Fragmentable for TypeRegistry {
    type Data = Vec<u8>;

    fn self_ref(&self) -> &Ref {
        &self.ref_
    }

    fn data(&self) -> &Vec<u8> {
        &self.encoded
    }

    fn children(&self) -> &[Self] {
        &[]
    }
}

/// What a namespace module provides when resolved.
#[derive(Clone, Debug)]
pub enum TemplateProvider {
    /// Inline templates (defined in the same .conv file or injected).
    Inline(HashMap<String, Template>),
    /// Reference to another .conv file (future: lazy resolution).
    External(String),
}

/// How to generate derivations for a domain.
#[derive(Clone, Debug, PartialEq)]
pub enum GenerateProvider {
    /// Default: walk grammar types (the Rust generator).
    /// This IS `@compiler.generate` — the parent implementation.
    Derived,
    /// Override: custom derivation from a generate block.
    /// Carries (type_name, variants) pairs that replace the grammar's types.
    Override(Vec<(String, Vec<String>)>),
}

/// A namespace maps module names to template providers.
///
/// In single-node mode, `@X` resolves to `namespace.modules["X"]`.
/// The `@` is the security boundary — control what `@` resolves to = control the sandbox.
#[derive(Clone, Debug, Default)]
pub struct Namespace {
    modules: HashMap<String, TemplateProvider>,
    grammar_store: Store<TypeRegistry>,
    generate_overrides: HashMap<String, GenerateProvider>,
}

impl Namespace {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a module with inline templates.
    pub fn register(&mut self, name: &str, provider: TemplateProvider) {
        self.modules.insert(name.to_string(), provider);
    }

    /// Register a compiled grammar for a domain.
    pub fn register_grammar(&mut self, domain: &str, registry: TypeRegistry) {
        let ref_name = format!("grammar/{}", domain);
        self.grammar_store.write_tree(&registry);
        let sha = registry.self_ref().sha.clone();
        self.grammar_store.update_ref(&ref_name, sha);
    }

    /// Look up a grammar by domain name.
    pub fn grammar(&self, domain: &str) -> Option<TypeRegistry> {
        let ref_name = format!("grammar/{}", domain);
        let sha = self.grammar_store.resolve_ref(&ref_name)?;
        self.grammar_store.read_tree(&sha.0)
    }

    /// Check if a grammar is registered for a domain.
    pub fn has_grammar(&self, domain: &str) -> bool {
        let ref_name = format!("grammar/{}", domain);
        self.grammar_store.resolve_ref(&ref_name).is_some()
    }

    /// All registered grammar domain names.
    pub fn grammar_domains(&self) -> Vec<String> {
        self.grammar_store
            .ref_names()
            .iter()
            .filter_map(|name| name.strip_prefix("grammar/"))
            .map(|s| s.to_string())
            .collect()
    }

    /// Register a generate override for a domain.
    pub fn register_generate(&mut self, domain: &str, provider: GenerateProvider) {
        self.generate_overrides.insert(domain.to_string(), provider);
    }

    /// Get the generate provider for a domain.
    pub fn generate_provider(&self, domain: &str) -> &GenerateProvider {
        self.generate_overrides
            .get(domain)
            .unwrap_or(&GenerateProvider::Derived)
    }

    /// Check if a module is registered.
    pub fn contains(&self, name: &str) -> bool {
        self.modules.contains_key(name)
    }

    /// All module names for did-you-mean hints.
    pub fn module_names(&self) -> Vec<&str> {
        self.modules.keys().map(|s| s.as_str()).collect()
    }

    /// Look up a module's inline templates.
    pub fn get_templates(&self, name: &str) -> Option<&HashMap<String, Template>> {
        match self.modules.get(name)? {
            TemplateProvider::Inline(map) => Some(map),
            TemplateProvider::External(_) => None,
        }
    }
}

/// The resolve traceable. AST → Conversation.
///
/// Known domains (Filesystem, Json, Git) and namespace modules resolve.
/// External domains must be registered via `with_domain` or `with_namespace`.
#[derive(Clone, Debug, Default)]
pub struct Resolve {
    externals: Vec<String>,
    namespace: Namespace,
}

/// What can go wrong during resolution.
#[derive(Clone, Debug)]
pub struct ResolveError {
    pub message: String,
    pub span: Option<Span>,
    pub hints: Vec<String>,
}

/// A resolved .conv file. Validated and ready to execute.
///
/// The type parameter `C` is the input domain's context.
/// `Conversation<Filesystem>` executes against `Prism<Folder>`.
/// `Conversation<Git>` executes against `Prism<GitNode>`.
#[derive(Debug)]
pub struct Conversation<C: Setting> {
    templates: HashMap<String, Template>,
    pub content: Prism<OutputNode>,
    _context: PhantomData<C>,
}

#[derive(Clone, Debug)]
pub struct Param {
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct Template {
    pub params: Vec<Param>,
    fields: Vec<Field>,
}

impl Template {
    /// Create a template with the given field names (no qualifiers or pipes).
    pub fn with_fields(names: &[&str]) -> Self {
        Template {
            params: Vec::new(),
            fields: names
                .iter()
                .map(|name| Field {
                    name: name.to_string(),
                    qualifier: None,
                    pipe: None,
                })
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Field {
    name: String,
    qualifier: Option<String>,
    #[allow(dead_code)] // read in tests; used by future pipe execution
    pipe: Option<String>,
}

/// A pattern in a branch arm.
#[derive(Clone, Debug)]
pub enum BranchPattern {
    /// Exact string match: `"hold"`
    Literal(String),
    /// Wildcard: `_`
    Wild,
}

/// An action in a branch arm.
#[derive(Clone, Debug)]
pub enum BranchAction {
    /// Passthrough: `..`
    Pass,
    /// Terminate: `exit`
    Exit,
    /// Arbitrary expression
    Expr(String),
}

/// A single arm in a branch dispatch.
#[derive(Clone, Debug)]
pub struct BranchArm {
    pub pattern: BranchPattern,
    pub action: BranchAction,
}

#[derive(Clone, Debug)]
pub enum OutputNode {
    Group {
        name: String,
    },
    Select {
        output_name: String,
        folder_name: String,
        template_name: String,
    },
    Branch {
        query: String,
        arms: Vec<BranchArm>,
    },
}

impl fragmentation::encoding::Encode for OutputNode {
    fn encode(&self) -> Vec<u8> {
        match self {
            OutputNode::Group { name } => format!("group:{}", name).into_bytes(),
            OutputNode::Select {
                output_name,
                folder_name,
                template_name,
            } => format!("select:{}:{}:{}", output_name, folder_name, template_name).into_bytes(),
            OutputNode::Branch { query, arms } => {
                let arm_strs: Vec<String> = arms
                    .iter()
                    .map(|a| {
                        let pat = match &a.pattern {
                            BranchPattern::Literal(s) => format!("\"{}\"", s),
                            BranchPattern::Wild => "_".into(),
                        };
                        let act = match &a.action {
                            BranchAction::Pass => "..".into(),
                            BranchAction::Exit => "exit".into(),
                            BranchAction::Expr(e) => e.clone(),
                        };
                        format!("{} => {}", pat, act)
                    })
                    .collect();
                format!("branch:{}:{}", query, arm_strs.join(",")).into_bytes()
            }
        }
    }
}

impl OutputNode {
    /// The node's name — group name or select output name.
    pub fn name(&self) -> &str {
        match self {
            OutputNode::Group { name } => name,
            OutputNode::Select { output_name, .. } => output_name,
            OutputNode::Branch { .. } => "branch",
        }
    }
}

impl Resolve {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.externals.push(domain.to_string());
        self
    }

    /// Register a full namespace for import resolution.
    pub fn with_namespace(mut self, namespace: Namespace) -> Self {
        self.namespace = namespace;
        self
    }

    /// Known domain names. C::id() already carries the identity;
    /// this list is only for validating `in @...` declarations.
    const KNOWN_DOMAINS: &'static [&'static str] = &["filesystem", "json", "git"];

    /// Is this domain name known or registered?
    fn is_known_domain(&self, name: &str) -> bool {
        Self::KNOWN_DOMAINS.contains(&name)
            || self.externals.iter().any(|e| e == name)
            || self.namespace.contains(name)
    }

    /// All domain names available for did_you_mean.
    fn all_domain_names(&self) -> Vec<&str> {
        let mut names: Vec<&str> = Self::KNOWN_DOMAINS.to_vec();
        names.extend(self.externals.iter().map(|e| e.as_str()));
        names.extend(self.namespace.module_names());
        names
    }
}

impl Resolve {
    /// Resolve templates from a Use node's namespace path.
    ///
    /// Walks the Use node's children to find the source module,
    /// then extracts the named templates into the provided map.
    fn resolve_use(
        &self,
        use_node: &Prism<AstNode>,
        templates: &mut HashMap<String, Template>,
    ) -> Result<(), ResolveError> {
        let children = use_node.children();

        // Collect template names to import
        let template_names: Vec<String> = children
            .iter()
            .filter(|c| c.data().is_ref("template-ref"))
            .map(|c| c.data().value.clone())
            .collect();

        // Find the source: DomainRef or Home/Self_ node
        let domain_ref = children.iter().find(|c| c.data().is_ref("domain-ref"));

        let module_name = if let Some(domain) = domain_ref {
            let raw = &domain.data().value;
            raw.strip_prefix('@').unwrap_or(raw).to_string()
        } else {
            // $HOME or $SELF paths — future: tree navigation
            // For now, these need a namespace module registered
            return Ok(());
        };

        // Look up module in namespace
        let ns_templates = self.namespace.get_templates(&module_name);
        match ns_templates {
            Some(provider_templates) => {
                for name in &template_names {
                    if let Some(tmpl) = provider_templates.get(name) {
                        if templates.contains_key(name) {
                            return Err(ResolveError {
                                message: format!(
                                    "imported template {} conflicts with local definition",
                                    name
                                ),
                                span: Some(use_node.data().span),
                                hints: vec![],
                            });
                        }
                        templates.insert(name.clone(), tmpl.clone());
                    } else {
                        let candidates: Vec<&str> =
                            provider_templates.keys().map(|s| s.as_str()).collect();
                        let hints = hint_did_you_mean(name, &candidates, |s| {
                            format!("did you mean {}?", s)
                        });
                        return Err(ResolveError {
                            message: format!("template {} not found in @{}", name, module_name),
                            span: Some(use_node.data().span),
                            hints,
                        });
                    }
                }
            }
            None => {
                if !self.is_known_domain(&module_name) {
                    let candidates = self.all_domain_names();
                    let hints = hint_did_you_mean(&module_name, &candidates, |s| {
                        format!("did you mean @{}?", s)
                    });
                    return Err(ResolveError {
                        message: format!("unknown source @{}", module_name),
                        span: Some(use_node.data().span),
                        hints,
                    });
                }
                // Known domain but not in namespace — no templates to import
                // (future: external file resolution)
            }
        }

        Ok(())
    }
}

impl<C: Setting> Vector<Prism<AstNode>, Conversation<C>> for Resolve {
    type Error = ResolveError;

    fn trace(&self, source: Prism<AstNode>) -> crate::Trace<Conversation<C>, ResolveError> {
        use crate::{ContentAddressed, Trace, TraceOid};
        match resolve_ast(self, source) {
            Ok(conv) => {
                let oid = conv.content_oid();
                Trace::success(conv, oid.into(), None)
            }
            Err(e) => Trace::failure(e, TraceOid::new("error"), None),
        }
    }
}

fn resolve_ast<C: Setting>(
    resolve: &Resolve,
    source: Prism<AstNode>,
) -> Result<Conversation<C>, ResolveError> {
    let children = source.children();

    // Pass 1: compile grammar blocks (validates type references)
    let mut grammar_domains: Vec<String> = Vec::new();
    for child in children {
        if child.data().is_decl("grammar") {
            let registry = TypeRegistry::compile(child)?;
            grammar_domains.push(registry.domain);
        }
    }

    // Validate domain declaration if present
    let in_node = children.iter().find(|c| c.data().is_decl("in"));
    if let Some(node) = in_node {
        let raw = &node.data().value;
        let name = raw.strip_prefix('@').unwrap_or(raw);
        if !resolve.is_known_domain(name) && !grammar_domains.iter().any(|g| g == name) {
            let candidates = resolve.all_domain_names();
            let hints = hint_did_you_mean(name, &candidates, |s| format!("did you mean @{}?", s));
            return Err(ResolveError {
                message: format!("unknown domain @{}", name),
                span: Some(node.data().span),
                hints,
            });
        }
    }

    // Extract local templates
    let mut templates = HashMap::new();
    for child in children {
        if child.data().is_decl("template") {
            let name = child.data().value.clone();
            templates.insert(name, resolve_template(child));
        }
    }

    // Resolve use imports — merge external templates into local map
    for child in children {
        if child.data().is_decl("use") {
            resolve.resolve_use(child, &mut templates)?;
        }
    }

    // Extract output
    let out_node = children.iter().find(|c| c.data().is_decl("out"));

    // Collect top-level branch nodes
    let branch_nodes: Vec<Prism<OutputNode>> = children
        .iter()
        .filter(|c| c.data().is_decl("branch"))
        .map(resolve_branch_node)
        .collect();

    let content = match out_node {
        Some(node) => {
            let name = node.data().value.clone();
            let mut output_children = resolve_output_nodes(node, &templates)?;
            output_children.extend(branch_nodes);
            let ref_ = Ref::new(sha::hash(&name), &name);
            prism::fractal(ref_, OutputNode::Group { name }, output_children)
        }
        None => {
            return Err(ResolveError {
                message: "missing output block".into(),
                span: None,
                hints: vec![],
            });
        }
    };

    Ok(Conversation {
        templates,
        content,
        _context: PhantomData,
    })
}

pub(crate) fn resolve_template(template_node: &Prism<AstNode>) -> Template {
    let mut params = Vec::new();
    let mut fields = Vec::new();
    for child in template_node.children() {
        let d = child.data();
        if d.is_atom("param") {
            params.push(Param {
                name: d.value.clone(),
            });
        } else if d.is_atom("field") {
            let mut qualifier = None;
            let mut pipe = None;
            for sub in child.children() {
                if sub.data().is_atom("qualifier") {
                    qualifier = Some(sub.data().value.clone());
                } else if sub.data().is_atom("pipe") {
                    pipe = Some(sub.data().value.clone());
                }
            }
            fields.push(Field {
                name: d.value.clone(),
                qualifier,
                pipe,
            });
        }
    }
    Template { params, fields }
}

fn resolve_output_nodes(
    node: &Prism<AstNode>,
    templates: &HashMap<String, Template>,
) -> Result<Vec<Prism<OutputNode>>, ResolveError> {
    let mut nodes = Vec::new();
    for child in node.children() {
        let d = child.data();
        if d.is_form("group") {
            let children = resolve_output_nodes(child, templates)?;
            let name = d.value.clone();
            let ref_ = Ref::new(sha::hash(&name), &name);
            nodes.push(prism::fractal(ref_, OutputNode::Group { name }, children));
        } else if d.is_form("select") {
            let select_children = child.children();
            let folder_name = select_children
                .iter()
                .find(|c| c.data().is_ref("domain-ref"))
                .map(|c| c.data().value.clone())
                .unwrap_or_default();
            let template_name = select_children
                .iter()
                .find(|c| c.data().is_ref("template-ref"))
                .map(|c| c.data().value.clone())
                .unwrap_or_default();

            // Validate template reference
            if !templates.contains_key(&template_name) {
                let candidates: Vec<&str> = templates.keys().map(|s| s.as_str()).collect();
                let hints = hint_did_you_mean(&template_name, &candidates, |s| {
                    format!("did you mean {}?", s)
                });
                return Err(ResolveError {
                    message: format!("unknown template {}", template_name),
                    span: Some(d.span),
                    hints,
                });
            }

            let output_name = d.value.clone();
            let ref_ = Ref::new(sha::hash(&output_name), &output_name);
            nodes.push(prism::shard(
                ref_,
                OutputNode::Select {
                    output_name,
                    folder_name,
                    template_name,
                },
            ));
        }
    }
    Ok(nodes)
}

/// Convert an AST Branch node to an OutputNode::Branch tree node.
///
/// AST structure: Branch(".action") → [Arm → [Literal/Wild, Expr], ...]
fn resolve_branch_node(node: &Prism<AstNode>) -> Prism<OutputNode> {
    let query = node.data().value.clone();
    let mut arms = Vec::new();

    for arm_node in node.children() {
        if !arm_node.data().is_form("arm") {
            continue;
        }
        let arm_children = arm_node.children();
        if arm_children.len() < 2 {
            continue;
        }

        let pattern_data = arm_children[0].data();
        let pattern = if pattern_data.is_atom("literal") {
            BranchPattern::Literal(pattern_data.value.clone())
        } else if pattern_data.is_atom("wild") {
            BranchPattern::Wild
        } else {
            continue;
        };

        let action_str = &arm_children[1].data().value;
        let action = match action_str.as_str() {
            ".." => BranchAction::Pass,
            "exit" => BranchAction::Exit,
            other => BranchAction::Expr(other.to_string()),
        };

        arms.push(BranchArm { pattern, action });
    }

    let label = format!("branch:{}", query);
    let ref_ = Ref::new(sha::hash(&label), &label);
    prism::shard(ref_, OutputNode::Branch { query, arms })
}

impl<C: Setting> crate::ContentAddressed for Conversation<C> {
    type Oid = ConversationOid;
    fn content_oid(&self) -> ConversationOid {
        use sha2::{Digest, Sha256};

        let mut hasher = Sha256::new();
        hasher.update(b"conversation:");
        hasher.update(self.content.data().name().as_bytes());
        let mut keys: Vec<_> = self.templates.keys().collect();
        keys.sort();
        for key in keys {
            hasher.update(key.as_bytes());
        }
        ConversationOid::new(hex::encode(hasher.finalize()))
    }
}

impl<C: Setting> Conversation<C> {
    /// Parse and resolve a `.conv` source string in one step.
    ///
    /// Chains Parse → Resolve via traceable composition.
    pub fn from_source(source: &str) -> Result<Self, ComposedError<ParseError, ResolveError>> {
        Self::from_source_with(source, Resolve::new())
    }

    /// Parse and resolve with a custom Resolve (e.g. carrying a Namespace).
    pub fn from_source_with(
        source: &str,
        resolve: Resolve,
    ) -> Result<Self, ComposedError<ParseError, ResolveError>> {
        use crate::parse::Parse;
        Parse
            .compose::<Conversation<C>, _>(resolve)
            .trace(source.to_string())
            .into_result()
    }
}

/// Conversation IS a traceable: `Prism<C::Token> → Value`.
///
/// The resolved program transforms domain trees into JSON output.
/// `trace` executes the program against a domain tree.
impl<C: Setting> Vector<Prism<C::Token>, Value> for Conversation<C>
where
    C::Token: Addressable,
{
    type Error = ResolveError;

    fn trace(&self, source: Prism<C::Token>) -> crate::Trace<Value, ResolveError> {
        use crate::{ContentAddressed, Trace};
        let body = emit_body(&self.content, &source, &self.templates);
        let mut map = serde_json::Map::new();
        map.insert(self.content.data().name().to_string(), body);
        let result = Value::Object(map);
        let oid = result.content_oid();
        Trace::success(result, oid.into(), None)
    }
}

fn emit_body<T: Addressable>(
    content: &Prism<OutputNode>,
    tree: &Prism<T>,
    templates: &HashMap<String, Template>,
) -> Value {
    let mut map = serde_json::Map::new();

    for child in content.children() {
        match child.data() {
            OutputNode::Group { name } => {
                if let Some(domain_child) = find_child(tree, name) {
                    map.insert(name.clone(), emit_body(child, domain_child, templates));
                }
            }
            OutputNode::Select {
                output_name,
                folder_name,
                template_name,
            } => {
                if let Some(folder) = find_child(tree, folder_name) {
                    let template = &templates[template_name];
                    let items: Vec<Value> = folder
                        .children()
                        .iter()
                        .map(|f| apply_template(template, f))
                        .collect();
                    map.insert(output_name.clone(), Value::Array(items));
                }
            }
            OutputNode::Branch { .. } => {} // spec node — no JSON emission
        }
    }

    Value::Object(map)
}

fn find_child<'a, T: Addressable>(tree: &'a Prism<T>, name: &str) -> Option<&'a Prism<T>> {
    tree.children()
        .iter()
        .find(|c| c.data().node_name() == name)
}

fn apply_template<T: Addressable>(template: &Template, tree: &Prism<T>) -> Value {
    let content = tree.data().node_content().unwrap_or("");
    let (frontmatter, body) = parse_frontmatter(content);

    let mut map = serde_json::Map::new();
    for field in &template.fields {
        match &field.qualifier {
            None => {
                let value = lookup_frontmatter(&frontmatter, &field.name);
                map.insert(field.name.clone(), Value::String(value));
            }
            Some(qual) if qual == "h2" => {
                let headlines = extract_headlines(body, "## ");
                map.insert(
                    field.name.clone(),
                    Value::Array(headlines.into_iter().map(Value::String).collect()),
                );
            }
            Some(_) => {
                map.insert(field.name.clone(), Value::Null);
            }
        }
    }
    Value::Object(map)
}

fn parse_frontmatter(content: &str) -> (HashMap<String, String>, &str) {
    let mut frontmatter = HashMap::new();

    if !content.starts_with("---") {
        return (frontmatter, content);
    }

    let after_first = &content[3..];
    if let Some(end_idx) = after_first.find("\n---") {
        let fm_text = &after_first[..end_idx];
        let body = &after_first[end_idx + 4..];
        let body = body.strip_prefix('\n').unwrap_or(body);

        for line in fm_text.lines() {
            let line = line.trim();
            if let Some((key, value)) = line.split_once(':') {
                frontmatter.insert(key.trim().to_lowercase(), value.trim().to_string());
            }
        }

        (frontmatter, body)
    } else {
        (frontmatter, content)
    }
}

fn lookup_frontmatter(fm: &HashMap<String, String>, key: &str) -> String {
    fm.get(&key.to_lowercase()).cloned().unwrap_or_default()
}

fn extract_headlines(content: &str, prefix: &str) -> Vec<String> {
    content
        .lines()
        .filter(|line| line.starts_with(prefix))
        .map(|line| line[prefix.len()..].trim().to_string())
        .collect()
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let location = self
            .span
            .as_ref()
            .map(|s| format!(" at {}..{}", s.start, s.end))
            .unwrap_or_default();
        write!(f, "resolve error{}: {}", location, self.message)?;
        for hint in &self.hints {
            write!(f, "\n  hint: {}", hint)?;
        }
        Ok(())
    }
}

impl std::error::Error for ResolveError {}

fn did_you_mean<'a>(name: &str, candidates: &[&'a str]) -> Option<&'a str> {
    let threshold = (name.len() / 3).max(1) + 1;
    candidates
        .iter()
        .map(|&c| (c, edit_distance(name, c)))
        .filter(|(_, d)| *d <= threshold)
        .min_by_key(|(_, d)| *d)
        .map(|(c, _)| c)
}

fn hint_did_you_mean(
    name: &str,
    candidates: &[&str],
    fmt: impl FnOnce(&str) -> String,
) -> Vec<String> {
    did_you_mean(name, candidates)
        .map(fmt)
        .into_iter()
        .collect()
}

fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    let m = a.len();
    let n = b.len();

    let mut dp = vec![vec![0usize; n + 1]; m + 1];

    for (i, row) in dp.iter_mut().enumerate() {
        row[0] = i;
    }
    for (j, cell) in dp[0].iter_mut().enumerate() {
        *cell = j;
    }

    for i in 1..=m {
        for j in 1..=n {
            let cost = if a[i - 1] == b[j - 1] { 0 } else { 1 };
            dp[i][j] = (dp[i - 1][j] + 1)
                .min(dp[i][j - 1] + 1)
                .min(dp[i - 1][j - 1] + cost);
        }
    }

    dp[m][n]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::conversation::Kind;
    use crate::domain::filesystem::{Filesystem, Folder};
    use crate::parse::Parse;
    use crate::prism;
    use crate::Vector;
    use fragmentation::ref_::Ref;
    use fragmentation::sha;

    fn test_ref(label: &str) -> Ref {
        Ref::new(sha::hash(label), label)
    }

    fn leaf_folder(name: &str, content: &str) -> Prism<Folder> {
        prism::shard(
            test_ref(name),
            Folder {
                name: name.into(),
                content: Some(content.into()),
            },
        )
    }

    fn dir_folder(name: &str, children: Vec<Prism<Folder>>) -> Prism<Folder> {
        prism::fractal(
            test_ref(name),
            Folder {
                name: name.into(),
                content: None,
            },
            children,
        )
    }

    /// Shorthand: resolve with Filesystem context.
    fn resolve_fs(ast: Prism<AstNode>) -> crate::Trace<Conversation<Filesystem>, ResolveError> {
        Resolve::new().trace(ast)
    }

    fn find_branch(tree: &Prism<OutputNode>) -> Option<&OutputNode> {
        match tree.data() {
            OutputNode::Branch { .. } => Some(tree.data()),
            _ => tree.children().iter().find_map(find_branch),
        }
    }

    fn expect_branch(node: &OutputNode) -> (&str, &Vec<BranchArm>) {
        match node {
            OutputNode::Branch { query, arms } => (query.as_str(), arms),
            _ => panic!("expected Branch"),
        }
    }

    #[test]
    #[should_panic(expected = "expected Branch")]
    fn expect_branch_panics_on_non_branch() {
        let node = OutputNode::Group {
            name: "test".into(),
        };
        expect_branch(&node);
    }

    // -- OutputNode::name --

    #[test]
    fn output_node_name() {
        let group = OutputNode::Group {
            name: "blog".into(),
        };
        assert_eq!(group.name(), "blog");

        let select = OutputNode::Select {
            output_name: "items".into(),
            folder_name: "sub".into(),
            template_name: "$t".into(),
        };
        assert_eq!(select.name(), "items");
    }

    // -- Resolve valid conv --

    #[test]
    fn resolve_valid_conv() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n\texcerpt\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $corpus }\n\t}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let resolved = resolve_fs(ast).unwrap();
        assert_eq!(resolved.content.data().name(), "blog");
        assert!(resolved.templates.contains_key("$corpus"));
    }

    #[test]
    fn resolve_extracts_template_fields() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n\theadlines: h2\n\thtml: article | @html\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let resolved = resolve_fs(ast).unwrap();
        let tmpl = &resolved.templates["$t"];
        assert_eq!(tmpl.fields.len(), 3);
        assert_eq!(tmpl.fields[0].name, "slug");
        assert!(tmpl.fields[0].qualifier.is_none());
        assert_eq!(tmpl.fields[1].name, "headlines");
        assert_eq!(tmpl.fields[1].qualifier.as_deref(), Some("h2"));
        assert_eq!(tmpl.fields[2].name, "html");
        assert_eq!(tmpl.fields[2].qualifier.as_deref(), Some("article"));
        assert_eq!(tmpl.fields[2].pipe.as_deref(), Some("@html"));
    }

    // -- Error: unknown domain --

    #[test]
    fn resolve_unknown_domain_errors() {
        let source = "in @filesytem\ntemplate $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
        assert!(err.message.contains("filesytem"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest @filesystem");
        assert!(err.hints[0].contains("filesystem"), "{}", err.hints[0]);
    }

    // -- Error: unknown template --

    #[test]
    fn resolve_unknown_template_errors() {
        let source = "in @filesystem\ntemplate $corpus {\n\tslug\n}\nout blog {\n\tpieces {\n\t\tdraft: 1draft { $coprus }\n\t}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
        assert!(err.message.contains("coprus"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest $corpus");
    }

    // -- Error: missing output --

    #[test]
    fn resolve_missing_output_errors() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let err = resolve_fs(ast).into_result().unwrap_err();
        assert!(err.message.contains("output"), "{}", err);
    }

    // -- did_you_mean --

    #[test]
    fn did_you_mean_finds_close_match() {
        let result = did_you_mean("filesytem", &["filesystem", "html", "json"]);
        assert_eq!(result, Some("filesystem"));
    }

    #[test]
    fn did_you_mean_no_match_when_too_far() {
        let result = did_you_mean("xyz", &["filesystem", "html", "json"]);
        assert_eq!(result, None);
    }

    // -- edit_distance --

    #[test]
    fn edit_distance_identical() {
        assert_eq!(edit_distance("abc", "abc"), 0);
    }

    #[test]
    fn edit_distance_single_edit() {
        assert_eq!(edit_distance("filesystem", "filesytem"), 1);
    }

    #[test]
    fn edit_distance_empty() {
        assert_eq!(edit_distance("", "abc"), 3);
        assert_eq!(edit_distance("abc", ""), 3);
    }

    // -- Display --

    #[test]
    fn resolve_error_display_with_hints() {
        let err = ResolveError {
            message: "unknown domain @filesytem".into(),
            span: Some(Span::new(3, 15)),
            hints: vec!["did you mean @filesystem?".into()],
        };
        let display = format!("{}", err);
        assert!(display.contains("unknown domain"), "{}", display);
        assert!(display.contains("did you mean"), "{}", display);
    }

    #[test]
    fn resolve_error_display_without_hints() {
        let err = ResolveError {
            message: "missing output block".into(),
            span: None,
            hints: vec![],
        };
        let display = format!("{}", err);
        assert!(display.contains("missing output"), "{}", display);
    }

    // -- Execute --

    #[test]
    fn emit_produces_correct_output() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder(
                    "post.md",
                    "---\nslug: hello-world\n---\nContent here",
                )],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        let items = result["root"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hello-world");
    }

    #[test]
    fn emit_missing_child_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\tmissing {\n\t\titems: sub { $t }\n\t}\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    #[test]
    fn emit_headlines_qualifier() {
        let source = "in @filesystem\ntemplate $t {\n\theadlines: h2\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("post.md", "## First\n## Second\n")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        let headlines = result["root"]["items"][0]["headlines"].as_array().unwrap();
        assert_eq!(headlines.len(), 2);
        assert_eq!(headlines[0], "First");
        assert_eq!(headlines[1], "Second");
    }

    #[test]
    fn emit_unknown_qualifier_produces_null() {
        let source =
            "in @filesystem\ntemplate $t {\n\tfield: unknown\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();

        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nfield: val\n---\n")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"]["items"][0]["field"].is_null());
    }

    // -- with_domain + Default --

    #[test]
    fn with_domain_registers_external() {
        let resolve = Resolve::new().with_domain("html");
        let source = "in @html\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved: Conversation<Filesystem> = resolve.trace(ast).unwrap();
    }

    #[test]
    fn default_same_as_new() {
        let source = "in @filesystem\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved: Conversation<Filesystem> = Resolve::default().trace(ast).unwrap();
    }

    // -- Error: missing domain declaration --

    #[test]
    fn resolve_unknown_domain_suggests_external() {
        let resolve = Resolve::new().with_domain("graphql");
        let source = "in @graphq\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(err.message.contains("graphq"), "{}", err);
        assert!(!err.hints.is_empty(), "should suggest @graphql");
        assert!(err.hints[0].contains("graphql"), "{}", err.hints[0]);
    }

    #[test]
    fn resolve_missing_in_declaration_still_resolves() {
        let source = "template $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Emit: missing folder in select skips --

    #[test]
    fn emit_missing_folder_in_select_skips() {
        let source = "in @filesystem\ntemplate $t {\n\tname\n}\nout root {\n\titems: nonexistent { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder("root", vec![]);
        let result = resolved.trace(tree).unwrap();
        assert!(result["root"].as_object().unwrap().is_empty());
    }

    // -- Emit: group with matching child --

    #[test]
    fn emit_group_with_child() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\tsub {\n\t\titems: data { $t }\n\t}\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![dir_folder(
                    "data",
                    vec![leaf_folder("a.md", "---\nslug: hi\n---\n")],
                )],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        let items = result["root"]["sub"]["items"].as_array().unwrap();
        assert_eq!(items[0]["slug"], "hi");
    }

    // -- Frontmatter: unclosed delimiter --

    #[test]
    fn emit_frontmatter_unclosed_returns_empty() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\n";
        let resolved = resolve_fs(Parse.trace(source.to_string()).unwrap())
            .into_result()
            .unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("f.md", "---\nslug: test\nNo closing")],
            )],
        );
        let result = resolved.trace(tree).unwrap();
        // Unclosed frontmatter returns empty fields
        assert_eq!(result["root"]["items"][0]["slug"], "");
    }

    // -- Resolver ignores when clauses --

    #[test]
    fn resolve_with_when_clause_succeeds() {
        let source = "in @filesystem\nwhen error.rate > 0.1\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Resolve ignores unknown AST children --

    #[test]
    fn resolve_template_ignores_unknown_child_kinds() {
        use crate::ast;
        // Manually construct an AST with a non-field child in a template
        let root = ast::ast_branch(
            Kind::Form,
            "group",
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Kind::Decl, "in", "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Decl,
                    "template",
                    "$t",
                    Span::new(15, 35),
                    vec![
                        ast::ast_leaf(Kind::Atom, "field", "slug", Span::new(20, 24)),
                        // A DomainRef in a template — should be ignored
                        ast::ast_leaf(Kind::Ref, "domain-ref", "@html", Span::new(25, 30)),
                    ],
                ),
                ast::ast_branch(
                    Kind::Decl,
                    "out",
                    "r",
                    Span::new(36, 50),
                    vec![ast::ast_branch(
                        Kind::Form,
                        "select",
                        "x",
                        Span::new(40, 48),
                        vec![
                            ast::ast_leaf(Kind::Ref, "domain-ref", "f", Span::new(42, 43)),
                            ast::ast_leaf(Kind::Ref, "template-ref", "$t", Span::new(44, 46)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        // Only the Field child is extracted, DomainRef is ignored
        assert_eq!(resolved.templates["$t"].fields.len(), 1);
    }

    #[test]
    fn resolve_output_ignores_unknown_child_kinds() {
        use crate::ast;
        // AST with an In node inside an Out block — should be ignored
        let root = ast::ast_branch(
            Kind::Form,
            "group",
            "root",
            Span::new(0, 50),
            vec![
                ast::ast_leaf(Kind::Decl, "in", "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Decl,
                    "out",
                    "r",
                    Span::new(15, 50),
                    vec![
                        ast::ast_leaf(Kind::Decl, "in", "@html", Span::new(20, 25)),
                        ast::ast_branch(Kind::Form, "group", "g", Span::new(26, 40), vec![]),
                    ],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        // Only the Group child is extracted, In is ignored
        assert_eq!(resolved.content.children().len(), 1);
    }

    // -- Template field with only qualifier, no pipe (branch form) --

    #[test]
    fn resolve_field_branch_with_unknown_sub_kind() {
        use crate::ast;
        // A Field branch where one child has kind Group — should be ignored
        let root = ast::ast_branch(
            Kind::Form,
            "group",
            "root",
            Span::new(0, 80),
            vec![
                ast::ast_leaf(Kind::Decl, "in", "@filesystem", Span::new(0, 14)),
                ast::ast_branch(
                    Kind::Decl,
                    "template",
                    "$t",
                    Span::new(15, 60),
                    vec![ast::ast_branch(
                        Kind::Atom,
                        "field",
                        "headlines",
                        Span::new(20, 40),
                        vec![
                            ast::ast_leaf(Kind::Atom, "qualifier", "h2", Span::new(25, 27)),
                            // Group as a child of Field — unusual, should be skipped
                            ast::ast_branch(
                                Kind::Form,
                                "group",
                                "noise",
                                Span::new(28, 35),
                                vec![],
                            ),
                        ],
                    )],
                ),
                ast::ast_branch(
                    Kind::Decl,
                    "out",
                    "r",
                    Span::new(61, 80),
                    vec![ast::ast_branch(
                        Kind::Form,
                        "select",
                        "x",
                        Span::new(65, 78),
                        vec![
                            ast::ast_leaf(Kind::Ref, "domain-ref", "f", Span::new(67, 68)),
                            ast::ast_leaf(Kind::Ref, "template-ref", "$t", Span::new(69, 71)),
                        ],
                    )],
                ),
            ],
        );
        let resolved = resolve_fs(root).unwrap();
        let field = &resolved.templates["$t"].fields[0];
        assert_eq!(field.qualifier.as_deref(), Some("h2"));
        assert!(field.pipe.is_none());
    }

    // -- ContentAddressed --

    #[test]
    fn conversation_content_addressed() {
        use crate::ContentAddressed;
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout blog {\n\titems: sub { $t }\n}\n";
        let a = Conversation::<Filesystem>::from_source(source).unwrap();
        let b = Conversation::<Filesystem>::from_source(source).unwrap();
        assert_eq!(a.content_oid(), b.content_oid());
    }

    #[test]
    fn value_content_addressed() {
        use crate::ContentAddressed;
        let a = Value::String("hello".into());
        let b = Value::String("hello".into());
        assert_eq!(a.content_oid(), b.content_oid());

        let c = Value::String("world".into());
        assert_ne!(a.content_oid(), c.content_oid());
    }

    // -- Composition: Parse → Resolve --

    #[test]
    fn parse_then_resolve_composes() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        // Parse → Resolve composes via explicit chaining
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Bridge: from_source --

    #[test]
    fn from_source_parses_and_resolves() {
        let source =
            "in @filesystem\ntemplate $t {\n\tslug\n}\nout blog {\n\titems: sub { $t }\n}\n";
        let resolved = Conversation::<Filesystem>::from_source(source).unwrap();
        assert_eq!(resolved.content.data().name(), "blog");
        assert!(resolved.templates.contains_key("$t"));
    }

    /// Shared extractor — single monomorphization, both arms covered
    /// across from_source_propagates_parse_error / _resolve_error.
    fn extract_composed_message(
        err: ComposedError<crate::parse::ParseError, ResolveError>,
    ) -> (bool, String) {
        match err {
            ComposedError::First(pe) => (true, pe.message),
            ComposedError::Second(re) => (false, re.message),
        }
    }

    #[test]
    fn from_source_propagates_parse_error() {
        let err = Conversation::<Filesystem>::from_source("garbage\n").unwrap_err();
        let (is_parse, msg) = extract_composed_message(err);
        assert!(is_parse);
        assert!(msg.contains("unexpected"), "{}", msg);
    }

    #[test]
    fn from_source_propagates_resolve_error() {
        let err =
            Conversation::<Filesystem>::from_source("in @bogus\nout r {\n\tx {}\n}\n").unwrap_err();
        let (is_parse, msg) = extract_composed_message(err);
        assert!(!is_parse);
        assert!(msg.contains("bogus"), "{}", msg);
    }

    // -- Litmus: test-only domain proves Conversation<C> is generic --

    #[test]
    fn conversation_generic_over_settings() {
        use test_domain::{TestDomain, TestToken};

        // Grammar declaration registers @test as a known domain
        let source = "grammar @test {\n  type = item | collection\n}\nin @test\ntemplate $t {\n\tname\n}\nout root {\n\titems: data { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let resolved: Conversation<TestDomain> = Resolve::new().trace(ast).unwrap();

        let item = prism::shard(
            test_ref("item-1"),
            TestToken {
                name: "item-1".into(),
                content: Some("hello".into()),
            },
        );
        let data = prism::fractal(
            test_ref("data"),
            TestToken {
                name: "data".into(),
                content: None,
            },
            vec![item],
        );
        let root = prism::fractal(
            test_ref("root"),
            TestToken {
                name: "root".into(),
                content: None,
            },
            vec![data],
        );

        let _result = resolved.trace(root).unwrap();

        // Exercise trait impls for coverage — monomorphized code must be called
        use crate::ContentAddressed;
        use fragmentation::encoding::Encode;
        assert_eq!(TestDomain::id(), "test");
        let with = TestToken {
            name: "a".into(),
            content: Some("b".into()),
        };
        let without = TestToken {
            name: "a".into(),
            content: None,
        };
        assert_ne!(with.content_oid(), without.content_oid());
        assert_ne!(with.encode(), without.encode());
    }

    // -- branch resolution --

    #[test]
    fn resolve_branch_produces_output_node() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let conv = Conversation::<Filesystem>::from_source(source).unwrap();
        // Branch should be present in the conversation's output tree or as a separate structure
        // For now, verify parsing + resolution doesn't fail
        assert!(!conv.content.data().name().is_empty());
    }

    #[test]
    fn resolve_branch_node_has_arms() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.action) {\n  \"hold\" => ..\n  \"exit\" => exit\n}\n";
        let conv = Conversation::<Filesystem>::from_source(source).unwrap();

        let branch = find_branch(&conv.content).expect("should have a Branch node");
        let (query, arms) = expect_branch(branch);
        assert_eq!(query, ".action");
        assert_eq!(arms.len(), 2);
        assert!(matches!(&arms[0].pattern, BranchPattern::Literal(s) if s == "hold"));
        assert!(matches!(&arms[0].action, BranchAction::Pass));
        assert!(matches!(&arms[1].pattern, BranchPattern::Literal(s) if s == "exit"));
        assert!(matches!(&arms[1].action, BranchAction::Exit));
    }

    #[test]
    fn output_node_branch_name() {
        let node = OutputNode::Branch {
            query: ".action".into(),
            arms: vec![],
        };
        assert_eq!(node.name(), "branch");
    }

    #[test]
    fn output_node_branch_encode() {
        use fragmentation::encoding::Encode;

        // Exercise all three Encode arms in one test to avoid
        // codegen-unit monomorphization coverage gaps.
        let group = OutputNode::Group {
            name: "root".into(),
        };
        assert!(String::from_utf8(group.encode())
            .unwrap()
            .contains("group:root"));

        let select = OutputNode::Select {
            output_name: "items".into(),
            folder_name: "sub".into(),
            template_name: "$t".into(),
        };
        assert!(String::from_utf8(select.encode())
            .unwrap()
            .contains("select:items:sub:$t"));

        let branch = OutputNode::Branch {
            query: ".action".into(),
            arms: vec![
                BranchArm {
                    pattern: BranchPattern::Literal("hold".into()),
                    action: BranchAction::Pass,
                },
                BranchArm {
                    pattern: BranchPattern::Wild,
                    action: BranchAction::Exit,
                },
                BranchArm {
                    pattern: BranchPattern::Literal("custom".into()),
                    action: BranchAction::Expr("handle".into()),
                },
            ],
        };
        let encoded = branch.encode();
        let s = String::from_utf8(encoded).unwrap();
        assert!(s.contains("branch"));
        assert!(s.contains(".action"));
        assert!(s.contains("_ => exit"));
        assert!(s.contains("handle"));
    }

    #[test]
    fn resolve_branch_with_wild_and_expr() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.status) {\n  \"ok\" => ..\n  \"custom\" => handle\n  _ => exit\n}\n";
        let conv = Conversation::<Filesystem>::from_source(source).unwrap();

        let branch = find_branch(&conv.content).expect("should have a Branch node");
        let (query, arms) = expect_branch(branch);
        assert_eq!(query, ".status");
        assert_eq!(arms.len(), 3);
        assert!(matches!(&arms[0].pattern, BranchPattern::Literal(s) if s == "ok"));
        assert!(matches!(&arms[0].action, BranchAction::Pass));
        assert!(matches!(&arms[1].pattern, BranchPattern::Literal(s) if s == "custom"));
        assert!(matches!(&arms[1].action, BranchAction::Expr(e) if e == "handle"));
        assert!(matches!(&arms[2].pattern, BranchPattern::Wild));
        assert!(matches!(&arms[2].action, BranchAction::Exit));
    }

    #[test]
    fn resolve_branch_skips_non_arm_children() {
        use crate::ast::{self, Span};

        // Build a Branch AST node with a non-Arm child (should be skipped)
        let span = Span::new(0, 10);
        let non_arm = ast::ast_leaf(Kind::Atom, "expr", "junk", span);
        let branch_ast = ast::ast_branch(Kind::Decl, "branch", ".x", span, vec![non_arm]);
        let result = resolve_branch_node(&branch_ast);
        let (_, arms) = expect_branch(result.data());
        assert!(arms.is_empty());
    }

    #[test]
    fn resolve_branch_skips_short_arm() {
        use crate::ast::{self, Span};

        // Arm with only one child (too short — needs pattern + action)
        let span = Span::new(0, 10);
        let pattern_only = ast::ast_leaf(Kind::Atom, "literal", "x", span);
        let short_arm = ast::ast_branch(Kind::Form, "arm", "", span, vec![pattern_only]);
        let branch_ast = ast::ast_branch(Kind::Decl, "branch", ".x", span, vec![short_arm]);
        let result = resolve_branch_node(&branch_ast);
        let (_, arms) = expect_branch(result.data());
        assert!(arms.is_empty());
    }

    #[test]
    fn resolve_branch_skips_unknown_pattern_kind() {
        use crate::ast::{self, Span};

        // Arm with an Expr pattern (not Literal or Wild — should be skipped)
        let span = Span::new(0, 10);
        let bad_pattern = ast::ast_leaf(Kind::Atom, "expr", "nope", span);
        let action = ast::ast_leaf(Kind::Atom, "expr", "..", span);
        let arm = ast::ast_branch(Kind::Form, "arm", "", span, vec![bad_pattern, action]);
        let branch_ast = ast::ast_branch(Kind::Decl, "branch", ".x", span, vec![arm]);
        let result = resolve_branch_node(&branch_ast);
        let (_, arms) = expect_branch(result.data());
        assert!(arms.is_empty());
    }

    #[test]
    fn emit_body_with_branch_produces_output() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout root {\n\titems: sub { $t }\n}\nbranch(.action) {\n  \"hold\" => ..\n}\n";
        let conv = Conversation::<Filesystem>::from_source(source).unwrap();
        let tree = dir_folder(
            "root",
            vec![dir_folder(
                "sub",
                vec![leaf_folder("post.md", "---\nslug: hello\n---\nContent")],
            )],
        );
        let result = conv.trace(tree).unwrap();
        // Branch node doesn't add to JSON output — just verify no panic
        assert!(result.is_object());
    }

    // -- Namespace + Use resolution --

    fn make_namespace_with_template(module: &str, tmpl_name: &str) -> Namespace {
        let mut templates = HashMap::new();
        templates.insert(
            tmpl_name.to_string(),
            Template {
                params: Vec::new(),
                fields: vec![Field {
                    name: "slug".into(),
                    qualifier: None,
                    pipe: None,
                }],
            },
        );
        let mut ns = Namespace::new();
        ns.register(module, TemplateProvider::Inline(templates));
        ns
    }

    #[test]
    fn resolve_use_imports_template() {
        let ns = make_namespace_with_template("shared", "$t");
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use $t from @shared\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let conv: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
        // The imported template should be available
        assert!(
            conv.templates.contains_key("$t"),
            "imported template $t should be in templates"
        );
    }

    #[test]
    fn resolve_use_unknown_source() {
        let resolve = Resolve::new();
        let source = "use $t from @missing\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(
            err.message.contains("missing"),
            "should mention missing source: {}",
            err
        );
    }

    #[test]
    fn resolve_use_missing_template() {
        let ns = make_namespace_with_template("shared", "$other");
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use $t from @shared\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(
            err.message.contains("$t") && err.message.contains("not found"),
            "should say template not found: {}",
            err
        );
    }

    #[test]
    fn resolve_use_destructured() {
        let mut templates = HashMap::new();
        templates.insert(
            "$a".to_string(),
            Template {
                params: Vec::new(),
                fields: vec![Field {
                    name: "x".into(),
                    qualifier: None,
                    pipe: None,
                }],
            },
        );
        templates.insert(
            "$b".to_string(),
            Template {
                params: Vec::new(),
                fields: vec![Field {
                    name: "y".into(),
                    qualifier: None,
                    pipe: None,
                }],
            },
        );
        let mut ns = Namespace::new();
        ns.register("shared", TemplateProvider::Inline(templates));
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use { $a, $b } from @shared\nout r {\n\tx: f { $a }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let conv: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
        assert!(conv.templates.contains_key("$a"), "should import $a");
        assert!(conv.templates.contains_key("$b"), "should import $b");
    }

    #[test]
    fn resolve_use_overrides_local_errors() {
        let ns = make_namespace_with_template("shared", "$t");
        let resolve = Resolve::new().with_namespace(ns);
        // Both local template $t AND use $t from @shared — should error
        let source = "use $t from @shared\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(
            err.message.contains("conflicts"),
            "should mention conflict: {}",
            err
        );
    }

    #[test]
    fn resolve_namespace_replaces_known_domains() {
        // in @filesystem should still validate even with namespace
        let ns = make_namespace_with_template("shared", "$t");
        let resolve = Resolve::new().with_namespace(ns);
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
    }

    #[test]
    fn resolve_with_domain_registers_module() {
        // with_domain("custom") → in @custom validates
        let resolve = Resolve::new().with_domain("custom");
        let source = "in @custom\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
    }

    #[test]
    fn resolve_in_validates_via_namespace() {
        // Register filesystem in namespace instead of relying on KNOWN_DOMAINS
        let mut ns = Namespace::new();
        ns.register("custom_domain", TemplateProvider::Inline(HashMap::new()));
        let resolve = Resolve::new().with_namespace(ns);
        let source = "in @custom_domain\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
    }

    #[test]
    fn resolve_in_unknown_via_namespace() {
        let mut ns = Namespace::new();
        ns.register("shared", TemplateProvider::Inline(HashMap::new()));
        let resolve = Resolve::new().with_namespace(ns);
        let source = "in @share\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(err.message.contains("share"), "{}", err);
        assert!(
            !err.hints.is_empty(),
            "should suggest @shared from namespace"
        );
        assert!(err.hints[0].contains("shared"), "{}", err.hints[0]);
    }

    #[test]
    fn resolve_use_home_path_noop() {
        // $HOME path without DomainRef → early return Ok (no resolution yet)
        let resolve = Resolve::new();
        let source = "use $t from $HOME/shared\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        // No error — $HOME use silently returns Ok for now
        let _: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
    }

    #[test]
    fn resolve_use_missing_template_did_you_mean() {
        // Close misspelling triggers did-you-mean hint
        let mut templates = HashMap::new();
        templates.insert("$corpus".to_string(), Template::with_fields(&["slug"]));
        let mut ns = Namespace::new();
        ns.register("shared", TemplateProvider::Inline(templates));
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use $corpu from @shared\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(err.message.contains("$corpu"), "{}", err);
        assert!(err.message.contains("not found"), "{}", err);
        assert!(!err.hints.is_empty(), "should have did-you-mean hint");
        assert!(err.hints[0].contains("$corpus"), "{}", err.hints[0]);
    }

    #[test]
    fn resolve_use_unknown_source_did_you_mean() {
        // Close misspelling of namespace module triggers did-you-mean
        let ns = make_namespace_with_template("shared", "$t");
        let resolve = Resolve::new().with_namespace(ns);
        let source = "use $t from @share\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result: Result<Conversation<Filesystem>, _> = resolve.trace(ast).into_result();
        let err = result.unwrap_err();
        assert!(err.message.contains("share"), "{}", err);
        assert!(!err.hints.is_empty(), "should have did-you-mean hint");
        assert!(err.hints[0].contains("shared"), "{}", err.hints[0]);
    }

    #[test]
    fn resolve_use_known_domain_no_templates() {
        // Use from a known domain (filesystem) that has no templates in namespace
        // → no error, just no templates imported
        let resolve = Resolve::new();
        let source =
            "in @filesystem\nuse $t from @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        // Should succeed — @filesystem is known but has no templates to import
        let _: Conversation<Filesystem> = resolve.trace(ast).into_result().unwrap();
    }

    #[test]
    fn namespace_default() {
        let ns = Namespace::default();
        assert!(ns.module_names().is_empty());
    }

    #[test]
    fn namespace_contains_and_names() {
        let ns = make_namespace_with_template("shared", "$t");
        assert!(ns.contains("shared"));
        assert!(!ns.contains("other"));
        assert!(ns.module_names().contains(&"shared"));
    }

    #[test]
    fn namespace_get_templates_external_returns_none() {
        let mut ns = Namespace::new();
        ns.register("ext", TemplateProvider::External("file.conv".into()));
        assert!(ns.get_templates("ext").is_none());
    }

    #[test]
    fn template_provider_clone() {
        let provider = TemplateProvider::Inline(HashMap::new());
        let _cloned = provider.clone();
        let ext = TemplateProvider::External("path".into());
        let _cloned_ext = ext.clone();
    }

    // -- Grammar passthrough --

    #[test]
    fn resolve_with_grammar_passthrough() {
        let source = "in @filesystem\ngrammar @conversation {\n  type = in | out\n}\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let _resolved = resolve_fs(ast).unwrap();
    }

    // -- Parameterized templates --

    #[test]
    fn resolve_template_with_params() {
        let source = "in @filesystem\ntemplate $t(@json, data: @csv) {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let conv = resolve_fs(ast).unwrap();
        let tmpl = &conv.templates["$t"];
        assert_eq!(tmpl.params.len(), 2);
        assert_eq!(tmpl.params[0].name, "json");
        assert_eq!(tmpl.params[1].name, "data");
        assert_eq!(tmpl.fields.len(), 1);
    }

    #[test]
    fn resolve_template_without_params_compat() {
        let source = "in @filesystem\ntemplate $t {\n\tslug\n}\nout r {\n\tx: f { $t }\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let conv = resolve_fs(ast).unwrap();
        let tmpl = &conv.templates["$t"];
        assert!(tmpl.params.is_empty());
        assert_eq!(tmpl.fields.len(), 1);
    }

    // -- TypeRegistry --

    /// Parse a grammar source and compile its TypeRegistry.
    fn compile_grammar(source: &str) -> TypeRegistry {
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("source must contain a grammar block");
        TypeRegistry::compile(grammar).unwrap()
    }

    #[test]
    fn type_registry_compile_anonymous_type() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        assert_eq!(reg.domain, "test");
        assert!(reg.has_type(""));
        assert!(reg.has_variant("", "a"));
        assert!(reg.has_variant("", "b"));
        assert!(reg.has_variant("", "c"));
        assert!(!reg.has_variant("", "d"));
    }

    #[test]
    fn type_registry_compile_named_type() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        assert!(reg.has_type("op"));
        assert!(reg.has_variant("op", "gt"));
        assert!(reg.has_variant("op", "lt"));
        assert!(!reg.has_variant("op", "eq"));
    }

    #[test]
    fn type_registry_compile_parameterized_variant() {
        let reg =
            compile_grammar("grammar @test {\n  type = plain | when(op)\n  type op = gt | lt\n}\n");
        assert!(reg.has_variant("", "when"));
        assert!(reg.has_variant("", "plain"));
        // The param reference should be recorded
        assert!(reg
            .params
            .contains_key(&("".to_string(), "when".to_string())));
        assert_eq!(reg.params[&("".to_string(), "when".to_string())], "op");
    }

    #[test]
    fn type_registry_compile_invalid_type_ref() {
        let source = "grammar @test {\n  type = when(ops)\n  type op = gt | lt\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("grammar");
        let err = TypeRegistry::compile(grammar).unwrap_err();
        assert!(
            err.message.contains("ops"),
            "should mention bad ref: {}",
            err
        );
        assert!(
            err.message.contains("@test"),
            "should mention domain: {}",
            err
        );
        assert!(!err.hints.is_empty(), "should suggest 'op'");
        assert!(err.hints[0].contains("op"), "{}", err.hints[0]);
    }

    #[test]
    fn type_registry_compile_invalid_named_type_ref() {
        // Bad ref in a NAMED type (exercises the else branch in the error message)
        let source =
            "grammar @test {\n  type color = red(shades)\n  type shade = light | dark\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("grammar");
        let err = TypeRegistry::compile(grammar).unwrap_err();
        assert!(err.message.contains("shades"), "{}", err);
        assert!(
            err.message.contains("color"),
            "should mention parent type: {}",
            err
        );
    }

    #[test]
    fn type_registry_compile_skips_non_typedef_children() {
        use crate::ast::{self, Span};
        // Grammar node with a non-TypeDef child (Field) — should be skipped
        let span = Span::new(0, 50);
        let stray_child = ast::ast_leaf(Kind::Atom, "field", "noise", span);
        let variant = ast::ast_leaf(Kind::Form, "variant", "a", span);
        let typedef = ast::ast_branch(Kind::Form, "type-def", "", span, vec![variant]);
        let grammar = ast::ast_branch(
            Kind::Decl,
            "grammar",
            "@test",
            span,
            vec![stray_child, typedef],
        );
        let reg = TypeRegistry::compile(&grammar).unwrap();
        assert!(reg.has_variant("", "a"));
    }

    #[test]
    fn type_registry_compile_skips_non_variant_children() {
        use crate::ast::{self, Span};
        // TypeDef with a non-Variant child (Field) — should be skipped
        let span = Span::new(0, 50);
        let stray = ast::ast_leaf(Kind::Atom, "field", "noise", span);
        let variant = ast::ast_leaf(Kind::Form, "variant", "a", span);
        let typedef = ast::ast_branch(Kind::Form, "type-def", "", span, vec![stray, variant]);
        let grammar = ast::ast_branch(Kind::Decl, "grammar", "@test", span, vec![typedef]);
        let reg = TypeRegistry::compile(&grammar).unwrap();
        assert!(reg.has_variant("", "a"));
        // Only 1 variant, not 2
        assert_eq!(reg.types[""].len(), 1);
    }

    #[test]
    fn type_registry_compile_empty_grammar() {
        let reg = compile_grammar("grammar @empty {}\n");
        assert_eq!(reg.domain, "empty");
        assert!(!reg.has_type(""));
    }

    #[test]
    fn type_registry_validate_type_ref_ok() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        assert!(reg.validate_type_ref("op").is_ok());
        assert!(reg.validate_type_ref("").is_ok());
    }

    #[test]
    fn type_registry_validate_type_ref_error() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let err = reg.validate_type_ref("ops").unwrap_err();
        assert!(err.message.contains("ops"), "{}", err);
        assert!(!err.hints.is_empty());
        assert!(err.hints[0].contains("op"), "{}", err.hints[0]);
    }

    #[test]
    fn type_registry_has_type_false_for_missing() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(!reg.has_type("missing"));
    }

    #[test]
    fn type_registry_has_variant_false_for_missing_type() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(!reg.has_variant("missing", "a"));
    }

    // -- TypeRegistry: action compilation --

    #[test]
    fn type_registry_compile_action() {
        let reg = compile_grammar(
            "grammar @test {\n  type address = email | uri\n  action send {\n    to: address\n  }\n}\n",
        );
        assert!(reg.has_action("send"));
        let fields = reg.action_fields("send").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, "to");
        assert_eq!(fields[0].1, Some("address".to_string()));
    }

    #[test]
    fn type_registry_compile_action_untyped_field() {
        let reg = compile_grammar("grammar @test {\n  action send {\n    subject\n  }\n}\n");
        assert!(reg.has_action("send"));
        let fields = reg.action_fields("send").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, "subject");
        assert_eq!(fields[0].1, None);
    }

    #[test]
    fn type_registry_compile_action_unvalidated_type_ref() {
        // Action field type-refs are semantic annotations — not validated against type names.
        // This mirrors real usage: garden's @mail uses `from: address` where `address`
        // is a variant, and `body: article` which is undeclared.
        let reg = compile_grammar(
            "grammar @test {\n  type address = email | uri\n  action send {\n    to: addres\n    body: article\n  }\n}\n",
        );
        assert!(reg.has_action("send"));
        let fields = reg.action_fields("send").unwrap();
        assert_eq!(fields[0], ("to".into(), Some("addres".into())));
        assert_eq!(fields[1], ("body".into(), Some("article".into())));
    }

    #[test]
    fn type_registry_compile_action_empty() {
        let reg = compile_grammar("grammar @test {\n  action noop {}\n}\n");
        assert!(reg.has_action("noop"));
        let fields = reg.action_fields("noop").unwrap();
        assert!(fields.is_empty());
    }

    #[test]
    fn type_registry_compile_action_skips_non_field_children() {
        use crate::ast::{self, Span};
        // Action-def with a non-field child — should be skipped
        let span = Span::new(0, 50);
        let stray = ast::ast_leaf(Kind::Ref, "type-ref", "noise", span);
        let field = ast::ast_leaf(Kind::Atom, "field", "to", span);
        let actiondef = ast::ast_branch(Kind::Form, "action-def", "send", span, vec![stray, field]);
        let grammar = ast::ast_branch(Kind::Decl, "grammar", "@test", span, vec![actiondef]);
        let reg = TypeRegistry::compile(&grammar).unwrap();
        assert!(reg.has_action("send"));
        let fields = reg.action_fields("send").unwrap();
        assert_eq!(fields.len(), 1);
        assert_eq!(fields[0].0, "to");
    }

    #[test]
    fn type_registry_has_action_false_for_missing() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(!reg.has_action("missing"));
    }

    #[test]
    fn type_registry_action_fields_none_for_missing() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(reg.action_fields("missing").is_none());
    }

    #[test]
    fn type_registry_compile_mail_conv() {
        let reg = compile_grammar(include_str!("../conv/mail.conv"));
        assert_eq!(reg.domain, "mail");
        // Types
        assert!(reg.has_variant("", "message"));
        assert!(reg.has_variant("", "server"));
        assert!(reg.has_variant("header", "from"));
        assert!(reg.has_variant("flag", "seen"));
        assert!(reg.has_variant("protocol", "jmap"));
        assert!(reg.has_variant("server", "stalwart"));
        assert!(reg.has_variant("dns", "dkim"));
        // Actions
        assert!(reg.has_action("send"));
        assert!(reg.has_action("reply"));
        assert!(reg.has_action("forward"));
        let send = reg.action_fields("send").unwrap();
        assert_eq!(send.len(), 4);
        assert_eq!(send[0], ("from".into(), Some("address".into())));
        assert_eq!(send[2], ("subject".into(), None));
        let forward = reg.action_fields("forward").unwrap();
        assert_eq!(forward.len(), 2);
        assert_eq!(forward[0], ("message".into(), Some("message-id".into())));
    }

    #[test]
    fn type_registry_compile_main_conv() {
        // Compile the actual main.conv grammar — the self-describing vocabulary
        let main_conv = include_str!("../main.conv");
        let reg = compile_grammar(main_conv);
        assert_eq!(reg.domain, "conversation");
        // Verify the anonymous type has the expected vocabulary
        assert!(reg.has_variant("", "in"));
        assert!(reg.has_variant("", "out"));
        assert!(reg.has_variant("", "template"));
        assert!(reg.has_variant("", "when"));
        assert!(reg.has_variant("", "cmp"));
        // Named type 'op' should exist
        assert!(reg.has_type("op"));
        assert!(reg.has_variant("op", "gt"));
        assert!(reg.has_variant("op", "ne"));
        // Parameterized variants reference 'op'
        assert_eq!(reg.params[&("".to_string(), "when".to_string())], "op");
        assert_eq!(reg.params[&("".to_string(), "cmp".to_string())], "op");
    }

    #[test]
    fn resolve_catches_bad_grammar_type_ref() {
        // A .conv source with a bad TypeRef should fail during resolution
        let source = "grammar @test {\n  type = when(ops)\n  type op = gt | lt\n}\nin @test\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let result = resolve_fs(ast).into_result();
        let err = result.unwrap_err();
        assert!(
            err.message.contains("ops"),
            "should mention bad ref 'ops': {}",
            err
        );
        assert!(!err.hints.is_empty(), "should suggest 'op'");
    }

    #[test]
    fn type_registry_multi_grammar() {
        let source = "grammar @first {\n  type = a | b\n}\ngrammar @second {\n  type = x | y\n}\nin @first\nout r {\n\tx {}\n}\n";
        let ast = Parse.trace(source.to_string()).unwrap();
        let grammars: Vec<_> = ast
            .children()
            .iter()
            .filter(|c| c.data().is_decl("grammar"))
            .collect();
        assert_eq!(grammars.len(), 2);

        let reg1 = TypeRegistry::compile(grammars[0]).unwrap();
        let reg2 = TypeRegistry::compile(grammars[1]).unwrap();
        assert_eq!(reg1.domain, "first");
        assert_eq!(reg2.domain, "second");
        assert!(reg1.has_variant("", "a"));
        assert!(reg2.has_variant("", "x"));
    }

    /// Test-only domain for proving Conversation<C> polymorphism.
    mod test_domain {
        use crate::domain::{Addressable, Setting};
        use crate::ContentAddressed;
        use sha2::{Digest, Sha256};

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct TestDomain;

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub struct TestToken {
            pub name: String,
            pub content: Option<String>,
        }

        impl Setting for TestDomain {
            type Token = TestToken;

            fn id() -> &'static str {
                "test"
            }
        }

        impl ContentAddressed for TestToken {
            type Oid = crate::Oid;
            fn content_oid(&self) -> crate::Oid {
                let mut hasher = Sha256::new();
                hasher.update(b"test-token:");
                hasher.update(self.name.as_bytes());
                if let Some(c) = &self.content {
                    hasher.update(b":");
                    hasher.update(c.as_bytes());
                }
                crate::Oid::new(hex::encode(hasher.finalize()))
            }
        }

        impl Addressable for TestToken {
            fn node_name(&self) -> &str {
                &self.name
            }

            fn node_content(&self) -> Option<&str> {
                self.content.as_deref()
            }
        }

        impl fragmentation::encoding::Encode for TestToken {
            fn encode(&self) -> Vec<u8> {
                let mut bytes = self.name.as_bytes().to_vec();
                if let Some(c) = &self.content {
                    bytes.push(b':');
                    bytes.extend_from_slice(c.as_bytes());
                }
                bytes
            }
        }
    }

    // -- from_source_with --

    /// Build a namespace with a grammar and module registered.
    fn namespace_with_grammar(source: &str, name: &str) -> Namespace {
        let mut namespace = Namespace::new();
        let ast = Parse.trace(source.to_string()).unwrap();
        for child in ast.children() {
            if child.data().is_decl("grammar") {
                let registry = TypeRegistry::compile(child).unwrap();
                let domain = registry.domain.clone();
                namespace.register_grammar(&domain, registry);
            }
        }
        // Extract templates from the source too
        let mut templates = HashMap::new();
        for child in ast.children() {
            if child.data().is_decl("template") {
                let tmpl_name = child.data().value.clone();
                templates.insert(tmpl_name, resolve_template(child));
            }
        }
        namespace.register(name, TemplateProvider::Inline(templates));
        namespace
    }

    #[test]
    fn from_source_with_namespace_domain() {
        // A source that uses `in @beam` should resolve when beam is in the namespace
        let namespace = namespace_with_grammar(
            "grammar @beam {\n  type = process | supervision | module\n}\n",
            "beam",
        );
        let resolve = Resolve::new().with_namespace(namespace);

        let source = "in @beam\ntemplate $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let conv = Conversation::<test_domain::TestDomain>::from_source_with(source, resolve);
        assert!(conv.is_ok(), "expected Ok, got: {:?}", conv.err());
    }

    #[test]
    fn from_source_with_imported_templates() {
        // Namespace provides templates that can be imported with `use $corpus from @blog`
        let namespace = namespace_with_grammar(
            "grammar @blog {\n  type = post\n}\ntemplate $corpus {\n\tslug\n}\n",
            "blog",
        );
        let resolve = Resolve::new().with_namespace(namespace);

        let source = "in @filesystem\nuse $corpus from @blog\nout r {\n\tx: f { $corpus }\n}\n";
        let conv = Conversation::<Filesystem>::from_source_with(source, resolve);
        assert!(conv.is_ok(), "expected Ok, got: {:?}", conv.err());
        let conv = conv.unwrap();
        assert!(conv.templates.contains_key("$corpus"));
    }

    #[test]
    fn from_source_with_namespace_grammars_merge() {
        // Grammars from namespace should be available for domain validation
        // A source with `in @beam` should NOT error when beam is a namespace grammar
        let namespace = namespace_with_grammar("grammar @beam {\n  type = process\n}\n", "beam");
        let resolve = Resolve::new().with_namespace(namespace);

        // Without namespace, this would fail with "unknown domain @beam"
        let source = "in @beam\ntemplate $t {\n\tname\n}\nout r {\n\tx: f { $t }\n}\n";
        let conv = Conversation::<test_domain::TestDomain>::from_source_with(source, resolve);
        assert!(conv.is_ok(), "expected Ok, got: {:?}", conv.err());
    }

    // -- Bootstrap --

    #[test]
    fn bootstrap_abstract_grammar_compiles() {
        let source = include_str!("../bootstrap.conv");
        let reg = compile_grammar(source);
        assert_eq!(reg.domain, "abstract");
        assert!(reg.has_type(""));
        assert!(reg.has_variant("", "grammar"));
        assert!(reg.has_variant("", "type"));
        assert!(reg.has_variant("", "variant"));
        assert!(reg.has_variant("", "template"));
    }

    #[test]
    fn bootstrap_two_pass_chain() {
        // Pass 1: bootstrap.conv → @abstract registered in namespace
        let bootstrap_src = include_str!("../bootstrap.conv");
        let namespace = namespace_with_grammar(bootstrap_src, "abstract");

        // Pass 2: compiler.conv parses successfully against that namespace
        let compiler_src = include_str!("../conv/compiler.conv");
        let ast = Parse.trace(compiler_src.to_string()).unwrap();
        let grammar = ast
            .children()
            .iter()
            .find(|c| c.data().is_decl("grammar"))
            .expect("compiler.conv must have a grammar block");
        let reg = TypeRegistry::compile(grammar).unwrap();

        // @compiler grammar has the expected types
        assert_eq!(reg.domain, "compiler");
        assert!(reg.has_variant("", "target"));
        assert!(reg.has_variant("", "artifact"));
        assert!(reg.has_variant("target", "gleam"));
        assert!(reg.has_variant("target", "elixir"));
        assert!(reg.has_variant("status", "ok"));
        assert!(reg.has_variant("status", "error"));
        assert!(reg.has_action("compile"));

        // @abstract is registered — the chain is live
        assert!(namespace.has_grammar("abstract"));
    }

    // -- TypeRegistry accessors --

    #[test]
    fn type_registry_type_names() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let mut names = reg.type_names();
        names.sort();
        assert_eq!(names, vec!["", "op"]);
    }

    #[test]
    fn type_registry_variants() {
        let reg = compile_grammar("grammar @test {\n  type = a | b | c\n}\n");
        let mut variants = reg.variants("").unwrap();
        variants.sort();
        assert_eq!(variants, vec!["a", "b", "c"]);
        assert!(reg.variants("missing").is_none());
    }

    #[test]
    fn type_registry_variant_param() {
        let reg =
            compile_grammar("grammar @test {\n  type = plain | when(op)\n  type op = gt | lt\n}\n");
        assert_eq!(reg.variant_param("", "when"), Some("op"));
        assert!(reg.variant_param("", "plain").is_none());
        assert!(reg.variant_param("missing", "x").is_none());
    }

    #[test]
    fn type_registry_act_names() {
        let reg = compile_grammar(
            "grammar @test {\n  type = a\n  action compile {\n    source: a\n  }\n  action run {\n    target\n  }\n}\n",
        );
        let mut names = reg.act_names();
        names.sort();
        assert_eq!(names, vec!["compile", "run"]);
    }

    #[test]
    fn type_registry_act_names_empty() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        assert!(reg.act_names().is_empty());
    }

    // -- Fragmentable --

    #[test]
    fn type_registry_same_grammar_same_oid() {
        let reg1 = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        let reg2 = compile_grammar("grammar @test {\n  type = a | b\n  type op = gt | lt\n}\n");
        assert_eq!(fragment::content_oid(&reg1), fragment::content_oid(&reg2),);
    }

    #[test]
    fn type_registry_different_grammar_different_oid() {
        let reg1 = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let reg2 = compile_grammar("grammar @test {\n  type = a | c\n}\n");
        assert_ne!(fragment::content_oid(&reg1), fragment::content_oid(&reg2),);
    }

    #[test]
    fn type_registry_different_domain_different_oid() {
        let reg1 = compile_grammar("grammar @foo {\n  type = a\n}\n");
        let reg2 = compile_grammar("grammar @bar {\n  type = a\n}\n");
        assert_ne!(fragment::content_oid(&reg1), fragment::content_oid(&reg2),);
    }

    #[test]
    fn type_registry_is_shard() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert!(reg.is_shard());
        assert!(!reg.is_fractal());
        assert!(reg.children().is_empty());
    }

    #[test]
    fn type_registry_self_ref_label() {
        let reg = compile_grammar("grammar @test {\n  type = a\n}\n");
        assert_eq!(reg.self_ref().label, "grammar/test");
    }

    use fragmentation::repo::Repo;
    use fragmentation::store::Store;

    #[test]
    fn type_registry_store_round_trip() {
        let reg = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let mut store = Store::<TypeRegistry>::new();
        let oid = store.write_tree(&reg);
        let read_back = store.read_tree(&oid).unwrap();
        assert_eq!(read_back.domain, reg.domain);
        assert_eq!(
            fragment::content_oid(&read_back),
            fragment::content_oid(&reg)
        );
    }

    #[test]
    fn type_registry_store_dedup() {
        let reg1 = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let reg2 = compile_grammar("grammar @test {\n  type = a | b\n}\n");
        let mut store = Store::<TypeRegistry>::new();
        store.write_tree(&reg1);
        store.write_tree(&reg2);
        assert_eq!(store.object_count(), 1);
    }
}
