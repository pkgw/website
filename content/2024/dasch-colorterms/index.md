+++
date = 2024-12-04T16:24:00-05:00 # deploytool
title = "DASCH Data Services Now Expose Colorterms"
+++

DASCH [Data Release 7][dr7] (DR7) is imminent. With the [scanning
complete](@/2024/dasch-scanning-completion.md) and [the astrometry and
photometry fully reprocessed][reproc], I’ve been working on finalizing the DR7
data access services. The move to [cloud-based data access
APIs](@/2024/dasch-cloud-apis/index.md) has been complete success, in my view —
not only has it reached feature parity with the existing systems, but it’s
unlocked my ability to rapidly deliver useful new features. The most recent
example of this is that DASCH’s “colorterm” data are finally accessible via the
new APIs, providing users with quantitative information about the different
emulsions used on different plates.

[dr7]: https://dasch.cfa.harvard.edu/dr7/
[reproc]: https://dasch.cfa.harvard.edu/news/#2024-october-15

<!-- more -->

When modern astronomers use visible-light telescopes, they nearly always place
some kind of colored [filter] in the optical path to control the wavelengths of
light that hit their detectors. There are profound reasons why you want to have
a precise control over the bandpass response of your overall detection system,
and you can’t control the bandpass response of a CCD detector once it’s
manufactured. So, slap some filters in front of it.

[filter]: https://en.wikipedia.org/wiki/Astronomical_filter

In the days of photographic astronomy, the situation was different. Instead of a
single expensive electronic detector, you had a stack of glass photographic
plates. Each plate had its own [photographic emulsion], and each emulsion could
have its own bandpass response. I’m far from an expert in the history here, but
my impression is that early photographic astronomers fully recognized the value
of being able to select emulsions with different spectral responses, and so
there was a ton of experimentation with and exploration of different emulsions
and their properties. Eventually (I don’t have a good sense as to when), things
settled down into some standard commercial options: early on, blue-sensitive
Eastman 103a-O and red-sensitive Eastman 103a-E. Later popular options included
Kodak IIIa-J, IIa-D, or IV-N. [This DSS page][dss] contains links to some nice
plots of the spectral responses of some of these. It also shows that even though
astronomers could choose emulsions on a plate-by-plate basis, they started
adding filters into their systems to get even more bandpass control. So, for
instance, the SERC-J survey was conducted in a “IIIa-J + GG395” configuration,
combining Kodak III-aJ with a [Schott GG395] filter. (Star magnitudes obtained
in this configuration are said to be in the “Jpg” system, which might be a bit
confusing to modern readers what with [the image format][jpeg]. The “pg” is
short for “photographic.”) It’s amazing to me that some of these specialized
products have been manufactured to consistent specifications for a century or
more, at this point.

[photographic emulsion]: https://en.wikipedia.org/wiki/Photographic_emulsion
[dss]: https://gsss.stsci.edu/SkySurveys/Surveys.htm
[Schott GG395]: https://www.us.schott.com/shop/advanced-optics/en/Matt-Filter-Plates/GG395/c/glass-GG395
[jpeg]: https://en.wikipedia.org/wiki/JPEG

Anyway, if you’re trying to do science with the Harvard plates, it’s generally
quite important to know what emulsions they used. If one star looks much
brighter on one plate compared to another, is it because it actually got
brighter, or because it’s a red star and the plate is red-sensitive?
Unfortunately, the historical logbooks didn’t always annotate which plates used
which emulsions, and that kind of information is always prone to transcription
errors anyway. In this area DASCH’s database of *a priori* plate information was
not nearly comprehensive enough to be useful for modern researchers.

This can be a real problem. Here’s an example copy-pasted from the DASCH docs:
the very southern star [TYC 9504-35-1][tyc]:

[tyc]: http://simbad.cds.unistra.fr/simbad/sim-id?Ident=%407766367&Name=TYC%209504-35-1&submit=submit

{% relfig(path="tyc9504351.jpg") %}
Partial DASCH lightcurve of TYC 9504-35-1, using the APASS calibration.
{% end %}

