+++
date = 2024-06-26T09:30:00-04:00
title = "Reprocessing DASCH’s Astrometry"
+++

Yesterday I completed a large effort to reprocess all of [DASCH’s][dasch]
astrometry. We now have astrometric solutions for 415,848 plates, and with this
groundwork laid I plan to start working on reprocessing the full DASCH
photometric database.

[dasch]: https://dasch.cfa.harvard.edu/

<!-- more -->

There were a couple of motivations for this reprocessing effort. The DASCH
astrometric calibrations have been a bit of a mess, because the astrometric
solutions were stored as WCS headers attached directly to the “mosaic” FITS
files. This is absolutely the most obvious way to store this information, but
I’d argue that it’s actually not a great approach from a data-management
standpoint. In this scheme if you *re*-calibrate a mosaic, you need to rewrite
the entire FITS file, opening up risks of data loss and confusion due to
changing file contents. There’s also no clear way to, say, try a *new*
calibration scheme and compare the results. You could see evidence of the
inflexibility of this approach: the DASCH filesystem indicates a mosaic’s level
of astrometric calibration in its filename (raw, “WW” for basic localization,
“TNX” for distortion correction), and there were lots of mosaics where the
filename was inconsistent with the actual FITS headers. There were also mosaics
with *very* old calibrations using IRAF headers that I’ve never even seen
before, as well as a few ones seemingly lingering from very early work (e.g.,
some files with a “W” tag in their name).

There were also a lot of files that looked like they *should* be easily solvable
that were missing calibrations, in addition to ones with solutions that were
bogus. The latter situation is documented on the DASCH website as the
[“Incorrect Astrometry” known issue][iaki], and it’s the one that bothered me
the most. One of the great things about the [Astrometry.Net] software, which is
the basis of DASCH’s astrometric solutions, is that it should virtually never
produce false positives: if it hands you a solution, that solution is pretty
much guaranteed to be pretty good. So, if the DASCH solutions include bogus
ones, that means that the pipeline is starting with a decent Astrometry.Net
solution and breaking it.

[iaki]: https://dasch.cfa.harvard.edu/drnext/ki/incorrect-astrometry/
[Astrometry.Net]: https://astrometry.net/

I dug into this issue and discovered that it seemed like the primary culprit was
the stage of the DASCH astrometric pipeline that comes after Astrometry.Net.
Once we get the initial solutions, we feed them into the [WCSTools] program
[imwcs], which attempts to refine the solutions. Unfortunately, [imwcs] uses a
numerical optimizer but doesn’t have any conception of a global goodness-of-fit,
which means that if the optimizer somehow converges on an incorrect solution,
it’s very hard to step back and say “no, I don’t like where we ended up”.
[imwcs] is also an extremely old tool written in gnarly C, so it’s challenging
to improve the code.

[WCSTools]: http://tdc-www.harvard.edu/wcstools/
[imwcs]: http://tdc-www.harvard.edu/wcstools/imwcs/

I did end up creating a DASCH-specific fork of [imwcs] and modifying a few
behaviors. For instance, [imwcs] optimizes to match the list of sources in your
image against a set of catalog sources, which is constructed from a list of the
brightest sources in an all-sky catalog in an RA/Dec box around your image’s
initial position. This is fine most of the time, but in DASCH where some images
are *seventy degrees tall*, a box in RA/Dec can be vastly bigger than the actual
image area, so that downselecting to the brightest sources means that only a
handful of reference sources are actually on your image. So, I added a step to
limit the catalog results to ones that actually overlap the image’s initial
position. (If your image is going to shift over a bit, this might mean that you
miss out on a handful of sources that might be useful, but since we’re starting
with an Astrometry.Net solution, we know that we’re starting very close to our
destination, so the vast majority of useful sources will be covered by our
initial guess of the image footprint.)

Another significant improvement wasn’t within [imwcs] itself, but related to the
list of image sources fed into it. This list was derived from a [SExtractor]
table in a pretty naive way, once again just selecting the brightest sources.
This scheme could fail badly for plates with really inhomogeneous backgrounds,
cracks, and other defects. Once again, you would end up with a source list that
only included a handful of actual stars from the image, which is great way to
get the optimizer to drive your solution somewhere unfortunate.

[SExtractor]: https://www.astromatic.net/software/sextractor/

There were a lot of other small fixes as well, trying to improve the robustness
of the pipeline. In the end, the success rate went from 94% to 97% — or, put
another way, I was able to eliminate fully half of the solution failures. What I
haven’t yet been able to look into is the number of incorrect solutions coming
out of the new codebase. I think that it should be a lot lower thanks to the
kinds of improvements I mentioned above, but it’s surprisingly hard to check
into this quantitatively. For any given plate, it’s easy to identify a way-off
solution by eye, but automating such an analysis to cover the full diversity of
the DASCH collection — some plates contain 50 stars, some contain 200,000 — is a
lot harder.

There are also borderline cases, often relating to plates with multiple
exposures. A plate with multiple exposures doesn’t have “an” astrometric
solution — it has a set of several solutions. Any given location on the plate
has several different, valid RA/Dec coordinates, and a single RA/Dec position
may appear at multiple pixel positions on the plate! As you might expect, this
can get pretty gnarly to deal with. The DASCH pipeline works by starting with an
initial list of all of the sources extracted from a plate image, and then
peeling away the sources that match the catalog after the best astrometric
calibration is arrived at. The remaining unmatched sources are then fed into
Astrometry.Net again, and you iterate until Astrometry.Net stops finding
solutions. (So, this process depends highly on Astrometry.Net’s lack of false
positives!)

