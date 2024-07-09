#! /bin/bash
#
# Quickie script to start editing a new post

set -e
cd $(dirname $0)
year=$(date +%Y)
slug="$1"

if [ x"$slug" = x ] ; then
    echo "usage: $0 new-post-filename-slug"
    exit 0
fi

if [[ "$slug" =~ \.md$ ]] ; then  # note, shell quoting not needed!
    :
else
    slug="$slug.md"
fi

mkdir -p "content/$year"

path="content/$year/$slug"

cat <<EOF >"$path"
+++
date = $(date -Iseconds)
title = ""
+++

<!-- more -->

EOF

exec hidpi-chromium-launcher.sh code --new-window --goto "$path" .
