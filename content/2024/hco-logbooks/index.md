+++
date = 2024-09-19T00:00:00-04:00
title = "Adventures in the HCO Logbooks"
+++

The [DASCH] scanning project may be [officially complete][dc], but that doesn’t
mean that every single plate held by the Harvard College Observatory (HCO)
[Plate Stacks][ps] has been digitized. As part of preparations to squeeze in a
few more scans of some “odds and ends” plates, I’ve been digging into the Plate
Stacks’ collection of historical observing logbooks to learn more about them.
The logbooks provide a fascinating window into how cutting-edge science was
performed, a hundred and twenty years ago.

[DASCH]: https://dasch.cfa.harvard.edu/
[dc]: https://aas.org/posts/news/2024/03/harvards-dasch-scanning-project-now-complete
[ps]: https://platestacks.cfa.harvard.edu/

<!-- more -->

It’s absolutely true that the point of DASCH was to digitize the Harvard plates
to support time-domain astronomy projects. But, as often happens when you take
on a big project like this, you find yourself heading in surprising directions.
You can’t do time-domain science without knowing *when* each plate was made, and
that information isn’t recorded on the plates themselves. (How could it be?)
Timestamps, and lots of other really useful metadata, were recorded in
handwritten observing logbooks that accompany the plates. These logbooks end up
being hardly any less important to doing science with the Plate Stacks than the
plates themselves.

And so a major part of the DASCH project was an effort to digitize these
logbooks. This started in 2006 and was led by volunteer [George Champine][gc],
who built a photo station and photographed all of the books himself. How many?
Try 813 physical volumes, totaling around 185,000 pages of material — a true
tour de force.

[gc]: https://www.legacy.com/us/obituaries/bostonglobe/name/george-champine-obituary?id=19757368

In partnership with [Harvard’s Library Imaging Services][hlis], the DASCH team
obtained improved imaging of some of these logbooks as well as new digitizations
of the personal research notebooks of [Annie Jump Cannon][ajc] and [Henrietta
Swan Leavitt][hsl]. (These notebooks don't record individual observations, but
they do capture the work of two legendary astronomers, and often reference
specific plates in the Harvard collection.) The final photography collection
covers 842 volumes, 166,062 image files, and 1.8 terabytes of data.

[hlis]: https://imaging.library.harvard.edu/
[ajc]: https://library.cfa.harvard.edu/women-at-hco/annie-jump-cannon
[hsl]: https://library.cfa.harvard.edu/women-at-hco/henrietta-swan-leavitt

All of these images are made available as part of the DASCH data products,
although they’re not convenient to browse at the moment. For instance, [this
directory listing][d15b] gives an index of all of the photographs of the 15th
volume of the “D” series of plates, which are somewhat unusual because they’re
photographic duplicates of *other* plates, rather than original exposures.

[d15b]: http://dasch.rc.fas.harvard.edu/scans/dasch/5/d15b/

I’ve spent a lot of time in the D logbooks lately because many of our
odds-and-ends plates come from the duplicates series. This is understandable —
precisely because they’re duplicates of other plates, not many of these will
necessarily improve DASCH’s utility for time-domain astronomy. But it’s still
valuable to create digital surrogates of these objects.

What I quickly noticed was that they seemed to be duplicating the same plates
over and over again. Notice all of the ditto marks in the “Orig. Pl. No.” column
of this page:

{% relfig(path="d15b_0104_sm_crop.jpg") %}
Page 104 of Harvard logbook D15b.
{% end %}

Based on our inventory, our duplicates cabinet appears to include no fewer than
21 copies of plate [A3393][a03393]! Seems a little excessive, no?

[a03393]: https://starglass.cfa.harvard.edu/plate/a03393

What I realized, halfway through my trawl of the D notebooks, was that these log
entries give us a detailed look into the *social* history of astronomy. Everyone
wanted a copy of A3393! Which turns out to make sense — it was one of the plates
used for Leavitt’s research into SMC Cepheids, leading to [the Leavitt Law][ll]
and the understanding that the universe was much bigger than the Milky Way.
Really, these duplicate logbooks are something like the access logs for a
present-day webserver — individual transactions that collectively tell us where
the zeitgeist is at. *Of course* there was a huge demand for plates of Halley’s
Comet right after [its passage in April/May 1910][hc]:

[ll]: https://en.wikipedia.org/wiki/Period-luminosity_relation
[hc]: https://en.wikipedia.org/wiki/Halley%27s_Comet#1910

{% relfig(path="d15b_0212_sm_crop.jpg") %}
Page 212 of Harvard logbook D15b.
{% end %}

You can also see hints of a history that’s much smaller, in a certain sense. Was
the person working in the darkroom on New Year’s Eve, 1927 annoyed that they had
to be there making a copy of [MC22997]? Did they go to a good party after the
day was done?

[MC22997]: https://starglass.cfa.harvard.edu/plate/mc22997

{% relfig(path="d11b_0048_sm_crop.jpg") %}
Page 48 of Harvard logbook D11b.
{% end %}

But as far as DASCH was concerned, the point of photographing the logbooks was
to be able to extract observing metadata about all of the plates. So as the
logbooks were photographed, their text was transcribed into data tables. This
was its own major undertaking, involving contributions from volunteers local to
the CfA, at [AMNH], the [Astronomical League][al], and the [Smithsonian
Transcription Center][stc] (STC), as well as commercial services. These
“logindex” data tables aren’t online, but their contents form the basis of the
[StarGlass][sg] database as well as virtually all DASCH data products relating
to the plates. Beyond DASCH’s focus area, work is [ongoing][cfastc] at the STC
to transcribe the research notebooks of the early HCO astronomers, particularly
the [Women Astronomical Computers][wac], as part of [Project PHaEDRA][phae].

[AMNH]: https://www.amnh.org/
[al]: https://www.astroleague.org/
[stc]: https://transcription.si.edu/
[sg]: https://starglass.cfa.harvard.edu/
[cfastc]: https://transcription.si.edu/browse?filter=owner%3A11
[wac]: https://library.cfa.harvard.edu/glass-plates/women-at-the-harvard-college-observatory
[phae]: https://library.cfa.harvard.edu/project-phaedra

Some of my recent projects, however, have hewed close to the traditional DASCH
activities. Our odds-and-ends collection includes a few dozen plates from a
short-lived “Y” series, obtained in Cambridge using the 8-inch Clark Doublet
telescope in 1888. It appears unlikely that these plates will have much
scientific value, but as long as we have them, I’d like to digitize them and
assemble the highest-quality metadata that I can. This raises its own
challenges.

For instance, as I was entering the Y-series data, I thought that I had everything
that I needed, but was very concerned by [this entry][x01b0156] for plate Y29:

[x01b0156]: http://dasch.rc.fas.harvard.edu/scans/dasch/1/x01b/x01b_0156_sm.jpg

{% relfig(path="x01b_0156_sm_crop.jpg") %}
Page 156 of Harvard logbook X01b.
{% end %}

Dated June 2, 1888, this seems to say that the exposure for Y29 started at
“7h23m (?)”. The written annotation says something like, “Taken 3m after five
[?] stars are visible ... Polaris seen 7 41.” The question is, what kind of time
is that “7h23m”? From the prior entries, I had thought that such timestamps were
giving [local sidereal time][lst], but on that day an LST around 7 hours would
correspond to around 2 PM — not exactly nighttime! Likewise, it’s clearly not a
local time — stars would only start coming out around 9 or 10 PM on a June night
in Cambridge, so even if the time had an implicit “PM” attached, it wouldn’t
work.

[lst]: https://en.wikipedia.org/wiki/Sidereal_time

After a great deal of flailing around, I was able to get some traction thanks to
a remark in [“Investigations in Astronomical Photography”][iap], an 1895 volume
from the [Annals of Harvard College Observatory][anhar]. I was only able to find
this remark thanks to an *entirely separate* imaging and transcription effort —
[ADS]’s unrelenting work to organize and index the pre-digital written history
of scientific astronomy. Currently, ADS appears to have digitized [626
publications in the Annals][anhar] — some of which are hundreds of pages long
individually — as well as 515 articles in the [Harvard College Observatory
Circular][harci]. I’ve found that these publication series provide invaluable
contextual information to help understand the Harvard plate collection.

