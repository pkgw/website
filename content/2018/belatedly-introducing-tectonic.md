+++
date = 2018-10-09T10:02:27-04:00
title = "(Belatedly) Introducing Tectonic"
+++

Even though I haven’t been blogging for the past year, it’s not like I haven’t
been busy! I’ve been doing a lot of astrophysics, but if I’m honest, one of
the things I’ve been working that I’m *most* excited about is a bit to one
side — or, maybe better, a lot *bigger* than just astrophysics. It’s a project
I call [Tectonic](https://tectonic-typesetting.github.io/).

The capsule summary on the website is:

> Tectonic is a modernized, complete, self-contained
> [TeX](https://www.tug.org/)/[LaTeX](https://www.latex-project.org/) engine,
> powered by [XeTeX](http://xetex.sourceforge.net/) and
> [TeXLive](https://www.tug.org/texlive/).

What does that actually mean and why do I think it matters so much? I think
the best explanation involves stepping back a bit.

<!-- more -->

I’m very interested in how complex, technical ideas are communicated —
especially in written documents, which are the most efficient mechanism ever
invented for getting new knowledge into a person’s brain (sorry,
[Khan Academy](https://www.khanacademy.org/)). In particular, I find it
infinitely frustrating that scientific and technical documents have *barely
changed* in the quarter-century since information technology really started
transforming how people communicate with one another. The “highest” form of
scientific communication, if you will, is still the journal article — a model
rooted in the time when there was no alternative to print, of course — and
scientists still primarily read articles as PDF files, formatted in a dense
two-column layout intended for printing onto little rectangles of paper. Don’t
get me wrong: this system works very well, and has worked well for a long
time. But, if nothing else, modern screens (your smartphone, your tablet) are
a *strictly* more powerful display technology than the printed page. We should
be able to use that power to communicate complex ideas more effectively than
we could before.

And, of course, we can: there’s a reason that things like
[Jupyter notebooks](https://jupyter.org/) have become popular,
[and I’ve explored using them pedagogically myself](@/2014/elementary-gaussian-processes-in-python.md).
But when it comes to “high-end” scientific communication — journal articles —
our technologies are still oriented around the print paradigm.

I am convinced that this state of affairs is largely due to a technology gap.
We have the display technologies: both physical screens, and the software
systems to create amazing interactive graphical experiences (namely, web
browsers). But we don’t have the technology to *create* technical documents
that leverage these display technologies.

Why is that? Technical documents have some authoring requirements that don’t
come up for the kinds of documents that fill the modern Web:

- You need to be able to *easily* create equations. They have to be capable of
  both being placed *inline* in text and in display equations, and they need
  to be *beautiful*. I think people underrate this last factor. Typographic
  quality is one of those things in life, like zoning regulations, that
  absolutely shapes the human experience and yet somehow operates in a way
  that most people don’t even realize it’s at work.
- You need to be able to *easily* cross-reference documents in a durable way.
  Obviously, the Web has hyperlinks. But they can break, and the information
  content of a link itself can be marginal or nonexistent. (What does
  [https://tinyurl.com/2fcpre6](https://tinyurl.com/2fcpre6) link to?) The
  very fact that academic writing persists in using its elaborate referencing
  schemes, and hasn’t just switched to using URLs for everything, demonstrates
  how important this functionality is.

Technical documents are also demanding on other axes — the typography of the
main body text (not just the math) should be excellent, and high-quality
graphics are a must. But non-technical documents are also demanding of these
things.

My claim is that the quality of 21st-century scientific communication is
genuinely and substantially limited by the fact that we don’t have the tools
to author technical documents that can meet the above requirements.

Except, of course, we do.

We have [TeX](https://www.tug.org/).

TeX is *the* de facto software system for scientific typesetting. Even though
it’s literally 40 years old (as of this year), many working scientists still
use it as their primary tool for creating high-end technical documents. I
argue that they still use it precisely because it’s the only freely-available
tool that lets you easily author beautiful equations and correct
cross-references — the latter relying on the fact that with a
[BibTeX](http://www.bibtex.org/) database you can just type
`\citep{williams.2017}` and all of the necessary detailed cross-referencing
information will automatically be inserted into your document.

So what’s the problem? The problem is that TeX was designed for the printed
page — and that its design is exceedingly baroque, such that it is extremely
hard to update it to keep up with evolving typographical and display
technologies. Furthermore, I’ve found that both TeX’s implementation and its
developer community are *extremely* resistant to change. The original code is
[written in a language that, for all intents and purposes, nobody uses](https://en.wikipedia.org/wiki/WEB),
[modifications to the original source code are legally questionable](https://www.latex-project.org/lppl/lppl-1-2/#conditions-on-distribution-and-modification),
[the development and distribution practices are old-fashioned](https://ctan.org/),
and there’s an enormous emphasis on not breaking compatibility with existing
work. The last of these is a value that I absolutely cherish and respect, but
is not well-suited to seizing the opportunities presented in this era of rapid
change in communications technology.

The vision of Tectonic is to have the best of both worlds. Take the proven TeX
engine, but modernize it. Rewrite the internals in better languages, using
modern development practies. Break compatibility when old practices get in the
way of improving things. Start emphasizing
[user experience](https://en.wikipedia.org/wiki/User_experience) as well as
technically correct output. Overall, the idea is to *make technical
typesetting hackable* — in the sense that an interested coder can *easily*
experiment with the software, modifying it to meet their needs. Because if
we’re going to take advantage of the ongoing revolution in communications and
display technology, we’re going to need to evolve our authoring tools.

The proximate goal is to produce outrageously good technical documents for the
Web. Beautiful typography, beautiful mathematics, and an absolutely
fundamental reconsideration of what technical documents should look like on
digital display. *Footnotes make no freaking sense on the Web!* Reference
information should pop up to the side of the main text when needed, not be
relegated to the bottom of the “page”. Anything that has to do with pages at
all, in fact, needs to go — websites are infinitely tall scrolls, not
rectangular pieces of paper.

Based on
[my experience with my earlier Webtex project](/2015/08/the-rotation-period-and-magnetic-field-of-the-t-dwarf-2massi-j1047539212423),
I think that the TeX ecosystem is very close to being able to deliver the
documents that we need and deserve. The ecosystem represents an *enormous*
investment of effort toward the very hard problem of programmatically creating
excellent mathematical typography, and projects like
[XeTeX](http://xetex.sourceforge.net/) have done incredible work on the core
of the engine so that it can handle modern fonts and
[Unicode](http://unicode.org/standard/WhatIsUnicode.html). That’s why I think
it is infinitely wiser to build on TeX, rather than try to build an entirely
new system. But to get across the finish line, I am convinced that you need to
throw away a few promises that the existing TeX projects are unable to break.

At the moment, Tectonic … doesn’t deliver the documents that I dream of. In
order to produce excellent Web-native technical documents, there are a few key
changes that need to be enabled in the core engine, and I haven’t yet
implemented them. However, I’m still *very* enthusiastic about where the
Tectonic project is at today:

1. The engine is much, much, more hackable. The core code is a mess of
   automatically-generated C code, but it’s getting cleaner and cleaner, and
   the project
   [is on GitHub](https://github.com/tectonic-typesetting/tectonic/) and
   generally uses modern, open-source development practices. The outer layers
   are now written in [Rust](https://rust-lang.org/), which is *wonderful*:
   powerful, expressive, and able to compile down to blazing-fast, low-level
   machine code (or [WebAssembly](https://webassembly.org/) …).
2. The engine is now *embeddable*. Existing TeX systems rely on a sprawling
   collection of tools and gigabytes of data files that interact with each
   other in complex ways. Tectonic comes as a single program, full stop. This
   helps hackability and distribution.
3. The user experience is vastly improved. Various old-fashioned behaviors of
   the classic TeX programs have been thrown away, and new code has been added
   to automate steps that were previously tedious.
4. All of the above lead to more *reproducibility* in documents. I haven’t
   mentioned reproducibility yet in this piece, but it’s another aspect to the
   authoring of technical documents that I believe is absolutely essential.
   Many subtle elements of Tectonic’s design have been aimed at allowing for
   reproducible document processing.
5. Finally, the project has attracted a good level of interest, and is
   building a supportive [community](https://tectonic.newton.cx/). I feel
   confident that as the project grows, the community will come to be seen as
   one of its most important assets.

As I mentioned in [my previous post](/2018/website-refresh), the process
of refreshing this very website has gotten me excited to push on the part of
Tectonic aimed at actually producing excellent HTML output. Based on my
experience with [Webtex](https://pkgw.github.io/webtex/), I feel that I have a
good idea of what needs to happen to really get that rolling. I do have this
“day job” that I’m supposed to be doing, but I’m excited to see how far we can
get in the next year!
