+++
date = 2023-10-30T11:44:07-04:00
title = "The Wind Speed on a Brown Dwarf"
+++

It’s time for some science! This post will summarize a paper that I co-authored
with a small team, [“A measurement of the wind speed on a brown dwarf”][avbw20].
I’m quite proud of our work in this one!

[avbw20]: https://ui.adsabs.harvard.edu/abs/2020Sci...368..169A

Now, the right time to write about this paper would have been when it came out …
back in 2020. [As I’ve alluded to][dormant], I ended up letting this website lie
dormant for a long time, so I kind of missed that opportunity. But better late
than never, right? It’s also true that the way that my job responsibilities have
broken down recently, I’ve had vanishingly little time for astrophysics, so this
is still one of my more recent projects.

[dormant]: @/2023/new-wwt-tutorial.md

<!-- more -->

Here’s the quick info about the paper:

- Citation: Allers, Vos, Biller, & Williams. *Science* 368 169–172 (2020).
- [The paper on ADS][avbw20]
- [The paper from the publisher](https://doi.org/10.1126/science.aaz2856)

Below I’ll give a narrative summary, focusing on a few parts that I thought were
fun. For the specialists, I’ll call out two points:

1. For this project I developed a utility for “peeling” VLA / MeasurementSet data.
   I don’t write about it here, but there’s an associated [AAS Research Note][peeling]
   and [the code is on GitHub][peelsrc].
1. I have some Thoughts about the how to associate uncertainty with probability
   distributions, which led us to express the radio period measurement in a
   perhaps surprising way.

[peelsrc]: https://github.com/pkgw/rubbl-rxpackage

One thing I like about this paper is that the title says it all: we, well, offer
a measurement of the speed of the wind on a [brown dwarf]. The final number that
we quote, 650 ± 310 meters per second, isn’t incredibly precise, but it’s very
cool that we were able to measure this quantity at all!

[brown dwarf]: https://en.wikipedia.org/wiki/Brown_dwarf

Brown dwarfs are between stars and planets in mass. While a lot of astronomers
tend to think of them as too-small stars (makes sense — we’re used to dealing
with stars), more and more I find myself think of them as extra-massive planets.
In terms of size, even a brown dwarf that weighs 90 times as much as Jupiter may
have a radius that’s only ~10% larger; brown dwarfs have clouds; and a lot of my
research has explored the ways in which the magnetic fields of brown dwarfs are
associated with phenomenologies that are much more planet-like than star-like.

So when you envision the brown dwarf that we studied in this paper, [2MASS
J10475385+2124234][1047] (a.k.a 2M1047), you should envision something that’s a
lot like Jupiter. How do you measure the speed of the wind on something that’s
made entirely of gas, with no “dry land” to serve as a reference point?

[1047]: https://simbad.u-strasbg.fr/simbad/sim-id?protocol=html&Ident=2MASS+J10475385%2B2124234

Well, we did exactly what people do for Jupiter: we compared the speed at which
its visible cloud features move, versus the speed at which its insides move. To
measure the former is pretty “easy” with modern instruments, know-how, and
careful data analysis. We can’t spatially resolve the surface of 2M1047, but we
can monitor its brightness precisely and look for periodic variations associated
with bright and dark surface features moving into and out of view. Because these
features get an extra speed boost from the average wind speed at the altitude of
photosphere, the periodicity of the photometric variability is the bulk rotation
period of the brown dwarf adjusted by that average wind speed.

To measure that wind speed, then, we have to compare that photometric
periodicity to that “bulk rotation period.” Our knowledge of fluid dynamics
tells us that the interiors of these objects can be more-or-less treated like
solid bodies over timescales that aren’t much larger than the rotation period.
So, this “bulk rotation period” is a well-defined concept. But how can we
measure it from afar?

The fun trick, first developed by planetary scientists studying Jupiter, is to
look for pulsed radio emission. The radio pulses are associated with auroral
current systems flowing along the planetary magnetic field — and the field is
anchored to the planetary interior, spinning with it. The radio pulses are more
or less like lighthouse beams, so that measuring their periodicity tells you the
rotation period of the object’s magnetic field, which in turn tells you the
rotation period of the core. Measure the radio rotation period and the
photospheric rotation period (best done in the infrared, here) precisely enough,
and their difference tells you the wind speed. (Well, almost. To get an actual
speed you also have to add an assumption about the object’s radius, which is
relatively well-estimated.)

My contribution was, unsurprisingly, the radio measurements. We observed 2M1047
for three consecutive ten-hour nights with the VLA, detecting around 19 pulses
all told:

{% relfig(path="avbw20fig3.jpg") %}
Figure 3 from [Allers et al., 2020](https://ui.adsabs.harvard.edu/abs/2020Sci...368..169A),
showing the radio pulse data.
{% end %}

The initial data analysis here was mostly fairly standard, although I had to
develop a new data processing utility to perform a “peeling” calibration step.
It‘s described in [this AAS Research Note][peeling], which I may write about
more in a separate post. Once we had our timeseries data, we needed to measure a
period from the data. This took some exploration because, if you look at the
figure above, the pulses are very inconsistent in strength and shape.

[peeling]: https://ui.adsabs.harvard.edu/abs/2019RNAAS...3..110W

We ended up adopting a pulsar-style “time-of-arrival” (TOA) analysis, computing
an arrival time for each pulse, and then computing a final period from that set
of times. With this sort of analysis, the total uncertainty in the period ends
up basically improving linearly as you lengthen the time baseline between the
first and last pulse detections. Pulsar people have really highly-refined
techniques for making these measurements, but they're a bit tricky to deploy
here: while a single pulsar observation might detect hundreds of thousands of
individual pulses, which can then be averaged together to yield a really nice
mean profile amenable to robust statistics, we have a lot less to work with. So,
in the end, the fairest uncertainty quantification that we could come up with
was basically a chi-by-eye:

{% relfig(path="avbw20fig4.jpg") %}
Figure 4 from [Allers et al., 2020](https://ui.adsabs.harvard.edu/abs/2020Sci...368..169A),
showing candidate phasings of the radio pulse data.
{% end %}

Periods between 1.751 and 1.765 hours seem plausible, but ones outside of that
range are noticeably inferior. You might be tempted to write this measuremement
as “1.758 ± 0.007 hr”, but I explicitly advocated that we *not* do so. When you
write down a plus-or-minus number like that, people automatically associate it
with a Gaussian probability distribution: the most likely value is 1.758 hr;
there's a 68% chance that the true value lies between 1.751 and 1.765 hr, etc. I
believe that the most likely value is 1.758 hr, but I really don't have any
confidence that the *shape* of the probability distribution around that value is
Gaussian. And I think it’s important to emphasize that
uncertainty-about-the-uncertainty, because if you just plug in a Gaussian
uncertainty into some downstream calculation, I think you’ll get answers that
are worryingly overconfident about their precision.

In the published paper, we used a simple uniform distribution between 1.751 and
1.765 hr to model the uncertainty in the radio period measurement. This choice,
of course, introduces its own limitations, but I think its fat-middled-ness
(there must be a stats term for that …) is a fairer representation of our state
of knowledge. And while it’s definitely extreme to say that there is *zero*
chance that the true period is below 1.751 or above 1.765, I do think that we
can exclude those values with quite high confidence.

Stepping back, those bounds on our plausible radio periods span just 50 seconds
— very cool! Meanwhile, the uncertainty on our infrared period (which is more
fairly characterized as Gaussian-shaped) is pretty much the same: ±25 seconds.
We can’t compete with the pulsar people for this kind of thing, but I love that
we can get such precise measurements of objects that are light-years away!

This post is already overlong, so I’ll only briefly mention that it was also a
pleasure to write this paper with my coauthors: Katelyn Allers (paper lead; now
out of the field, unfortunately for us), [Johanna Vos][vos] (currently at
Trinity College Dublin), and [Beth Biller][biller] (University of Edinburgh).
Not all science can be done by such small groups, but it’s so enjoyable when it
can!


[vos]: https://johannavos.github.io/
[biller]: https://www.ph.ed.ac.uk/people/beth-biller