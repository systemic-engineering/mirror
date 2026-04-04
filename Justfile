# skeleton-key — spectral cryptographic break POC

check: lint test

lint:
    nix develop -c cargo clippy -- -D warnings

test:
    nix develop -c cargo test --test crypto_break -j2

format-check:
    nix develop -c cargo fmt -- --check

format:
    nix develop -c cargo fmt

pre-commit: check
pre-push: check