It appears that in the 70’s, this source goes haywire, with its brightness
seemingly becoming multimodal. Surely this is unphysical, and indeed it is. The
measurements come from three sets of “Damons South” plates, which used three
different emulsions: “blue“, “red”, and “yellow”. (These are plate series `dsb`,
`dsr`, and `dsy`.) This star is relatively red (Gaia DR3 T<sub>eff</sub> of
around 4700 K), and apparently the APASS catalog has a missing or low-quality
color measurement for it. The different groups of measurements correspond to
plates using the different emulsions, with different errors as the calibration
scheme attempts to adjust all of the magnitudes to the main APASS system
(Johnson *B*) using the information available. This example demonstrates a
second-order danger of unknown emulsions: since the use of different emulsions
waxed and waned over time, emulsion-related photometric systematics can manifest
as time-dependent systematics. This is a big problem for DASCH, where
time-domain photometry is the major science driver.

Fortunately, self-calibration comes to the rescue. By this, I mean that given a
typical DASCH image, I can calibrate it photometrically with virtually zero
additional data — modern photometric catalogs contain so many data points that
my image is practically guaranteed to contain hundreds of stars with
well-measured magnitudes; enough to solve for any needed calibration parameters,
such as the spectral response of the plate. No logbook information needed!

In the DASCH pipeline, the spectral response parameter is called the
“colorterm”. This is because the pipeline isn’t actually solving for a bandpass
shape or anything; instead, it’s determining a coefficient that’s used to shift
the reference magnitudes as a function of the stellar color, another quantity
recorded in the reference catalog. For the APASS reference catalog, this is
specifically:

> *M<sub>plate</sub>* = *f(B + c(B-V))*

The reference catalog provides a magnitude in the Johnson *B* system as well as
a *B-V* color; *M<sub>plate</sub>* is the “instrumental” magnitude determined by
analyzing the plate image; *c* is the colorterm, which the pipeline solves for;
and the function *f* represents the effect of all of the other photometric
calibration parameters, which we’re not worrying about right now.

The colorterm *c* therefore represents a linear interpolation between working in
the *B* and *V* magnitude systems. This is … not really well-founded, if you
think about it. If we take these *B* and *V* numbers to be representing an
integral of a product of the source’s intrinsic spectrum multiplied and the
bandpasses of the *B* and *V* filters (I think even [Hogg] would be OK with
that), the interpolation gives us pseudo-magnitudes that we’d obtain if we
observed the source with some filter whose bandpass was a
wavelength-by-wavelength linear interpolation between the *B* and *V* filters.
There is almost surely no actual filter that can be described that way. But, the
colorterm scheme works well enough, especially in DASCH, where our measurements
are realistically only going to be accurate at the 0.1-mag level or so.

[Hogg]: https://arxiv.org/abs/2206.00989

At any rate, while colorterms may not directly measure the bandpass response of
anything, they do diagnose the plates’ color sensitivities. Here’s a histogram
of all of the APASS colorterms in the DASCH corpus:

