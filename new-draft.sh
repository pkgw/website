#! /bin/bash
#
# Quickie script to stub a draft

set -e
cd $(dirname $0)
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

mkdir -p drafts

path="drafts/$slug"

cat <<EOF >"$path"
+++
date = @DATE@
title = ""
+++

<!-- more -->

EOF

exec hidpi-chromium-launcher.sh code --new-window --goto "$path" .
