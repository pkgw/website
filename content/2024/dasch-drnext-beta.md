+++
date = 2024-03-21T11:06:05-04:00
title = "Beta-Testing DASCH “DRnext”"
+++

The work on [DASCH] continues to move forward! Yesterday, I posted a first draft
of a new set of resources for astronomers. These are collected under the [DASCH
DRnext] moniker and are now ready to be checked out.

[DASCH]: https://dasch.cfa.harvard.edu/
[DASCH DRnext]: https://dasch.cfa.harvard.edu/drnext/

<!-- more -->

The fact underlying the [DRnext] designation is that while DASCH has
historically had a series of “data releases” (DRs), they weren’t really releases
in the usual sense. Normally a DR is associated with a specific set of immutable
artifacts, so that if you choose to work with, say, [SDSS DR12], you’ll always
get exactly the same results if you repeat the same queries. But DASCH only has
one set of data servers, and they're always being updated as scanning proceeds
and the data processing gets refined, so we’re unable to provide locked-in
artifacts. Historically, the DASCH DRs were basically about lifting restrictions
on public access to certain portions of the sky.

[DRnext]: https://dasch.cfa.harvard.edu/drnext/
[SDSS DR12]: https://skyserver.sdss.org/dr12/

It would be nice to be able to provide traditional, immutable DRs, but with the
current resources and system architecture that’s not feasible. In the meantime,
the “DRnext” label is the place where the latest-and-greatest DASCH docs and
tools will accumulate — sort of like the `main` branch of a repository as
opposed to a versioned release.

So, what’s just landed on `main`?

The centerpiece of DRnext is an effort to deliver better tools for scientific
data analysis via *daschlab*, the Python package that I’ve [mentioned
here](@/2024/daschlab-sneak-peek.md) [a few
times](@/2024/fun-python-filtering-pattern.md). In support of this, we now have:

- [The demo video I posted about earlier](https://www.youtube.com/watch?v=GofXy8BZxjY)
- [Python API reference docs](https://daschlab.readthedocs.io/) (which I also mentioned earlier)
- [A tutorial slideshow][rycnc] that lets you work through
  the notebook shown in the video, via [a MyBinder notebook][rycncbinder]
- [The beginnings of a lightcurve reduction cookbook](https://dasch.cfa.harvard.edu/drnext/reduce-lightcurve/)
  based on *daschlab* (although there's no reason you couldn’t use the same techniques in
  a different data-analysis system)
- [Instructions for installing *daschlab* locally](https://dasch.cfa.harvard.edu/drnext/install-daschlab/)

[rycnc]: https://dasch.cfa.harvard.edu/drnext/rycnc/
[rycncbinder]: https://mybinder.org/v2/gh/pkgw/daschlab/HEAD?labpath=notebooks%2FRY+Cnc.ipynb

Integrated with this are several other new resources:

- An [**Introduction to DASCH** slideshow][intro] aimed at astronomers
- A cloud-powered [sample “quicklook” notebook][ql] aiming to provide an
  alternative to the [Cannon web-based plotting tools][cannonlc]
- Reference documentation of the [DASCH lightcurve table columns][lccols]
- Thorough cross-referencing to the newly-launched [StarGlass] website and API
  where appropriate
- Reorganizing the existing material to hopefully make it more manageable

[intro]: https://dasch.cfa.harvard.edu/drnext/introduction/
[ql]: https://mybinder.org/v2/gh/pkgw/daschlab/HEAD?labpath=notebooks%2FQuicklook.ipynb
[cannonlc]: http://dasch.rc.fas.harvard.edu/lightcurve.php
[lccols]: https://dasch.cfa.harvard.edu/drnext/lightcurve-columns/
[StarGlass]: https://starglass.cfa.harvard.edu/

If you look at the [DRnext landing page][DRnext], I have very plainly used [the
Diátaxis (née Divio) documentation model][diataxis] for organizing things. This
is the first time that I’ve put together a landing page with an explicit
Tutorial/How-To/Explainer/Reference breakdown; in this particular case, at
least, I think it works well. (All of this might also help explain why I’ve been
[thinking about](@/2024/digital-docs-are-web-apps.md) digital documentation
[lately](@/2024/digital-docs-have-apis.md), although that’s something I do
pretty often regardless.)

[diataxis]: @/2023/divio-documentation-system/index.md

I’ve tried a couple of new things in assembling this documentation. One is the
way that I’ve presented the [“Introduction to DASCH” material][intro] as a
web-based slideshow; historically I would have written this kind of material up
as a brief document. I find it a little painful to admit, but more and more I
feel like slide decks are a good way to deliver this kind of information; the
pagination breaks things up into digestible chunks in a way that’s easier to
approach than even a relatively short single-column presentation. (Drive-by hot
take: in practice if not stated intent, [Knuth’s literate
programming](http://brokestream.com/tex-web.html) is basically a way to
PowerPoint-ify source code.) I would hate to only deliver this kind of slideshow
in a properietary format unrecognized by the browser, like a PPTX file, but
since I can deliver the slides seamlessly using [reveal.js], I’m much more
comfortable making them the exclusive publication format for this information.

[reveal.js]: https://revealjs.com/

Building on that, [the *daschlab* RY Cnc tutorial][rycnc] is delivered as yet
another slideshow, one in which most of the slides contain video clips showing
how the software is used interactively. The idea is that people can follow along
the slides, and play and replay the clips, while using the notebook in a
separate browser window. I think that video examples are super important for
showing people how to use the highly interactive *daschlab* software; hopefully
the slide-based presentation once again breaks things into nicely digestible
chunks. It turned out to be relatively easy to record one long screencast of
myself running through the notebook, then to use [Kdenlive] to slice it up into
short clips suitable for each slide.

[Kdenlive]: https://kdenlive.org/

Finally, I’m hopeful that the [“cloud quicklook notebook”][ql] idea will provide
a decent alternative to DASCH’s legacy web-based lightcurve plotting tool. While
the legacy tool is not super featureful, can produce genuinely misleading
output, and is nigh-unmaintainble, it is *clearly* really important to give
people a way to peek at some data right in their web browser, and that’s
something that’s generally tricky with Jupyter/code-oriented tools. I’m crossing
my fingers that a lightweight notebook can meet that need, and hopefully the
“quicklook” framing will help people see it in the right light.

There are plenty of systems aiming to integrate this kind of notebook into web
sites/apps in a much smoother way, but I don’t think that DASCH has the
resources to pursue them right now. So for the time being the quicklook is based
on [MyBinder], which is a great service, especially because it’s free … but it’s
slow, and not always reliable. Even if we don’t add some slick
[Solara-type](https://solara.dev/) integration where you don’t even realize that
you’re running code, I think that the UX would feel a *lot* better if the
notebook could reliably spin up in just five or ten seconds. One of the next
things I want to do is look into setting up *daschlab* on a science platform
service like [SciServer], or something that can hopefully offer ease-of-use
comparable to MyBinder with better performance. Here at the CfA, there’s a
vision of building a “Nexus” science platform (based on [Rubin][rsp]). It’s
still in the early stages, so it’s not something that I expect to be able to
integrate any time soon, but it would be an *ideal* home for this kind of
capability. 

[MyBinder]: https://mybinder.org/
[SciServer]: https://sciserver.org/
[rsp]: https://data.lsst.cloud/