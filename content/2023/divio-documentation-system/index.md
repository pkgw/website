+++
date = 2023-11-02T11:29:38-04:00
title = "The Divio Documentation System (AKA Diátaxis)"
+++

How do you create high-quality documentation? It’s a question that I spend a lot
of time thinking about. One of the frameworks that I’ve found most useful when
pondering this is something that I’ve been calling **The Divio Documentation
System**, recently renamed to **Diátaxis**.

<!-- more -->

*2023 Nov 3: This entry has been revised since I discovered that this system
has been renamed!*

You can read all about the system at its homepage: <https://diataxis.fr/>. It
was originally published at <https://documentation.divio.com> — the authors
simply referred to their idea as “the documentation system”, but that’s
nigh-unGoogleable and makes it harder to talk about alternatives, so I thought
it was more helpful to call it “the Divio system”. But now it’s been given a new
name, Diátaxis, by the primary author, [Daniele Procida][vurt], who no longer
works at Divio. The Diátaxis site has [a blurb about the relationship between
the two][blurb].

[vurt]: https://vurt.org/
[blurb]: https://diataxis.fr/colofon/#divio-and-diataxis

The core idea of the Diátaxis system is this:

> There isn’t one thing called documentation, there are four. They are:
> tutorials, how-to guides, technical reference and explanation. They represent
> four different purposes or functions, and require four different approaches to
> their creation.

One claim in the Diátaxis page — which I absolutely agree with — is that in most
software projects, the only available documentation is the [**technical
reference**][tr]. These is detailed, authoritative information along the lines
of “function X takes these arguments and returns this result”. Sometimes, this
is exactly what you need! If you’re already familiar with the system in question
and are implementing something and need to remind yourself about how to use
function X, you’re all set.

[tr]: https://diataxis.fr/reference/

But, looking at this through a [UX] design lens, such material only meets the
needs of one particular [user persona][up]. Someone who is *not* familiar with
the software, who *isn’t* in the midst of coding against its APIs, will get
little if anything out of straight reference material. Such a person might need
a [**tutorial**][tut]. Tutorials are helpful for people who know that your
system *might* be something useful to them, but “don’t know what they don’t
know” — they’re not even sure what it really does. You need to take them by the
hand and show how your system solves a problem — any problem. It doesn’t really
matter whether it’s a problem that the reader actually has, or not!

[UX]: https://www.usability.gov/what-and-why/user-experience.html
[up]: https://www.usability.gov/how-to-and-tools/methods/personas.html
[tut]: https://diataxis.fr/tutorials/

In between tutorials and reference, in a certain sense, are [**how-to
guides**][howto]. These are for people who *do* “know what they don’t know” —
they’re familiar enough with the system to believe that it can solve some
problem for them, but they’re not quite sure how to go about it. The overall
“shape” of a how-to might be quite similar to a tutorial, but the user’s
attitude is different — less exploratory, more goal-oriented. Or, perhaps, we
should say that the user’s goal is to solve their specific problem, not to learn
about your system.

[howto]: https://diataxis.fr/how-to-guides/

The final piece of the Diátaxis model is [**explanations**][expl], which they also
call “discussions” and could also be labeled (sigh) “explainers”. I think of
these as a mirror to how-to guides: while both focus on a certain topic, how-to
guides are all about getting something done now, while explainers are all about
general understanding, without a particular goal in mind. They’re the kind of
thing you sit down to read before undertaking a big project in the area, or, if
the writing is really good, even just to relax while pretending that you’re
still doing something productive.

[expl]: https://diataxis.fr/explanation/

The Diátaxis model is really illuminating, I think. It’s a good taxonomy and it
really helps start you thinking about the different needs of different users. It
reminds me of the *very* early days of [Rust], when one of the innovations was
to have different sigils for different “kinds” of pointers — one of those aha!
ideas that kicks off lots of useful thinking. I’ve found it super helpful.

[Rust]: https://rust-lang.org/

That being said, I also have a feeling that we can dig a little deeper
here. The Diátaxis model organizes its kinds of documentation into quadrants:

{% relfig(path="divio-quadrants.jpg") %}
The Divio system quadrants, from [documentation.divio.com](https://documentation.divio.com/).
Diátaxis gives the same arrangement.
{% end %}

To me this feels pretty good, but perhaps a victim of the well-known phenomenon
of “we have four things, so let’s put them into quadrants”. Are reference
materials really about “theoretical knowledge”? Is there really a firm boundary
between tutorials and how-tos? I’m not sure if this organization actually adds
anything to our thinking here, although the axes of “studying – working” and
“theoretical – practical” do seem like useful ones to keep in mind.

Whenever I see a taxonomy, I also start thinking about the things it *doesn’t*
cover. Thinking about my own experiences writing documentation, one thing that
the Diátaxis system doesn’t address is what we might call [finding aids][fa].
Imagine that I have a wonderful corpus of documentation covering everything:
tutorials, how-tos, explainers, and reference information — for every reader,
we’ve got exactly what they need. But we still need to get them *to* the right
document. Presumably this collection has some kind of landing page that will be
many visitors’ first entry into the corpus. What do we put there, and how do we
design it to get them to the document they need as quickly as possible?

[fa]: https://en.wikipedia.org/wiki/Finding_aid

Relatedly, more and more I think that we could be doing a lot more with
*indices*. Here, I mean not just “here’s a list of every interesting term in
this book” — in the digital world, those kinds of indices are basically
superseded by search — but any kind of organized List Of Stuff that’s actually
rewarding to browse. Think of the experience of scanning a library shelf and
stumbling on exactly the book you need. Or, I always wish that my cocktail
cookbooks had four or five indices. “What are all the drinks that use Aperol?”
“What are all the drinks that have cozy vibes?” “What are the other drinks
credited to [Forbidden Island]?” Something that really bothers me is that it
feels — this probably isn’t strictly true, but it *feels* — like browsing these
lists is only fun in the physical world, not on a screen. Surely, surely, things
don’t have to be that way, though.

[Forbidden Island]: https://www.forbiddenislandalameda.com/

Finally, imagine going above and beyond the Diátaxis taxonomy. If we really want to
educate people about something, we probably have videos, podcasts, master’s
programs. How do those fit in?

I don’t know. But all of these questions make me feel like there’s lots of
useful work to be done in this area. My thoughts are still fuzzy, but it seems
to me like maybe it is more useful to think about the problem — how we *inform
and educate* people about a topic — rather than one particular solution —
written texts that we call “documentation”. It’s a little scary, though, because
“how to educate people” turns out to be a pretty challenging problem!

If we’re planning to “document” something, then, maybe the first step is to
think about our *information strategy* (education strategy?) — who are we
informing? How will do we best inform them using the resources we have
available? Once we’ve thought about those questions, hopefully we’ll have
clarity about which kinds of projects — writing documentation, making videos,
shouting in the street — are called for. Depending on the amount of resources
available, we may or may not cover the four Diátaxis quadrants in our written
materials, but hopefully we’ll at least be consciously choosing what we *do*
provide.
