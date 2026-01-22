+++
date = 2012-03-15T10:22:41Z
title = "Imaging Algorithms vs. Perfect Data"
path = "2012/03/imaging-algorithms-vs-perfect-data"

[extra]
wp_rel_permalink = "/2012/03/imaging-algorithms-vs-perfect-data/"
wp_shortlink = "/?p=531"
+++

Fun fact: when I’m not rooting my phone or Linuxing my computer, I do some
astronomy. I’ve been doing some data simulations and used the opportunity to
look into the effect that different approaches to interferometric imaging
have. The image below summarizes what I’ve found:

{% figure(path="wp/wp-content/uploads/2012/03/data.png") %}
Residuals from point-source imaging.
{% end %}

Using real ATA observations as a template, I generated some _perfect_ fake
data of a single point source — no noise, calibration errors, etc. I’m
interested in wide-field imaging so I put the source at 0.63 deg from phase
center. I then imaged the data using the
[`casarest`](https://svn.astron.nl/viewvc/casarest/) `lwimager` fork of the
CASA imager with various options.

The _top-left_ panel shows the imaging residuals using traditional
grid-and-FFT and 200 iterations of
[Högbom (1974) clean](https://scixplorer.org/abs/1974A%26AS...15..417H).
The S/N is just 1001 — for ideal data with no noise!

The _top-right_ panel turns on the
[_w_-projection algorithm](http://dx.doi.org/10.1109/JSTSP.2008.2005290) with
128 planes. The S/N jumps to 2987. I’m too lazy to measure it, but the speed
penalty is nontrivial. Annoyingly, a bunch of time is spent in startup
calculating convolution kernels that are constant from one imaging run to
another in most cases. In the imaging that I do, a lot of time could be saved
by caching those kernels on disk.

The _bottom-left_ panel turns on the “wide- field” Högbom clean which
subtracts the modeled CLEAN components from the visibilities. This also adds a
significant time penalty. The S/N increases to 8713.

Finally, and most interestingly to me, in the _bottom-right_ panel I’ve moved
the source slightly to land it precisely in the center of the nearest image
pixel. The S/N jumps to 34542. This effect is investigated in
[Cotton & Uson (2008)](http://dx.doi.org/10.1051/0004-6361:20079104).

_Not shown:_ using [Cotton-Schwab clean](http://dx.doi.org/10.1086/113605)
with _w_-projection gives results nearly identical to those of the wide-field
Högbom clean. Wide-field Högbom should theoretically give more accurate
results since it uses the full dirty beam in the image domain. In a quick
test, CS clean is about 30% faster than wide-field Högbom.
