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
} > features.toml

{
    curl  https://www.unicode.org/license.txt
} > UNICODE_LICENSE.txt

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
