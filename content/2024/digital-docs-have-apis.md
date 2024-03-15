+++
date = 2024-03-14T10:25:41-04:00
title = "Digital Documents have APIs"
+++

Following up on last week’s viral sensation [Digital Documents are Web
Applications](@/2024/digital-docs-are-web-apps.md), I want to add another idea
into the mix. Here’s an additional thing to keep in mind about digital technical
documents: yes, they’re incarnated as web applications. But unlike many web
applications, they also expose nontrivial APIs — by which I mean that they
expose interfaces aimed not just at *humans* (the user interfaces) but ones
aimed at computers as well. In particular, the primary API exposed by a digital
technical document is its cross-referencing structure.

<!-- more -->

Consider the reference documentation for a Python library like [Numpy]. It
describes various symbols exposed by the package, like [`numpy.hstack()`]. If
I’m documenting my own Python library, I might want to reference that piece of
documentation from my own text. Any reference like this implies a sort of
contract — I’m expecting, or at least hoping, that the URL embedded in the link
above will deliver you to documentation about the [`numpy.hstack()`] function
and that it will continue doing so into the future. You could quibble about the
terminology, but I’m happy to refer to any sort of contract between two distinct
digital systems as an API — and that’s what we have here. (The bare word
“interface” might be better, except that I think it too easily conjures up the
idea of a *user* interface, which is specifically not what I want to focus on.)
My document depends on a thing that I can call “the Numpy docs”, and in
particular it depends on the Numpy docs providing a thing called “the
`numpy.hstack()` docs”.

[Numpy]: https://numpy.org/
[`numpy.hstack()`]: https://numpy.org/doc/stable/reference/generated/numpy.hstack.html

The most fundamental API offered by any web document is its URL structure: what
links into it are valid? Through this lens we can see [linkrot] as a sort of API
break, which feels right to me: you told me that I could rely on this thing, and
now you’ve gone and taken it away. Please don’t do that!

[linkrot]: https://en.wikipedia.org/wiki/Link_rot

URLs, however, are semantically opaque in the aggregate. And I’d argue that one
characteristic of technical documents is that they tend to expose *multiple*
referencing structures that wish to be *semantically rich*. Think of a long
scientific article. Discussing its contents, I might want to refer to Figure 1,
Figure 2, Figure 3; or Table 1, Table 2, Table 3; or Equation 1; or Section 2;
or Reference #3 (if its references are numbered); and so on. While each of these
references can likely be resolved — flattened — into a specific URL in the case
of a digital article, as an author I want to work at a higher level.

But the phrase “want to” isn’t strong enough. In the technical sphere, the
ability to make reliable, semantically-rich references among documents is
*essential* functionality. People usually agree that scientists use TeX because
of its ability to render equations, but I’ll claim that [BibTeX] is just as
important. The ability to write `\citep{latest.astropy}` and get a properly
formatted reference (both in-text and metadata in the References section at the
bottom) is *huge*. I’m happy to go even farther and argue we can think of the
institutional architecture of worldwide academic publishing is being largely
designed to promote reference-ability. If you hand me a journal name, volume,
and page number, I can probably locate the article that you’re talking about,
even if it was published a century ago; contrast that with the durability of
your typical web link. I’ve seen people talk about how impressed they are by how
academics do such a good job of citing prior work, and I think that you can
explain a lot of that as being because *we have designed our entire profession
to make this kind of citation robust and convenient*. The framework of citation
allows us to build a (mostly) coherent intellectual edifice out of individual
workers’ labor.

[BibTeX]: https://www.bibtex.org/

And we see the same phenomenon in software! As programming languages have
evolved, they’ve tended to provide increasingly sophisticated and reliable
systems for expressing and resolving dependencies between independent pieces of
software: from downloading libraries from random websites, to [CPAN] to [PyPI]
to [NPM] and [Cargo]. Just as in the case of scholarship, these affordances
unlock scalability. While it’s hard to build a C project with more than a
handful of external dependencies, Python projects easily have dozens, and a Rust
project like [Tectonic] has hundreds. The key distinction between software and
scholarship is the semantic richness of the interfaces between items. Scholarly
citations are famously about as empty as you can get: if you write a paper
saying “Williams *et al.* (2020) is garbage” or “Williams *et al.* (2020) is
brilliant”, it’s a citation either way. Meanwhile, the APIs that capture the
relationship between software components are complex and getting moreso all the
time: from function prototypes and header files in C to complex type systems
with public/private visibility annotations in languages like Rust or Go.

[CPAN]: https://www.cpan.org/
[PyPI]: https://pypi.org/
[NPM]: https://www.npmjs.com/
[Cargo]: https://crates.io/
[Tectonic]: https://tectonic-typesetting.github.io/

Technical documentation lags behind all of this. Within the framework of a
specific programming language, cross-referencing is usually well-supported:
tools like [Intersphinx] enable this in Python, and [rustdoc] has it built in.
But if I want to reference a Python method from the documentation of a Rust
project? Or if I want to link to a particular passage of an online manual in a
way that will keep working when the manual’s authors inevitably change their URL
structure? I’m on my own.

[Intersphinx]: https://docs.readthedocs.io/en/stable/guides/intersphinx.html
[rustdoc]: https://doc.rust-lang.org/rustdoc/

For all of the reasons given above, I think that the lack of infrastructure in
this area really limits our ability to create great digital technical documents.
When it comes to declaring and resolving dependencies, we probably have the
baseline of what we need — namely, URLs and the web. But right now we don’t have
the tools to make explicit the “APIs” (cross-referencing structures) exposed by
documents, and in my view, this lack makes it so that we don’t do a good job of
rationalizing those APIs or monitoring compatibility. Further, it prevents us
from building authoring tools that can span multiple technical silos. This feels
to me like a very solvable problem.