{% relfig(path="apass_colorterms.svg") %}
Histogram of DASCH colorterms derived from the APASS reference catalog
(*B*/*V* magnitude systems).
{% end %}

The histogram has three obvious peaks, corresponding to “red”, “yellow”, and
“blue” emulsions, going from left to right. The majority of Harvard plates are
blue, driving the height of the rightmost peak. You may discern what looks like
a fourth small peak around a colorterm value of -0.6; this is real, and appears
to correspond to yellow plates exposed behind yellow filters, based on the
annotations associated with some of the plates in that group.

Great! Armed with this plot (and an analogous one for the ATLAS refcat) we can
infer which plates used which emulsions, subject to the inevitable ambiguities
at intermediate values. With a better color measurement we may be able to
correct the Damons photometry of TYC 9504-35-1, or at least throw away the
non-blue measurements.

The problem is that historically, there was simply no way for external DASCH
users to gain access to this information.

For each plate, the DASCH photometric calibration pipeline generates a few tens
of megabytes of metadata, including colorterms and a bunch of other information.
But as far as I can tell, all of that information was either thrown away, or
squirreled away on the Harvard HPC cluster’s storage in a place where only DASCH
insiders could access it. None of the historical DASCH analyses seem to have
made systemic use of the colorterm information.

I felt that this was totally unacceptable — if you’re going to do any kind of
careful work with the DASCH photometry, you’re going to want to understand the
calibration products. So before I embarked upon the Great Reprocessing, I set up
new code to gather all of the pipeline products that looked at all useful,
compile them into new data files, and archive those files.

Once the Great Reprocessing was complete, I scanned all of those files to
extract the colorterms for every plate, allowing me to construct the histogram
shown above. This was actually a bit more complex than you might think: the
pipeline sometimes computes *dozens* of colorterms for each plate, so you have
to check if it’s valid to boil those down into a single representative number.
The short answer is that yes, that appears to be fine.

At this point, we just have to capture the results and plumb them through the
systems. With the new cloud-oriented code, this is all very easy. The new system
has a NoSQL [DynamoDB] database table that stores metadata about each plate, and
all it takes is a quick Python script to bulk-insert the new colorterm values
into that. Exposing the results at the API level is a matter of [mapping the
database fields to new output columns][dsl]. Turning the API results into
user-friendly form is [another bit of table plumbing on the client side][dl].
Tadaa!

[DynamoDB]: https://aws.amazon.com/dynamodb/
[dsl]: https://github.com/pkgw/dasch-science-lambda/commit/ac212ff004a35a688a6e803a85a967ffc6a49c32
[dl]: https://github.com/pkgw/daschlab/commit/181e8e01c57766ae7954d104d9e38c9f4d2cecad

To be honest, the ease of doing all this was a lot more about having
well-structured code throughout the stack, rather than any particular magic of
the cloud. It’s a bit easier to add new “columns” to the DynamoDB than the
legacy DASCH MySQL database, but even without any real [migration]
infrastructure, the difference is marginal at best.

[migration]: https://en.wikipedia.org/wiki/Schema_migration

I didn’t mention one supremely important piece of this project: documentation.
Along with the new data columns, there is a new [colorterm documentation
page][ctdocs] on the DASCH DR7 site. It includes analyses that I hope will be
useful for future DASCH users, such as a linear equation to approximately
transform between APASS (B/V) and ATLAS (g/r) colorterm values, and suggested
cutoffs if you want to categorize emulsions from colorterm values. That being
said, the new colorterm data haven’t been plumbed through *all* aspects of
DASCH: the documentation and software need much more work to really take full
advantage of the new data. For instance, [daschlab] should probably provide some
new [selectors](@/2024/fun-python-filtering-pattern.md) that use the
information; a tutorial should make use of the colorterm data; and the
[Lightcurve Reduction Guide][lrg] absolutely needs to be updated to make use of
the new data.

[ctdocs]: https://dasch.cfa.harvard.edu/dr7/colorterms/
[daschlab]: https://github.com/pkgw/daschlab/
[lrg]: https://dasch.cfa.harvard.edu/dr7/reduce-lightcurve/

There’s a whole other set of goodies that are also available at the low level,
but not yet documented! I’ve just finished uploading all of those photometric
calibration files to AWS, and made them available via [a new API
endpoint][asdfapi]. (Making them useful required [yet more][apl1] [table
plumbing][apl2].) Because the calibration data are quite heterogeneous, I ended
up deciding to serialize them using the [ASDF] container format. This is my
first time using ASDF, and I feel pretty good about it. The data would have been
extremely annoying to try to flatten into FITS, and I’m not at all happy with
HDF5. ASDF solves many of the same problems as HDF5, but the on-disk format is
actually comprehensible to humans — you can actually imagine writing your own
ASDF parser, whereas with HDF5 you’re virtually locked in to their disgusting
C++ library. (Same vibe as [casatables](@/2024/rubbl-casatables-0-8.md).) I’m
not eager to adopt new file formats, but I feel like ASDF is filling a
legitimate niche.

[asdfapi]: https://docs.api.starglass.cfa.harvard.edu/#/default/daschDr7AssetPhotcalAsdf
[apl1]: https://github.com/pkgw/dasch-science-lambda/commit/84634e02392e6029c10cf6aad88e881e3078d1f1
[apl2]: https://github.com/pkgw/daschlab/commit/8a026004629acc4637f331a00440a86742fe4016
[ASDF]: https://asdf.readthedocs.io/

But wait, there’s more! The exposures table also now includes limiting-magnitude
information; catalog query results *finally* include the number of detections in
the DASCH imaging; and *finally finally* there’s a new API to pull out all of
the sources detected in (a portion of) a plate, even ones that aren’t matched to
refcat objects. The goal of the next week or so is to write up documentation of
all of these things, and to leverage them in the software where appropriate
(e.g., [Exposures.candidate_nice_cutouts()][ecnc] can be vastly simplified).
Stay tuned!

[ecnc]: https://daschlab.readthedocs.io/en/latest/api/daschlab.exposures.Exposures.html#daschlab.exposures.Exposures.candidate_nice_cutouts
