+++
date = 2025-04-28T00:00:00-04:00
title = "CASA 6 is Now in Conda-forge"
+++

I’m pleased to report that [CASA 6][casa] is now available in [conda-forge]!
CASA is a software suite for processing radio astronomy data from telescopes
such as the [Very Large Array][vla], [ALMA][alma], and more. The availability
includes the `casatools` and `casatasks` Python packages but not the full suite
of CASA end-user applications. Just run:
```sh
$ conda install casatasks
```
or the equivalent command in a Python environment that [has conda-forge
enabled][10].

[casa]: https://casa.nrao.edu/
[conda-forge]: https://conda-forge.org/
[vla]: https://www.vla.nrao.edu/
[alma]: https://www.almaobservatory.org/

<!-- more -->

Somewhat terrifyingly, I’ve been working on packaging CASA [for almost exactly a
decade][1]. The origin of this is that like many complex data reduction packages
of its time, CASA was (and still is) distributed by [NRAO][nrao] as a large,
self-contained software environment: the latest CASA installer for Linux is just
shy of a gigabyte in size, including not just data files but also all of the
support libraries needed to ensure that the binaries can run on as many systems
as possible. The issue with CASA was that it was *also* trying to embrace Python
as a scripting language. In NRAO’s monolithic distribution model, this meant
embedding an entire freestanding Python interpreter in the CASA distribution.

[1]: https://github.com/pkgw/conda-recipes/commit/a4a4b55416a403eb17b3182c3854d68dc98cfc84
[nrao]: https://public.nrao.edu/

This might seem like a minor packaging choice, but my claim is that it hugely
affects how you can work with the software. A key advantage to a scripting
language like Python is that it allows you to bring together a huge range of
codebases in one “place:” you can write a program that glues together the
[netlib] numerical libraries via [Scipy], with GUI toolkits like [GTK] using
[PyGObject], and data I/O packages like [HDF5] through [h5py]. When a package
like CASA bundles its own interpreter, if you want access to these kinds of
libraries you can’t just reuse whatever software collection you’ve painstakingly
set up over the years — you have to install everything from scratch into *its*
environment. In this model, CASA isn’t a tool to add to your toolbox — it’s a
quarantine zone that can only be entered or exited through an airlock.

[netlib]: https://www.netlib.org/
[Scipy]: https://scipy.org/
[GTK]: https://www.gtk.org/
[PyGObject]: https://pygobject.gnome.org/
[HDF5]: https://www.hdfgroup.org/solutions/hdf5/
[h5py]: https://www.h5py.org/

Even worse, back then CASA’s Python installation was missing key development
files, so that many packages with binary components couldn’t be installed into
its environment at all! At least, not unless you were willing and able to devise
some extreme hacks to fill in the needed files.

Of course, NRAO wasn’t distributing CASA as a monolith out of spite: for a long
time, that was the only realistic way to deliver a large application (or suite
of applications) with complex, specialized dependencies. But in 2012 ([around
the time of CASA 3.3][2]) [Anaconda first released the conda package
manager][3]. I’ve mentioned before that in my view [much of conda’s design was
more or less evolutionary][4], but that’s not to downplay its impact — in my
view it has truly transformed the way that we can deliver scientific software to
users. In particular, conda and [conda-forge] have combined to create an
ecosystem where it can be amazingly straightforward to install complex
dependencies into arbitrary Python environments.

[2]: https://www.asu.cas.cz/~barta/ARC-doc/casa-intro-prague-2012.pdf
[3]: http://ilan.schnell-web.net/prog/anaconda-history/
[4]: @/2024/all-in-on-pixi.md

Back in 2015, that was what I wanted for CASA. So I started slogging through its
obscure and out-of-date dependencies and developed conda recipes [for the whole
stack][5], starting with version 4.4. And lo, it was good.

[5]: https://github.com/pkgw/conda-recipes/blob/2cce9ed2fb0f0b650cbbc68cd38b942cee9b2889/ORDERED.md#the-casa-stack

I did discover that I had to write [a whole bunch of code][6] to make a bunch of
CASA’s functionality meaningfully usable from Python. It turned out that despite
having its outermost layers written in Python and claims of “scriptability”,
much of the CASA simply wasn’t usable like a regular Python package.

[6]: https://github.com/pkgw/pwkit/tree/master/pwkit/environments/casa

In 2017 (CASA 4.7, Python 3.6), it was clear that it was time to really shift to
Python 3. But it was also clear that CASA wasn’t going to be supporting Python 3
for a long time yet, and that there wasn't anything that individuals outside of
NRAO could do about that. So I [added Python 3 support to CASA myself][7], which
was … painful. But once again, I found it worthwhile to be able to actually have
CASA be a first-class member of my general software toolkit.

[7]: https://github.com/pkgw/casa/commits/casa3k-5.6/

Around 2019, [CASA 6][9] was coming out, which both added support for Python 3
and started the process of making CASA’s architecture more Python-native. But [I
had started spending a lot less time thinking about astrophysics][8], so I
didn’t spend much time exploring it. I took an initial look at updating my
recipes for CASA 6, saw that things seemed about as challenging as in the CASA
5.x series, and decided not to worry about it for a while.

[8]: @/2018/operation-innovation.md
[9]: https://science.nrao.edu/enews/casa_008/index.shtml#casa6

Now, after a long hiatus, I’ve had some reason to take another look at updating
my CASA conda packages. Finally, *finally*, the upstream source code is in
pretty good shape! I was able to put together conda recipes for [casacpp],
[casatools], and [casatasks] over the course of just a few days. Instead of
thousands of lines of patches, I only needed a handful. And pretty much all of
the old, outdated dependencies needed by CASA 4/5 are now gone. The combination
of all of these factors made it feasible for me to get these recipes integrated
into conda-forge, rather than building the packages myself. This will offer
massive maintainability gains going forward, thanks both to conda-forge’s
impressive infrastructure and a much more realistic possibility for other people
to help keep the packages up-to-date. Huzzah!

[casacpp]: https://github.com/conda-forge/casacpp-feedstock/
[casatools]: https://github.com/conda-forge/casatools-feedstock/
[casatasks]: https://github.com/conda-forge/casatasks-feedstock/

If you want to install CASA using these packages — or, really, create and manage
any kind of customized software environments — [I highly recommend using
Pixi][4]. Run `pixi add casatasks` and you should be good to go. But you
can use these packages to install CASA into any conda-backed environment just by
[activating conda-forge][10] and running the analogous installation step.

[10]: https://conda-forge.org/docs/user/introduction/#how-can-i-install-packages-from-conda-forge

As for *why* I’ve returned to CASA … I would say “more soon,” but that’s
unlikely. My wife is due to give birth to our first child within the week so I
don’t expect to be posting much for a while! I won’t be able to take as much of
a formal leave as I’d like because I’m within my first year of Smithsonian
employment and hence not eligible for [FMLA], but I have a sneaking suspicion
that my hands will be pretty full for the foreseeable future.

[FMLA]: https://www.dol.gov/agencies/whd/fmla
