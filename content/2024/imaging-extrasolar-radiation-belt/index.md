+++
date = 2024-02-15T10:19:02-05:00
title = "Imaging an Extrasolar Radiation Belt"
+++

Time for some actual astrophysics! I want to point people to a result from last
year that I’m really excited about: [Resolved imaging confirms a radiation belt
around an ultracool dwarf][kmvs23], by [Melodie Kao][kao] and colleagues ([2023
Nature 617(7969) 272–275][doi]).

[kmvs23]: https://scixplorer.org/abs/2023Natur.619..272K/abstract
[kao]: https://melodiekao.com/
[doi]: https://doi.org/10.1038/s41586-023-06138-w

<!-- more -->

One way to think about being a scholar is to ask: what ideas am I fighting for
or against? A field like astronomy is made up of thousands of interconnected
ideas about the world. The finitude of human experience being what it is, you
can only engage with a small number of them in any deep, significant sense. The
ideas that you engage with deeply aren’t *yours* — they don’t belong to any of
us — but they do become part of your academic identity.

As modern scholarship gets more and more specialized, the ideas that define our
professional identities get narrower and narrower. Einstein was The Relativity
Guy. Me, one of the ideas that I’ve tried to promote over the years is that we
should think of magnetically active ultracool dwarfs as being scaled-up planets,
magnetically speaking, rather than scaled-down stars. (Best citation for that:
[Williams 2018][w18].) It’s not the flashiest idea out there, for sure. But I
think it’s a good, solid one that deserves to be more widely appreciated.

[w18]: https://scixplorer.org/abs/2018haex.bookE.171W/abstract

An important line of evidence for this idea is that radio observations of
ultracool dwarfs show a lot of features that can be fruitfully interpret as
signatures of planetary [radiation belts][rb] (aka Van Allen belts). These belts
are associated with intense, bursty, highly polarized maser emission, which we
definitely observe in the ultracool dwarfs, and also host a more quiescent
population of energetic electrons forming what I like to call a “radio donut”. A
super neat paper by Bob Sault et al. ([1997 A&A 324 1190–1196][sodl97]) imaged
Jupiter’s radio donut in 3D:

[rb]: https://en.wikipedia.org/wiki/Van_Allen_radiation_belt
[sodl97]: https://scixplorer.org/abs/1997A%26A...324.1190S/abstract

{% relfig(path="sodl97_fig3.jpg") %}
Interferometric 3D reconstruction of Jupiter’s synchrotron radiation belt
emission at 2 GHz (Sault et al., Figure 3, partial).
{% end %}

If you look at Jupiter using a radio telescope without Bob’s fancy techniques,
you see a 2D projection of the donut. It varies periodically with the planet’s
rotation, in a way that’s suprisingly strong due to synchrotron beaming effects:

{% relfig(path="vlamoviecropped.gif") %}
Jupiter's radiation belts in 2D over time (NASA/JPL — Caltech;
<a href="http://www.vofoundation.org/blog/nasas-juno-spacecraft/">source</a>).
{% end %}

I will assert without proof that there’s a ton of neat physics in these
radiation belts, and as an astronomer it’s really cool to dig into this stuff
and learn about all the fun things that planetary scientists have been doing in
this field for decades. There’s also a promising opportunity the planetary
scientists to profit from what we observe outside the solar system: Jupiter’s
radiation belts are in many ways still quite mysterious, to the extent that
there’s a NASA Heliophysics mission concept called [COMPASS] that envisions
spending a billion-ish dollars to send a probe to understand them better. (I’m
involved in its science definition team.)

[COMPASS]: https://scixplorer.org/abs/2023BAAS...55c.067C/abstract

Melodie is also an advocate for the idea that ultracool dwarfs have planet-like
magnetospheres that should host radiation belts. If this idea’s actually
correct, then around these objects you should have radio donuts resembling the
Jovian one. I would argue that time-series data certainly support that
conclusion, but can we actually make an image of one?

It turns out: yes!

{% relfig(path="kmvs23_fig1.jpg") %}
Three epochs of VLBI imaging of LSR J1835+3259 at a spatial resolution
of around 1 mas (Kao et al., Figure 1).
{% end %}

Melodie and her colleagues observed one of the well-known radio-emitting
ultracool dwarfs with the ultra-high-resolution [Very Long Baseline Array][VLBA]
in an inspired gambit to image any radiation belts that might be there. And it
worked! The overall morphology is stable across three epochs of observations,
suggesting a long-lived structure as one would hope to observe, and the physical
characteristics that you can infer from the data (the separation of the two
lobes, total luminosity, etc.) are all plausible.

[VLBA]: https://public.nrao.edu/telescopes/vlba/

(I should mention that just after Melodie et al.’s paper came out, [Climent et
al.][cgpt+23] published essentially the same result in *Science* ([2023
*Science* 381(6662) 1120–1124][cgpt+23doi]; [arXiv:2303.06453][cgpt+23arxiv]).
Presumably there’s a bit of a story there, but I’m not familiar with it. I know
Melodie and her coauthors, and have talked with her about this work, so that’s
where I’m focusing.)

[cgpt+23]: https://scixplorer.org/abs/2023Sci...381.1120C/abstract
[cgpt+23doi]: https://doi.org/10.1126/science.adg6635
[cgpt+23arxiv]: https://arxiv.org/abs/2303.06453

As the [EHT] folks can tell you, it can be a really big deal to be able to
attach an image to an idea! My job responsibilities have kept me away from doing
much in this field (or really, any kind of day-to-day astrophysical research)
for a while, so it’s genuinely really cool to see other people pushing forward
some of the ideas that I want to see succeed. The results of Kao et al. and
Climent et al. are awesome steps forward, and hopefully harbingers of more great
stuff to come!

[EHT]: https://eventhorizontelescope.org/
