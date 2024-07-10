+++
date = 2024-07-09T11:29:08-04:00
title = "LaTeX Can Be The New LaTeX"
+++

There’s a lot of interest in modernizing the tools for scientific and technical
typesetting. Tools like [MyST], [Nota], [Quarto], [Living Papers],
[showyourwork!], and many others are trying to make it easier — well, possible —
to create technical documents that take advantage of the capabilities of today’s
digital environment. Of these systems, different ones aim to work at different
levels (low-level typesetting, vs. complete document authoring systems) but we
can broadly think of them as tools aiming to become “the new LaTeX”.

[MyST]: https://mystmd.org/
[Nota]: https://nota-lang.org/
[Quarto]: https://quarto.org/
[Living Papers]: https://idl.uw.edu/papers/living-papers
[showyourwork!]: https://show-your.work/

I’m not sure if I’ve made the argument in writing before, but I believe that
“the new LaTeX” could, maybe even should, be founded on … LaTeX. Not LaTeX as it
currently exists, but LaTeX as it could be.

<!-- more -->

There are two parts to this argument. The first is that TeX/LaTeX (hereafter
just TeX, to emphasize the underlying language) gets some things right that are
worth preserving; the second is that its problems are fixable. This post is
going to skip the first part, and simply observe that here we are in the year
2024 still using TeX for precision technical typesetting, despite its age and
all of its problems. It is probably the oldest piece of end-user software that’s
still in common usage — TeX predates Unix! It’s worth pondering why TeX is still
relevant, but clearly it’s doing *something* right. A huge amount of human
effort and ingenuity has gone into designing the TeX ecosystem; if we can build
on that instead of throwing it all away, that’s a huge win for everyone.

That being said, TeX as it currently exists absolutely has a ton of issues.
It’s annoying to install. The error messages are inscrutable. The syntax is
fully of booby traps. It only truly targets PDF output. There are a dozen
different ways to do the same thing and you need to be an expert to understand
which is appropriate for your situation. It can feel impossible to dig up useful
documentation on even the most basic commands. And so on, and so on.

The ultimate causes of virtually all of these problems, I’d claim, are that TeX
is simply very, very old, and that from the start the culture of TeX has been
obsessed with stability. Neither of these are intrinsically bad things, of
course! But there are also advantages to being less laden with historical
baggage.

For instance, a major issue with advancing the (La)TeX ecosystem is simply that
the core TeX code has traditionally been nearly impossible to hack on. I often
mention to people that when I first got interested in trying to modify the core
TeX engine, it took me, an expert in this kind of task, something like several
weeks to even figure out where the core source code actually lived, and how to
compile it myself. That’s, like, the fundamental step involved in getting
someone to contribute to your open-source project. If people can’t easily fork
your project and try out changes, then no, you’re not going to streamline your
installation process, or gradually evolve quality-of-life improvements.

The other huge issue that I see is that TeX’s documentation is, to be blunt,
**awful**. People sometimes seem to have trouble honing in on this — I mean,
isn’t LaTeX literally a document preparation system? Aren’t there thousands and
thousands of pages of text been written about every [CTAN] package under the
sun? And yet. Despite all of that effort, I find that existing TeX documentation
generally fails completely at serving my day-to-day needs. I routinely struggle
to pull up convenient, clear reference material about how a certain command
works, or the design of certain fundamental concepts of the TeX language. The
irony is staggering. But we can see how things got to where they are.

[CTAN]: https://ctan.org/

First, there’s absolutely a first-mover penalty at play here. When TeX was
invented, there was no World Wide Web. Software documentation came printed on
paper. So that how the whole ecosystem evolved: targeting print formats. The
inexorable outcome of that is that these days, all of the ecosystem
documentation is locked up in PDFs. They're often very *nice* PDFs, but that
doesn’t do anything to help searchability, cross-linkage, and quick access. Can
you think of any software system created in the past, say, decade whose
documentation is *primarily* delivered in PDF format?

Second, the TeX world’s obsession with stability led to fragmentation: if you
wanted to add a new feature, you had to fork the mainline TeX engine and call it
[ε-tex], or [ptex], or [uptex], or [pdftex], or [xetex], or [luatex], or
whatever else. One of the *many* unfortunate consequences of this is that the
documentation has become both fragmented, and highly conditional. The
information you need might be out there, but associated with a different engine;
or a piece of information might be engine-specific without being labeled as
such; or a really thorough piece of documentation might address a bunch of
engine options but become nigh-unreadable by virtue of being mired in choices
like “if you’re using XeTeX, do *this*; if you’re using LuaTeX, do *that*”.
Documentation authors can’t assume that certain features are available because,
hey, maybe you’re still using Knuth’s original TeX with only 256 registers.

