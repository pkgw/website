+++
date = 2024-10-03T11:24:23-04:00 # deploytool
title = "On-The-Fly DASCH Cutouts in the Cloud"
+++

Over the past few days I've been working on a bit of an experimental project for
DASCH data access, and I’ve reached the point where I’m pretty confident that
it’s going to work out well! Barring unforeseen issues, the code that generates
DASCH image cutouts is going to move to be entirely cloud-based, running as an
[AWS Lambda][lam]. Getting this to work is, if I may say so myself, a pretty
slick technical accomplishment.

[lam]: https://aws.amazon.com/lambda/

<!-- more -->

The basic challenge is this: DASCH full-plate “mosaic” images average about 750
megabytes, and the biggest ones are more than 2 gigabytes. That’s too big to
conveniently download, so it’s really important to provide some kind of
“cutout” or “thumbnailing” or “postage stamp” service allowing people to
download just the parts of the images that they’re interested in. This is an
incredibly common need in astronomy, and web services to emit postage stamps
easily date back to the 90’s. Some of these, like the ones for [NVSS][nvss] or
[DSS][dss], are still running to this day.

[nvss]: https://www.cv.nrao.edu/nvss/postage.shtml
[dss]: https://archive.stsci.edu/cgi-bin/dss_form

Sometimes, it’s not too hard to extract a cutout. If your images are stored in
uncompressed [FITS] format, the data are laid out in a predictable, linear way
on disk. To extract a sub-rectangle of an overall image, you just have to copy
out bytes from specific pieces of the file, and it’s pretty straightforward to
calculate the locations of those pieces. If such a sub-rectangle works well as a
cutout that you can send to a user, you’re in good shape.

[FITS]: https://en.wikipedia.org/wiki/FITS

The DASCH case is more challenging. The DASCH plates are wildly heterogeneous,
and to get good astrometric solutions for them, we need to solve for substantial
distortions relative to standard projections like [gnomonic]. To enable people
to compare cutouts of the same sky location from various plates — which is
hugely important, and incredibly relevant because DASCH covers most parts of the
sky thousands of times — we can’t just copy same-sized rectangles from different
plate mosaic files. We need to resample the underlying imagery onto a common
projection, which is a lot more computationally intensive than the case where
you can just copy out a rectangle.

[gnomonic]: https://en.wikipedia.org/wiki/Gnomonic_projection

What’s more, the DASCH images stored in the cloud are compressed losslessly
using the [fpack] tool. Using [fpack] shrinks our images by around 30% in size,
and when your total (uncompressed) image corpus comes in at more than 400
terabytes of data, that 30% gain is nothing to sneeze at. The downside is that
fpack'ed images are *not* stored on disk in the convenient, easily-subsettable
form that uncompressed FITS images are. As an example of this: while [Astropy]
can do efficient I/O on uncompressed FITS files, only reading a small amount of
data from disk if your code only needs a small piece of a large image, it can’t
do this with compressed files. If you try to access a small piece of a large
fpack'ed file, it has to load the entire image and decompress it into memory.
Fortunately, this isn’t an inherent limitation of the fpack format (see below).

[fpack]: https://heasarc.gsfc.nasa.gov/fitsio/fpack/
[Astropy]: https://www.astropy.org/

Extracting DASCH cutouts therefore requires a nontrivial software stack: we need
sophisticated [WCS] (world coordinate system) code that can handle inverting
projections with distortions; we need FITS I/O code that can handle compressed
images; and we need software like Tom Robitaille’s [reproject] that can resample
an image from one coordinate system to another.

[WCS]: https://fits.gsfc.nasa.gov/fits_wcs.html
[reproject]: https://reproject.readthedocs.io/

Fortunately, code like this is all available, much of it bundled into [Astropy].
The core of the current DASCH cutout-generating webservice, running in a virtual
machine on the Harvard [Cannon] cluster, is a Python script that’s about 200
lines long, including a lot of boilerplate. A key fact that enables this is that
the copy of the DASCH imagery stored on the cluster uses *uncompressed* FITS
files, so that Astropy can efficiently load subsets of each mosaic as described
above, yielding major performance benefits. As a matter of fact, when I first
took over responsiblity for the DASCH data, I started a project to compress the
on-disk mosaics, but was soon forced to abandon it because the implications for
the performance of the cutout service were so bad as to make it unusable.

