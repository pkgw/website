+++
date = 2024-06-11T11:13:33-04:00
title = "The Software Citation Station (Wagg & Broekgaarden 2024)"
+++

A new software citation project is hot off the presses — [The Software Citation
Station][scs], as described in Wagg & Broekgaarden 2024
([arXiv:2406.04405][arxiv]). I was starting to build toward a discussion of this
topic in [my previous post][indexing1] so this is great timing, as far as I’m
concerned!

[scs]: https://www.tomwagg.com/software-citation-station/
[arxiv]: https://arxiv.org/abs/2406.04405
[indexing1]: @/2024/indexing-discoverability.md

<!-- more -->

In [Software Indexing and Discoverability][indexing1] I wrote about software
indices (i.e., Big Lists of All The Software; a.k.a registries, catalogs), which
are strangely common in the research world, if you consider how few people in
the general open-source community are interested in building such things. One
oft-cited reason for indexing software is “discoverability”, a goal which I
argued is actually (and perhaps, surprisingly) very much in tension with the
general desire for indices to be as complete as possible.

The other, probably dominant, motivation for indexing software is to enhance
citeability. Everyone knows that software work doesn’t get enough credit in
academia; not only do researchers not cite the software that they *should* be
citing, but even when they want to appropriately credit a certain piece of
software, it’s often not clear *how* to do so. Of course, the latter trend tends
to induce the former.

One popular approach to fix this situation is to try to index software and ask
people to *cite through the index*. For instance, this is explicitly what the
[Astrophysics Source Code Library][ASCL] (ASCL) is all about. Once a piece of
software is indexed in the ASCL (e.g., [pwkit] becomes [ascl:1704.001]), the
index entry is potentially citeable; for instance, ASCL has an agreement with
[ADS] such that [ascl:1704.001] becomes [2017ascl.soft04001W] with associated
[BibTeX][pwkitbt] you can use in your next paper. If *everyone* puts their
software in the same index, then you have a nice uniform way to cite software.

[ASCL]: https://ascl.net/
[pwkit]: https://github.com/pkgw/pwkit/
[ascl:1704.001]: https://ascl.net/1704.001
[ADS]: https://ui.adsabs.harvard.edu/
[2017ascl.soft04001W]: https://ui.adsabs.harvard.edu/abs/2017ascl.soft04001W
[pwkitbt]: https://ui.adsabs.harvard.edu/abs/2017ascl.soft04001W/exportcitation

My blunt assessment, however, is that this approach is **fundamentally broken**.
The fundamental issue is that there is only one entity that gets to determine
how to cite some scholarly product: its publisher. When I publish an article in
[ApJ], it is [AAS Publishing] that ultimately determines how that article should
be cited: “People can cite your article by referencing: ApJ, volume 123, e-id
456, DOI: 10.yadda/yadda, authors Williams, ...”. A significant responsibility
of the AAS Publishing organization is to ensure that such citations will remain
useful into the indefinite future.

[ApJ]: https://apj.aas.org/
[AAS Publishing]: https://journals.aas.org/

The problem with cite-through-the-index is that the index is not the publisher.
The index may record information *about* various entities but is not ultimately
in control of those entities. This means that the index is going to get out of
date with regards to the actual citable objects in question, both in terms of
keeping up with newly-published entities and changes to existing ones. This may
sound like a fixable problem, but I have become more and more convinced that it
is foundational, and dooms the whole enterprise. The problem is that maintaining
a high-quality index of stuff that *other* people publish is enormously
labor-intensive and therefore costly. If you’re an index you need to provide
something really valuable in order to be long-term sustainable. Meanwhile,
publishers — people who make things citeable — are essentially fungible; think
of how many different scholarly journals there are! So there will always be
bargain-basement publishers; and in the field of software, publishing generally
costs zero. The value provided by the publishing service is not enough to offset
the costs of maintaining an index worth using.

[Zenodo]: https://zenodo.org/

