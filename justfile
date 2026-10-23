[doc("Check the crate boundaries deny.toml states.")]
arch:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! command -v cargo-deny >/dev/null; then
        echo "cargo-deny is not installed: cargo install cargo-deny"
        exit 1
    fi
    cargo deny check -D warnings bans
