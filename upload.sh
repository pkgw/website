#! /bin/bash
#
# Generate and publish the site.

cd $(dirname "$0")
set -ex

rm -rf public
zola build

mkdir public/feed
cp public/rss.xml public/feed/index.html

rsync -avP --exclude '*~' public/ newton.cx:public_html/
