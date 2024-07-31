+++
date = 2024-07-31T08:35:22-04:00
title = "Mitigating “Source Splitting” in DASCH"
+++

Last week I added a new feature to [DASCH]’s new data analysis toolkit,
[*daschlab*]. There is now code that aims to help mitigate the [“source
splitting” known issue][sski], in which photometric measurements of a single
source end up divided among several DASCH lightcurves. Supporting this new
feature are some API changes in *daschlab* that will affect most existing
analysis notebooks, a new tutorial notebook, and associated documentation
updates.

[DASCH]: https://dasch.cfa.harvard.edu/
[*daschlab*]: https://dasch.cfa.harvard.edu/drnext/
[sski]: https://dasch.cfa.harvard.edu/drnext/ki/source-splitting/

<!-- more -->

“Source splitting” is the name that I’ve given to a phenomenon that occurs in
the DASCH data with some regularity. Sometimes, if you pull up the lightcurve
for a decently bright star, there will only be a handful of detections, when
there should instead be thousands. If you dig deeper, what you’ll usually find
is that the detections of your source have been erroneously associated with
other nearby stars in the reference catalog. The detections are all there, but
they’re mislabeled as to which star they belong to.

To understand how this happens, it helps to think about how DASCH lightcurves
are constructed. The pipeline processing of each plate image is totally
standard: extract a source catalog from an image, then derive astrometric and
photometric calibrations. Once this is done, the DASCH pipeline “pre-compiles”
lightcurve photometry: the image catalog is matched to a reference catalog
(“refcat”; [APASS] or [ATLAS-REFCAT2]), and any matched detections are appended
to a giant database of photometry for every refcat source. This scheme is
basically how every time-domain lightcurve database works.

[APASS]: https://www.aavso.org/apass
[ATLAS-REFCAT2]: https://archive.stsci.edu/hlsp/atlas-refcat2

(At the moment, the photometric calibration and per-image source catalog are then
basically thrown away. This means that DASCH cannot support forced photometry at
specific locations, among other useful operations. Hopefully we’ll be able
to add this capability in the future.)

One of the issues with DASCH, though, is that our astrometry is quite uncertain.
A major factor here is the fundamental fact that the underlying data are analog,
so we have to construct a digital astrometric solution. But also, DASCH plates
often came from tiny telescopes, which means that they both cover huge areas of
the sky and often have significant distortions away from the optical axis. The
pipeline can do an extremely good job of astrometric calibation, but in a
collection of more than 400,000 images, there are going to be errors.

Personally, I’ve found it hard to get used to the fact that if you have a stack
of DASCH cutouts that are nominally centered around some RA/Dec, the coordinates
for some images might have *significant* systematic offsets. If I have a bunch
of detections of a source, I’m used to, say, plotting the detection RA/Decs and
flagging ones with large offsets as outliers. But in DASCH, those detections
might be totally fine — it could be the WCS solution that’s wrong, not the
source measurement.

Making all of this worse is that the DASCH collection is so heterogeneous. Some
plates in the collection have spatial resolutions 25 times better than others.
For the highest-resolution plates, you might have a region where in the best
cases you can accurately assign photometric measurements to any of, say, a dozen
stars. For the lowest-resolution plates, all of those stars might blend
together, and an astrometric solution that’s overall quite good might still
cause one star to be misidentified with another. You can see how this would lead
to source splitting.

Finally, there’s a contributing factor that I have to confess that I don’t fully
understand. The DASCH pipeline had a lot of infrastructure to search for
transients, like flaring X-ray binaries. As best I can tell, the pipeline had
some mechanism to identify promising transients and add them to the refcat,
presumably with the goal of identifying additional outbursts if there should be
any. But this functionality seems to have interacted poorly with the astrometric
issues above. I believe, but am not entirely sure, that in some cases the
pipeline would end up creating new refcat entries for “transients” that were
really well-known sources identified with the wrong location due to astrometric
calibration errors. All of this is clouded in uncertainty because I’ve chosen to
completely ignore the issue of DASCH transients for the time being, so I haven’t
spent any time learning about this aspect of the historical pipeline. There’s a
ton of scientific potential in this area, but I think that the underlying
calibrations and data products need to be improved before transient searches
will really become worthwhile.

The good news is that when source splitting happens, it’s generally a
well-behaved phenomenon. You’ll generally find that the reference catalog
contains a number of entries near your target of interest (say, 20 or 30
arcsec), and that detections of that target are in effect randomly assigned to
one of those targets. So, for any given exposure containing your source, exactly
one of those refcat entries has a good detection, and the rest are upper limits.
“All” you need to do is merge the detections together and ignore the upper
limits.

The [*daschlab*] software now supports this, in the function
[`daschlab.lightcurves.merge()`]. Given a set of input lightcurves, it will do
precisely what I just wrote, with one minor elaboration. First, the algorithm
matches up all of the lightcurves by exposure and groups together entries that
are totally unambiguous: cases where there is exactly one detection among all of
the lightcurves. It will then use those data to determine a mean source
magnitude. Next, it will make a pass over the ambiguous cases: ones where, at a
given exposure, multiple lightcurves contain detections. For those, it will
choose the lightcurve point whose magnitude is closest to the mean. This is a
pretty dumb heuristic, but in my samples there are only a handful of ambiguous
points (fewer than ten, in lightcurves with thousands of detections), so in most
cases it should be good enough.

