#!/usr/bin/env bash
set -euo pipefail

TOOLCHAIN_FILE="${1:-rust-toolchain.toml}"

# Update nightly toolchain
rustup update nightly

# Grab the exact version + date
NIGHTLY_VERSION=$(rustc +nightly --version | awk '{gsub(/[()]/,"",$4); print "nightly-" $4}')
# NIGHTLY_VERSION=$(rustc +nightly --version | grep -o '[0-9]\{4\}-[0-9]\{2\}-[0-9]\{2\}' | sed 's/^/nightly-/')

# sed -i.bak "s/channel = \".*\"/channel = \"$NIGHTLY_VERSION\"/" "$TOOLCHAIN_FILE"
# rm -f "${TOOLCHAIN_FILE}.bak"

# Patch rust-toolchain.toml
if sed --version >/dev/null 2>&1; then
    # GNU sed
    sed -i "s/channel = \".*\"/channel = \"$NIGHTLY_VERSION\"/" "$TOOLCHAIN_FILE"
else
    # BSD/macOS sed
    sed -i '' "s/channel = \".*\"/channel = \"$NIGHTLY_VERSION\"/" "$TOOLCHAIN_FILE"
fi

echo "Updated $TOOLCHAIN_FILE to use $NIGHTLY_VERSION"

# Commit & push
git add "$TOOLCHAIN_FILE"
git commit -m "chore(build): bump Rust toolchain to ${NIGHTLY_VERSION}"
