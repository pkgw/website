+++
date = 2024-03-28T02:00:00-04:00
title = "DASCH Scanning is Complete — What’s Next?"
+++

Today’s the day! After two decades of work, the DASCH scanning “prime mission”
is now complete. Today we celebrated the scanning of the final DASCH-able plates
in the HCO collection with a small event and a champagne toast.

<!-- more -->

This is the end of a chapter, but far from the end of the story. Just how much
work is left to do is a question whose answer depends on how broad of a
perspective you want to take.

When it comes to making the existing DASCH data available and useful to the
astronomical research community, there’s still plenty to be done. In the near
future, that work will mostly take the form of the next data release, DR7,
currently under development as [DRnext]. As my recent posts have perhaps
conveyed, this is going to be a lot of effort — there’s basically an unbounded
amount of work that could go into writing documentation, fleshing out software
like *daschlab*, and other support activities — let alone actual improvement of
the underlying data products. But DR7 also needs to come out in a finite amount
of time. I can already tell that it’s going to be difficult to draw the line at
which the polishing has to stop and the thing simply has to get out the damn
door.

[DRnext]: @/2024/dasch-drnext-beta.md

Regardless of where exactly that line gets drawn, there’s absolutely no chance
that DR7 will completely plumb the depths of the DASCH data. You could spend the
rest of your life improving and expanding the analysis of a dataset as large and
heterogeneous as DASCH. Whether we’ll see Data Releases 8, 9, *etc.* off into
the future is, as ever, going to depend on money. If you ask me, a dataset as
rich and unique as DASCH certainly deserves lots of funding to analyze and
enhance it — and that’s not even considering its cultural and historical
implications — but lots of projects deserve funding.

It’s also important to underline that “DASCH-able plates” represents only a
portion of the Plate Stacks holdings. About 430,000 plates have been scanned for
DASCH, compared to an estimated 550,000–600,000 plates in the entire collection.
This gap exists because DASCH explicitly focused on plates suitable for
astronomical photometric and time-domain analysis. Plates that did not fit this
definition include solar, lunar, and eclipse observations, and above all, the
spectrum plates, containing the hundreds of thousands of stellar spectra studied
by Annie Jump Cannon. These plates might not be useful for time-domain
photometry, but that is not at all to imply that they're not valuable in their
own right. For instance, it’s believed that the spectrum plates could provide
unique insight into climate change by measuring the gradual changes in the
absorption lines that Earth’s atmosphere imprinted on the data.

But wait, there’s more! The Plate Stacks holdings also include film, written
materials, and other artifacts. We can consider DASCH to be one tentpole among
many in the broader effort to digitize the holdings of the Plate Stacks. It was
certainly the tallest tentpole, but others remain.

Heading in a different direction, we can look beyond the Plate Stacks
collection. [The DASCH scanner](https://dasch.cfa.harvard.edu/scanner/) is still
going strong, and I think it’s fair to say that in many ways it remains the
highest-quality plate scanner in the world. It’s very tempting to consider
digitizing other plate collections. This presents logistical challenges: the
scanner is essentially immovable, but large collections of glass plates are
difficult and expensive to transport as well. It’s also true that the scanner is
a one-of-a-kind piece of hardware, analogous to a telescope instrument. Many of
its components are virtually irreplaceable — and getting quite old. If you
wanted me to promise that the system will keep working for the next decade,
you’d have to give me a fairly significant sum of money to invest in documenting
the existing setup, assessing potential failure modes, and developing recovery
plans.

Granting that baseline risk, the Plate Stacks are already in possession of a few
external plate collections whose digitization with the DASCH scanner we hope to
demonstrate. We have copies of [POSS-I], the ESO Southern Sky Survey (which
appears to be surprisingly undocumented, but see [Schuster in *Messenger*,
1980][1]), the [SERC] Equatorial Red survey (SERC-ER) and associated [AAO]
Second-Epoch Survey ([AAO-SES], aka AAO-R), and the Palomar “Quick V” (Pal-QV,
[Lasker+ 1990][2]) survey done in support of the Hubble Guide Star Catalog
project. Together these form a substantial fraction of the plates that went into
building the [Digitized Sky Survey][3]. I’d love to compare DASCH results to the
DSS data! This could dovetail beautifully with my longstanding desire to upgrade
the “DSS Terapixel” map that’s the default optical basemap for [WorldWide
Telescope], which is generally great but has a few longstanding issues (most
notably for scientists, a few-arcsecond global astrometric offset). What’s kind
of amazing is that, as far as I’ve been able to figure, digitizations of these
photographic surveys *still*, in the year 2024, represent the best way to obtain
a deep, high-resolution optical map of every last nanosteradian of the sky.
Modern surveys go awfully deep and awfully wide, but I’m not aware of a set of
surveys that can be homogeneously combined across the entire sphere.

To try to make concrete the sophistication of the DASCH system — not just the
scanner, but the people and processes surrounding it — I’ll note that we could
probably digitize all of POSS-I in a couple of good weeks.

[POSS-I]: https://skyserver.sdss.org/dr5/en/proj/advanced/skysurveys/poss.asp
[1]: https://ui.adsabs.harvard.edu/abs/1980Msngr..22....7S
[SERC]: https://en.wikipedia.org/wiki/Science_and_Engineering_Research_Council
[AAO]: https://aat.anu.edu.au/about-us/AAT
[AAO-SES]: https://ui.adsabs.harvard.edu/abs/1989BICDS..37...13L
[2]: https://ui.adsabs.harvard.edu/abs/1990AJ.....99.2019L
[3]: https://archive.stsci.edu/missions-and-data/dss--gsc
[WorldWide Telescope]: https://worldwidetelescope.org/home/

Finally, a call to action: these are things that I would like to see happen, but
in truth probably *none* of them are going to occur (except DR7) without
community support and collaboration. If any of these topics are at all
interesting to you, please let me know! I would genuinely love to help other
groups launch their own projects based on the DASCH data, the scanner, or other
elements of the Plate Stacks collection.
