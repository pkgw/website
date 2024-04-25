+++
date = 2024-04-25T12:02:13-04:00
title = "A Modest Proposal for How to Do Digital Conference “Posters”"
+++

Apropos of nothing in particular (really), here is how conference organizers
should handle “posters” when they need to be digital, either because the
conference has a virtual component or there’s no standard poster hall.

<!-- more -->

# The Proposal

1. The digital equivalent of a traditional conference poster shall be called a
   “six-pack”.
2. It shall consist of a slide deck containing no more than six slides.
3. There are no other rules.


# Discussion

The “conference poster” form factor for scientific communication is not a good
one. How many posters have you seen that are hard to navigate at a high level
and impossible to read in detail? Some of this could be called “operator error”:
certain posters are designed with a lot more care than others. But I generally
find that even the best-designed posters are a bit dissatisfying, and I think is
indicative of some basic difficulties imposed by the form factor. Posters are
too large; printed text almost always comes in pages about the size of your face
because that’s what matches the human field-of-view at normal reading distances.

This is not to imply that the time-honored in-person *poster session* isn’t
useful. And if you’re going to have a traditional in-person poster session, the
posters are definitely important — there’s a reason we don’t have people just
standing around in assigned spots with nothing to point at. But on the other
hand, if we’re seeking to extend the poster-session experience into the digital
realm, there’s no particular reason to [cling onto the poster form factor][1].
[Skeumorphic designs for digital artifacts are bad][2], usually.

[1]: https://ipostersessions.com/
[2]: https://andrewberls.com/blog/post/trouble-with-skeuomorphic-design

So, what are the functions being fulfilled by a traditional printed poster?
Loosely speaking, it’s tangible: a visitor can benefit from it even when you’re
not physically standing next to it. And it’s accessible, in the intellectual
sense: it should help someone quickly get the gist of the work you’re
presenting, even if they’re not especially familiar with its context.

I’ll assert without proof that in the digital-native realm, when one wants to
fulfill these functions, there’s one form factor that *clearly* reigns supreme:
the slide deck. Sometimes I feel like this fact ought to bother me, but it
doesn’t, really, nor can I figure out why it should. There are certainly times
when people give the PowerPoint treatment to a complex topic that really should
be worked out carefully, but that’s not PowerPoint’s fault; and there are surely
just as many cases where complex topics deserve accessible, high-level
summaries! A poster session is exactly one such case.

(Drive-by hot take: [literate programming][lp] as implemented by Knuth is
basically a way to PowerPoint-ify source code.)

[lp]: https://en.wikipedia.org/wiki/Literate_programming

This sets the stage for the six-pack idea. A slide deck is the right,
digital-native form factor for delivering an accessible, tangible summary of a
research project. But, as anyone who’s been to a large conference knows, some
people just can’t help themselves: tell them to prepare a poster, and they’ll
give you wall-to-wall text in ten-point font. You can’t completely prevent this,
but setting a limit on the number of slides is an easy and extremely concrete
way to enforce some level of compactness.

Why six slides? Partially because it seems like about the right number, but
mainly because then you can call the slide deck a “six-pack”, which is
(hopefully?) catchy.

You could try to also set rules about font sizes or slide sizes (some people use
PowerPoint to prepare full-size posters — it can work well as a page-layout
tool), but it doesn’t seem worthwhile. If someone wants to go overboard, they’ll
find a way, and more and more I feel that it’s important to lean into the
ephemerality of posters or six-packs: they simply aren’t important enough to be
worth the effort to rigorously prevent that. Likewise, I don’t see a need to try
to set down rules about ”the first slide shall be a title slide,” or whatever —
most people will do something sensible, and in this case it’s better to have a
simple set of official rules than to try to legislate common sense.

What I *would* try to legislate, if I were running a conference that involved
“poster sessions” with six-packs, would be the choice of file formats. I’m
fairly optimistic that we’ll collectively have the capability of viewing
PowerPoint and Keynote files from now until the end of time, but not everyone
“attending” a digital conference is going to be able to open them. Six-packs
should be provided in PDF format for long-term archiving, and either PDF or HTML
for cross-platform viewing. [I love HTML slide decks][3], and I think they’re a
perfect match for a “virtual poster session” scenario, but I don’t think it’s
realistic to expect most people to be able to author them, just yet.

[3]: @/2013/slides-for-scientific-talks-in-html.md