[`daschlab.lightcurves.merge()`]: https://daschlab.readthedocs.io/en/latest/api/daschlab.lightcurves.merge.html

To demonstrate this code, I created a new [tutorial notebook][tut] for the
example eclipsing binary [HD 5501]. For whatever reason, the “official” DASCH
catalog match for this source only has a few dozen detections; the vast majority
of the detections are associated with a source about 10 arcsec away. The
notebook demonstrates how to identify this issue, how to choose which
lightcurves to merge, and how to actually do the merge. In this case, the number
of usable lightcurve points goes from about 2,000 (associated with a single
refcat source) to about 2,500 (merged from ten separate lightcurves).

[tut]:https://dasch.cfa.harvard.edu/drnext/#tutorials
[HD 5501]: http://simbad.u-strasbg.fr/simbad/sim-id?protocol=html&Ident=hd%205501&NbIdent=1&Radius=2&Radius.unit=arcmin

Of course, this is one of those situations where software support is nice, but
the real thing to do is to **actually fix the problem**. I think that the
pipeline’s matching code probably ignores a-priori brightness information
inappropriately. If I have a measurement of 10.0 mag, I should match it with a
refcat source logged with a mean brightness of 10.1 mag, even if the coordinates
of the detection are nominally slightly closer to something with a brightness of
15.0 mag. And, of course, [improving the astrometric calibration][astrom] will
help. But I don’t want to get deep into mucking around with the photometry /
lightcurve pipeline right now, so it seemed worthwhile to try to provide a
mitigation in the meantime.

[astrom]: @/2024/dasch-astrometry.md

To support the merge functionality, I made an update to *daschlab* that I’ve
been avoiding for a little while. One of the key tables in a *daschlab* analysis
session used to be the “plate list”: a table of information about every plate
overlapping the coordinates of interest. But, this was actually the wrong table
to offer. Why? Because some plates record multiple exposures, and each exposure
has its own sky coordinates. Sometimes, these exposures were of totally
different parts of the sky: an equatorial survey field and a polar calibration
field, for instance. So we should really have a table of *exposures* that
overlap your target. There might be multiple exposures on a single plate, each
of which covers your target but maps to a different portion of the plate’s
digital image. I realized this a while ago, but put off dealing with it because
it would be a pain to make all of the necessary changes. To do the merges
properly, though, I needed to get this right, so I’ve torn off the band-aid: the
*daschlab* plate table has becomes the [`Exposures`] table, and lots of
associated aspects of the API have been reworked. This is a somewhat annoyingly
invasive change, but these early days are absolutely the time to get things
right. My apologies if this has messed up anyone’s existing notebooks.

[`Exposures`]: https://daschlab.readthedocs.io/en/latest/api/daschlab.exposures.Exposures.html

One additional wrinkle is that exposures come in two kinds. From the written
logbooks that were created contemporaneously with the plates, we know about what
exposures should exist in principle: plate [C09375] is logged to have two
exposures. But we can also identify exposures from astrometric analysis of plate
images: the DASCH pipeline uses the [Astrometry.Net] software to find an
astrometric solution, analyzes it, then searches for another solution hidden in
all of the sources that don’t match to the refcat using the first one. Do these
two kinds of analyses always agree? Hah — of course not.

[C09375]: https://starglass.cfa.harvard.edu/plate/c09375
[Astrometry.Net]: http://astrometry.net/

The DASCH data model therefore counts both “exposures,” described in the
historical logbooks, and “WCS solutions,” obtained from analysis of plate
images. When possible, WCS solutions are matched to known exposures. But
sometimes you have more exposures than solutions, and sometimes you have more
solutions than exposures. If a plate has been scanned, some exposures might be
mappable to the scan image (if it has an associated WCS solution), and some
might not.

While exposures lacking WCS solutions obviously don’t have high-quality
astrometric information, we do have basic information about their rough sky
positioning, so we’re still interested in them. The *daschlab* exposures list
therefore includes all available information. This means that some exposures in
the list can be mapped to a digital image, and others can’t — and the latter
situation may occur even if the plate in question has been scanned.

Correcting this situation caused me to come to understand an important
limitation of the DASCH mosaic-cutout software. In both [the “Cannon” legacy
DASCH data access portal][cannon] and *daschlab*, the cutout-generating software
doesn’t know about multiple exposures. If you request a plate cutout at a
certain position, you’ll get a cutout using the first derived WCS solution
(which may or may not be the first logged exposure), even if perhaps those
coordinates only land on the plate using one of the other WCS solutions. Due to
the way that the legacy pipeline handled multiple-exposure plates, I can’t
easily fix this issue immediately. But, in the aftermath of [my reprocessing of
the DASCH astrometry][astrom], the associated data products are going to be
significantly improved, and I’ll address this problem.

[cannon]: https://dasch.cfa.harvard.edu/data-access/#cannon-data-portal

