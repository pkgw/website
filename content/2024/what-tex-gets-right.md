+++
date = 2024-07-17T08:22:08-04:00
title = "What TeX Gets Right"
+++

Last week [I wrote about how LaTeX might evolve to stay
relevant](@/2024/the-new-latex.md) to 21st-century technical writing. But
technology has come a long way since the 1970’s. Should we even be encouraging
people to create documents using the venerable TeX language, which was designed
at a time when computers — and computing — were so different than they are
today? This week I want to write a bit about why it’s worth the effort to build
on TeX/LaTeX, instead of starting fresh.

<!-- more -->

(This post is strongly derived from [“Why TeX?”][ttwhy], an explainer I’ve
written as part of my prototyping of the [Tectonopedia].)

[ttwhy]: https://github.com/tectonic-typesetting/tectonopedia/blob/main/txt/explain/why-tex.tex
[Tectonopedia]: https://github.com/tectonic-typesetting/tectonopedia

First things first: I'll happily admit that there are plenty of circumstances
where TeX is not the best solution, and you'll be better off using some other
kind of technology — whether that's [Markdown], [Microsoft Word], pen and paper,
or whatever else. The very notion of “creating documents” is so broad that it
should go without saying that no single system is going to be the best choice
for every situation.

[Markdown]: https://en.wikipedia.org/wiki/Markdown
[Microsoft Word]: https://www.microsoft.com/en-us/microsoft-365/word

That being said, my work on Tectonic is inspired by the belief that despite its
age TeX is still the very best tool in the world for solving certain kinds of
problems. For instance, if you know one thing about TeX, it's probably that it's
good for mathematics. And that reputation is well-earned! A proficient TeX user
can easily write a single line of code to conjure up complex typeset equations.

But what should actually impress you more than long, complex equations are
equations typeset inline with body text, like *y = x²*. Readability requires
that the placement, sizing, and appearance of the math and text symbols all
agree well, issues that can be fudged a bit in “display” equations. TeX is one
of the few tools out there that can get all these things just right. (This blog
is *not* typeset using TeX, but the above equation is also quite simple; I’d say
that it looks OK but not great in my browser.)

But we wouldn't be writing all this verbiage in honor of a finicky math layout
algorithm. Why can't other tools just copy TeX’s algorithms and do math equally
well? My claim is that the real challenge of typesetting mathematics is that
written math is an open-ended, generative visual language, admitting infinitely
varied forms in unique, unpredictable, recursive combinations. TeX can handle
math well not because it got some specific fiddly bits right — although it did —
but because *TeX is itself an expressive, open-ended language*. You need an
open-ended language to be able to reproduce the open-endedness of written math.
Other tools give you building blocks; only TeX gives you the machinery to create
and use new blocks of your own. (This is not an equivalent statement, but for
what it’s worth, the TeX language is [Turing-complete].) And your own “blocks”
can be just as easy and natural to use as the built-in ones. This is far from
the only reason that TeX is good at what it does, but it's the most essential
one.

[Turing-complete]: https://en.wikipedia.org/wiki/Turing_completeness

The power of TeX manifests itself not only in low-level ways, such as math
typesetting, but in higher-level ones as well. For instance, scholarly documents
have detailed conventions for handling bibliographic references. Although
neither the core TeX language nor Microsoft Word have any built-in, structured
way to represent reference metadata, TeX has been extended to support them in
the [BibTeX] framework. BibTeX's new commands don't feel like awkward
extensions: they integrate straightforwardly with the rest of the language and
are intuitive for users. While it's true that you can manually typeset your
references in Word and assemble them into a bibliography, it's fair to say
BibTeX provides a fundamentally more powerful way to work with them. I’ll wager
that most people who have gotten the hang of BibTeX would *hate* the idea of
giving it up and going back to managing their references manually.

[BibTeX]: https://en.wikipedia.org/wiki/BibTeX

Now, if you’re a healthy skeptic of abstraction, you'll likely respond: “whoa, I
don't want some elaborate system that can do anything — I just want a tool that
helps me get the job done”. This is the right response! Human lifetimes have
been wasted on the refinement of elegant but useless ideas, and we have
deadlines to meet. But hopefully that you'll agree that in some situations, a
system of abstraction is exactly what you need to get the job done. Try doing
physics without calculus.

I'll assert, but can't possibly prove, that once you stop accepting the
limitations that less powerful tools impose on you, you’ll start seeing
opportunities to use TeX's capabilities everywhere. Many kinds of documents have
a sort of “internal logic” that becomes easier to express given the right tools.
That being said, the ones where TeX's capabilities generally add the most are
ones where this internal logic is easy to find: documents with lots of
cross-references, figures, tables, equations, or code — the things that I call
technical documents. And it’s likely no coincidence that these are the sorts of
documents that TeX has historically most often been used to create.

It’s worth emphasizing that this “internal logic” of certain documents can be
open-ended and generative in the same way that written math notation is. For
instance, I often find in API documentation that certain software frameworks
introduce new conceptual structures that don’t exist in the underlying
implementation language. One example would be the notion of store
[state][pinia-state] introduced in the [Pinia] framework. You can write API
documentation that discusses this state in terms of concrete
JavaScript/TypeScript statements, but it’s really a new concept that deserves to
be documented as a “first-class” object on equal footing with other pre-existing
language concepts like classes and methods — I should be able to reference “the
`isAdmin` state variable of the `Server` store” in a convenient and natural way.
A documentation framework needs to give you the tools *to give yourself the
tools* to do this.

[pinia-state]: https://pinia.vuejs.org/core-concepts/state.html
[Pinia]: https://pinia.vuejs.org/

I’ll also boldly claim that despite its internal sophistication, TeX is easy to
start using. I can't deny that TeX has a reputation for confusing output and
sometimes inexplicable behavior, or that there are reasons that this reputation
is deserved. Nevertheless, I'll point out that many mathematicians and
scientists who do not care *at all* about its guts successfully use it for
everyday work, even if it drives them off the wall at times. You can think of it
as being a bit like [git] in this way.

[git]: https://git-scm.com/

Now, it’s certainly possible that one could develop a new, generative
typesetting language that captures the virtues that I’ve discussed above *and*
is free of TeX’s historical baggage. If you asked me to take on that task, I’d
ask for … maybe a decade to do it? Designing a nicer syntax is one thing;
building a whole new ecosystem is another. TeX may be old, but by the same token
it is *battle-tested* and amazingly reliable — its parser can recover from the
most pathological, hateful input documents that you can conceive of. While this
kind of robustness often comes at a performance cost, TeX is *fast* and
ingeniously efficient. It is *supported* by a worldwide community of users, who
have gone to incredible lengths to modernize it and develop a dizzying array of
extension packages and supporting software tools. A lot of very smart people
have put a lot of effort into this language, which is still going strong after
forty years — and those facts tell you something important.

That is not to imply that today's TeX is perfect — [far from
it](@/2024/the-new-latex.md). The error messages are famously hard to
understand. Its documentation is, ironically, a mess. Indeed, a major premise of
[the Tectonic project][tt] is that some aspects of the TeX ecosystem are in need of
dramatic change. But not all of them. TeX *can be* the document language that
the 21st century deserves.

[tt]: https://tectonic-typesetting.github.io/