[Cannon]: https://www.rc.fas.harvard.edu/services/cluster-computing/#Cannon

The “problem,” such as it is, is that the tricky code in this software stack
resides in foundational C libraries, namely Bill Pence’s [cfitsio] and Mark
Calabretta’s [wcslib]. These libraries are battle-tested, which is important.
But they are not exactly “cloud-native”.

[cfitsio]: https://heasarc.gsfc.nasa.gov/fitsio/
[wcslib]: https://www.atnf.csiro.au/people/mcalabre/WCS/

These libraries have very few external dependencies, so it’s not hard to get
them running in something like a cloud container. The issue is I/O. On AWS, my
mosaics aren’t sitting on some local storage device that’s legible to the
existing code as a [POSIX] filesystem. They’re [S3 objects][s3o], best accessed
using [RESTful HTTP APIs][rest]. I might be able to run the cfitsio code in a
[Lambda][lam], but it’s not going to be able to read my data.

[POSIX]: https://en.wikipedia.org/wiki/POSIX
[s3o]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/UsingObjects.html
[rest]: https://en.wikipedia.org/wiki/REST

(OK, OK, there are things like [s3fs-fuse] and AWS’s [S3 mountpoint][s3mp] that
fill this gap exactly: they make S3 look like a POSIX filesystem. I haven’t
looked into whether these could be used to solve my problem. I suspect that they
might not be available in a Lambda environment, but as much as AWS wants us to
think otherwise, you can actually build web APIs *without* using Lambda.)

[s3fs-fuse]: https://github.com/s3fs-fuse/s3fs-fuse
[s3mp]: https://docs.aws.amazon.com/AmazonS3/latest/userguide/mountpoint.html

So, to make DASCH cutouts available on the cloud, I basically need to solve one
of two problems. Either I need to reimplement the battle-tested logic of
libraries like cfitsio and wcslib in a way that plays nicely with a modern cloud
environment; or I need to find a way to bridge the gap between what these
codebases expect and what that environment delivers.

Thankfully, some preliminary research showed that the second option might not be
too major of an undertaking. It’s basically undocumented, but cfitsio turns out
to actually have a whole framework to support generic I/O backends, [and it’s
user-extensible][frd]. There’s in fact already support for [accessing files over
HTTP/HTTPS][fdn], but it quickly became clear that it wouldn’t suffice: besides
being unable to parse some sample S3 presigned URLs, the code also works by
downloading the FITS file in its entirety, a non-starter in the DASCH context.

[frd]: https://github.com/HEASARC/cfitsio/blob/develop/fitsio2.h#L1166-L1182
[fdn]: https://github.com/HEASARC/cfitsio/blob/develop/drvrnet.c

This connects to the other key requirement needed for cloud-based thumbnailing
to be possible. If I have to read in the entire FITS file to create a cutout
from a *compressed* DASCH mosaic, I’m probably sunk. After a little bit of
digging, I found out that the `fpack` compression technique thankfully uses a
tiling scheme, where the image is broken into indepedent chunks that are
compressed separately. To access a pixel within a given tile, you have to read
and decompress the full tile, but you *don’t* have to read in the entire image.
I further found out that the default tiling scheme is row-by-row: each “tile” is
really a needle, compressing one scanline of the image. That’s not optimal for
the roughly-square cutouts that I’m seeking to generate, but it’s a good start.
(And, yes, DASCH uses the default scheme.)

So we have the outlines of a solution: if we can add a backend to cfitsio able
to talk to S3 directly, and we can write a custom cutout-and-reprojection
routine that can get away with requesting only the image rows that are needed to
generate the cutout, we’re in business.

