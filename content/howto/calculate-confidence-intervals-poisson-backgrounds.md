+++
date = 2014-03-24T16:37:20Z
title = "Calculate Confidence Intervals for Poisson Processes with Backgrounds"
aliases = ["2014/03/confidence-intervals-for-poisson-processes-with-backgrounds"]
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"

[extra]
wp_rel_permalink = "/2014/03/confidence-intervals-for-poisson-processes-with-backgrounds/"
wp_shortlink = "/?p=851"
+++

For some recent X-ray work, I’ve wanted to compute confidence intervals on the
brightness of a source given a known background brightness. This is
straightforward when the quantities in question are measured continuously, but
for faint X-ray sources you’re in the Poisson regime, and things get a little
trickier. If you’ve detected 3 counts in timespan τ, and you expect that 1.2
of them come from the background, what’s the 95% confidence interval on the
number of source counts?

Of course, the formalism for this has been worked out for a while.
[Kraft, Burrows, and Nousek (1991)](http://dx.doi.org/10.1086/170124) describe
the fairly canonical
([222 citations](https://scixplorer.org/abs/1991ApJ...374..344K/))
approach. Their paper gives a lot of tables for representative values, but the
formalism isn’t that complicated, so I thought I’d go ahead and implement it
so that I can get values for arbitrary inputs.

Well, I wrote it, and I thought I’d share it in case anyone wants to do the
same calculation.
[Here it is](https://github.com/pkgw/pwkit/blob/master/pwkit/kbn_conf.py) — in
Python of course. There are a few subtleties but overall the calculation is
indeed pretty straightforward. I’ve checked against the tables in KBN91 and
everything seems hunky-dory. Usage is simple:

```python
from pwkit.kbn_conf import kbn_conf

n = 3 # number of observed counts
b = 1.2 # expected number of background
counts cl = 0.95 # confidence limit
source_rate_lo, source_rate_hi = kbn_conf(n, b, cl)

# here, source_rate_lo = 0, source_rate_hi = 6.61 -- we have an upper limit on
# the source count rate
```

Get in touch if you have any questions or suggestions!
