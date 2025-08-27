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

The corollary of this, though, is that a truly comprehensive and systematic
survey of documentation tools is virtually impossible. So, I won’t try to
present an exhaustive list of software that one could use.

But that’s not all. I also wrote that I think it’s important to construe the
word “tools” expansively. Sure, there are things that everyone would agree fits
that definition: [Sphinx], [mdBook]. But what about [Diátaxis], the “four kinds
of docs” paradigm [that I’ve written about before][dtxs]? If we think of a
“tool” as “something that helps us accomplish a goal”, then I would say that
Diátaxis absolutely fits that definition.

[Sphinx]: https://www.sphinx-doc.org/en/master/
[mdBook]: https://rust-lang.github.io/mdBook/
[Diátaxis]: https://diataxis.fr/
[dtxs]: @/2023/divio-documentation-system/index.md


*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.