The next problem to think about is whether that cutout-and-reproject step is
going to be too hard to implement. In the current approach, I use the
[reproject] package in its [interpolation] mode. I had never really sat down to
think about how this algorithm must work, but after reading the code and
thinking about things, I decided that this was something that I could probably
reimplement from scratch. The approach really isn’t too complicated in our
simple 2D use case:

[interpolation]: https://reproject.readthedocs.io/en/stable/celestial.html#interpolation

1. We start with the desired output. We want to generate a square image with a
   known, simple coordinate system. With wcslib (or even without — the
   projection is simple), it’s easy to compute the associated sky coordinates:
   what is the RA/Dec of each pixel in the desired output image?
1. We can than work backwards towards the input image. Given those sky
   coordinates, and the known WCS solution of the full-plate mosaic, we ask: for
   each pixel in the desired output image, what would its pixel position on the
   input image be? These pixel positions will generally be fractional. This step
   is where we need the full power of wcslib, in order to correctly invert the
   complex distortions that are generally present in the input image’s
   astrometric solution. Importantly, once we’ve done this step, we know exactly
   what subset of the input image we need to do our computation — we won’t need
   to load the entire thing.
1. Now, computing the value of each output pixel is a simple interpolation
   problem. Given a fractional pixel coordinate (something like `(3.6, 8.3)`),
   and the actual values of the input pixels around that area (at coordinates
   `(3, 8)`, `(3, 9)`, etc.), what’s a good interpolated pixel value for that
   coordinate? This is the kind of problem that people invented computers in
   order to be able to solve. It’s not something you want to implement yourself,
   but nearly any computational environment you can find will have some kind of
   library available that can do this for you.

This only works because we’re OK with interpolation-based resampling. This
approach can produce poor results some corners of parameter space and isn’t
flux-conserving, and so isn’t appropriate for certain high-precision contexts.
But for cutouts of DASCH images, where the underlying pixel values aren’t
calibrated to physical units, it’s OK. An important feature of the [reproject]
library is that offers [several other resampling methods][rm] that are slower
but more precise.

[rm]: https://reproject.readthedocs.io/en/stable/celestial.html

Finally — what would the tangible implementation of this service actually look
like? Last week, I did some exploratory work on implementing other, much simpler
DASCH APIs on AWS Lambda, so I had a sense of what approach I would take.

In particular, it’s possible to [implement a Lambda as a Docker
container][lamdock], and there’s [an official AWS Rust toolkit for implementing
Lambdas][lamrust]. The Rust toolkit is billed as “experimental”, but it’s
full-featured and I believe that there’s every reason to think that it will
continue to be supported for a long time to come. In the not-so-formal DASCH
development effort, relying on something experimental is OK.

[lamdock]: https://docs.aws.amazon.com/lambda/latest/dg/images-create.html
[lamrust]: https://docs.aws.amazon.com/lambda/latest/dg/lambda-rust.html

The existing Lambda implementations that power [Starglass] are written in [Go],
but in this case I felt that it was a pretty clear choice to use Rust for this
service. The key issue is that implementing this cutout service was going to
require some kind of [FFI] to the cfistio and wcslib C libraries, and in my
experience that’s the kind of thing that often takes a really long time to get
working in an unfamiliar language. I know very little about Go, but I’ve done a
ton of work with Rust FFI, and I’m absolutely sure that it would be a lot
quicker to figure out how to implement a Lambda in Rust than it would be to
figure out how to string together some fairly complex FFI interactions (as well
as every other piece of the program!) in Go. After all, one of the nice benefits
of using things like Lambdas is that you can make these sorts of technical
choices on a case-by-case basis.

[Go]: https://go.dev/
[Starglass]: https://starglass.cfa.harvard.edu/
[FFI]: https://en.wikipedia.org/wiki/Foreign_function_interface

