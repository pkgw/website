+++
date = 2024-05-02T14:00:46-04:00
title = "DASCH Data are Now Queryable In VO Tools"
+++

Earlier this week, [Markus Demleitner] let me know that he launched a project
whose development we’ve been discussing for the past six months or so: [DASCH]
data are now available in the [Virtual Observatory][vo]!

[Markus Demleitner]: http://www.tfiu.de/homepage/
[DASCH]: https://dasch.cfa.harvard.edu/
[vo]: https://ivoa.net/

<!-- more -->

If you’re familiar with [DASCH’s legacy “Cannon” data services][1], you might be
asking: “hasn’t DASCH had VO support for a while?” Yes, DASCH has had some
limited support: basically, you could download certain data tables in the
[VOTable] format as well as more basic plain-ASCII formats. If you manually
navigate to the DASCH website, do a query, and set the right options, you can
download a table in a VO format and then load it into a tool that recognizes
that format, like [TOPCAT].

[1]: https://dasch.cfa.harvard.edu/data-access/#cannon-data-portal
[VOTable]: https://www.ivoa.net/documents/VOTable/
[TOPCAT]: https://www.star.bris.ac.uk/~mbt/topcat/

You could call this “VO support” in a certain sense, but it’s a far cry from the
vision that motivates the VO effort. In [the current words of the IVOA About
page][2], “The goal of the Virtual Observatory is to achieve the [feeling that
astronomical data are] all available to explore in a single transparent system.”
The 2001 US VO whitepaper, [“Toward a National Virtual Observatory”][3],
descibes a data access layer that “provides a uniform interface to *all* data
and services within the NVO” (emphasis original). My read of the original VO
documents is that the vision here was quite literal — there would be one VO
portal where you would go to get “the data”, for any astronomical data in the
world.

[2]: https://www.ivoa.net/about/what-is-vo.html
[3]: https://scixplorer.org/abs/2001ASPC..225..353./abstract

That hasn’t quite happened, but we do have an ecosystem of services and tools
that use VO standards for data discovery, and DASCH has not been plugged into
that ecosystem at all. Until now.

Markus has taken an export of the DASCH plate database and [made it
available][4] as an [ObsCore] table, allowing the list of DASCH images to be
queried using standard tools, and a [SIAP2] service, providing a mechanism for
people to actually download the images described in the ObsCore table. (That
service in turn builds on the new [StarGlass] backend API.) His [blog post on
the GAVO site][5] very nicely demonstrates how these building blocks can be
integrated into existing VO tools to suddenly make DASCH data available and
discoverable in ways that simply weren’t possible before.

[4]: http://dc.g-vo.org/browse/dasch/q
[ObsCore]: https://www.ivoa.net/documents/ObsCore/
[SIAP2]: https://www.ivoa.net/documents/SIA/
[StarGlass]: https://starglass.cfa.harvard.edu/
[5]: https://blog.g-vo.org/dasch-is-now-in-the-vo.html

I’m excited for this service to become available, and hopeful that it will help
make DASCH data more widely available to astronomers. That being said, I also
expect that we’ll continue developing *non*-VO data access APIs based on the
StarGlass platform, and hopefully customized support in [astroquery]. In my
experience, any non-trivial dataset always has facets that aren’t captured well
by cross-cutting standards like ObsCore, and you find that power users of such a
dataset virtually always benefit from a query interface that’s really
specifically targeted at it. That’s especially true for DASCH, whose data are
not only *quite* unusual, but also potentially interesting to people coming from
all sorts of disciplines.

[astroquery]: https://astroquery.readthedocs.io/

We’ve already seen a first example of how one-size-fits-all can fall down:
Markus has gotten some complaints because [Aladin] positional searches for
images started yielding numerous hits for DASCH “meteor” plates, which are
shallow, low-resolution images covering absolutely huge areas of the sky —
dozens of degrees on a side, if I recall correctly. Not only do such large
images cause problems for the Aladin UI, which was probably designed for images
more like arcminutes on a side, but they’re also unlikely to be useful
scientifically for most users either. Some of these plates were made with
“telescopes” that consisted of literally three-inch camera lenses, and they have
horrible vignetting, in addition to their terrible depth, resolution, and
cosmetic defects. It may be true that if you pick any point in the
northern-hemisphere sky, you’ll find that hundreds of DASCH meteor plates
overlap it; but there are probably only a handful of people in the world who
will ever want to look at them. We’ll probably restrict the default DASCH VO
database to only include the narrow-field plates, which are more likely to be
useful.

[Aladin]: https://aladin.cds.unistra.fr/

I think this issue nicely demonstrates the kind of challenges that arise when
you take on a mission like that of the VO. The meteor plates can be perfectly
well-described in the ObsCore format, and the query results being returned by
Aladin are presumably perfectly correct — but it turns out when someone says
that they want to see “all available data”, they really mean “all available
data, but, like, obviously not including *that*”. Note that while there are
technical elements to this problem, it’s not purely technical — it’s got design
elements and potentially political ones as well. (What if I angrily insisted to
Markus that every DASCH plate should turn up in every image cone search that
intersects it, even if that means swamping other results in hundreds of meteor
plates?) And the more you optimize for the common “well obviously I didn’t want
*that*” case, the more you run the risk of losing out on the serendipitous
discovery that many people clearly hope that the VO will promote. It’s a tricky
balancing act!
