+++
date = 2024-06-03T17:22:09-04:00
title = "Software Indexing and Discoverability"
+++

Scientists have a thing about indexing their software. That is, a lot of them
seem to think that it’s important for someone to maintain a big list of All of
The Software for them to look at. (You could call this an “index,” or a
“registry,” “catalog,” etc.; as far as I’m concerned these terms are all
interchangeable.) This impulse is interesting to me because it’s basically
absent in, say, the broader open-source software community. Why is that?

<!-- more -->

To pick a few concrete examples, in astronomy, we have the [Astrophysics Source
Code Libary][ascl] (ASCL); for exoplanets there’s NASA’s [EMAC][emac]; in
heliophysics there’s the [PyHC Projects list][pyhcp]. NASA has a general
[software catalog][nasasc]. There’s a generic [Research Software
Directory][rsd]. It was even quite easy for me to find at least one [“Awesome
Research Software Registries” list][arsr], which is essentially an index of
scientific software indices.

[ascl]: http://ascl.net/
[emac]: https://emac.gsfc.nasa.gov/
[pyhcp]: https://heliopython.org/projects/
[nasasc]: https://software.nasa.gov/
[rsd]: https://research-software-directory.org/software
[arsr]: https://github.com/NLeSC/awesome-research-software-registries

Meanwhile, if I told someone in the open-source world that I wanted to make a
list of all of the open-source software, I’m quite confident that I’d get two
chief reactions: “why” and “also, that is obviously impossible”. To the extent
that one can make any generalizations about large and diverse communities, the
mindset is just very different. The prototypical scientist, if they think about
this at all, is attracted to the possible upsides of having a good index without
necessarily dwelling on the work needed to construct and maintain one, while the
prototypical developer does the opposite.

To cut to the chase, I’m generally on what you might call the “developer” side
of things: there are a lot of attempts to index software that, frankly, I don’t
feel are very valuable. But I don’t feel that way about all of them! To me, this
signals that it’s important to try to think carefully about *when* and *why*
these kinds of indices can be useful and successful. It’s an undertheorized
topic, from what I’ve seen.

A commonly-cited motivation for creating one of these indices is to enhance
software “discoverability”. For example, the [NASA HPOSS solicitation][hposs]
specifically calls out “advancing access, curation, and discoverability of
research software” as a priority of the NASA Open-Source Science Initiative. The
rest of this post is going to focus on just this one aspect, but of course there
are a variety of others at play — I’ll probably write more about them in the
future.

The discoverability angle has always puzzled me. They know that we have Google,
right? Presumably there’s a feeling that general-purpose search doesn’t meet the
needs of researchers, but I haven’t seen any arguments to that effect explicitly
articulated.

Signal-to-noise in search results is a possible factor, but I find that software
packages are generally quite “Google-able”. It’s also true that domain-specific
indices can include faceted search features not available in general-purpose
engines, but I’m not at all persuaded that this is actually that important in
this area. In my experience, faceted search is only really useful when you have
*lots* of competing products within a well-established parameter space (shoes,
hard drives). Meanwhile, research software is so bespoke that there’s hardly
ever more than a handful of options that truly meet a particular need. In fact,
research software is so bespoke that you’re pretty much always going to need to
read the README (or equivalent) of any package you might want to use, anyway —
no matter how detailed your domain-specific metadata are, your software index
isn’t going to contain everything I need to make a decision. In other words,
domain-specific indices are neither necessary nor sufficient for research
software discovery. Google can index the README, and I’m going to need to read
the README anyway.

[hposs]: https://nspires.nasaprs.com/external/solicitations/summary.do?solId=%7b21419978-190B-811F-35A7-6D2DEEE24E4E%7d&path=&method=init

Furthermore, we already have a well-established domain-specific software
discovery engine: the literature. If I’m doing work in a particular field and
the papers keep on mentioning [GADGET], well, now I know about it. And a
general-purpose Google for [gadget astrophysics][gga] yields pages upon pages of
relevant resources.

