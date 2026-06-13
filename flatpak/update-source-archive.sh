#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
archive="$repo_root/flatpak/diskette-source.tar.gz"

cd "$repo_root"
rm -f "$archive"
tar \
  --sort=name \
  --mtime='UTC 2026-06-07' \
  --owner=0 \
  --group=0 \
  --numeric-owner \
  -czf "$archive" \
  Cargo.lock \
  Cargo.toml \
  LICENSE \
  README.md \
  data \
  i18n \
  src
