+++
date = 2023-12-11T11:00:21-05:00
title = "Launching the Refreshed DASCH Website"
+++

I’m pleased to be able to report that the refreshed DASCH website is now live!
Visit [dasch.cfa.harvard.edu](https://dasch.cfa.harvard.edu/) for all of your
DASCH informational needs.

<!-- more -->

The key piece of context is that this refresh is aiming to preserve the general
structure of the previous DASCH website — which I refer to as the “legacy”,
“PHP”, or [“Cannon”][cannon] site — as much as possible. *For now.* In my honest
opinion the legacy site has a lot of problems that need correcting, but I don’t
want to change too much about it at once. So while I rearranged some of the
material and tried to make a few small-bore fixes, the overall design is largely
the same for the time being.

[cannon]: https://www.rc.fas.harvard.edu/services/cluster-computing/#Cannon

Part of the reason for this is that a lot has changed behind the scenes. I’ve
finally put the site’s source code in version control, with both the
public-facing site and the private version (which is basically the same thing,
with various access controls turned off) deriving from the same source code. The
websites are backed by various C programs that I’ve also brought into version
control and recompiled. In several cases, this revealed bugs that seem to have
been fixed on the production site, but whose fixes I couldn’t locate in *any* of
the source code trees I’ve been able to get my hands on — this is why we use
version control, people! I’ve also updated the data-access layers to start using
new-and-improved filesystem layouts that I’m setting up on the [Cannon][cannon]
cluster. Now that the webserver code is aware of these layouts, I can start
migrating the files and (re)organizing all of the data holdings, which is
something I’ve been aiming to do for the entire year.

Now that the initial update is complete, I can really get to work. One of the
top goals is to disable the spatial restrictions that we’ve inherited from the
most recent data release, [DR6] from 2019 (!). Historically, DASCH released data
in tranches going from high galactic latitudes to lower ones, up to DR6 which
released data for all sources with *b* > 0. Specifically, the public website
doesn’t let you access photometry or thumbnails for sources with negative
galactic latitudes. But the data are all there, and the strong feeling is that
we should just make everything available without restrictions. I intend to do so
as soon as it feels like the website update has shaken down — hopefully in a
matter of weeks. The only caveat is that these data are a product of an
always-evolving pipeline, and they should be used carefully. *Hopefully*
everything is fine, and more to the point, the data should be of the same
quality as everything that’s currently publicly available. But one of the
benefits of doing a formal data release is that it forces you to sit down and
evaluate all of the data and really make sure that everything is in good order.

[DR6]: https://dasch.cfa.harvard.edu/dr6/

Separate from the “DR6 restrictions,” there are further restrictions on download
full-size mosaics and photos of plates. The (stated) reasoning is concern about
overloading the legacy website, which makes sense since it’s a basic PHP server
running in a Cannon VM, with few tools for monitoring or load management.

But other members of the DASCH team have been cooking away on a new
cloud-powered site that will blow the Cannon site out of the water:
**Starglass**. It’s going to be really slick, and the backend is nearly ready to
expose to the world. We have copies of the key data hosted on S3 and the site is
built with all of the modern bells and whistles, including rate-limiting and
key-based API access. Once that’s ready to go — which, with luck, will also be a
matter of weeks — I’ll update the Cannon site to serve up data from the
Starglass services. That way we can finally make these data available to
everyone.

I’ll also be updating the Cannon site to point people to use Starglass directly
— that’s the direction we want things to go in the future. But there are a bunch
of people who are used to the existing site, and we don’t want to leave them out
in the cold. So my aim is to keep it functioning as-is for as long as possible,
while building out a bunch of great new features on the basis of the Starglass
backend.
