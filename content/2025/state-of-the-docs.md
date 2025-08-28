+++
date = 2025-08-21T00:00:00-04:00
title = "The State of the Docs"
+++

As part of my [BSSw Fellowship](@/2024/bssw-fellowship.md) project on scientific
software documentation, I’ve conducted free-form interviews with some of my
colleagues to try to learn a little about how working scientists are approaching
research software documentation in the year 2025. In this post I’ll report my
findings.

<!-- more -->

But first, some disclaimers. I’m describing an *extremely* modest, qualitative
undertaking. While I very much enjoyed my conversations, and so have plans to
interview a few more people (reach out if you’d like to volunteer!), at the
moment my sample size is *n = 4*. Even if that sample size were a lot bigger,
I’m not doing structured interviews here (nor am I trained to do so), so this is
far from rigorous in any sense of the word. I’ve also tried to pursue a group of
interviewees that’s relatively diverse, but … again, we’re talking about four
people here. Three of them are astronomers, and of course the set of people
willing to sit down and talk to me is a biased subsample of the universe of
people working with research software. All this being said, I suspect that the
patterns I’m noticing are likely to generalize.

If I had to choose one word to summarize how my interviewees felt about
scientific software documentation in general, I’d go with *dissatisfaction*. I
think it's fair to say that everyone I talked to has a real appreciation for
good documentation; but no small amount of that appreciation stems from its
rarity. Good documentation is hard to find. Part of the reason for that is
intrinsic: it’s hard to write well! But, based on my interviews, I’m not alone
in feeling that there are a lot of extrinsic factors holding things back,
especially authoring tools that are limited and limiting.

In that context, I was a little bit surprised that none of my interviewees
complained about documentation being *unappreciated*. A perennial — and
completely valid — complaint from scientific software developers is that they
don’t get the recognition that they deserve from their peers. I had expected
that at least one or two people would report that they were discouraged from
spending time on docs because it seemed like the results weren’t being valued.
But no one offered that sentiment, although I didn’t make a point of trying to
elicit it either. Nor did anyone bring up the challenges of making documentation
work legible to the traditional academic reward structures (e.g., making docs
citeable), which is another preoccupation when it comes to research code. It
could be that, given a baseline expectation that time spent on research software
is going to be underappreciated in general, a lack of appreciation for time
spent on software docs goes without saying.

In a similar vein, only one interviewee specifically mentioned the connection
between funding and software documentation, or the lack thereof. I’m confident
that the potential impact of funding is obvious to everyone involved; it’s just
that the prospects for any kind of systematic financial support for software
documentation feel grim at best, at the moment. I can imagine the contours of a
pitch to change this: fundamentally, documentation is education, and education
deserves investment, right? Well, lately there’s less agreement on that point
than I’d like. But I’d be happy to argue that most students would learn not just
as well but actually *more* effectively from great software documentation than
from a great textbook: it’s hard to beat learning by doing. Anyway, maybe one
day there’ll be somebody to listen to this pitch, but I’m not holding my breath.

The lack of systematic support — both financial and less tangible kinds — surely
has much to do with another theme that I noticed: a lack of a [community of
practice][cop] around research software documentation. It feels (emphasis on the
*feels*) like everybody’s out there on their own, figuring things out
independently. While some of my interviewees mentioned learning how to create
better documentation from looking at *other docs*, no one gave much indication
that they felt that they had learned much from *other people*.

[cop]: https://en.wikipedia.org/wiki/Community_of_practice

This is another result that can be filed under “Absolutely Zero Surprise
Involved”. But I do want to emphasize that there are people out there actively
trying to create exactly these communities. The two groups that come to mind
immediately are [Write The Docs][wtd] and [The Good Docs Project][gdp], the
latter of which I only learned about recently. I’m sure there are more, and
everything I’ve seen suggest that the people in these organizations are doing
exactly what they ought to — it’s just that this kind of community-building is
slow, tedious, *difficult* work. It’s also worth pointing out that there is a
*much* bigger world out there once you start considering the field of technical
writing in general, as opposed to the narrower group of
scientists-doing-software-docs-amateurishly. (On the other hand, I was going to
link to the Society for Technical Communication as an example, but I see that
[they closed their doors earlier this year][stc-out].)

