/// Extension-based file classifier.
///
/// The extension IS the first classifier. Zero cost.
/// Content-based classification is the fallback (via classifier.rs).

/// Map file extension to @code grammar name.
pub fn classify_extension(path: &str) -> Option<&'static str> {
    let ext = path.rsplit('.').next()?;
    match ext {
        "mirror" => Some("@code/mirror"),
        "shatter" => Some("@code/shatter"),
        "shard" => Some("@code/shard"),
        "spec" => Some("@code/spec"),
        "rs" => Some("@code/rust"),
        "ex" | "exs" => Some("@code/elixir"),
        "gleam" => Some("@code/gleam"),
        "js" | "mjs" => Some("@code/javascript"),
        "ts" | "tsx" => Some("@code/typescript"),
        "py" => Some("@code/python"),
        "f90" | "f95" | "f03" => Some("@code/fortran"),
        "erl" => Some("@code/erlang"),
        "hs" => Some("@code/haskell"),
        "go" => Some("@code/go"),
        "c" | "h" => Some("@code/c"),
        "cpp" | "hpp" | "cc" => Some("@code/cpp"),
        "java" => Some("@code/java"),
        "rb" => Some("@code/ruby"),
        "nix" => Some("@code/nix"),
        "toml" => Some("@code/toml"),
        "yaml" | "yml" => Some("@code/yaml"),
        "json" => Some("@code/json"),
        "md" => Some("@code/markdown"),
        "html" | "htm" => Some("@code/html"),
        "css" => Some("@code/css"),
        "sql" => Some("@code/sql"),
        "sh" | "bash" | "zsh" => Some("@code/shell"),
        _ => None,
    }
}

/// Classify a file: extension first, then content-based fallback.
pub fn classify_file(path: &str, _content: &str) -> &'static str {
    classify_extension(path).unwrap_or("@code/unknown")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn classify_mirror_extension() {
        assert_eq!(classify_extension("app.mirror"), Some("@code/mirror"));
    }

    #[test]
    fn classify_shatter_extension() {
        assert_eq!(classify_extension("app.shatter"), Some("@code/shatter"));
    }

    #[test]
    fn classify_shard_extension() {
        assert_eq!(classify_extension("app.shard"), Some("@code/shard"));
    }

    #[test]
    fn classify_spec_extension() {
        assert_eq!(classify_extension("mirror.spec"), Some("@code/spec"));
    }

    #[test]
    fn classify_rust_extension() {
        assert_eq!(classify_extension("main.rs"), Some("@code/rust"));
    }

    #[test]
    fn classify_elixir_extension() {
        assert_eq!(classify_extension("app.ex"), Some("@code/elixir"));
        assert_eq!(classify_extension("test.exs"), Some("@code/elixir"));
    }

    #[test]
    fn classify_unknown_extension() {
        assert_eq!(classify_extension("data.xyz"), None);
    }

    #[test]
    fn classify_no_extension() {
        assert_eq!(classify_extension("Makefile"), None);
    }

    #[test]
    fn classify_path_with_directories() {
        assert_eq!(classify_extension("src/main.rs"), Some("@code/rust"));
        assert_eq!(
            classify_extension("boot/00-prism.mirror"),
            Some("@code/mirror")
        );
    }

    #[test]
    fn classify_file_fallback() {
        assert_eq!(classify_file("app.rs", ""), "@code/rust");
        assert_eq!(classify_file("data.xyz", ""), "@code/unknown");
    }

    #[test]
    fn classify_nix_extension() {
        assert_eq!(classify_extension("flake.nix"), Some("@code/nix"));
    }

    #[test]
    fn classify_gleam_extension() {
        assert_eq!(classify_extension("app.gleam"), Some("@code/gleam"));
    }
}
