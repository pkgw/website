+++
date = 2024-08-14T11:50:46-04:00
title = "rubbl_casatables 0.8"
+++

Yesterday I put out the [first release in the 0.8.x series][rc08] of the
[`rubbl_casatables`] [Rust] crate, which provides access to the “casatable” data
container file format used by the [CASA] radio interferometry package (sometimes
called the [CASA Table Data System][ctds]). CASA uses the casatable format for
virtually all of its data files, most notably the [MeasurementSets][ms] that
store interferometric visibilities. This release is primarily the work of
[@d3v-null] at [Curtin], who undertook the difficult and tedious project of
updating the backing codebase to use the [casacore 3.5.0] implementation of the
casatables format.

[rc08]: https://github.com/pkgw/rubbl/releases/tag/rubbl_casatables%400.8.0
[`rubbl_casatables`]: https://github.com/pkgw/rubbl
[Rust]: https://rust-lang.org/
[CASA]: https://casa.nrao.edu/
[ctds]: https://casacore.github.io/casacore-notes/255.html
[ms]: https://casadocs.readthedocs.io/en/stable/notebooks/casa-fundamentals.html#MeasurementSet-Basics
[@d3v-null]: https://github.com/d3v-null/
[Curtin]: https://astronomy.curtin.edu.au/
[casacore 3.5.0]: https://github.com/casacore/casacore/releases/tag/v3.5.0

<!-- more -->

To understand the niche occupied by [`rubbl_casatables`], it’s helpful to be
careful about how we discuss CASA’s data files. In particular, the distinction
between a MeasurementSet and a casatable is important.

When I discuss the casatable format, I’m referring to what I call a [container
format][cf] or sometimes “serialization format”. It defines *how* to store
various kinds of data into files on disk, but is silent on the *why*: what the
data actually mean. File archive formats like [Zip] are perhaps the most obvious
examples of container formats: the Zip specification tells you how to pack
various files into a Zip archive, but it doesn’t (and can’t, and shouldn’t) make
any claims about the meaning of those files. Other file formats, like Java’s
[JAR] files, actually use the Zip container format for their underlying storage,
then apply additional layers of semantics. For instance, a JAR file should
contain a Zip entry called `META-INF/MANIFEST.MF` whose contents and meaning are
defined by the JAR specification.

[cf]: https://en.wikipedia.org/wiki/Container_format
[Zip]: https://en.wikipedia.org/wiki/ZIP_(file_format)
[JAR]: https://en.wikipedia.org/wiki/JAR_(file_format)

[The MeasurementSet specification][msspec], to be contrasted with casatables, is
closer to the JAR format in that works at a more semantic level, defining a way
to represent interferometry data in particular. This representation can then be
captured in a casatable, but it can also potentially be mapped to other
serialization formats like [Arrow], [Zarr], or [Parquet] (see, e.g., the [arcae]
Python package).

[msspec]: https://casadocs.readthedocs.io/en/stable/notebooks/casa-fundamentals.html#MeasurementSet-v2
[Arrow]: https://arrow.apache.org/
[Zarr]: https://zarr.dev/
[Parquet]: https://parquet.apache.org/
[arcae]: https://github.com/ratt-ru/arcae/

In astronomy, we often talk about [FITS] as an image format, and indeed the “I”
in “FITS” does stand for “image”, but modern FITS is really a container format
as well. After all, FITS files can contain multiple [Header Data Units][hdu],
each of which can contain an N-dimensional array, or a binary table, or
[interferometry data][idi], or [all sorts of other custom data structures][ext].
I would claim that the longevity of FITS stems in large part from its evolution
from a pure image format to a more flexible container format with a very simple
specification, which makes it easy to implement FITS readers and writers in a
variety of languages. For instance, I also wrote a small [`rubbl_fits`] crate
that exposes FITS I/O to Rust. It’s beta-quality at best, but I’m still pretty
confident that it can correctly understand the structure of any valid FITS file
that you throw at it.

[FITS]: https://en.wikipedia.org/wiki/FITS
[hdu]: https://docs.astropy.org/en/stable/io/fits/api/hdus.html
[idi]: https://fits.gsfc.nasa.gov/registry/fitsidi.html
[ext]: https://fits.gsfc.nasa.gov/fits_registry.html
[`rubbl_fits`]: https://docs.rs/rubbl_fits/

The casatables format is another member of the astronomical data container
family. In my view it’s most similar to [HDF5]: they’re both relatively “modern”
formats for complex scientific data, designed as extensible container formats
from the start.

