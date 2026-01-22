+++
date = 2011-07-25T16:23:01Z
title = "One Piece of the Puzzle: w-Projection"
path = "2011/07/one-piece-of-the-puzzle-w-projection"

[extra]
wp_rel_permalink = "/2011/07/one-piece-of-the-puzzle-w-projection/"
wp_shortlink = "/?p=448"
+++

In my previous post I mentioned that
[w-projection](https://scixplorer.org/abs/2005ASPC..347...86C) might help
the issues I was seeing with the sources on the very edges of my Cyg X-3
image. I’ve gotten to the point where I can use CASA’s imager on some sample
Cyg X-3 data, and indeed it makes a difference.

Below are some zoomed-in images, with the cross representing the catalog
position of DR 21. Here’s it without w-projection:

{% figure(path="wp/wp-content/uploads/2011/07/no-w-300x240.png") %}
DR 21 without *w*-projection — [full size version](../../../wp/wp-content/uploads/2011/07/no-w.png).
{% end %}

And here it is _with_ w-projection:

{% figure(path="wp/wp-content/uploads/2011/07/yes-w-300x240.png") %}
DR 21 with *w*-projection — [full size version](../../../wp/wp-content/uploads/2011/07/yes-w.png).
{% end %}

I believe that there are still issues to resolve regarding the hour angle
dependence, but it’s nice to see the algorithm work so well for these
particular sources.
