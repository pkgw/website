+++
date = 2015-08-05T15:31:38Z
title = "“The Rotation Period and Magnetic Field of the T Dwarf 2MASSI J1047539+212423”"
path = "2015/08/the-rotation-period-and-magnetic-field-of-the-t-dwarf-2massi-j1047539212423"

[extra]
wp_rel_permalink = "/2015/08/the-rotation-period-and-magnetic-field-of-the-t-dwarf-2massi-j1047539212423/"
wp_shortlink = "/?p=946"
+++

Long time, no blog! I’ve been busy with a bunch of things this summer. These
have mostly _not_ been my latest paper
([DOI](http://dx.doi.org/10.1088/0004-637X/808/2/189),
[arxiv](http://arxiv.org/abs/1502.06610),
[ADS](https://scixplorer.org/abs/2015ApJ...808..189W/)), which
was submitted in February, but it’s finally out in
[ApJ](http://iopscience.iop.org/0004-637X/) so it feels like a good time to
finally write about it.

The full title is _“The Rotation Period and Magnetic Field of the T Dwarf
[2MASSI J1047539+212423](http://simbad.u-strasbg.fr/simbad/sim-id?protocol=html&Ident=2MASSI%20J1047539%2B212423)
Measured From Periodic Radio Bursts”_ and that pretty much sums it up. The
work builds on
[an earlier paper](https://scixplorer.org/abs/2013ApJ...767L..30W/)
describing VLA follow-up of this awesome, cold brown dwarf, a.k.a. 2M 1047+21,
which was discovered to have radio emission by
[Route & Wolszczan in 2012](https://scixplorer.org/abs/2012ApJ...747L..22R/).
The main thing about this radio-emitting brown dwarf is that it’s _much_
colder than the other published radio emitters; its effective temperature is
just 900 K, while
[the next-coolest emitter at the time of discovery](http://simbad.u-strasbg.fr/simbad/sim-id?protocol=html&Ident=2MASS%20J00361617%2B1821104)
is at 1900 K. This is really encouraging since a big theme in the field is
pushing to detect radio emission from cooler, and less massive objects,
hopefully down into the (exo)planetary regime one day. The initial detections
suggested that even at 1000 Kelvin cooler, the radio and magnetic activity
levels held up pretty well.

Our follow-up paper confirms this. With a longer VLA observation, we both
confirmed the first detection and discovered the periodic, highly- polarized
radio bursts that are a hallmark of planet-style auroral current systems. This
isn’t shocking, since both planets and higher-mass brown dwarfs show these
bursts, but this is great evidence that the processes really do scale
continuously between these regimes. And we also get to read the object’s
rotation period off of the burst periodicity. The burst shapes are erratic so
it’s a bit challenging to do this very precisely (see the paper if you’re
curious), but the basic number is about 1.8 hours for a full rotation. This is
about 150 times the Earth’s rotation speed, if you’re talking miles per hour,
though it’s in line with similar objects. That being said, this is one of only
a handful of rotation periods measured for mid/late T dwarfs, since they’re
very faint and seem to have pretty uniformly-colored atmospheres (so that it’s
hard to see clouds rotating in and out of view).

We also observed 2M 1047+21 at slightly higher frequencies and saw a single
burst that seems to be due to the same process. The radio frequency tells you
about the dwarf’s magnetic field strength, so we deduced that this dwarf’s
magnetic field reaches strengths of about 3.5 kG. That’s as strong as you get
in active stars, and about 250 times stronger than what we see in Jupiter,
even though this object is only about 8 times warmer than Jupiter. This makes
me wonder whether there’s some kind of big dropoff in magnetic field strengths
somewhere between the T dwarfs and planets, or whether maybe some planets can
have magnetic fields _much_ stronger than we expect.

Since the discovery of radio emission from 2M 1047+21,
[Melodie Kao](http://www.tauceti.caltech.edu/mkao/) and
[Gregg Hallinan](http://www.astro.caltech.edu/people/faculty/Gregg_Hallinan.html)
at Caltech have announced that they’ve detected radio emission from similarly
cold objects, although the work isn’t published yet. Their target-selection
strategy also yielded a much higher success rate than we typically have in
this field. This is pretty encouraging and means that the race is on to see if
we can get radio emission from even colder objects!

The submitted version of the paper is available not only on
[arxiv.org](http://arxiv.org/abs/1502.06610), but also
{{ulink(path="webtex/goreham", text="on my personal website as an HTML document")}}
using the “Webtex” system I’ve been developing. My goal here is to make it so
that scientific articles are better looking and more valuable on the computer
screen than on paper; all of our writing tools are designed to make printed
documents, leading to a bunch of practices that don’t make sense in the
electronic context (footnotes!). I’ve found it hard to articulate why I think
this is so important, and I don’t think the current prototype makes it
self-evident, but it’s what I’ve got right now. (That being said, there’s
[a lot going on under the hood](https://github.com/pkgw/webtex/commits/master)!)

There’s also
[a video of me giving a short talk about this work at the Geneva “BCool” workshop](http://youtu.be/OqCtFxcBB2s):

{{ youtube(id="OqCtFxcBB2s", aspect="4by3") }}
