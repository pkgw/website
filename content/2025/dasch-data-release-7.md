+++
date = 2025-01-22T10:41:58-05:00 # deploytool
title = "DASCH Data Release 7"
+++

It’s out! On December 29th, I finalized [DASCH Data Release 7][dr7]. Depending
on how you want to think about it, this data release represents the culmination
of two years of my effort on the project, or of 19+ years of work on the [DASCH]
project, or maybe even of more than 130 years spent building and curating the
Harvard College Observatory [Astronomical Photographic Glass Plate
Collection][stacks].

[dr7]: https://dasch.cfa.harvard.edu/dr7/
[DASCH]: https://dasch.cfa.harvard.edu/
[stacks]: https://platestacks.cfa.harvard.edu/

<!-- more -->

You can be confident that some astronomical datasets — say, cosmological
simulations — are going to be superseded after just a few years: there will
always be a newer, bigger simulation, or a deeper observation made with a better
telescope. But the chief scientific appeal of the Harvard plates is their
century-long time baseline, which isn’t something you can beat without spending
a hundred years gathering your own data. So even though we’re not creating new
photographic plates anymore, it’s not unreasonable to think that the Harvard
collection may still be scientifically relevant when future graduate students
are celebrating the dawn of the 22nd century.

This in turn means that if you’re going to make a data release based on the
Harvard plates, you should do so with an eye cast towards the long term. For
DASCH DR7, this was especially true. Because, while I sincerely hope that there
will be major projects in the future that build on the legacy of DASCH, the
honest truth is that there might not be: there isn’t any funded work happening
right now, and it’s not clear if anyone with the ability to change that has the
interest. It’s possible that DR7 will serve as *the* foundation upon which the
ultimate scientific legacy of the Harvard plate collection is built. No
pressure.

With this context in mind, my final weeks working on DR7 were unfortunately not
spent on fun analyses, or even developing new tutorials and documentation.
Instead I was focused on making sure that the foundations of the data release
were secure, and that the absolutely essential data products were in place.
Those data products include: full-plate FITS images of more than 428,000 plates
(the mosaics); about 800,000 photographs of the plates and their paper jackets;
about 170,000 photographs of observing logbooks and astronomers’ scientific
notebooks; astrometric and photometric calibration solutions derived from the
plate images; and calibrated photometry organized into a database of lightcurves
for around 250 million sources.

I expect that most scientific users of DASCH will only ever interact with that
lightcurve database. This feels like a bit of a shame, since the DASCH imagery
collection is so impressive. But in a certain sense, the whole goal of the DASCH
science pipeline is to make it so that you don’t have to look at the actual
plates.

Perhaps my biggest regret regarding DR7 is that I don’t have a good sense of how
well that goal is actually achieved. It’s definitely true that you have to
review any DASCH lightcurves and apply at least some level of data quality
filtering: the underlying data are just too heterogeneous to be turned into a
standardized product with blanket quality guarantees. What I don’t know, though,
is how automatable that filtering is for typical science cases, given the
metadata that we’ve been able to make available. The lightcurve tables carry a
lot of metadata columns, but do they capture the right quantities?

I’m particularly concerned about cases where the astrometric solution is far
enough off that it leads to source misidentification. This can happen even when
a plate’s solution is, overall, extremely good; some of the plates are large
enough (both physically, and on the sky) that a relatively small inaccuracy in
the distortion solution in one corner of a plate can lead to misidentifications,
even if 99.9% of the sources on the plate overall are in perfectly good shape.
The offset between the cataloged source position and measured source position
will give you a hint about whether this has happened, but a correctly-identified
source might always have a relatively high offset, and every so often errors
will place the wrong source exactly in the position of the true source of
interest. The wide range in spatial resolutions of the plates doesn’t make it
any easier to set cutoffs here, and single-source metrics can’t easily reveal
these situations because misidentified sources are in fact perfectly real and
trustworthy.

In a perfect world, a fully polished DASCH data release simply wouldn’t have
these kinds of issues. In a slightly more realistic world, the data release
would address them as much as possible, and provide tons of guidance and support
tools for handling any lingering pitfalls. Unfortunately, DR7 is a long way away
from that. I’ve tried to structure some potential community resources around DR7
— the [mailing list][ml], the [*daschlab* GitHub repo][dl] — so that users at
least have the chance to come together and start developing the kinds of tools
and best practices that will be needed to get the best science out of DR7. In
the best-case scenario, a true DASCH user community will emerge in these spaces,
and perhaps this community will be able to mobilize the resources needed to
undertake bigger projects based on DASCH — a Data Release 8 and beyond. Whether
this comes to pass or not is, realistically, almost entirely out of my hands,
but I hope to do my best to encourage it.

