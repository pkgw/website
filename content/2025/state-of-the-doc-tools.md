+++
date = 2025-08-27T00:00:00-04:00
title = "The State of the Doc Tools"
+++

Last week I [presented](@/2025/state-of-the-docs.md) some of my takeaways
stemming from a small set of interviews with researchers about scientific
software documentation. This week, I’m reporting out the results of a survey and
review of tools for documenting research software: **The State of The Doc
Tools**, 2025.

<!-- more -->

An early, big-picture finding: as a result of both my interviews and my survey,
I’ve concluded that it’s really important to construe the words “documentation”
and “tools” expansively. That first word, “documentation”, might conjure up an
image of a hefty spiral-bound manual. But that’s just one form-factor out of
many possibilities. I think that more than ever, it should be emphasized that
documentation can come in many shapes and sizes. YouTube videos?
[Sure](https://www.youtube.com/watch?v=HTXScEOwze8)! StackExchange Q&As?
[Indeed](https://unix.stackexchange.com/questions/744675/resizing-an-lvm-storage-repository-in-xcp-ng).
Discord chat groups? [Empirically,
yes](https://shkspr.mobi/blog/2023/07/discord-is-not-documentation/). Zines?
[Why not](https://wizardzines.com/)? Custom-trained LLMs? I wouldn’t be
surprised if they become standard for large projects, soon.

That’s not to suggest that one ought to try to occupy all of these spaces —
especially at the scale of a typical scientific software project! — but I think
it’s really important to think very openly about where your documentation could
live. And perhaps where it *does* live, in practice. Maybe a hefty book is right
for your project, but maybe not.

The corollary of this, however, is that it’s impossible to survey documentation
tools in a truly comprehensive and systematic way. So, I won’t try to present an
exhaustive list of software that one could use.

But that’s not all. I also wrote that I think it’s important to construe the
word “tools” expansively. Sure, there are things that everyone would agree fits
that definition: [Sphinx], [mdBook]. But what about **[Diátaxis]**, the “four
kinds of docs” paradigm [that I’ve written about before][dtxs]? If we think of a
“tool” as “something that helps us accomplish a goal”, then I would say that
Diátaxis absolutely fits that definition. And that is indeed what I’m saying!

[Sphinx]: https://www.sphinx-doc.org/en/master/
[mdBook]: https://rust-lang.github.io/mdBook/
[Diátaxis]: https://diataxis.fr/
[dtxs]: @/2023/divio-documentation-system/index.md

[Last week][tsod] I observed that, despite valiant efforts to build one, there
is not much of a “community of practice” for many of the people who are
documenting scientific software. I believe that the lack of this community,
while it definitely doesn’t do us any favors when it comes to tangible tools
like Sphinx or mdBook, is even *more* damaging when it comes to intangible,
intellectual tools like Diátaxis. The key difference is that intangible tools
tend to remain nebulous, un-named, un-documented, and therefore more difficult
to research on your own. I keep on coming back to the Diátaxis example, after
all, because Daniele Procida has turned what could have been a somewhat vague
mental model into a tangible, referenceable thing! (We see a bit of this now in
how everyone falls over themselves to brand [their security vulnerability
discoveries](https://www.heartbleed.com/).) If we had stronger communities of
practice, the intellectual tools would spread more easily. In the meantime, more
people should follow Daniele’s lead.

[tsod]: @/2025/state-of-the-docs.md

All that being said, even after doing a fair amount of digging I’ve been unable
to unearth many other “intangible tools” to include in my survey. There is the
**[Information Mapping][im]** methodology, which might be useful, but I’m not
going to find out because it’s a proprietary system (!) — you have to pay to be
taught the model. For both practical and ethical reasons, I consider this to be
a total non-starter.

[im]: https://informationmapping.com/

The kinds of intellectual tools that I’m looking for would, for the most part,
probably be found in the field known as **[Information Architecture][ia]** (IA).
There are techniques associated with the IA field like **[card sorting]** and
**[tree testing]** that could be useful tools in the software documentation
toolbox to sit alongside Diátaxis. I’ve found the website of the **[Nielsen
Norman Group][nng]** to be surprisingly useful in learning about some of these
concepts. While NNG is your typical corporate consultancy trying to make a buck
off of you, I’ve found their online materials to be *much* more useful than the
usual content-marketing drivel that you find on such sites. I’ve bought a few IA
books (including the aptly-named [*Information Architecture*][iabook]) and will
see if any of them seem worth recommending to non-specialists.

[ia]: https://en.wikipedia.org/wiki/Information_architecture
[card sorting]: https://en.wikipedia.org/wiki/Card_sorting
[tree testing]: https://en.wikipedia.org/wiki/Tree_testing
[nng]: https://www.nngroup.com/
[iabook]: https://search.worldcat.org/title/86110226



*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.