[ε-tex]: https://www.ctan.org/pkg/etex
[ptex]: https://ctan.org/pkg/ptex
[uptex]: https://www.ctan.org/pkg/uptex
[pdftex]: https://www.ctan.org/pkg/pdftex
[xetex]: https://www.ctan.org/pkg/xetex
[luatex]: https://www.ctan.org/pkg/luatex

The longevity of TeX further complicates things. If you want to look up
information about font selection, you might get material that was written before
TrueType was invented. This is a good problem to have, in a certain sense, but
it makes things more difficult for readers. This is especially true when
combined with the ecosystem’s fragmentation and the often baked-in assumption
that key elements will never change, so that there’s no need to plan for
providing multiple versions of documentation.

A particular culprit is [The TeXbook]. The TeXbook is, undoubtedly, an enormous
accomplishment and a stunningly *whole* work, and it remains the definitive
reference for the design and behavior of the innermost aspects of the TeX
engine. But it’s also pretty bad documentation. If nothing else, it took me a
while to appreciate that many things written in The TeXbook are **simply not
true** of modern TeX systems, and haven’t been true for decades; now-fundamental
features added in ε-TeX (1996) just don’t exist, as far as The TeXbook is
concerned, let alone ones added in more modern extensions like XeTeX.

[The TeXbook]: https://search.worldcat.org/title/876762638

The TeXbook has a further problem that I see mirrored in other major pieces of
documentation in the TeX ecosystem. To borrow the terminology of [Diátaxis],
it’s trying to be two kinds of documentation at once: not just a detailed
reference of the precise behavior of the system, but also a comprehensive
explanation of the system design. These are both really wonderful kinds of
documentation to have, but interleaving the two types of material makes the book
hard to work with. Descriptions of behavior are scattered around the book based
on the overall pedagogical flow; the explanations are repeatedly derailed by
precise definitions. It may feel churlish to criticize the book for being *too
comprehensive* but it is a legitimate usability problem if people can’t find the
information they need, when they need it!

[Diátaxis]: @/2023/divio-documentation-system/index.md

There’s also the basic matter of availability. It is possible to find PDFs of
The TeXbook online, but they’re not supposed to exist. Reading between the
figurative lines, I’m pretty sure that Knuth had a specific business plan: he’d
give the software away for free, but make some money by selling the
documentation. Nothing wrong with that choice at the time, but once again:
notice that this is not how anyone does things anymore. When you actually have
to compete for mindshare, high-quality documentation that’s freely available is
one of the most valuable assets you can have.

All of this is to say: while there may be a large quantity of documentation
about the TeX ecosystem, I find that sadly it’s quite difficult to actually
profit from. This is especially unfortunate since TeX is indeed a very complex
system that, largely due to its age, does some things in ways that are quite
foreign to modern users.

The flip side of this is: imagine if TeX’s documentation was as sophisticated
and cohesive as the underlying software. *It would change everything.* I
sincerely believe that true best-in-class documentation would completely
transform how people feel about using TeX. Instead of being some mysterious
anachronism, full of ancient magic, it would be the quintessential power tool,
sometimes challenging to master but ready to solve the hardest problems you can
throw at it. It would go from dinosaur to thoroughbred.

The [Tectonic] project is, of course, motivated by this vision. At the basic
technical level, much of the Tectonic approach is simply about making the TeX
engine hackable. As elaborated in [the Tectonic TUGBoat article][w22], at the
higher level, the project’s distinctive branding gives us the room to break with
the past when appropriate. I can’t deny that launching a new brand only worsens
the fragmentation issue, but I think it’s a necessary step to unstick everything
else that needs improvement. Thus far the Tectonic project has not provided much
at all in the way of new, high-quality documentation, but that’s something of an
intentional choice — I’m dissatisfied with the currently available tools and am
trying to sketch out a better system in the [tectonopedia] project. Part of that
effort is building a wholly-new set of engine features and LaTeX classes to
generate “native” HTML output — that is, targeting original TeX source documents
aimed solely at producing optimal HTML outputs, instead of trying to generate
good HTML from existing documents.

[Tectonic]: https://tectonic-typesetting.github.io/
[w22]: https://doi.org/10.47397/tb/43-2/tb134williams-tectonic
[tectonopedia]: https://github.com/tectonic-typesetting/tectonopedia

My hope is that Tectonic can help TeX to become a sort of assembly language for
complex documents. In plenty of cases, Markdown will be all you need. But if you
want to create a sophisticated [technical-document-as-web-application][1], it
will probably be one of the final tools that runs before your web bundler,
generating precision-typeset prose components and cross-referencing databases,
and linking them together with all of the other elements of your document:
interactive figures, navigational chrome, and the rest.

[1]: @/2024/digital-docs-are-web-apps.md

You might ask: “why does this assembly language need to be TeX?” That gets to the
question of what TeX gets right — a topic for another post.