These plates can fail in a few hard-to-handle ways, though. Particularly tricky
are the ones containing close multiple exposures, like [A21562]. If you have two
exposures very close to one another, the [imwcs] optimizer is vulnerable to what
I call the “split-the-difference” effect, where it converges to a solution that
lands right between each source pair. In a least-squares sense this is indeed
the optimal solution, but it’s incorrect. You can also get an effect where if,
say, the source pairs are in the going in the left-right direction, the solution
has a global skew where it’s bang-on for the left sources in the top-left of the
plate, but bang-on for the *right* sources in the bottom-right of the plate.
This one is even trickier to detect since you can have really high-quality
matches across large areas of the plate — and since many plates have large-area
defects, you can’t expect a plate to have high-quality solutions *everywhere*.
All of these problems get even hairier for plates with *many* exposures, like
[C21253], which appears to contain 11 exposures in a tight spatial sequence.

[A21562]: https://starglass.cfa.harvard.edu/plate/a21562
[C21253]: https://starglass.cfa.harvard.edu/plate/c21253

As best I can see, these plates will need to be handled by some preprocessing.
You could make a histogram of pixel separations for every pair of sources in an
image, and then decompose the peaks to infer how many close-multiple exposures
there are. (Not all multiple exposures are close: some plates have, say, one
exposure on the celestial equator, and one at the pole.) For instance, if a
plate has four exposures, you might find up to six peaks in the source-pair
separation histogram: one for each pair of exposures. (But you might not find
that many if peaks coincide; e.g., for a sequence of exposures with a roughly
regular spacing on the plate.) You could potentially increase the power of the
search by adding a delta-instrumental-magnitude axis to the histogramming
process, since if you have a pair of exposures where one has half the duration
of the other, the repeated source images should all appear fainter by a constant
factor, modulo the nonlinearity of the photographic medium. All of this analysis
could be done before any astrometric or photometric calibration, and then you
could use it to filter down the source lists that you feed into Astrometry.Net
and imwcs, hopefully preventing issues like split-the-difference. I’ve looked
into all of this in a preliminary way, but unfortunately I don’t know if I’ll
get the chance to really pursue this idea.

Anyway. Due to issues like the above, there might be more astrometric
reprocessing in my future, but hopefully we’ve improved the baseline
significantly. The updated astrometric database includes 415,848 solutions, and
everything is based on [Gaia DR2] via the [ATLAS-REFCAT2] catalog. This project
also got me to really straighten out my understanding of how to handle
multiple-exposure plates, as well as some of the technical infrastructure
surrounding them. They represent only a fraction of the corpus, but I think it’s
important to deal with them properly.

[Gaia DR2]: https://www.cosmos.esa.int/web/gaia/dr2
[ATLAS-REFCAT2]: https://archive.stsci.edu/hlsp/atlas-refcat2

The processing took a span of around 46 days all told, starting with a list of
428,416 mosaics. Over the course of that time I did a lot of work to speed the
running of the pipeline, so that if I had to restart everything now, I think
that I should be able to finish in 17 days or fewer. Fun fact: I only made one
improvement to optimize the actual DASCH pipeline code, to fix some catastrophic
failures where a certain step would sometimes take ~forever. Literally
everything else was about making more optimal use of the [Cannon] cluster
resources, which was enough to speed up my processing by a factor of *five* or
more. It pays to understand your platform, folks! The biggest win there was
making a change to avoid the [Slurm] scheduler as much as possible and use my
own job-dispatching system instead. It’s a little disappointing, in a certain
sense, to be able to beat the scheduler at its own game so badly, but on the
other hand Slurm is definitely optimized for highly-parallel simulations, not
lots of little interdependent data-processing jobs.

[Cannon]: https://www.rc.fas.harvard.edu/about/cluster-architecture/
[Slurm]: https://slurm.schedmd.com/documentation.html

As a final point, these new astrometric solutions are *not* yet available in
[the DASCH data access services][dda]. As I mentioned at the outset, the legacy
DASCH data organization just serves up whatever WCS headers are attached to each
FITS image. The reprocessed approach stores the astrometric data separately, as
small data packages that I’m calling “results”. Eventually, these results will
be made available as their own data products, and the various data access
services will combine the mosaic imagery and the astrometric results on-the-fly
as appropriate. But, I need to write and deploy the code to do all of that. In
the meantime, if you’re interested in the new solutions, get in touch.

[dda]: https://dasch.cfa.harvard.edu/data-access/

Now that I have this new baseline of astrometry results, though, I can turn to
the next step: reprocessing all of the photometry. Like the DASCH astrometric
data, the DASCH photometric databases contain 18+ years of accumulated cruft,
and I’m very eager to clean them out! I’ll also use the “result” framework to do
a better job of exposing the photometric calibration data, which I expect to be
very valuable for people who want to dig into the DASCH lightcurves in detail.
In particular, I should finally be able to surface the information that
indicates which plates use which emulsion, which is a super important quantity
that we can’t actually pull out right now.
