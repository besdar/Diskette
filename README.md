# Diskette

Diskette is a GTK4/Libadwaita desktop wrapper for the Yandex Disk Linux CLI.

It does not reimplement Yandex Disk synchronization. It runs `yandex-disk`
with typed command options, keeps the GTK UI responsive while commands run,
and provides a graphical setup/configuration workflow for the CLI config file.

## Build with Podman

Rust and Cargo are not required on the host. Build packages with:

```bash
./packaging/podman-build-deb.sh
./packaging/podman-build-rpm.sh
```

Packages are written to `dist/deb/` and `dist/rpm/`.

The native packages depend on `yandex-disk`. Configure the Yandex package
repository before installing Diskette on systems where `yandex-disk` is not
already available from package manager metadata.

## Flatpak

The Flathub-oriented manifest lives in `flatpak/`. It uses GNOME runtime 50,
downloads the Yandex CLI deb during the build, installs the CLI into the app
prefix, and persists the app-private `~/Yandex.Disk` folder without granting
full home filesystem access.
