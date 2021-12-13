#! /bin/bash
#
# Quickie script to start editing a new howto

set -e
cd $(dirname $0)
slug="$1"

if [ x"$slug" = x ] ; then
    echo "usage: $0 new-howto-filename-slug"
    exit 0
fi

if [[ "$slug" =~ \.md$ ]] ; then  # note, shell quoting not needed!
    :
else
    slug="$slug.md"
fi

path="content/howto/$slug"

cat <<EOF >"$path"
+++
date = $(date -Iseconds)
title = ""
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

EOF

exec hidpi-chromium-launcher.sh code --new-window "$path"
