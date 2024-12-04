+++
date = 2024-11-01T09:18:44-04:00 # deploytool
title = "Preview Access to DASCH DR7 Now Available"
+++

Here’s a slightly edited version of an email I sent to the [DASCH
Astrophysics][da] mailing list at the end of the day yesterday:

[da]: https://gaggle.email/join/dasch@gaggle.email

<!-- more -->

I'm pleased to report that you can now get early, preview access to the DASCH
Data Release 7 (DR7) data via the [daschlab][dl] Python package.

[dl]: https://dasch.cfa.harvard.edu/dr7/install-daschlab/

DR7 will contain images of 428,408 photographic plates from the Harvard
collection. Out of those, 414,748 have astrometric solutions; 361,689 have
photometric calibrations based on the APASS reference catalog; and 379,754 have
photometric calibrations from the ATLAS-refcat2 reference catalog. The ATLAS
photometry collection consists of 27,966,413,880 measurements, while APASS has
23,574,404,199. Every part of the sky is covered by thousands of plates spanning
about a century of observations.

This preview access comes with some important limitations to be aware of:

- Only the APASS photometric calibration is currently available; ATLAS will come
  later.
- The [in-progress documentation][docs] hasn't been updated to reflect changes made
  during recent work to prepare for DR7. Nothing should be majorly off, but you
  might notice some inconsistencies or incompleteness. There are some nice data
  improvements that haven't yet been documented!
- While the preview access includes all major data products available in
  previous DASCH data releases (cutouts, lightcurves), it is still missing some
  elements that are likely to be of interest. Full-plate FITS images ("mosaics")
  can be downloaded via [Starglass][sgmos], but they still contain DR6
  astrometry and some wonky headers. Valuable photometric calibration metadata
  are not yet available for retrieval.
- The DR7 data are being served by new AWS-based API endpoints that probably
  still have some lingering bugs.
- The classic "Cannon" DASCH data access portal, accessed via the
  `dasch.rc.fas.harvard.edu` website, is still serving DR6 data.
- Due to extremely limited resources, daschlab (using Python and Jupyter) is
  going to be the *only* supported mechanism for DR7 preview access. If you want
  to work with DASCH data in another framework, however, the underlying data
  access APIs are all quite simple JSON exchanges.

[docs]: https://dasch.cfa.harvard.edu/dr7/
[sgmos]: https://starglass.cfa.harvard.edu/docs/api/mosaics.html

To start exploring the DR7 early preview data:

1. Make sure to [install the very latest version of daschlab][dl].
1. Check out [the updated "DRnext" documentation, especially the tutorials][docs].
1. Please send me your feedback! This is the most valuable thing you can do at
   this stage.