[ml]: https://gaggle.email/join/dasch@gaggle.email
[dl]: https://github.com/pkgw/daschlab

That being said, my other major hope for DASCH DR7 is that it won’t be used just
for science. Don’t get me wrong: I hope that tons of cutting-edge astrophysics
is done with the DASCH data. But the Harvard plate collection isn’t just a
scientific resource. It’s a *cultural and historical* resource too. Above all
else, of course, it’s associated with the pioneering [“women astronomical
computers”][wac] from the early days of HCO. That historical connection
motivates all sorts of people to be interested in the plates: authors like [Dava
Sobel][tgu], playwrights like [Stella Feehily][sf], and especially visual
artists, including [Anna Von Mertens][avm], [Lia Halloran][lh], [Aura Satz][as],
[Ligia Bouton][lb], and [Victoria Burge][vb]. Hopefully, the DASCH data products
will be useful to many other people like these women, even if they only capture
one aspect of why we’re interested in the plates.

[wac]: https://library.cfa.harvard.edu/terminology-women-astronomical-computers
[tgu]: http://www.davasobel.com/books-by-dava-sobel/the-glass-universe
[avm]: https://annavonmertens.com/portfolio/measure-exhibition-for-the-radcliffe-institute-harvard-university/
[lh]: https://liahalloran.com/lax-installations-your-body-is-a-space-that-sees
[as]: https://www.iamanagram.com/HerLuminousDistance.php
[lb]: https://www.mtholyoke.edu/news/news-stories/art-professor-installs-25-variable-stars-commuters
[sf]: https://www.hampsteadtheatre.com/whats-on/2024/the-lightest-element/
[vb]: https://victoriaburge.com/works-on-paper/star-data

In that vein, I always try to point out that one of the wonderful things about
the plate collection is that there’s so much more to it than the plates
themselves. I’m no historian, but it seems to me that the plates are the nexus
of a remarkably meaty collection. As alluded to above, there are hundreds of
thousands of paper jackets, which can contain comments and annotations that
inform our understandings of the plates. The thousands upon thousands of pages
of observing logs offer similar [insight into the historical work of the
observatory](@/2024/hco-logbooks/index.md). Beyond the strict confines of the
Plate Stacks’ physical holdings, there’s the massive collection of historical
astronomer notebooks that underpins [Project PHaEDRA][phaedra]; since the plates
were in many cases the life’s work of these astronomers, the notebooks are
tightly bound to the collection. Then there’s the associated scientific research
record, especially the historical [Annals][ha], [Bulletin][hb], and
[Circulars][hc] of Harvard College Observatory.

[phaedra]: https://platestacks.cfa.harvard.edu/project-phaedra
[ha]: https://ui.adsabs.harvard.edu/search/q=bibstem%253AAnHar
[hb]: https://onlinebooks.library.upenn.edu/webbin/serial?id=bulharvcolobs
[hc]: https://ui.adsabs.harvard.edu/search/q=bibstem%253AHarCi

I have difficulty of thinking of any examples of other comparable collections:
ones that are both deeply relevant to cutting-edge scientific research, *and*
have such historical and cultural weight. I’m not exactly sure what people will
do with that, but it feels like an extremely powerful thing to me. At first
blush, I’d like to think that in the right hands, the plates and associated
materials ought to be a gold mine for education and public outreach.

I guess I have one final hope for DASCH DR7, returning to the topic of the
long-term perspective. Right now, a lot of the DR7 data live in just two places:
on [AWS S3][s3], and on spinning disk at [FAS Research Computing][fasrc]. The
vast majority of the raw scan data also have copies on two sets of magnetic
backup tapes, currently boxed up in my office. I don’t view this situation as an
emergency, but not exactly an archiving setup to be proud of — honestly, it’s
closer to embarassing, given our aspirations for the dataset.

[s3]: https://aws.amazon.com/pm/serv-s3/
[fasrc]: https://rc.fas.harvard.edu/

I would like to see there be at least one other complete copy of DR7 held
somewhere other than AWS, and preferably several. To help allow this to happen,
as one of the final steps of preparing DR7 I put together something that I’m
calling a [“digital inventory”][di]. It’s a table of every single data file
created as part of DASCH — OK, not *every* file, but everything judged to be
worth archiving as part of DASCH’s legacy. The files are organized in a
folder-like naming hierarchy, and for every single file the table records its
size, its MD5 digest, and one or more locations where it’s currently archived:
at least one location in [S3][s3], and potentially other locations on the
magnetic backup tapes.

