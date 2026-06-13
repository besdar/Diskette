# Packaging

Use the scripts in this directory from the repository root or directly:

```bash
./packaging/podman-build-deb.sh
./packaging/podman-build-rpm.sh
```

The package builders run entirely inside Podman containers. The host does not
need Rust or Cargo installed.

Native packages declare `yandex-disk` as a dependency. Users may need to enable
the Yandex package repository before installing Diskette.