By constrast, consider ADS. It is also an index of published objects, but while
astronomers will certainly trade around bibcodes like [2022ApJ...935...16E]
informally, when it comes time to make a formal citation, they “resolve” that
bibcode to the *actual* citation, [Eftekhari et al., 2022 ApJ 935 16][eftekdoi].
ADS indexes citeable items but is not construed as making them citeable itself.

[2022ApJ...935...16E]: https://ui.adsabs.harvard.edu/abs/2022ApJ...935...16E
[eftekdoi]: https://doi.org/10.3847/1538-4357/ac7ce8

That may be so, but it doesn’t make it any cheaper to maintain ADS’s index. ADS
is clearly doing something that the community finds incredibly valuable. If ADS
isn’t making things citeable, what is it doing?

In my view, the key is that ADS provides uniform search of the citation network
*across publishers, in a regime where references between items from different
publishers are both interesting and ubiquitous*. Articles cite each other across
publisher boundaries willy-nilly, which makes it very valuable to be able to
have a unified search interface that crosses publisher boundaries as well. If I
don’t care about that cross-referencing network, or items within one publisher
only reference other items from the same publisher, the value of an ADS-like
service is a lot less clear. (This is why it’s not clear to me that “ADS for
datasets” is a viable concept: the network of links between datasets is, I
think, a lot less interesting.) In general, a multi-publisher index is going to
provide some kind of homogeneous view of its collection, and there has to be
something about that homogenization that people find valuable *in and of
itself*.

This analysis also helps us see why software citation is in such sorry shape to
begin with. **Software is hard to cite because it is self-published.** I don’t
send my latest Python package to AAS for them to review and archive it; I just
upload it to GitHub myself. This is great in a lot of ways, but it turns out
that high-quality publishing is harder than it looks! In particular, archiving
and preservation, the bedrock of citeability, are specialized tasks that really
ought to be done by professionals. Return to the analogy with traditional
articles: when I publish through ApJ, I’m not told to manually upload my article
to Zenodo or to figure out what the appropriate reference should be; AAS does
the necessary tasks on my behalf and then tells me the result. *This is how it
should be*. Amateur-hour attempts to do these things are what get you, well, the
current state of software citation. Instead of a few professionals doing things
in a consistent way, you get a bunch of well-meaning people trying to figure
things out one at a time.

One path to improving the state of software citation, then, is to make it easier
for people to do a decent job of publishing software. Tools like [Zenodo] and my
[Cranko] aim to help on this end, and it is easy to see how the value provided
by ASCL was much greater before GitHub and its ilk emerged.

[Cranko]: https://pkgw.github.io/cranko/book/latest/

But the other half is that we have an enormous corpus of self-published stuff
out there that deserves citation, and we need to make it easier for people to do
so. This finally brings us back to the [Software Citation Station][scs]. At
first blush this might seem similar to something like [ASCL], but it’s different
in a very important way. SCS is not attempting to provide its own citations;
instead, it is like ADS, indexing entities that were published elsewhere. This
is, in my view, the correct approach.

The SCS also, I believe, captures a key insight about the role of indices in the
field of software citation. If we’re trying to maintain an index of software
packages that are published in a variety of external locations, the key question
we have to ask is: why is anyone actually going to use this thing? What are we
providing that a Google search doesn’t?

There’s a beautiful answer: standardized information about how things should be
cited! The whole key is that there will *never* be a one-size-fits-all way to
cite any piece of software that’s ever been posed online. This is why people
write [whole guidebooks][htc1] about [how to cite][htc2] [software][htc3], while
no one sees any need to document “how to cite an article published in a major
journal”. Because software is self-published and each package is a unique
snowflake, citation-wise, citation instructions are unclear and are scattered
across the internet in READMEs, Zenodo pages, published articles, and elsewhere.
This is exactly the kind of inhomogeneous chaos that a good index can simplify.
The actual citations can’t be homogenized, but the information about *how to
cite* can be. Probably the *only* commonality about all the different pieces of
research software out in the world is that people want to know how to cite them!

[htc1]: https://libguides.mit.edu/c.php?g=551454&p=3900280
[htc2]: https://www.software.ac.uk/publication/how-cite-and-describe-software
[htc3]: https://www.ncbi.nlm.nih.gov/pmc/articles/PMC7805487/

