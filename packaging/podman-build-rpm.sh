#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image="localhost/diskette-rpm-builder:latest"

podman build -f "$repo_root/packaging/Containerfile.rpm" -t "$image" "$repo_root"
mkdir -p "$repo_root/dist/rpm"
podman run --rm -v "$repo_root:/work:Z" "$image"
