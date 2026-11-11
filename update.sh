#!/usr/bin/env bash
set -euo pipefail

names=$(for f in emoji-names/data/annotations/*.xml; do
    s=$(basename "$f" .xml)
    [ "$s" = "root" ] && continue
    echo "$s" | tr '_' '-' | tr 'A-Z' 'a-z'
done | sort)

{
    echo "[features]"
    echo 'default = ["en"]'
    echo -n 'all = ['
    echo "$names" | sed 's/.*/"&"/' | paste -sd, -
    echo "]"
    echo "$names" | sed 's/.*/& = []/'
} > emoji-names/features.toml

curl -L --fail --show-error \
    https://www.unicode.org/license.txt \
    --output UNICODE_LICENSE

curl -L --fail --show-error \
    https://www.mozilla.org/media/MPL/2.0/index.f75d2927d3c1.txt \
    --output LICENSE

{
    echo "# Emoji Names"
    echo ""
    echo "This crate provides a mapping of emoji to their CLDR short names. The data is derived from the Unicode CLDR project."
    echo ""
    echo "By default only English is enabled, other languages can be enabled by enabling the corresponding feature."
    echo "All languages that are supported by the CLDR project are available (so long as this crate is kept up to date)."
    echo ""
    echo "## Supported Locales"
    echo "$names" | sed 's/.*/- &/'
    echo ""
} > README.md
cp README.md emoji-names/README.md
