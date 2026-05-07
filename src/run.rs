//! run — codegen for the @run grammar.
//!
//! Compiles and verifies .mirror files declared in the spec's
//! run {} block. Running IS verifying. The proof chain is the program.
//!
//! Layer 3: codegen. Source of truth: boot/std/run.mirror
//! When mirror compiles @run natively, this file is generated.

use std::path::Path;

use crate::lambda_phases::{Parse, SourceText};
use prism::lambda::LambdaFn;

#[derive(Debug)]
pub struct RunError(pub String);

impl std::fmt::Display for RunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for RunError {}

/// Codegen target for `@run.run(targets)`.
///
/// Reads the project's mirror.spec, resolves the run {} default
/// targets, compiles each .mirror file, reports results.
pub fn run_project(root: &Path) -> Result<String, RunError> {
    // Read spec
    let spec_path = root.join("mirror.spec");
    if !spec_path.exists() {
        return Err(RunError(format!(
            "no mirror.spec in {}",
            root.display()
        )));
    }
    let spec_source = std::fs::read_to_string(&spec_path)
        .map_err(|e| RunError(format!("cannot read mirror.spec: {}", e)))?;
    let spec = crate::spec::parse_spec_source(&spec_source)
        .map_err(|e| RunError(format!("cannot parse mirror.spec: {}", e)))?;

    // Resolve run targets — default to mirror/*.mirror
    let mirror_dir = if spec.run.default.is_empty() {
        root.join("mirror")
    } else {
        // Parse the directory from the glob pattern.
        // For "mirror/*.mirror", the directory is "mirror".
        let pattern = &spec.run.default[0];
        let dir_part = pattern.split('/').next().unwrap_or("mirror");
        root.join(dir_part)
    };

    // Collect .mirror files
    let mut files = Vec::new();
    if mirror_dir.is_dir() {
        for entry in std::fs::read_dir(&mirror_dir)
            .map_err(|e| RunError(format!("cannot read {}: {}", mirror_dir.display(), e)))?
        {
            let entry = entry.map_err(|e| RunError(e.to_string()))?;
            let path = entry.path();
            if path.extension().map_or(false, |ext| ext == "mirror") {
                files.push(path);
            }
        }
    }

    files.sort();

    if files.is_empty() {
        return Ok(format!("{} — no .mirror files found", spec.oid));
    }

    // Compile each file through the parse pipeline
    let mut report = Vec::new();
    let mut total_loss = 0.0f64;

    for file in &files {
        let source = std::fs::read_to_string(file)
            .map_err(|e| RunError(format!("cannot read {}: {}", file.display(), e)))?;

        let result = Parse.reduce(SourceText(source));
        let file_name = file.file_name().unwrap_or_default().to_string_lossy();

        match result {
            prism::Imperfect::Success(_) => {
                report.push(format!("  {} — crystal", file_name));
            }
            prism::Imperfect::Partial(_, loss) => {
                // loss is MirrorLoss; holonomy() is on MirrorLoss directly
                let h = loss.holonomy();
                total_loss += h;
                report.push(format!("  {} — partial (loss: {:.4})", file_name, h));
            }
            prism::Imperfect::Failure(err, _) => {
                report.push(format!("  {} — FAIL: {}", file_name, err));
            }
        }
    }

    let status = if total_loss == 0.0 { "crystal" } else { "partial" };
    Ok(format!(
        "{} — {} ({} files)\n{}",
        spec.oid,
        status,
        files.len(),
        report.join("\n")
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_compiles_mirror_files() {
        let dir = tempfile::tempdir().unwrap();
        crate::scaffold::scaffold_project(dir.path(), "test-run").unwrap();
        std::fs::write(
            dir.path().join("mirror/hello.mirror"),
            "type greeting = hello | goodbye\n",
        )
        .unwrap();
        let result = run_project(dir.path());
        assert!(result.is_ok(), "run should succeed: {:?}", result.err());
    }

    #[test]
    fn run_reports_compiled_files() {
        let dir = tempfile::tempdir().unwrap();
        crate::scaffold::scaffold_project(dir.path(), "test-report").unwrap();
        std::fs::write(
            dir.path().join("mirror/types.mirror"),
            "type color = red | blue\n",
        )
        .unwrap();
        let result = run_project(dir.path()).unwrap();
        assert!(result.contains("types.mirror"), "should report compiled file");
    }

    #[test]
    fn run_with_no_mirror_files() {
        let dir = tempfile::tempdir().unwrap();
        crate::scaffold::scaffold_project(dir.path(), "empty").unwrap();
        let result = run_project(dir.path());
        assert!(result.is_ok());
        assert!(result.unwrap().contains("no .mirror files"));
    }

    #[test]
    fn run_reads_spec_identity() {
        let dir = tempfile::tempdir().unwrap();
        crate::scaffold::scaffold_project(dir.path(), "identity-test").unwrap();
        std::fs::write(
            dir.path().join("mirror/x.mirror"),
            "type x\n",
        )
        .unwrap();
        let result = run_project(dir.path()).unwrap();
        assert!(result.contains("@identity-test"));
    }

    #[test]
    fn run_fails_without_spec() {
        let dir = tempfile::tempdir().unwrap();
        let result = run_project(dir.path());
        assert!(result.is_err());
    }
}