There’s one piece, however, that I think the SCS is missing. Maybe Tom and Floor
are about to get a bunch of funding, but I suspect that the maintenance of this
index will be challenging. As I alluded to in [my kicker last week][indexing1],
I can think of basically one “form factor” that is proven to at least be *able*
to yield a reliable online knowledgebase without paid staff: a wiki.

While the SCS has a form for people to submit new software, I expect that
framing will discourage involvement. If SCS is missing an entry for, say,
[MOSFiT] (and it is), I probably won’t feel comfortable “submitting” a record
for it unless I’m the primary author (and I’m not). But if we cast it as a wiki,
then it opens the door up for me to do my best to create a record for the
software, even if it’s not “mine”. Of course, maybe I’ll make a mistake and
someone will have to come in and correct it, but that’s exactly how people
expect to use wikis. If I’m an active maintainer of a package, I’ll want to come
in and check that the SCS record is correct; but if a package becomes
unmaintained, it is wholly appropriate, and perhaps necessary, for other people
to keep the citation information up-to-date. For instance, maybe a project was
published using some service that got shut down and absorbed into Zenodo; you
don’t need to be the original author to assess whether the citation information
should change to refer to the new service. While the desires of active
maintainers should always take precedence, of course, there are plenty of cases
where third parties are perfectly competent to maintain the citation
information.

[MOSFiT]: https://mosfit.readthedocs.io/

So really what I think we need is a “Cite-o-pedia”: one page per citeable
package, creatable or editable by anyone in the world. You’d certainly want some
structure under the hood to record common elements like associated articles,
Zenodo concept DOIs, and so on, but fundamentally the citation information might
have to be free text, because the wild diversity of practices that authors want
people to use.

And with this perspective, nothing about this idea is specific to software. For
instance, you could imagine a Cite-o-pedia entry for [Erik Mamajek’s star
notes][mamajek]; I know for a fact that people want to cite them even if Erik
himself isn’t very concerned with that! The Cite-o-pedia concept is only
specific to *self-published scholarly entities*, simply because things that come
out of traditional publishers are homogeneous enough that it’s “obvious” how to
cite them. 

[mamajek]: https://www.pas.rochester.edu/~emamajek/doc.html

That being said, one thing that a Cite-o-pedia could do that would dovetail well
with a software focus would be helping people deal with versioning. If I’m
citing a piece of software, I should ideally refer to two items: the overall
software package, and the exact version that I’m using; the difference is
formalized by Zenodo in the distinction between [concept and version DOIs][cvd].
But most people are a little sloppy about this stuff. You could imagine
Cite-o-pedia pages having decision trees that walk you through how to figure out
which version you’re using and what, therefore, to cite. Versioning is also not
specific to software: you could imagine an entry for [SDSS] that helps you
figure out which data release you’re using and the appropriate citation; see
also AAS’s [living articles].

[cvd]: https://pkgw.github.io/cranko/book/latest/integrations/zenodo.html#orientation-software-dois
[SDSS]: https://www.sdss.org/
[living articles]: https://doi.org/10.3847/1538-4357/aae58a

A couple more minor comments on the SCS article:

- The article states that if you cite a piece of software, you should cite
  everything in its dependency graph as well. It is *far* from obvious to me
  that this is true: if nothing else, as a matter of practicality, the
  dependency graph for a large application might include hundreds of packages.
  As an analogy, if I’m citing an article, I don’t cite every article that *it*
  cites! Choosing to cite something is ultimately just that — a choice, made by
  the author.
- The SCS is currently hosted on the [tomwagg.com]; it will probably have to
  move to a non-personal domain to really start feeling like a community-wide
  resource. Fortunately, domains are pretty cheap these days.

[tomwagg.com]: https://www.tomwagg.com/

It’s exciting to see that people are thinking along these same lines, and the
Software Citation Station feels very close to what I think the community needs.
I’ve convinced myself that the “wiki” framing will be super important for
success, but I’ll be very curious to see if that bears out in practice.