[HDF5]: https://www.hdfgroup.org/

The other thing that these two formats have in common — which honestly
infuriates me in both cases — is that their byte-level serializations are
extremely complex and at best weakly documented; effectively the *only* way to
use these container formats is through extremely gnarly C++ libraries provided
by the format developers. HDF5 does at least have a [written specification of
the on-disk format][hdf5spec], while to the best of my knowledge there isn’t one
at all for casatables. But in either case, if you wanted to implement a parser
for the format from scratch in another programming language, I strongly suspect
that you’d basically have to reverse-engineer the C++ codebase.

[hdf5spec]: https://docs.hdfgroup.org/hdf5/v1_14/_f_m_t3.html

If you ask me, the cardinal virtue of a container format is to have a
minimally-complex, clear specification that you can imagine implementing from
scratch if needed. Once again, I think this is why FITS has lasted for so long.
It’s not that I’m worried about literally losing the ability to compile the
relevant implementation libraries, although C++ code in particular seems to need
regular maintainance just to stay buildable: the language has an unfortunate
combination of high complexity and constant evolution as people try to patch up
its many flaws. It’s a more general sense of unease with the design of such
formats. One thing I’ll point to is that both casatables and HDF5 have acquired
pluggable I/O backends, which essentially formalize the requirement that the
only way to reliably decode datasets is through the official C++ libraries.
Casatables has extensible “storage managers” like [Dysco], while HDF5 has things
like [virtual file layers][vfl] and [virtual object layer connectors][vol].

[Dysco]: https://github.com/aroffringa/dysco
[vfl]: https://docs.hdfgroup.org/hdf5/v1_12/_v_f_l.html
[vol]: https://docs.hdfgroup.org/hdf5/v1_14/_h5_v_l__u_g.html

Compounding the complexity in CASA is that the casatables container format
implementation is embedded within the rest of the CASA C++ library ecosystem,
which makes things even more baroque. Even the “streamlined” [casacore] codebase
consists of, by my quick estimate, around 2,300 source files, with dependencies
on a number of external libraries like [wcslib], [fftw3], [HDF5], and [ncurses]
(!). If you just want to understand what’s in a casatable file tree, you need to
build this whole suite of libraries, although to be fair you only need to link
with a subset of them. But still, this is ridiculously onerous for what should
be a low-level operation: *understand the contents of this container*.

[casacore]: https://github.com/casacore/casacore/
[wcslib]: https://www.atnf.csiro.au/people/mcalabre/WCS/index.html
[fftw3]: https://www.fftw.org/
[ncurses]: https://invisible-island.net/ncurses/

If you use CASA, your analysis sessions are creating casatables data all over
the place: MeasurementSets, calibration files, images, source lists, and
probably other kinds of data as well. With the standard CASA software, if you
want to interact with these files, you need to use these unwieldy C++ libraries
— either directly, or using wrappers that rely on them, like the [casacore
Python bindings][pc]. I can say from experience that doing so is pretty
unpleasant, which in turn limits the development of a software ecosystem around
these kinds of data. I’m absolutely certain this is why we see the various
efforts to express MeasurementSet data to other serialization formats like
[Arrow] that I mentioned above.

[pc]: https://github.com/casacore/python-casacore

This is, finally, where [`rubbl_casatables`] comes in. This [Rust crate]
providers support for using the casatables container format in a self-contained
library that provides a hopefully-clean API.

[Rust crate]: https://crates.io/

It accomplishes this through brute force: it bundles the subset of the
[casacore] C++ code needed to work with casatables data, and nothing more. There
are a small number of modifications to make the codebase more “standalone”, but
in the end only a few are needed. Compared to stock casacore, there are “only”
783 C++ source files to compile, and no external dependencies.

