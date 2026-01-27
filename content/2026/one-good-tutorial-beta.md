+++
date = 2026-01-21T22:18:48-05:00 # deploytool
title = "One Good Tutorial: Beta Release!"
+++

I’m delighted to announce that a beta-testing version of my scientific software
documentation resource, [One Good Tutorial][ogt], is ready for your feedback!
Check it out at [onegoodtutorial.org][ogt].

[ogt]: https://onegoodtutorial.org/

<!-- more -->

*(You might have noticed that my blogging pace has fallen off a cliff. Having a
kid will do that to you! I aspire to ramp things back up, but it’ll probably be
a little while …)*

One Good Tutorial is the guide that I’ve been developing over the past ~year
thanks to the support of [a Better Scientific Software (BSSw)
Fellowship](@/2024/bssw-fellowship.md). My proposal was all about helping
maintainers of small-to-medium scientific software projects create better
documentation with more confidence, and in [my last
update](@/2025/one-good-tutorial-plan.md), I laid out my first-draft vision for
what I would build.

Based on some initial work [surveying](@/2025/state-of-the-docs.md) [the state
of the field](@/2025/state-of-the-doc-tools.md), I came to some decisions about
where I wanted to direct my focus. First, I wanted to hammer my target audience
over the head with the idea that there’s more to docs than API docs (hugely
influenced by [Diátaxis][dtx] here). Second, I wanted try to help out people who
want to do a good job with their docs, but don’t really know how to get started.

[dtx]: https://diataxis.fr/

My initial concept was a “checklist matrix” supported by more detailed guides.
The idea was that a checklist would help give a sense of accomplishment and
manageability: if your documentation includes everything mentioned in the
checklist, you’ve done a good job. The “matrix” component was an effort to guide
people into a process for planning out how they’d write all their docs, rather
than just asking them to sit down and start scribbling.

In [the beta version][ogt], I tweaked this design slightly. There’s still a
checklist, front-and-center on [the landing page][ogt]. But rather than show
this checklist as a matrix, I lay out a structured approach in [a
“playbook”][pb], a suggested workflow of about 20 steps that aims to guide
people from a (metaphorical) blank page to a filled-out checklist. As soon as I
created my first checklist matrix mockup for presentation at the
[US-RSE'25](https://us-rse.org/usrse25/) meeting last fall, I could tell that it
just had too many boxes, and I never found a way to fix that fundamental issue.
In comparison, I’m much happier with the playbook model. It provides a structure
for tackling the checklist, but avoids overcomplicating the checklist itself,
and I think the framing makes it clear that if you want to tackle the checklist
in some other fashion, that’s more than fine.

[pb]: https://onegoodtutorial.org/playbook/

On the website I deliver the playbook as [an HTML slideshow][pb], following in
the footsteps of [some of my work on
DASCH](https://dasch.cfa.harvard.edu/dr7/introduction/) (and, more broadly, a
hobbyhorse I’ve been riding [for more than a
decade](@/2013/slides-for-scientific-talks-in-html.md)). I’m getting more and
more enthusiastic about HTML slideshows as a very useful form-factor for
pedagogical, web-based documentation; I’m coming around to believing that
they’re the best way to avoid the “wall of text” fatigue that arises so easily
when reading lengthy material on a screen. I hope other people actually agree!

One thing that didn’t change from my initial plan is that in addition to the
checklist and playbook I do indeed have [a series of “in-depth guides”][guides]
offering advice about how to prepare different elements of the documentation
checklist. Some of them aren’t really about writing; two elements on my
checklist are [citation
information](https://onegoodtutorial.org/in-depth/software-citation/) and [a
licensing
statement](https://onegoodtutorial.org/in-depth/licensing-statements/), both of
which are the kind of thing that may only take up a few sentences in one’s final
documentation, but may require a great deal of prep work in order to be able to
write those two sentences. These guides will probably need tweaking here and
there but I’m very happy with how they’ve turned out so far, and I’d like to
think that they might become generally useful resources going forward.

[guides]: https://onegoodtutorial.org/in-depth/

I certainly have ideas about how to make One Good Tutorial even better, but I
can tell that it’s ready to be sent out into the world for beta-testing. So,
here we are! I’ll be reaching out to some folks individually, but if you’re
reading this and and you have some experience with documenting scientific
software, I’d love some feedback. You can get in touch via [the One Good
Tutorial repository](https://github.com/pkgw/onegoodtutorial/) on GitHub or
by [contacting me directly](@/about-me/index.md#contact).

Over the last few months of my fellowship I’ll be polishing the website and
working to get the word out. I’m quite proud of how this project has come
together so far and I hope you find it useful! Check out the One Good Tutorial
materials at [onegoodtutorial.org][ogt].

*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.
