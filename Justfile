# conversation — gradients over trees

check: lint test format-check coverage

lint:
    nix develop -c cargo clippy -- -D warnings

test:
    nix develop -c cargo test

test-git:
    nix develop -c cargo test --features git

format-check:
    nix develop -c cargo fmt -- --check

format:
    nix develop -c cargo fmt

# 100% line coverage or fail
coverage:
    nix develop -c cargo llvm-cov --fail-under-lines 100

# HTML report
coverage-html:
    nix develop -c cargo llvm-cov --html --open

pre-commit: check
pre-push: check