The other choice, of course, would be to implement the whole thing in C or C++.
(Python is a no-go due to the need for custom S3-backed I/O on compressed FITS
files; solving that problem requires enough systems-level code that — in this
case — you might as well do the whole thing in one language. If we were aiming
to provide a flexible library to be used by lots of people in lots of ways,
instead of a web service with one narrow job, it would be a different story.)
There is indeed an [AWS SDK for C++][cppsdk] that could be used to interact with
the storage backend APIs. But this tool needs to string together a wide range of
technologies: Lambda runtime code (basically, an HTTP server), S3 I/O, queries
to [DynamoDB] for reasons not yet mentioned, FITS and WCS manipulations, and
fast 2D interpolation. You can do this in C/C++, but to be blunt, it completely
sucks. One of the reasons that I love [Rust] is that it makes it *so* much
easier to build these kinds of complex codebases while delivering performance
and reliability that are light-years ahead of scripting languages like Python.
It’s not *as* easy as Python (cf. the 200-line script mentioned above), but in
cases like this the performance and reliability tradeoffs are absolutely worth
the extra effort.

[cppsdk]: https://aws.amazon.com/sdk-for-cpp/
[DynamoDB]: https://aws.amazon.com/dynamodb/
[Rust]: https://rust-lang.org/

Finally, it was time to start writing code! I started with working out a simple
S3 backend for cfitsio. Since the “deliverable” for this service is basically a
simple executable that runs in a Docker container, there’s no need to worry
about proper APIs: I can just link cfitsio into the final executable statically,
and do whatever I need to do to get that backend working. If I needed to, I
could go ahead and hack up the cfitsio source itself. But it turned out that I
could use the stock library. Read-only S3 support really only needs to implement
three driver APIs: one to get the total size of the “file”, one to seek, and one
to read. [HTTP Range requests][hrr] make it easy to read only a subset of the
file, so this was actually quite easy to write — just a day of work, more or
less. The most tricky part was that I discovered/realized that even though the
Rust Lambda server is running an asynchronous event loop, and the [Rust S3
client][s3rs] is asynchronous, they can’t be asynchronous together: cfitsio
expects I/O to happen in a blocking style, and, as far as I can reason out,
there’s no way for the little asynchronous state machine *inside* that I/O loop
to hand control back up to the [toplevel asynchronous runtime][tokio]. So, I
believe that I have to run the cfitsio S3 operations inside a “blocking thread”,
which then starts up its own secondary async runtime to manage its requests.
Pretty gross, but life goes on. There might well be a better approach to this.

[hrr]: https://developer.mozilla.org/en-US/docs/Web/HTTP/Range_requests
[s3rs]: https://crates.io/crates/aws-sdk-s3
[tokio]: https://tokio.rs/

Next I wired up all of the ancillary interactions needed to make things work:
getting a [wcslib WCS state object][wcsprm] from the FITS headers; synthesizing
simple WCS for the projection target; creating an in-memory FITS “file” that can
then be extracted at the Rust layer. This last task was fairly annoying due to
some particularities of the design of the relevant cfitsio APIs. It did give me
a fun excuse to use Rust’s concept of [“pinning”][pin] to preserve some
necessary invariants; I think it’s genuinely wonderful that Rust gives you the
tools to both think about these kinds of concepts and actually implement them in
code. It was also a reminder of how much we, collectively, have learned about
API design since the days when libraries like cfitsio were being written. This
has nothing inherently to do with fancy languages — an experienced developer
writing a modern library would do a bunch of things differently even if they
were required to work in straight C. This is no knock on cfitsio; it’s just to
underscore that despite all of the ridiculous churn in the tech industry, there
really are some fundamental aspects of the craft that have improved over the
decades. Anyway, this was all another day of work or so.

[wcsprm]: https://www.atnf.csiro.au/people/mcalabre/WCS/wcslib/structwcsprm.html
[pin]: https://doc.rust-lang.org/std/pin/

Finally, yesterday I sat down to tie these things together with a resampling
step in the middle. For interpolation, I’m using a library called
[ndarray-interp] which has some rough edges, and doesn’t appear to be very
actively maintained, but it gets the job done. On the left, a sample cutout
produced by the current production code; on the right, the sample produced by
the S3-backed Rust version:

[ndarray-interp]: https://crates.io/crates/ndarray-interp

