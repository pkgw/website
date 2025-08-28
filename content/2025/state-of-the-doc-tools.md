+++
date = 2025-08-28T15:13:21-04:00 # deploytool
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
documentation can live in some surprising places. YouTube videos?
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
that definition: **[Sphinx]**, **[mdBook]**. But what about **[Diátaxis]**, the
“four kinds of docs” paradigm [that I’ve written about before][dtxs]? If we
think of a “tool” as “something that helps us accomplish a goal”, then I would
say that Diátaxis absolutely fits that definition. And that is indeed what I’m
saying!

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
toolbox to sit alongside Diátaxis. I’ve found the website of an outfit called
the [Nielsen Norman Group][nng] to be surprisingly useful in learning about some
of these concepts. While NNG is your typical corporate consultancy trying to
make a buck off of you, I’ve found their materials to be *much* more useful than
the usual content-marketing drivel that you find on such sites. I’ve also bought
a few IA books (including the aptly-named [*Information Architecture*][iabook])
and will see if any of them seem worth recommending to non-specialists.

[ia]: https://en.wikipedia.org/wiki/Information_architecture
[card sorting]: https://en.wikipedia.org/wiki/Card_sorting
[tree testing]: https://en.wikipedia.org/wiki/Tree_testing
[nng]: https://www.nngroup.com/
[iabook]: https://search.worldcat.org/title/86110226

Turning back towards the traditionally-recognized software documentation tools,
we can return to **[Sphinx]** as a starting point. One direction to go is to
consider comparable tools aimed at other programming systems (granting that
Sphinx is not actually Python-specific), which of course are numerous:
**[tsdoc]** for TypeScript, **[rustdoc]** for Rust, **[Doxygen]**, **[godoc]**,
**[Swagger]** for web APIs, **[rdoc]**, and on and on and on and on. It’s
understandable that most modern programming languages deliver integrated
documentation systems; each language has its own distinctive semantics that need
to be captured in its API documentation framework. It’s also understandable that
developers may be naturally inclined to try to author *all* of their
documentation using these tools. But it’s worth pointing out explicitly that
there’s no particular reason that a tool designed to document APIs in a certain
language will be very good, or even adequate, for authoring general-purpose,
non-API documentation.

[tsdoc]: https://tsdoc.org/
[rustdoc]: https://doc.rust-lang.org/rustdoc/what-is-rustdoc.html
[Doxygen]: https://doxygen.nl/
[godoc]: https://go.dev/blog/godoc
[Swagger]: https://swagger.io/
[rdoc]: https://ruby.github.io/rdoc/

A very good reason to try to stick with language-specific documentation tools,
though, is that modern ones can pair with public services that allow you to host
documentation online for free. **[ReadTheDocs]** is the standard for Python,
**[docs.rs]** for Rust, and in a certain sense, **[CTAN]** for LaTeX are
examples. If there’s one thing I’ve come to appreciate over the years, it’s that
to first order nobody wants to host their own content, and in my experience
services like these have been transformative for how scientific software is
documented. That is, the universe of documentation tools to consider includes
not just authoring tools, but also publishing platforms.

[ReadTheDocs]: https://readthedocs.io/
[docs.rs]: https://docs.rs/
[CTAN]: https://www.ctan.org/

Continuing along that thread, there are [*huge* numbers][ssgs] of static site
generators out there that can (or could) be used to generate software
documentation websites, as well as “* Pages” hosting services to host such sites
for free. A random sub-sample from the former group: **[Jekyll]**, **[Hugo]**,
**[Gatsby]**; I’m partial to **[Zola]** and have used it for docs;
**[Docusaurus]**, **[MkDocs]**; **[mdBook]**, **[GitBook]**. In the latter
group: **[GitHub Pages]**, **[GitLab Pages]**, **[Netlify]**; **[ReadTheDocs]**
can effectively act as a static pages host; and all of the cloud providers make
this drop-dead easy. This is a completely saturated market.

[ssgs]: https://github.com/myles/awesome-static-generators
[Jekyll]: https://jekyllrb.com/
[Hugo]: https://gohugo.io/
[Gatsby]: https://www.gatsbyjs.com/
[Zola]: https://getzola.org/
[Docusaurus]: https://docusaurus.io/
[MkDocs]: https://www.mkdocs.org/
[GitBook]: https://www.gitbook.com/
[GitHub Pages]: https://pages.github.com/
[GitLab Pages]: https://docs.github.com/en/pages
[Netlify]: https://www.netlify.com/

A market that’s a bit *less* saturated, somewhat to my surprise, is the one for
wikis. This is probably because wiki software needs to implement both authoring
and publishing (i.e., hosting) features, unlike the static-site case where
there’s a clean separation between the two halves. But more and more I’ve come
to feel that the basic wiki paradigm is a strong one — it works pretty well for
[one of the top ten sites on the internet][wp] — and that it would be a great
fit for a lot of use cases in the software-documentation. But for some reason
wiki software tends to have a 90’s feel, and a 90’s look as well. I’m becoming
more and more convinced that there’s a lot of untapped opportunity in this
space. Anyway, some of the main wiki tools are: **[MediaWiki]**, **[GitHub’s
wikis]**, **[DokuWiki]**, **[MoinMoin]**; you could put **[Confluence]** into
this box too.

