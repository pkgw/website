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

set +x
echo
echo "Is it time to put out a new newsletter? https://buttondown.email/emails/new"
echo
echo "Toot about it? https://mastodon.world/home"
echo