{% relfig(path="comparison.jpg") %}
Comparison of cutouts: left is current, right is the new S3-backed Rust version.
{% end %}

I’ll admit that I haven’t dared to do a pixel-level comparison, but I don’t see
any differences. Implementing this was another day of work.

At this point, I’m confident that I’ve retired all of the risks and can declare
this experiment a success. There are still a bunch of things that I need to do,
but they’re “just programming” and shouldn’t present any major problems:

- Actually integrate this code into a Lambda server. I have other Rust
  prototypes that I’ve built and deployed to Lambda, so I’m not worried about
  this.
- Improve core I/O performance. Cfitsio pulls in data with a lot of small
  reads. Right now, each of these is a separate HTTPS request to S3, so it’s all
  pretty slow. A simple buffering layer ought to make this way better; actually
  running the code in an AWS datacenter, as opposed to on my laptop, will help
  too. This kind of buffering is the kind of thing that’s pretty tedious to
  implement in C (memory management, bounds-checking, ...) but super-easy to add
  in Rust.
- Change how I fetch the WCS for the mosaics. The way that the DASCH pipeline
  works, the best WCS for each plate image is stored in a FITS header file
  that’s actually separate from the mosaic FITS file containing the image data.
  I need to add some code that will fetch WCS from those separate headers, which
  will be stored elsewhere on S3 (or perhaps in DynamoDB). This doesn’t present
  any implementation issues since the WCS information is small enough that I can
  just fetch it all in a hunk and store it in memory.

This last item is a particularly important one. Many Harvard plates were exposed
multiple times, giving them [*multiple* WCS
solutions](@/2024/dasch-astrometry.md). If I want to look up a particular source
on a plate, I need to know which solution to use: one might tell me that the
source is in dead center, while another might tell me that it's a million pixels
off of the image! But the mosaics FITS files contain only the *first* WCS
solution, and the current (and previous) cutout code could only use that one
solution. It simply hasn’t been possible to generate cutouts for the additional
solutions. Once I wire in this functionality, we’ll finally start handling
multiple-exposure plates properly.

You’ll note that I haven’t linked to any source code. The code isn’t online yet,
and to integrate into the DASCH/Starglass CI/CD systems, the primary repository
will need to belong to the private DASCH/Starglass project that runs on [Harvard
Research Computing’s GitLab organization][rcgl]. But, I think that people might
be interested in seeing the end-to-end implementation of all this, so I’ll
mirror it onto my public GitHub too. Coming soon!

[rcgl]: https://gitlab.com/HarvardRC/

Overall, I’d like to think that this is a good demonstration of the leverage
that can happen in the world of software development. You might think that
“teach cfitsio to work with S3” is a big project — and, to be honest, someone
without the right experience might spend a long time flailing around without
making any progress. But, and I guess there’s no way not to be tooting my own
horn here, for someone *with* the right experience, it can be literally a day of
work. (I’m benefiting from having a very narrow use-case, but I’ll claim that a
more general backend would be hardly any more effort.) When this kind of stuff
works, you can understand why software developers are so expensive — I’ve read
about single-person projects at cloud-reliant companies that directly yield
tens-of-millions-per-year savings. Developers can be worth their literal weight
in literal gold.

But where there is high demand and high price, there must be low supply. Time
management is always hard — personally, I feel like there are routinely problems
on my plate that linger for months and then only take a few hours of work to
solve. It’s depressing. More broadly, I see so many examples of projects in
astronomy that are just *stuck* that just need to get the right person on task
for a few weeks, if not just a few hours. For instance, another case that I’m
personally familiar with: some colleagues have been struggling with issues in
the [PinpointWCS][ppwcs] program for literally a *decade*, and it took me about
a week to demonstrate some major improvements. If science were run by a wise,
benevolent dictator, I’m pretty sure that they would have people like me working
on these sorts of problems day-in-day-out. Sadly, the actual career tracks that
we have don’t exactly support that.

[ppwcs]: https://github.com/kapadia/PinpointWCS