[di]: https://doi.org/10.5281/zenodo.14563521

On the basis of this inventory, I can tell you that DASCH DR7 consists of
exactly 33,791,530 files, adding up to 745,627,062,858,355 bytes of data.
(That's about 680 TiB. The inventory itself is about 10 gigabytes!) So, copying
the whole thing isn’t a minor undertaking — it’s way too much data just drop
into a research data archive like [Zenodo]. But you don’t exactly need to be a
nation-state to handle this much data, either. I don’t expect that I’ll wake up
one day to find an email from someone just offering to host a copy of the
archive, but hopefully Harvard can find one or more institutions to partner with
to help further secure the data. As recently argued in a [“Century-Scale
Storage”][css], a very nice piece by Maxwell Neely-Cohen at the Harvard Law
[Library Innovation Lab][lil], rampant duplication is the only long-term
preservation scheme that you can really trust in.

[Zenodo]: https://zenodo.org/
[css]: https://lil.law.harvard.edu/century-scale-storage/
[lil]: https://lil.law.harvard.edu/

Since DR7 is a legacy data release, the 34 million files listed in the digital
inventory include a lot more than just the immediate scientific data products.
They include the raw data products from the DASCH scanner; its calibrations;
logs from the DR7 data processing runs; supporting databases; other derived
files needed to support the DASCH data access services; the source code to all
DASCH software; and all available project documentation. My hope is that
whatever may happen to Harvard’s servers or anything else, if you have those
files, you have essentially everything there is to know about DASCH. Of course,
there’s a tension here: if you just throw every file you can find into a giant
bucket, that can be even worse in practice than an archive that’s missing a few
key files. (This is exactly what I experienced when I first came on board DASCH,
as a matter of fact.) I’ve done my best to make the archive exhaustive, without
being exhaust*ing*.

Finally, some thanks are in order. In the various DR7 materials, I made a point
of recognizing Daina Bouquin, former Head Librarian of Wolbach Library, above
anyone else. Not only did she secure the [Smithsonian American Women’s History
Initiative Pool][awhip] grant that directly supported my work on DR7, and play
an essential role in steering DASCH towards its successful completion; she also
set the tone for a team at the Wolbach library that was truly visionary,
simultaneously looking forward to a future of digital-native librarianship,
without losing sight of the importance and meaning held by historical materials
such as the plates. If it were up to me, she and the Wolbach team would be
showered in more resources than they know what to do with. Instead, [Wolbach is
now closed](@/2024/requiem-for-a-library.md).

[awhip]: http://amwh.us/smithsonians-american-womens-history-initiative/

Since this blog doesn’t have length limits, here I can also thank everyone who
worked at the Plate Stacks over the past few years: curator Thom Burns,
assistant curator Lisa Bravata, and assistants Elijah Berk-Silverman, Autumn
Brown, Claire Chandler, Elizabeth Coquillette, Sam Imperato, Kevin Jean-Poix,
Samantha Joyce, Amy Kushlan, Adam Leibell-McLean, Victoria Lento, Kayleigh
MacDonald, Julia McGehean, Samantha Notick, Meta Partenheimer, and Gav Spano.
(My sincerest apologies if I missed a name here.) The tale of the plates has
traditionally been one of unsung heroes, but some traditions need updating. Of
course, assistants from previous years are no less deserving of recognition even
if I haven’t personally met them; the [DASCH Team page][dt] attempts to name all
of them.

[dt]: https://dasch.cfa.harvard.edu/team/

Ben Sabath, Tegshjargal (Tejka) Jambaltseren, and Ricardo DeLima developed the
[Starglass] platform in parallel with my DASCH work, and the revamped DR7 data
access services that I put together in the last few months of 2024 built on
(i.e., massively exploited) the web API framework that Ben in particular
developed.

[Starglass]: https://starglass.cfa.harvard.edu/

It’s been a privilege to work on this project; rarely do you get a chance to be
so directly involved with capital-H History. I see DASCH as an astronomy
research project but also, wittingly or not, part of a broader effort to honor
the legacy of the women astronomical computers and everything that they’ve come
to stand for. I can honestly say that I played my part in this story to the best
of my ability, and I’d like to think that I’ve joined a chain of human
connection that stretches back more than a century. Here’s hoping that chain
will continue extending just as far into the future.