The design of the Rust packaging ecosystem plays a major role here. While Rust
crates are basically reusable libraries in the C/C++ tradition, you don’t
compile them into shared libraries and install them into `/usr/lib64`. Rust,
like [Go], has a static-first model. These languages strongly encourage you to
only produce binary executables, not shared libraries. If your executable
depends on another package, you compile it directly into the executable, rather
than installing it as a separate shared library that your executable then
depends on. This makes for some relatively large binaries (you're including all
of the code that would be separated out into a library) and compile times
(you're compiling all of that code from scratch, as well), but in my experience
it’s absolutely the right paradigm. We’ve come a long way from [DLL Hell], but
every shared library dependency still adds a number of failure modes to a
software deployment.

[Go]: https://go.dev/
[DLL Hell]: https://en.wikipedia.org/wiki/DLL_Hell

My main use of [`rubbl_casatables`] is in a companion project called
[`rubbl-rxpackage`], which implements a few low-level data-processing tasks that
are not available in mainline CASA. The most interesting one is [the key data
transformation][peelrs] needed to support [the peeling workflow that I’ve
developed](@/2024/peeling-tool/index.md). Another good example is a utility
called [`spwglue`] the merges together adjacent spectral windows in a
MeasurementSet. My first implementation of `spwglue` was in Python using the
casacore Python bindings; porting to Rust sped it up by a factor of **twenty**.

[`rubbl-rxpackage`]: https://github.com/pkgw/rubbl-rxpackage
[peelrs]: https://github.com/pkgw/rubbl-rxpackage/blob/master/src/peel.rs
[`spwglue`]: https://github.com/pkgw/rubbl-rxpackage/blob/master/src/spwglue.rs

More broadly, these kinds of programs *could* be implemented in C++, but are
much, much less pleasant to do so. Besides the heaviness of the casacore library
dependency, I cannot emphasize enough how much I dislike working in C++ (see
[timely toot][toot]). C++ was a step forward in its time, but “its time” was
thirty years ago. We’ve learned *so much* about designing programming languages
since then, both in terms of ergonomics and safety. Obviously there are immense
amounts of legacy code and expertise that we can’t and shouldn’t just throw
away, but there are huge benefits when you can move to better tools. Rust makes
me actually enjoy writing systems-level code, while (and in no small part
because) it simultaneously gives me much more confidence in that code’s
correctness.

[toot]: https://mastodon.world/@vitaut@mastodon.social/112957324812448747

As I mentioned at the top, the [new release of `rubbl_casatables`][rc08]
primarily upgrades the backing C++ code from version 3.1.1 of casacore to
version 3.5.0. To the best of my knowledge, this shouldn’t affect much in the
way of major functionality, but it does include a bunch of that necessary C++
maintenance that I mentioned above. [@d3v-null] undertook this project to get
the codebase building on additional CPU targets, and put in a lot of effort to
untangle the casacore changes necessary to perform the update — see issue
[#345][issue345] for all of the gory details. It took me a very long time to
follow up on all their work, which is something I hope to do better about going
forward.

[issue345]: https://github.com/pkgw/rubbl/issues/345

The only really tricky thing I did was [address a longstanding weakness][pr393]
dealing with some uses of uninitialized memory in the Rust side of the
casatables implementation. We had some code that would allocate uninitialized
array buffers and then pass them off to the casacore C++ code to fill with data.
This is the kind of thing that, coming from a C/C++ background, is second
nature, but it turns out that you actually have to do this with a lot more care
than you might think. [Rust RFC 2930][rfc2930] has a good discussion of the
issues at play; see also [this blog by Ralf Jung][ralf]. This is the kind of
thing that makes me really thankful that people involved in Rust are super
fastidious about getting right, while there’s so much legacy C++ out in the
world that you can, at best, only hope that a nontrivial project does the right
thing consistently.

[pr393]: https://github.com/pkgw/rubbl/pull/393
[rfc2930]: https://github.com/rust-lang/rfcs/blob/master/text/2930-read-buf.md#background
[ralf]: https://www.ralfj.de/blog/2019/07/14/uninit.html

You might wonder about this “Rubbl” name. Rubbl is an umbrella collection of my
foundational Rust projects relating to astrophysics (“Rust + Hubble”), mostly
data formats: casatables, FITS, and [MIRIAD]. I hope that it at least has the
possibility of one day growing into something truly foundational like [Astropy],
but right now the casatables implementation is basically the only thing that
gets regular usage. My plan is that if I find myself needing to write data
analysis code that would be in C or C++, I’ll try to do it in Rust and extend
Rubbl as needed. So far, that need has only come up rarely, although the
[`rubbl-rxpackage`] tools are great examples of times that it has. It’s been
very pleasing that the existing code has at least been useful enough for people
like [@d3v-null] to get involved.

[MIRIAD]: https://github.com/astroumd/miriad
[Astropy]: https://www.astropy.org/

The DOI of this new release is [10.5281/zenodo.13315460][doi] (automatically registered in Zenodo with [Cranko]). The API docs for `rubbl_casatables`
may be found [on docs.rs][api].

[doi]: https://doi.org/10.5281/zenodo.13315460
[Cranko]: https://pkgw.github.io/cranko/
[api]: https://docs.rs/rubbl_casatables/latest/rubbl_casatables/