[gdp]: https://www.thegooddocsproject.dev/
[wtd]: https://www.writethedocs.org/
[stc-out]: https://www.reddit.com/r/technicalwriting/comments/1id0m5d/stc_is_gone/

Getting a bit more pragmatic, one high-level idea that I took away from my
interviews is that there are basically two levels of documentation. At the lower
level, we have the things that we can think of as, approximately, individual
documents: a single tutorial, or the API reference for a single Python package.
At the higher level, we have document collections: a whole suite of tutorials
and API references, design docs, and so on. At both levels, we face challenges
relating to design, organization, and tooling, but the details of those
challenges are actually quite different. Smaller projects are basically
operating only at the lower level, but for people working in larger projects, it
seems that attention quickly migrates to primarily focus on the high-level
issues, rather than the details of individual documents. For my BSSw project, I
plan to primarily address the lower level — preparing individual documents — but
I came out of my interviews feeling that it’ll be helpful to draw a line between
the two levels (even if it is, inevitably, fuzzy) and offer some guidance about
how to tackle the higher-level issues.

Another theme from my interviews was interest in experimenting with different
documentation “form factors”. I think that Jupyter notebooks came up in all of
my discussions, and everyone was pretty much on the same page in feeling that
they can be a very good vehicle for certain kinds of software documentation. I
brought up videos in a few interviews, because I’ve been struck by the seeming
trend in industry to produce video documentation for *everything*, even cases
where it feels like a few paragraphs of text would be more than sufficient. I
still don’t quite understand where that’s coming from — and none of my
interviewees seemed to see any special appeal either. There are cases where
video documentation can be very helpful (as I’ve experienced with [WorldWide
Telescope][wwtdoc]) but, given that it takes a lot of work to produce, my
interviewees’ conception of “documentation” still centers on text.

[wwtdoc]: https://docs.worldwidetelescope.org/

One interviewee was sympathetic to an idea that’s been bouncing around in my
head: perhaps we should be making a lot more use of slide decks for
documentation. There’s no need to belabor the ways in which they can go wrong,
but at their best, slide decks can break down material and interleave text and
graphics in a way that seems a lot more digestible than a linear document. And,
as I’ve harped on for, gosh, [more than a
decade](@/2013/slides-for-scientific-talks-in-html.md), you can make nice HTML
decks that can be viewed without the need to “context switch” into a dedicated
app. I tried this approach [in the DASCH documentation][daschdoc] and am
personally pretty happy with the results, although the scale of deployment is
small enough that I don’t have any firm feedback about what anyone else thinks.
One of the challenges for this idea is that authoring these decks is a bit
tricky, although maybe most people would be satisfied with links to Google
Slides or something along those lines.

[daschdoc]: https://dasch.cfa.harvard.edu/dr7/introduction/

Attention to authorship barriers — how hard or easy is it to just sit down and
write something? — is looming large in my thinking as a result of these
interviews. One interviewee was adamant: write docs using whatever tool you have
at hand; the best doc is one that actually exists. Others were focused on
getting the right tooling into place: setting up CI to run doctests (using a
tool like [Sybil]) ensuring that they never break, or to automatically execute
Jupyter notebooks and integrate them into a Sphinx tree. Both of these attitudes
have their appeal, but there’s definitely a tension between them.

[Sybil]: https://sybil.readthedocs.io/

I’m not quite sure how to navigate this tension in the resource that I’ll be
creating. My personal bias ought to be pretty clear: whatever the opposite of
quick-and-dirty is, that’s usually me. Turns out, though, that most people
aren’t like me. I’d like to think that my resource can offer people some
cookbook recipes to help them get the benefits of some of the more complex tools
(it’s good to verify that your example code actually runs!) without getting
mired in a slog of Sphinx configuration, but I’ll have to be careful not to go
overboard.

*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.