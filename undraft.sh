#! /bin/bash
#
# Quickie script to convert a draft into a post ready to go. The only bit worth
# scripting is that we sub in a `@DATE@` template.

set -e
cd $(dirname $0)
year=$(date +%Y)
draft="$1"

if [ ! -f "$draft" ] ; then
    echo "usage: $0 <path to draft>"
    exit 1
fi

mkdir -p "content/$year"

path="content/$year/$(basename "$draft")"
echo "$draft" '->' "$path"
mv "$draft" "$path"
sed -i -e "s/@DATE@/$(date -Iseconds)/" "$path"
