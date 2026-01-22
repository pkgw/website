+++
date = 2010-08-02T11:22:08Z
title = "Qual: Previous Work"
path = "2010/08/qual-previous-work"

[extra]
wp_rel_permalink = "/2010/08/qual-previous-work/"
wp_shortlink = "/?p=325"
+++

(Deadline blown, quasi-intentionally; some work on the Cyg X-3 lightcurve
project has become higher-priority as we’ve realized that the summer is coming
to an end …)

What have I been doing for the past four years, anyway? The usual approach to
this portion of the qual seems to ignore classes, teaching, and outreach, so
we’ll skip those.

# Exoplanets Work

In my first few
years, I did some observing for Geoff Marcy’s group, which led to
coauthorships on a few papers:

- Winn+: HAT-P-1b lightcurve:
  [2007](https://scixplorer.org/abs/2007arXiv0707.1908W),
  [2007 erratum](https://scixplorer.org/abs/2008AJ....136.1753W)
- Johnson+: HAT-P-1b Rossiter-McLaughlin:
  [2008](https://scixplorer.org/abs/2008ApJ...686..649J)
- Johnson+: two Jovian exoplanets:
  [2008](https://scixplorer.org/abs/2008ApJ...675..784J)
- Peek+: two Jovian exoplanets:
  [2009](https://scixplorer.org/abs/2009PASP..121..613P)

I gained some great optical observing experience from this involvement but
don’t think it’s worth treating in any depth.

# ATA Broadband Spectra

My first major ATA project culminated in
[Williams & Bower 2010](https://scixplorer.org/abs/2010ApJ...710.1462W), a
study of the broadband spectra of three starforming galaxies in an attempt to
understand the physical underpinnings of the FIR/radio correlation of
starforming galaxy emission. Here’s a bibliography of some relevant
theoretical and observational work:


- [Thompson+ 2006](https://scixplorer.org/abs/2006ApJ...645..186T) pretty
  directly inspired the project
- [Völk 1989](https://scixplorer.org/abs/1989A%26A...218...67V) proposed
  the “calorimeter theory” that we investigate
- [Blandford & Eichler 1987](https://scixplorer.org/abs/1987PhR...154....1B)
  is a classic reference on cosmic ray acceleration in shocks
- [Lisenfeld+ 1996](https://scixplorer.org/abs/1996A%26A...306..677L) is
  one somewhat recent example of theoretical work on the FIR/radio correlation
- [Price & Duric 1992](https://scixplorer.org/abs/1992ApJ...401...81P) is
  another
- [Thompson+ 2009](https://scixplorer.org/abs/2009MNRAS.397.1410T) updates
  Thompson+ 2006 with some ideas based on SNRs
- More recent work:
  [Lacki+ 2010](https://scixplorer.org/abs/2010ApJ...717....1L),
  [Lacki & Thompson 2010](https://scixplorer.org/abs/2010ApJ...717..196L)
  are further work from the same group.
- [Sargent+ 2010](https://scixplorer.org/abs/2010ApJS..186..341S) is one of
  many papers looking at the evolution of the FRC with redshift.

# ATA Infrastructure

In the course of working on the broadband spectra project, I also spent a lot
of time developing various pieces of the ATA infrastructure and data analysis
chain, mostly in the form of software:

- Time spent modernizing the MIRIAD build system, fixing bugs, maintaining its
  port to Mac OS X (and I don’t even use a Mac! That’s how nice of a guy I
  am.)
- [miriad-python](https://www.cfa.harvard.edu/~pwilliam/miriad-python/), a
  Python binding for MIRIAD that I use all the time
- ATA data analysis tools built on miriad-python: calctsys, dualscal, qimage,
  many and varied one-off tools
- My UV data visualizer/editor
- “hex” scripts for weekly ATA calibration observations
- A system for running commensal observations with the beamformer and
  correlators on the ATA
- A few other smaller things of somewhat less interest: the broadband spectra
  observing scripts, quasi-defunct atacursesstatus program, a Python plotting
  library, work on pympfit, Python bindings to NOVAS/SOFA/etc, …
