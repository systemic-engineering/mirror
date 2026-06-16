#!/usr/bin/env bash
# Install the pinned mirror binary for this runner's OS+arch.
#
# Two modes:
#   1. Pre-built (MIRROR_BINARY env var set to an existing path) — used by
#      mirror's own self-host gate, which builds the binary from source
#      before invoking the composite. No download, no checksum check.
#   2. Release download (default) — resolves the release artifact
#      corresponding to $1 (the mirror version), downloads it from the
#      GitHub Release, verifies SHA-256 against the checksum file, and
#      places `mirror` on $PATH for subsequent steps.
#
# Per kintsugi-ci-v0.1 §5.2 — ships inside the action.

set -euo pipefail

VERSION="${1:?usage: install-mirror.sh <version>}"
REPO="systemic-engineering/mirror"

# Pre-built mode: caller (the self-host gate) supplies the binary path.
if [[ -n "${MIRROR_BINARY:-}" ]]; then
  if [[ ! -x "$MIRROR_BINARY" ]]; then
    echo "MIRROR_BINARY=$MIRROR_BINARY not found or not executable" >&2
    exit 2
  fi
  INSTALL_DIR="${RUNNER_TEMP:-/tmp}/mirror-install"
  mkdir -p "$INSTALL_DIR"
  cp "$MIRROR_BINARY" "$INSTALL_DIR/mirror"
  chmod +x "$INSTALL_DIR/mirror"
  echo "$INSTALL_DIR" >> "$GITHUB_PATH"
  echo "Using pre-built mirror at $MIRROR_BINARY"
  exit 0
fi

# Strip any leading `refs/tags/` (when called with github.action_ref).
VERSION="${VERSION#refs/tags/}"

# Resolve OS+arch to the release asset slug.
OS="$(uname -s | tr '[:upper:]' '[:lower:]')"
ARCH="$(uname -m)"
case "$OS-$ARCH" in
  linux-x86_64)   ASSET="mirror-x86_64-unknown-linux-gnu" ;;
  linux-aarch64)  ASSET="mirror-aarch64-unknown-linux-gnu" ;;
  darwin-x86_64)  ASSET="mirror-x86_64-apple-darwin" ;;
  darwin-arm64)   ASSET="mirror-aarch64-apple-darwin" ;;
  *) echo "unsupported runner: $OS-$ARCH" >&2; exit 2 ;;
esac

INSTALL_DIR="${RUNNER_TEMP:-/tmp}/mirror-install"
mkdir -p "$INSTALL_DIR"

BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
echo "::group::Downloading mirror ${VERSION} (${ASSET})"
curl -fsSL --retry 3 -o "$INSTALL_DIR/$ASSET.tar.gz" "$BASE_URL/$ASSET.tar.gz"
curl -fsSL --retry 3 -o "$INSTALL_DIR/checksums.txt" "$BASE_URL/checksums.txt"
echo "::endgroup::"

echo "::group::Verifying SHA-256"
(
  cd "$INSTALL_DIR"
  # Filter to just this asset's checksum line and verify.
  grep " $ASSET.tar.gz\$" checksums.txt > expected.sum
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum -c expected.sum
  else
    # macOS fallback: shasum -a 256
    shasum -a 256 -c expected.sum
  fi
)
echo "::endgroup::"

echo "::group::Extracting + placing on PATH"
tar -xzf "$INSTALL_DIR/$ASSET.tar.gz" -C "$INSTALL_DIR"
chmod +x "$INSTALL_DIR/mirror"
echo "$INSTALL_DIR" >> "$GITHUB_PATH"
"$INSTALL_DIR/mirror" --version || true
echo "::endgroup::"
