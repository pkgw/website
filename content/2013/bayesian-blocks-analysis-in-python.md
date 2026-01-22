+++
date = 2013-05-06T19:21:47Z
title = "Bayesian Blocks Analysis in Python"
path = "2013/05/bayesian-blocks-analysis-in-python"

[extra]
wp_rel_permalink = "/2013/05/bayesian-blocks-analysis-in-python/"
wp_shortlink = "/?p=737"
+++

I wrote
[a Python implementation of the Bayesian Blocks algorithm](https://github.com/pkgw/pwpy/blob/master/scilib/xbblocks.py).
There’s already
[a good implementation out in public from Jake Vanderplas](http://jakevdp.github.io/blog/2012/09/12/dynamic-programming-in-python/),
but it had some issues that I’ll briefly mention below. I’m being a bad member
of the community by writing a new version instead of improving the existing
one, and I feel bad about that, but I needed to get something going quickly,
and I might as well put it out there.

Bayesian Blocks algorithms are nice for binning X-ray lightcurves into
sections of different count rates; or, you can think of them as providing
dynamic binning for histogram analysis. J. Scargle has been the main person
developing the theory, and has put together a very nice detailed paper
explaining the approach in
[2013 ApJ 764 167](https://scixplorer.org/abs/2013ApJ...764..167S). The
paper’s written in a Reproducible Research style, coming with code to
reproduce all the figures — which is really awesome, but something to talk
about another day.

The Scargle implementation is in Matlab, with an IDL implementation mentioned
but accidentally not provided, as far as I can
tell. [Jake Vanderplas’ code in the AstroML package](http://jakevdp.github.io/blog/2012/09/12/dynamic-programming-in-python/)
seems to be the main Python implementation. It’s very nice, but has a few
issues with “time-tagged” events, which is Scargle’s term for the X-ray-style
case of binning up rates from individual Poisson events with precise timing.
In particular, the AstroML code doesn’t let you specify the start and stop
times of the observation, which can have huge semantic consequences — e.g., if
you observed two events at times 1.0 and 2.0, your interpretation is very
different depending on whether you observed from times 0.0 to 3.0 or times
-1000 to +1000. The Vanderplas code also doesn’t implement iteration on the
“p0” parameter or bootstrap-based assessment of bin height uncertainties, as
suggested by Scargle.

My implementation has these features. It also has a fix for a mistake in the
Scargle paper: equation 21 has a right- hand side of “4 – \[term\]” when it
should be “4 – log \[term\]”. The new module is
[here on GitHub](https://github.com/pkgw/pwpy/blob/master/scilib/xbblocks.py),
and
[a script for doing a blocks analysis on a Chandra events file is here](https://github.com/pkgw/pwpy/blob/master/xbin/chandrabblock).
Docstrings, tests, etc, are lacking, because as mentioned before I’m a bad
community member. But if you want to fool around with the code, there it is.
