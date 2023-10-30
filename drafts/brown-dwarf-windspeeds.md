+++
date = @DATE@
title = "The Wind Speed on a Brown Dwarf"
+++

It’s time for some science! This post will summarize a paper that I co-authored
with three colleagues, [“A measurement of the wind speed on a brown
dwarf”][avbw20]. I’m quite proud of our work in this one!

[avbw20]: https://ui.adsabs.harvard.edu/abs/2020Sci...368..169A

Now, the right time to write about this paper would have been when it came out …
back in 2020. [As I’ve alluded to][dormant], I ended up letting this website lie
dormant for a long time, so I kind of missed that opportunity. But better late
than never, right? It’s also true that the way that my job responsibilities have
broken down recently, I’ve had vanishingly little time for astrophysics, so this
is still one of my more recent projects.

[dormant]: ./new-wwt-tutorial.md

<!-- more -->

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