[wp]: https://en.wikipedia.org/
[MediaWiki]: https://www.mediawiki.org/
[GitHub’s wikis]: https://docs.github.com/en/communities/documenting-your-project-with-wikis/about-wikis
[DokuWiki]: https://www.dokuwiki.org/
[MoinMoin]: https://moinmo.in/
[Confluence]: https://www.atlassian.com/software/confluence

There are other platforms that, integrate authoring and publishing like wikis,
but have modernized WYSIWYG styles. **[Curvenote]** (a commercial product) is
specifically aimed at scientific authoring. **[PubPub]** is a unique open-source
platform aimed at a more general academic audience (combining Google-Docs-like
collaborative editing with features like DOI minting and citations), but sadly
the project’s very existence is under threat due to the withdrawal of a major
funder. PubPub is the only substantial noncommercial player in this space that
I’m aware of, so I really hope something the worst doesn’t come to pass. (In
case it’s not obvious, I’m focusing almost exclusively on open-source and
noncommercial tools here; beyond generalized academic cheapness, I believe that
there are profound reasons to prefer them in this domain.) I’m a bit suprised
that **[Overleaf]**, the online collaborative LaTeX editor, hasn’t gotten into
the publishing business, but as far as I can tell they haven’t yet.

[Curvenote]: https://curvenote.com/
[PubPub]: https://pubpub.org/
[Overleaf]: https://www.overleaf.com/

If you want to go a little bit farther, you can think of mainstream social media
platforms as being on the same continuum. It’s not incorrect to describe
**[YouTube]** as a platform for creating and publishing content, after all, and
one can imagine scientific software projects where it would not be an
unreasonable place to host (nontextual) documentation. Likewise, the bulk of the
documentation regarding some software projects probably lives on
**[StackExchange]** sites, in the form of answers to user questions. This line
of thought brings us to publicly-visible “forum”-type systems (**[Discourse]**,
**[phpBB]**) and more-synchronous “chat”-type platforms (**[Discord]**,
**[Gitter]**, **[Zulip]**, **[Slack]**) which are usually less- or non-public.
These platforms are not generally what people think of when they think of
“documentation”, but I reiterate that it can very much happen that they end up
hosting a significant or even dominant portion of the recorded information about
a software project.

[YouTube]: https://www.youtube.com/
[StackExchange]: https://stackexchange.com/
[Discourse]: https://www.discourse.org/
[phpBB]: https://www.phpbb.com/
[Discord]: https://discord.com/
[Gitter]: https://gitter.im/
[Zulip]: https://zulip.com/
[Slack]: https://slack.com/

Finally, we can narrow our focus to the tools used to produce individual
documents. In the context of our touchstone Sphinx, this takes us to the
underlying markup options like **[reStructuredText]** and the hugely popular
family of **[Markdown]** syntaxes ([less well-defined than you would
hope][cmark]), especially the relatively new entrant **[MyST]** which emphasizes
technical applications. My perennial favorite
**[TeX/LaTeX](@/2024/what-tex-gets-right.md)** is relevant, but for the vast
majority of users it only targets PDF output, and you’ll note that virtually
everything that I’ve discussed revolves around HTML and the web browser. (To me,
this is *the* fundamental issue holding TeX back in the 21st century.) You can
think of the **[Jupyter notebook]** as a document file format (one that happens
to come with a standard WYSIWYG editor), in which case **[nbconvert]** joins
**[pandoc]** in the category of tools that connect the low-level document file
formats to higher-level systems like Sphinx and mkdocs.

[reStructuredText]: https://docutils.sourceforge.io/rst.html
[MyST]: https://mystmd.org/
[Markdown]: https://en.wikipedia.org/wiki/Markdown
[cmark]: https://commonmark.org/
[Jupyter notebook]: https://nbformat.readthedocs.io/
[nbconvert]: https://nbconvert.readthedocs.io/
[pandoc]: https://pandoc.org/

Whew! I could keep going, too, but I *think* I’ve managed to touch on the major
categories of tools that go into producing software documentation.

Synthesizing all of the above, I think it might be not be unreasonable to talk
about documentation as being produced within *documentation systems*. Here I’m
referring only to the technical implementation, not conceptual tools like
Diátaxis. I’ll claim that these technical systems include four kinds of
technologies:

1. Low-level file formats of individual documents
2. Tools for authoring individual documents
3. Tools for assembling documents into structured collections
4. Tools for publishing (i.e., hosting) such collections

Some tools blur the boundaries between these layers, and you could probably
write a whole PhD thesis on the definition of the word “document”, but I’m going
to claim that if you look at anything that you can call “documentation” with a
straight face, you’ll be able to meaningfully isolate the technologies that are
used for each of these four layers.

*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.
