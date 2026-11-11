#!/usr/bin/env bash
set -euo pipefail

# File stem -> feature name: zh_Hant -> zh-hant, pt_PT -> pt-pt
names=$(for f in data/annotations/*.xml; do
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
