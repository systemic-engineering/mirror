//! `@mirror/index` — mirror's own @fractal-coherence measurement primitive.
//!
//! Rung 8 Landing 3 GREEN (Reed) discharging Landing 2 RED (`8e6e6ea`) per
//! Taut `77b8e14` spectral→mirror migration mapping + Mara `317e830`
//! `shards/mirror/index.mirror` substrate-decl.
//!
//! Alex 2026-07-13 in-transcript: "the spectral__spectral_index is
//! something that currently lives in spectral I presume? This is something
//! that needs to be pulled into mirror" + "Fire" authorization.
//!
//! ## Substrate authority
//!
//! - Alex 2026-07-13 in-transcript directive + "Fire"
//! - Mara `317e830` — `shards/mirror/index.mirror` substrate-decl (508 LOC)
//! - Taut `77b8e14` — spectral→mirror migration mapping scout
//! - Taut `b52b008` — Fiedler = λ₀(Δ_F) empirical scout (0.0612 = 6% H¹(F))
//! - Mara `2c64060` §4 — Mandelbrot identification
//! - Recognition #43 (mirror IS content-addressed build system)
//! - Recognition #55 (form/process partition; DAG is form, measurement is
//!   process; belong at same altitude)
//!
//! ## The fork
//!
//! Forked from `/Users/alexwolf/dev/projects/spectral/crates/gestalt/src/`
//! (2026-06-04 baseline): `detect.rs` + `graph.rs` + `eigenvalue.rs`
//! essentials adapted to mirror altitude. Key substrate-pull adaptations:
//!
//! - Eigenvalue computation routes through `prismqueer::ffi::eigenvalues`
//!   (LAPACK `dsyev`) instead of the hand-rolled Jacobi sweep. Same
//!   primitive `sheaf_laplacian::lambda_zero` uses. Substrate-pull win
//!   per Taut `77b8e14` §6 — mirror owns its own eigenvalue path.
//! - Same laplacian formula as spectral (unnormalized `L = D - A`; sort
//!   ascending; take top-16; normalize by max). Preserves exact Fiedler
//!   value comparability with spectral's live envelope emission.
//!
//! Provisional home under two-tick discipline; collapses to
//! `shards/fractal/index.mirror` after Alex adjudicates #6.

use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// Grammar detection
// ---------------------------------------------------------------------------

/// Grammar kind of a detected file.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum GrammarKind {
    Markdown,
    GestaltNative,
    Mirror,
    Code(String),
    Config(String),
    Asset,
    Unknown,
}

#[derive(Clone, Debug)]
pub struct DetectedFile {
    pub path: PathBuf,
    pub kind: GrammarKind,
}

#[derive(Clone, Debug, Default)]
pub struct GestaltBreakdown {
    pub markdown: u32,
    pub code: u32,
    pub config: u32,
    pub asset: u32,
    pub gestalt_native: u32,
    pub mirror: u32,
    pub other: u32,
}

impl GestaltBreakdown {
    pub fn total(&self) -> u32 {
        self.markdown + self.code + self.config + self.asset
            + self.gestalt_native + self.mirror + self.other
    }
    pub fn record(&mut self, kind: &GrammarKind) {
        match kind {
            GrammarKind::Markdown => self.markdown += 1,
            GrammarKind::GestaltNative => self.gestalt_native += 1,
            GrammarKind::Mirror => self.mirror += 1,
            GrammarKind::Code(_) => self.code += 1,
            GrammarKind::Config(_) => self.config += 1,
            GrammarKind::Asset => self.asset += 1,
            GrammarKind::Unknown => self.other += 1,
        }
    }
}

