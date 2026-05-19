//! Git crystal cache via `std::process::Command`. Mirrors C popen calls.

use std::io::Write;
use std::process::{Command, Stdio};

/// Run a command, capture stdout, trim trailing whitespace.
fn exec_capture(program: &str, args: &[&str]) -> Option<String> {
    let out = Command::new(program)
        .args(args)
        .stderr(Stdio::null())
        .output()
        .ok()?;
    let mut s = String::from_utf8_lossy(&out.stdout).into_owned();
    while s.ends_with('\n') || s.ends_with('\r') || s.ends_with(' ') {
        s.pop();
    }
    Some(s)
}

/// `git hash-object -w <tmpfile>` containing `content`.
fn git_store(content: &str) -> Option<String> {
    // Make a tempfile via std::env::temp_dir + random suffix.
    let pid = std::process::id();
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let path = std::env::temp_dir().join(format!("mirror_git_{}_{}.tmp", pid, nanos));
    {
        let mut f = std::fs::File::create(&path).ok()?;
        f.write_all(content.as_bytes()).ok()?;
    }
    let path_str = path.to_string_lossy().to_string();
    let result = exec_capture("git", &["hash-object", "-w", &path_str]);
    let _ = std::fs::remove_file(&path);
    result.filter(|s| !s.is_empty())
}

pub fn git_store_crystal(source_hash: &str, crystal_oid: &str) {
    let blob = match git_store(crystal_oid) {
        Some(b) if !b.is_empty() => b,
        _ => return,
    };
    let refname = format!("refs/crystals/{}", source_hash);
    let _ = Command::new("git")
        .args(["update-ref", &refname, &blob])
        .stderr(Stdio::null())
        .stdout(Stdio::null())
        .status();
}

pub fn git_crystal_exists(source_hash: &str) -> Option<String> {
    let refname = format!("refs/crystals/{}", source_hash);
    let s = exec_capture("git", &["cat-file", "-p", &refname])?;
    if s.is_empty() {
        None
    } else {
        Some(s)
    }
}
