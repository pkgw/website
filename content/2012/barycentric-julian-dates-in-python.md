+++
date = 2012-11-12T08:24:37Z
title = "Barycentric Julian Dates in Python"
path = "2012/11/barycentric-julian-dates-in-python"

[extra]
wp_rel_permalink = "/2012/11/barycentric-julian-dates-in-python/"
wp_shortlink = "/?p=588"
+++

(I wrote some code to calculate BJDs in Python. Skip the next few paragraphs
if you don’t want any context.)

I’ve been working on a problem that involves (somewhat) precise timing of
astronomical events over the span of a year or two. Doing this right can be
tricky since we generally learn about events by the light they give off, and
this light can take varying amounts of time to reach the Earth from its
origin; most notably, our motion around the sun changes the distance between
us and sources by enough to affect the apparent time of an event by several
minutes.

Of course, astronomers have known about this issue for a while, which is why
we have
[Barycentric Julian Dates](http://en.wikipedia.org/wiki/Barycentric_Julian_Date)
(BJDs). Times in astronomy are usually reported as some kind of
“[Julian date](http://en.wikipedia.org/wiki/Julian_date),” which is just a
large number counting days since some reference value.
([This is a much subtler task than you might think](http://en.wikipedia.org/wiki/Time_standard).)
When we report a BJD, the apparent time of an event has been adjusted to be
what it would be as if it we were observing at the
[barycenter](http://en.wikipedia.org/wiki/Barycenter) of the Solar System,
which is the coordinate origin of all modern, precise astronomical positional
calculations. _If_ the source of the events is stationary with respect to the
barycenter, this gives us a steady clock with which to measure when each event
happened.

To time precisely, then, we need to be able to compute the BJD of an event
from its apparent time as measured at some telescope. Pulsar astronomers need
to do these calculations to _nanosecond_ precision, so there must be some
ridiculously detailed code out there somewhere, but Google doesn’t bring up
many options. [Jason Eastman](http://www.astronomy.ohio-state.edu/~jdeast/)
and collaborators at Ohio State University have a
[great resource](http://astroutils.astronomy.ohio-state.edu/time/) with
documentation, online calculators, and code. Their core seems to be based on a
routine called
[BARYCEN](http://astro.uni-tuebingen.de/software/idl/aitlib/astro/barycen.html)
by [Eckart Göhler](http://astro.uni-tuebingen.de/~goehler/). Both reference
[documentation](http://lheawww.gsfc.nasa.gov/Craig.Markwardt/bary/) and some
[helper code](http://www.physics.wisc.edu/~craigm/idl/) from Craig Markwardt.

Unfortunately, all of this code is written in IDL. Let us never forget that 1)
IDL is not free and 2) it is a lame, bad, unpleasant language. I generally do
things in Python instead. I haven’t been able to find any Python libraries for
BJD computations, so I
[added support](https://github.com/pkgw/precastro/commit/4d8926366a7ee081afab79beeb527d1e352a4080)
to my
[library for precision astronomy in Python](https://github.com/pkgw/precastro/).

Now, **both the library and the BJD support are halfassed**. The library
builds off of two well-vetted libraries, [IAU SOFA](http://www.iausofa.org/)
and [USNO NOVAS](http://aa.usno.navy.mil/software/novas/novas_info.php), but
itself is not well-tested and only exposes a few routines I’ve needed — none
of which has, in fact, required precision beyond the most basic levels. The
BJD support, meanwhile, **only corrects for the Earth’s orbit**, and so is
accurate to only 0.1 second or so: that is, a hundred million times worse than
what pulsar astronomers need.

But it’s good enough for me! And if you want to compute BJDs in Python,
hopefully you can build off of [precastro](https://github.com/pkgw/precastro/)
rather than starting from scratch. It should be _capable_ of extremely precise
calculations even if there may be some wrinkles to work out, and as always
I’ve tried to make the API as nice as possible.

### Further References

These are basically copied from the resources linked above; I haven’t read
them myself. Standing on the shoulders of giants and all that.

- Eastman J, Siverd R, Gaudi BS.
  [2010 PASP 122 935](https://scixplorer.org/abs/2010PASP..122..935E).
- Fairhead L, Bretagnon P.
  [1990 A&A 229 240](https://scixplorer.org/abs/1990A&A...229..240F)
- Fairhead L, Bretagnon P, Lestrade J-F.
  [1988AUS..128..419F](https://scixplorer.org/abs/1988IAUS..128..419F).
- Fukushima T.
  [1995 A&A 294 895](https://scixplorer.org/abs/1995A&A...294..895F).
- Irwin AW, Fukushima T.
  [1999 A&A 348 642](https://scixplorer.org/abs/1999A&A...348..642I).
- Petit G, Luzum B, eds. IERS Conventions (2010),
  [IERS Technical Note #36](http://www.iers.org/nn_11216/IERS/EN/Publications/TechnicalNotes/tn36.html).
- Standish, E.
  [1998 A&A 336 381](https://scixplorer.org/abs/1998A&A...336..381S).
- [_Tempo_ pulsar timing software](http://tempo.sourceforge.net/).
