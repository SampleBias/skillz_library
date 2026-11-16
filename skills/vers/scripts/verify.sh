#!/usr/bin/env bash
set -euo pipefail
skill_dir="$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.." && pwd)"
# Preserve the caller's working directory as the candidate project.
exec cargo run --offline --locked --quiet --manifest-path "$skill_dir/Cargo.toml" -- "$@"