[iap]: https://ui.adsabs.harvard.edu/abs/1895AnHar..32....1P/abstract
[anhar]: https://ui.adsabs.harvard.edu/search/q=bibstem%3Aanhar&sort=date%20desc%2C%20bibcode%20desc&p_=0
[ADS]: https://ui.adsabs.harvard.edu/
[harci]: https://ui.adsabs.harvard.edu/search/q=bibstem%3Aharci&sort=date%20desc%2C%20bibcode%20desc&p_=0

In my particular case, descriptions of some lunar photographs give precise
timestamps of certain plates made and logged contemporaneously with the Y
series:

{% relfig(path="1895AnHar__32____1P_111.jpg") %}
Page 111 of [Pickering & Pickering (1895)][iap]. “Figure 5 represents Alphonsus
and the surrounding region. The Sun has just risen upon the crater floor, and the
large, variable spot *a*, Plate VII., Fig. 7, is just beginning to appear. *b*
is still more faintly outlined. This photograph was taken at Cambridge, 1888
September 13d 16h 02m G. M. T. Age of the Moon 8d.0.”

[iap]: https://ui.adsabs.harvard.edu/abs/1895AnHar..32....1P/abstract
{% end %}

Various cross-checks convince me that this description refers to plate [X00981]
or [X00982], logged here:

[X00981]: https://starglass.cfa.harvard.edu/plate/x00981
[X00982]: https://starglass.cfa.harvard.edu/plate/x00982

{% relfig(path="x02b_0138_sm_crop.jpg") %}
From page 138 of Harvard logbook X02b.
{% end %}

Other examples give consistent results: if the logbook timestamp system is
*TSS*, we have *TSS = GMT - 8*.

But wait! 16:00 GMT is 11 AM Eastern Time — once again, not a great time of day
for astronomical photography. *Now* what’s going on? This one is a bit easier to
solve: before the introduction of the Universal Time in 1928, GMT days could
interchangeably start at either noon or midnight, depending situationally.
([Thanks, Wikipedia!][wpgmt]) So the Pickerings’ GMT must start at noon. In that
case, the logbooks’ timestamp system must be measuring ... hours after 3 PM
Eastern Time! I don’t understand this choice at all, but it’s consistent with
the evidence: it would put the Y29 observing night as starting around 10:20 PM,
not unreasonable for a Cambridge night near the summer solstice.

[wpgmt]: https://en.wikipedia.org/wiki/Greenwich_Mean_Time

In case you’re wondering, the now-standard US timezone system and GMT as the
world standard were [adopted in 1883 and 1884][tzhist], so the Y-series
observations were happening just a few years after these conventions were
codified. I can imagine that various other aspects of timekeeping that we now
take for granted were still in flux, although it’s still hard for me to see why
the Cambridge times would be referenced to 3 PM local time. It occurred to me
that the timing system is equivalent to counting hours after noon in the
*Pacific* zone, which is “more round” in a certain sense, but I have a great
deal of trouble seeing how it would make sense to anchor your Cambridge time
standard to the West Coast. At least one Cambridge telescope was shipped to
California by the following year (1889), but it would seem utterly bizarre to
adopt a Pacific-oriented timing in the logbooks in advance of the shipment.
Anyway, it’s a bit silly for me to get too deep into speculation — I’m sure that
there are experts in early-modern timekeeping who could shed light on all this.

[tzhist]: https://en.wikipedia.org/wiki/Time_in_the_United_States#History

Of course, it then turns out that halfway through the X02b logbook, they switch
to timestamping using LSTs. Then, some of those are off by 12 hours, a problem
that apparently was observed in a variety of the Harvard logbooks.

While I’m undertaking my investigations for fairly narrow purposes, they
underscore to me just what level of historical richness is centered on the Plate
Stacks. The DASCH dataset represents just one piece of it, and depending on your
perspective it certainly needn’t be considered the most important piece. There
are the glass plates; their jackets; the observing logbooks; the astronomers’
notebooks; card catalogues and looseleaf logs that I haven’t even mentioned yet;
the publications in the Annals and other venues; and a whole wealth of other
historical material spidering out from there. Hundreds of thousands of
photographs and pages of written material. Hundreds of human lives. A whole
[Glass Universe][gu], if you will. The plate stacks are a wonderful historical
resource, representing a unique intersection of science, technology, and
society, and we’ve barely scratched the surface of what they have to offer.

[gu]: https://www.goodreads.com/book/show/29496512-the-glass-universe