pub fn detect_grammar(path: &Path) -> GrammarKind {
    let ext = match path.extension().and_then(|e| e.to_str()) {
        Some(e) => e.to_lowercase(),
        None => return GrammarKind::Unknown,
    };
    match ext.as_str() {
        "md" | "mdx" => GrammarKind::Markdown,
        "gestalt" => GrammarKind::GestaltNative,
        "mirror" => GrammarKind::Mirror,
        "rs" => GrammarKind::Code("rust".into()),
        "ex" | "exs" => GrammarKind::Code("elixir".into()),
        "ts" | "tsx" => GrammarKind::Code("typescript".into()),
        "js" | "jsx" | "mjs" | "cjs" => GrammarKind::Code("javascript".into()),
        "gleam" => GrammarKind::Code("gleam".into()),
        "py" => GrammarKind::Code("python".into()),
        "go" => GrammarKind::Code("go".into()),
        "rb" => GrammarKind::Code("ruby".into()),
        "java" => GrammarKind::Code("java".into()),
        "kt" => GrammarKind::Code("kotlin".into()),
        "swift" => GrammarKind::Code("swift".into()),
        "c" | "h" => GrammarKind::Code("c".into()),
        "cpp" | "cc" | "cxx" | "hpp" => GrammarKind::Code("cpp".into()),
        "cs" => GrammarKind::Code("csharp".into()),
        "php" => GrammarKind::Code("php".into()),
        "sh" | "bash" | "zsh" => GrammarKind::Code("shell".into()),
        "scala" => GrammarKind::Code("scala".into()),
        "clj" | "cljs" => GrammarKind::Code("clojure".into()),
        "hs" => GrammarKind::Code("haskell".into()),
        "ml" | "mli" => GrammarKind::Code("ocaml".into()),
        "nim" => GrammarKind::Code("nim".into()),
        "toml" => GrammarKind::Config("toml".into()),
        "yaml" | "yml" => GrammarKind::Config("yaml".into()),
        "json" => GrammarKind::Config("json".into()),
        "xml" => GrammarKind::Config("xml".into()),
        "ini" | "cfg" | "conf" => GrammarKind::Config("ini".into()),
        "lock" => GrammarKind::Config("lock".into()),
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "svg" | "webp" | "ico"
        | "pdf" | "zip" | "tar" | "gz" | "woff" | "woff2" | "ttf"
        | "otf" | "eot" | "mp3" | "mp4" | "wav" | "ogg" | "webm" => GrammarKind::Asset,
        _ => GrammarKind::Unknown,
    }
}

fn should_skip_dir(name: &str) -> bool {
    matches!(
        name,
        ".git" | ".hg" | ".svn" | ".direnv"
            | "node_modules" | "target" | "build" | "dist"
            | "_build" | ".build" | "__pycache__"
            | ".cache" | ".npm" | ".yarn"
            | ".spectral" | ".next" | ".nuxt"
            | "vendor" | "deps"
    )
}

fn load_gitignore(root: &Path) -> Vec<String> {
    match std::fs::read_to_string(root.join(".gitignore")) {
        Ok(content) => content
            .lines()
            .filter(|l| {
                let t = l.trim();
                !t.is_empty() && !t.starts_with('#')
            })
            .map(|l| l.trim().to_string())
            .collect(),
        Err(_) => Vec::new(),
    }
}

fn is_gitignored(relative: &Path, patterns: &[String]) -> bool {
    let rel_str = relative.to_string_lossy();
    for pattern in patterns {
        let pat = pattern.trim_end_matches('/');
        if rel_str == pat {
            return true;
        }
        if rel_str.starts_with(&format!("{}/", pat)) {
            return true;
        }
        for component in relative.components() {
            let comp = component.as_os_str().to_string_lossy();
            if comp == pat {
                return true;
            }
            if let Some(ext_pat) = pattern.strip_prefix("*.") {
                if let Some(ext) = Path::new(comp.as_ref()).extension() {
                    if ext.to_string_lossy() == ext_pat {
                        return true;
                    }
                }
            }
        }
        if let Some(ext_pat) = pattern.strip_prefix("*.") {
            if let Some(ext) = relative.extension() {
                if ext.to_string_lossy() == ext_pat {
                    return true;
                }
            }
        }
    }
    false
}

