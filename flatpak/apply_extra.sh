#!/bin/sh
set -eu

extra_dir=${DISKETTE_EXTRA_DIR:-/app/extra}
package="$extra_dir/yandex-disk_0.1.6.1080_amd64.deb"

if [ ! -f "$package" ]; then
    echo "Yandex Disk package is missing: $package" >&2
    exit 1
fi

work_dir=$(mktemp -d "${TMPDIR:-/tmp}/diskette-extra.XXXXXX")
trap 'rm -rf "$work_dir"' EXIT HUP INT TERM

data_member=$(bsdtar -tf "$package" | sed -n '/^data[.]tar/p' | head -n 1)
if [ -z "$data_member" ]; then
    echo "Yandex Disk deb did not contain a data.tar payload" >&2
    exit 1
fi

data_archive="$work_dir/$data_member"
bsdtar -xOf "$package" "$data_member" > "$data_archive"

bsdtar -tf "$data_archive" | while IFS= read -r entry; do
    case "$entry" in
        /* | ../* | */../* | */.. | ..)
            echo "Unsafe archive path: $entry" >&2
            exit 1
            ;;
    esac
done

bsdtar -xpf "$data_archive" -C "$extra_dir"
chmod a+x "$extra_dir/usr/bin/yandex-disk"
