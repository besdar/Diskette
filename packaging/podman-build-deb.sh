#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image="localhost/diskette-deb-builder:latest"

podman build -f "$repo_root/packaging/Containerfile.deb" -t "$image" "$repo_root"
mkdir -p "$repo_root/dist/deb"
podman run --rm -v "$repo_root:/work:Z" "$image"