pub fn walk_detected(root: &Path) -> (Vec<DetectedFile>, GestaltBreakdown) {
    let mut files = Vec::new();
    let mut breakdown = GestaltBreakdown::default();
    let patterns = load_gitignore(root);
    walk_recursive(root, root, &patterns, &mut files, &mut breakdown);
    (files, breakdown)
}

fn walk_recursive(
    root: &Path,
    current: &Path,
    patterns: &[String],
    files: &mut Vec<DetectedFile>,
    breakdown: &mut GestaltBreakdown,
) {
    let entries = match std::fs::read_dir(current) {
        Ok(e) => e,
        Err(_) => return,
    };
    let mut entries_vec: Vec<_> = entries.flatten().collect();
    entries_vec.sort_by_key(|e| e.file_name());

    for entry in entries_vec {
        let path = entry.path();
        let file_name = entry.file_name();
        let name = file_name.to_string_lossy();
        if name.starts_with('.') {
            continue;
        }
        let relative = match path.strip_prefix(root) {
            Ok(r) => r.to_path_buf(),
            Err(_) => continue,
        };
        if is_gitignored(&relative, patterns) {
            continue;
        }
        let file_type = match entry.file_type() {
            Ok(ft) => ft,
            Err(_) => continue,
        };
        if file_type.is_symlink() {
            continue;
        }
        if file_type.is_dir() {
            if should_skip_dir(&name) {
                continue;
            }
            walk_recursive(root, &path, patterns, files, breakdown);
        } else if file_type.is_file() {
            let kind = detect_grammar(&path);
            breakdown.record(&kind);
            files.push(DetectedFile { path: path.clone(), kind });
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct MarkdownShape {
    pub heading_count: u32,
    pub paragraph_count: u32,
    pub word_count: u32,
    pub link_count: u32,
    pub wiki_link_targets: Vec<String>,
}

pub fn extract_markdown_shape(content: &str) -> MarkdownShape {
    let mut shape = MarkdownShape::default();
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            shape.heading_count += 1;
        } else if !trimmed.is_empty() {
            shape.paragraph_count += 1;
            shape.word_count += trimmed.split_whitespace().count() as u32;
        }
        let mut rest = trimmed;
        while let Some(pos) = rest.find("[[") {
            rest = &rest[pos + 2..];
            if let Some(end) = rest.find("]]") {
                let target = &rest[..end];
                let actual_target = target.split('|').next().unwrap_or(target);
                shape.wiki_link_targets.push(actual_target.to_string());
                shape.link_count += 1;
                rest = &rest[end + 2..];
            } else {
                break;
            }
        }
        let mut rest2 = trimmed;
        while let Some(pos) = rest2.find("](") {
            rest2 = &rest2[pos + 2..];
            if rest2.contains(')') {
                shape.link_count += 1;
                if let Some(end) = rest2.find(')') {
                    rest2 = &rest2[end + 1..];
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
    shape
}

// ---------------------------------------------------------------------------
// Concept graph
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum GraphNode {
    Directory {
        path: PathBuf,
        name: String,
        depth: usize,
        file_count: u32,
    },
    Root {
        path: PathBuf,
        file_count: u32,
    },
}

impl GraphNode {
    pub fn name(&self) -> &str {
        match self {
            GraphNode::Directory { name, .. } => name,
            GraphNode::Root { .. } => "<root>",
        }
    }
    pub fn file_count(&self) -> u32 {
        match self {
            GraphNode::Directory { file_count, .. } => *file_count,
            GraphNode::Root { file_count, .. } => *file_count,
        }
    }
}

#[derive(Clone, Debug)]
pub enum GraphEdge {
    Contains { parent_idx: usize, child_idx: usize, weight: f64 },
    SimilarContent { a_idx: usize, b_idx: usize, weight: f64 },
    CrossRef { source_idx: usize, target_idx: usize, weight: f64 },
}

impl GraphEdge {
    pub fn indices(&self) -> (usize, usize) {
        match self {
            GraphEdge::Contains { parent_idx, child_idx, .. } => (*parent_idx, *child_idx),
            GraphEdge::SimilarContent { a_idx, b_idx, .. } => (*a_idx, *b_idx),
            GraphEdge::CrossRef { source_idx, target_idx, .. } => (*source_idx, *target_idx),
        }
    }
    pub fn weight(&self) -> f64 {
        match self {
            GraphEdge::Contains { weight, .. } => *weight,
            GraphEdge::SimilarContent { weight, .. } => *weight,
            GraphEdge::CrossRef { weight, .. } => *weight,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ConceptGraph {
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

impl ConceptGraph {
    pub fn empty() -> Self {
        ConceptGraph { nodes: Vec::new(), edges: Vec::new() }
    }

    pub fn adjacency_matrix(&self) -> (Vec<f64>, usize) {
        let n = self.nodes.len();
        if n == 0 {
            return (Vec::new(), 0);
        }
        let mut matrix = vec![0.0_f64; n * n];
        for edge in &self.edges {
            let (i, j) = edge.indices();
            let w = edge.weight();
            if i < n && j < n {
                matrix[i * n + j] += w;
                matrix[j * n + i] += w;
            }
        }
        (matrix, n)
    }

    /// Unnormalized graph Laplacian `L = D - A`. Row-major.
    pub fn laplacian_matrix(&self) -> (Vec<f64>, usize) {
        let n = self.nodes.len();
        if n == 0 {
            return (Vec::new(), 0);
        }
        let (adj, _) = self.adjacency_matrix();
        let mut laplacian = vec![0.0_f64; n * n];
        for i in 0..n {
            let mut degree = 0.0;
            for j in 0..n {
                let w = adj[i * n + j];
                if i != j {
                    laplacian[i * n + j] = -w;
                    degree += w;
                }
            }
            laplacian[i * n + i] = degree;
        }
        (laplacian, n)
    }
}

/// Build a directory-level concept graph. Edges: (1) Contains (structural
/// nesting, weight 1.0), (2) SimilarContent (cosine sim > 0.3, weight sim*0.5),
/// (3) CrossRef (wiki-links across dirs, weight 0.3).
pub fn build_concept_graph(root: &Path) -> (ConceptGraph, Vec<DetectedFile>, GestaltBreakdown) {
    let (files, breakdown) = walk_detected(root);
    if files.is_empty() {
        return (ConceptGraph::empty(), files, breakdown);
    }

    let mut dir_files: HashMap<PathBuf, Vec<&DetectedFile>> = HashMap::new();
    for file in &files {
        let parent = file.path.parent().unwrap_or(root).to_path_buf();
        dir_files.entry(parent).or_default().push(file);
    }

    let mut nodes = Vec::new();
    let mut dir_to_idx: HashMap<PathBuf, usize> = HashMap::new();

    let root_count = dir_files.get(root).map(|f| f.len() as u32).unwrap_or(0);
    nodes.push(GraphNode::Root { path: root.to_path_buf(), file_count: root_count });
    dir_to_idx.insert(root.to_path_buf(), 0);

    let mut dirs: Vec<PathBuf> = dir_files.keys().cloned().collect();
    dirs.sort();

    for dir in &dirs {
        if dir == root {
            continue;
        }
        let name = dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| dir.to_string_lossy().to_string());
        let depth = dir
            .strip_prefix(root)
            .map(|r| r.components().count())
            .unwrap_or(0);
        let file_count = dir_files.get(dir).map(|f| f.len() as u32).unwrap_or(0);
        let idx = nodes.len();
        nodes.push(GraphNode::Directory { path: dir.clone(), name, depth, file_count });
        dir_to_idx.insert(dir.clone(), idx);
    }

    let mut edges = Vec::new();

    // 1. Contains
    for dir in &dirs {
        if dir == root {
            continue;
        }
        let child_idx = match dir_to_idx.get(dir) {
            Some(&i) => i,
            None => continue,
        };
        let parent = dir.parent().unwrap_or(root);
        let parent_idx = if let Some(&i) = dir_to_idx.get(parent) {
            i
        } else {
            let mut ancestor = parent.to_path_buf();
            loop {
                if let Some(&i) = dir_to_idx.get(&ancestor) {
                    break i;
                }
                match ancestor.parent() {
                    Some(p) => ancestor = p.to_path_buf(),
                    None => break 0,
                }
            }
        };
        edges.push(GraphEdge::Contains { parent_idx, child_idx, weight: 1.0 });
    }

    // 2. SimilarContent
    let dir_type_distributions = compute_type_distributions(&dir_files);
    let dir_indices: Vec<(PathBuf, usize)> =
        dir_to_idx.iter().map(|(p, &i)| (p.clone(), i)).collect();
    for i in 0..dir_indices.len() {
        for j in (i + 1)..dir_indices.len() {
            let (ref path_a, idx_a) = dir_indices[i];
            let (ref path_b, idx_b) = dir_indices[j];
            if let (Some(dist_a), Some(dist_b)) = (
                dir_type_distributions.get(path_a),
                dir_type_distributions.get(path_b),
            ) {
                let sim = cosine_similarity(dist_a, dist_b);
                if sim > 0.3 {
                    edges.push(GraphEdge::SimilarContent {
                        a_idx: idx_a,
                        b_idx: idx_b,
                        weight: sim * 0.5,
                    });
                }
            }
        }
    }

    // 3. CrossRef
    edges.extend(extract_cross_references(root, &files, &dir_to_idx));

    (ConceptGraph { nodes, edges }, files, breakdown)
}

fn compute_type_distributions(
    dir_files: &HashMap<PathBuf, Vec<&DetectedFile>>,
) -> HashMap<PathBuf, Vec<f64>> {
    const N_CATEGORIES: usize = 7;
    let mut distributions: HashMap<PathBuf, Vec<f64>> = HashMap::new();
    for (dir, files) in dir_files {
        let mut counts = vec![0.0_f64; N_CATEGORIES];
        for file in files {
            match &file.kind {
                GrammarKind::Markdown => counts[0] += 1.0,
                GrammarKind::Code(_) => counts[1] += 1.0,
                GrammarKind::Config(_) => counts[2] += 1.0,
                GrammarKind::Asset => counts[3] += 1.0,
                GrammarKind::GestaltNative => counts[4] += 1.0,
                GrammarKind::Mirror => counts[5] += 1.0,
                GrammarKind::Unknown => counts[6] += 1.0,
            }
        }
        distributions.insert(dir.clone(), counts);
    }
    distributions
}

fn cosine_similarity(a: &[f64], b: &[f64]) -> f64 {
    let dot: f64 = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum();
    let mag_a: f64 = a.iter().map(|x| x * x).sum::<f64>().sqrt();
    let mag_b: f64 = b.iter().map(|x| x * x).sum::<f64>().sqrt();
    if mag_a == 0.0 || mag_b == 0.0 {
        return 0.0;
    }
    dot / (mag_a * mag_b)
}

fn extract_cross_references(
    root: &Path,
    files: &[DetectedFile],
    dir_to_idx: &HashMap<PathBuf, usize>,
) -> Vec<GraphEdge> {
    let mut edges = Vec::new();
    let mut file_name_to_dir: HashMap<String, usize> = HashMap::new();
    for file in files {
        let name = file
            .path
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default();
        let dir = file.path.parent().unwrap_or(root);
        if let Some(&idx) = dir_to_idx.get(dir) {
            file_name_to_dir.insert(name, idx);
        }
    }
    for file in files {
        if file.kind != GrammarKind::Markdown {
            continue;
        }
        let content = match std::fs::read_to_string(&file.path) {
            Ok(c) => c,
            Err(_) => continue,
        };
        let shape = extract_markdown_shape(&content);
        let source_dir = file.path.parent().unwrap_or(root);
        let source_idx = match dir_to_idx.get(source_dir) {
            Some(&idx) => idx,
            None => continue,
        };
        for target_name in &shape.wiki_link_targets {
            let normalized = Path::new(target_name)
                .file_stem()
                .map(|s| s.to_string_lossy().to_string())
                .unwrap_or_else(|| target_name.clone());
            if let Some(&target_idx) = file_name_to_dir.get(&normalized) {
                if target_idx != source_idx {
                    edges.push(GraphEdge::CrossRef {
                        source_idx,
                        target_idx,
                        weight: 0.3,
                    });
                }
            }
        }
    }
    edges
}

// ---------------------------------------------------------------------------
// EigenvalueProfile
// ---------------------------------------------------------------------------

/// A 16-value eigenvalue profile — the spectral fingerprint of a graph.
/// Values normalized to `[0.0, 1.0]` (top-16 eigenvalues / max). Fiedler
/// = values[1] post-normalization.
#[derive(Clone, Debug, PartialEq)]
pub struct EigenvalueProfile {
    pub values: [f64; 16],
}

impl EigenvalueProfile {
    pub fn dark() -> Self {
        EigenvalueProfile { values: [0.0; 16] }
    }
    pub fn is_dark(&self) -> bool {
        self.values.iter().all(|&v| v == 0.0)
    }
    pub fn fiedler_value(&self) -> f64 {
        if self.values.len() < 2 {
            return 0.0;
        }
        self.values[1]
    }
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(128);
        for &v in &self.values {
            buf.extend_from_slice(&v.to_le_bytes());
        }
        buf
    }
}

/// Compute the eigenvalue profile via LAPACK `dsyev` (mirror's own
/// substrate primitive `prismqueer::ffi::eigenvalues` — the same one
/// `sheaf_laplacian::lambda_zero` uses at T8). Substrate-pull win per
/// Taut `77b8e14` §6 — replaces spectral's hand-rolled Jacobi sweep.
pub fn eigenvalue_profile(graph: &ConceptGraph) -> EigenvalueProfile {
    let n = graph.nodes.len();
    if n < 2 {
        return EigenvalueProfile::dark();
    }
    let (laplacian, dim) = graph.laplacian_matrix();
    let mut sorted = match prismqueer::ffi::eigenvalues(dim, &laplacian) {
        Ok(v) => v,
        Err(_) => return EigenvalueProfile::dark(),
    };
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    build_profile(&sorted)
}

fn build_profile(eigenvalues: &[f64]) -> EigenvalueProfile {
    let mut values = [0.0_f64; 16];
    let count = eigenvalues.len().min(16);
    for i in 0..count {
        values[i] = eigenvalues[i];
    }
    let max = values.iter().cloned().fold(0.0_f64, f64::max);
    if max > 1e-12 {
        for v in &mut values {
            *v /= max;
        }
    }
    EigenvalueProfile { values }
}

// ---------------------------------------------------------------------------
// Public entry point
// ---------------------------------------------------------------------------

/// Compute mirror's own @fractal-coherence measurement on a directory tree.
///
/// RED-lock reference (Landing 2, `8e6e6ea`): Fiedler = 0.0612 on the
/// mirror repo root (1141 files, 165 nodes, 6676 edges per Mara `317e830`
/// commit-hook capture).
pub fn index(peer_home: &Path) -> EigenvalueProfile {
    let (graph, _files, _breakdown) = build_concept_graph(peer_home);
    eigenvalue_profile(&graph)
}