[GADGET]: https://en.wikipedia.org/wiki/GADGET
[gga]: https://www.google.com/search?hl=en&q=gadget%20astrophysics

All that being said, people do discover software through means other than
general-purpose text search. The Awesome Research Software Registries list
mentioned above is an instance of a popular trend of making “[awesome lists]”,
which generally present a big, er, list of resources in some topic that are
judged to be, um, awesome. Many of them relate to software and technology, such
as a list of [software for self-hosting network services][awesome-selfhosted],
but the pattern now ranges from areas like [interview
questions][awesome-interview] to [sci-fi novels][awesome-scifi].

[awesome lists]: https://github.com/topics/awesome-list
[awesome-selfhosted]: https://github.com/awesome-selfhosted/awesome-selfhosted
[awesome-interview]: https://github.com/DopplerHQ/awesome-interview-questions
[awesome-scifi]: https://github.com/sindresorhus/awesome-scifi

These might at first seem quite similar to research software indices, but
there’s an essential difference. It’s right there in the name: awesome lists are
curated and selective, rather than exhaustive. This is even emphasized in very
first sentences of [the awesome manifesto]: “If you want your list to be
included on [the awesome list of awesome lists], try to only include actual
awesome stuff in your list. After all, it's a curation, not a collection.” The
model of a “registry”, on the other hand, generally abdicates the curatorial
role: the whole idea is that random people can come along and slot their item
into the collection wherever they deem appropriate. There isn’t a hard
 demarcation here — a hurried awesome list curator might accept submissions with
minimal review; any competently-run registry is going to have some level of
centralized editorial oversight — but the philosophical difference is important.

[the awesome manifesto]: https://github.com/sindresorhus/awesome/blob/main/awesome.md

(I see this issue with [arxiv.org] periodically. Many people think of it as a
“registry” type service and expect their submissions to flow through the system
automatically; but under the hood, it is a moderated and to some extent curated
system. People often do not react well when the influence of the moderators
becomes apparent.)

[arxiv.org]: https://arxiv.org/

There’s a potentially interesting relationship between modes of discovery and
expertise to be explored here. When you’re a well-versed in a topic,
general-purpose search supports discovery quite well: you know what terms to
use, and you can quickly delve into results to identify which ones truly meet
your needs. As a person with a lot of software experience, this is surely a big
reason that research software indices don’t feel that useful to me.

When you’re less expert, however, effective discovery requires more guidance.
Curation — deference to the expertise of others — becomes more important. You’re
more likely to spend time browsing a list of options, rather than narrowing it
down as quickly as possible. From this standpoint, it’s to see why awesome lists
are popular; I think it’s fair to say that they’re aimed at people who aren’t
already experts in their fields. “Discoverability” turns out to be a relative
term, depending on who exactly is trying to do the discovering.

The problem with casting research software indices as discovery tools is that
nearly all of the other motivations for their existence — e.g., enabling or
promoting software citation — require them to be exhaustive, not curated. This
puts them in a bind. If you’re not going to give people an opinionated list of
recommendations, you need to be better than Google. It’s not hard to beat Google
in terms of signal-to-noise of your results, but you also need near-perfect
completeness — anything less feels like a catastrophic failure to the user.

Unfortunately, achieving near-perfect completeness is, in general, awfully
expensive. But it’s possible. One route to this kind of completeness is to
provide a service so valuable that software creators are *compelled* to use it.
This happens with language package systems like [NPM] or [Crates.io], etc.,
where the value of integration with the system is indisputable. And, indeed,
when I need to search for JavaScript packages I’ll do it on [NPM] rather than
Google. I don’t use any fancy faceting, but the completeness is just as good,
and the signal-to-noise is better.

The other route is to continuously put in a ton of curatorial effort, like
[ADS]. This is also awfully expensive, and a non-starter for all but the
best-funded of projects. Unless … you can convince people to donate their
effort: the Wikipedia model.

[NPM]: https://www.npmjs.com/
[ADS]: https://ui.adsabs.harvard.edu/
[Crates.io]: https://crates.io/


