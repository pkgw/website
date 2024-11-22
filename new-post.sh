#! /bin/bash
#
# Quickie script to start editing a new post

set -e
cd $(dirname $0)
year=$(date +%Y)

if [ "$1" = --dir ] ; then
    dirmode=true
    shift
else
    dirmode=false
fi

slug="$1"

if [ x"$slug" = x ] ; then
    echo "usage: $0 new-post-filename-slug"
    exit 0
fi

if $dirmode ; then
    mkdir -p "content/$year/$slug"
    path="content/$year/$slug/index.md"
else
    if [[ "$slug" =~ \.md$ ]] ; then  # note, shell quoting not needed!
        :
    else
        slug="$slug.md"
    fi

    mkdir -p "content/$year"
    path="content/$year/$slug"
fi

cat <<EOF >"$path"
+++
date = $(date +"%FT00:00:00%:z")
title = ""
+++

<!-- more -->

EOF

exec code --new-window --goto "$path" .
