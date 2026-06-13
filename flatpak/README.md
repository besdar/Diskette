# Flatpak

`app.diskette.Diskette.yml` is a Flathub-oriented manifest.

Important submission notes:

- The manifest uses GNOME runtime 50, which is currently available on Flathub.
- The Yandex Disk CLI is downloaded as `extra-data` from Yandex during user
  installation. The binary is not committed into this repository.
- `--persist=Yandex.Disk` keeps the Flatpak sync folder app-private while
  avoiding full home filesystem access.
- The `app.diskette.Diskette` app ID is a neutral default. Replace it if the
  final maintainer uses a different controlled domain or source-hosting ID.
- Flathub's repo linter also requires hosted screenshots mirrored into the
  repository. Add real Linux screenshots to the MetaInfo file once the project
  has a public, maintainer-controlled hosting location.
- Flathub submission pull requests must be created and reviewed by a human
  maintainer.

Local build:

```bash
flatpak-builder --force-clean --user --install --install-deps-from=flathub build-dir flatpak/app.diskette.Diskette.yml
flatpak run app.diskette.Diskette
```

Flathub-style sandboxed build:

```bash
flatpak run --command=flathub-build org.flatpak.Builder flatpak/app.diskette.Diskette.yml
```

Adding `--install` makes Flatpak apply the Yandex Disk `extra-data` package.
That installation step requires unprivileged user namespaces for `bwrap`; in
nested or locked-down environments it can fail even when the build and export
are valid.

Flathub-style repo lint:

```bash
flatpak-builder --force-clean --repo=repo build-dir flatpak/app.diskette.Diskette.yml
flatpak run --command=flatpak-builder-lint org.flatpak.Builder repo repo
```
