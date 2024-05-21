+++
date = 2024-05-21T10:54:19-04:00
title = "Envisioning the Thingiverse"
+++

A [recent essay and talk][usf] by [Aaron Straup Cope][asc] touches on a *lot* of
different interesting ideas, but one that particularly struck me was his
experiment with puttings things — in his case, a musuem collection — on the
Fediverse.

[usf]: https://www.aaronland.info/weblog/2024/04/26/matrix/#usf
[asc]: https://www.aaronstraupcope.com/

<!-- more -->

Cope currently works at the [SFO Museum](https://www.sfomuseum.org/), as in, the
museum embedded in the San Francisco International Airport, and seems to have a
history straddling the cultural heritage and technology sectors. This sounds
both super cool and, probably, often very frustrating — my impression is that
museums and similar institutions have technology challenges similar to, but even
bigger than, those found in scientific education and public outreach. In fields
like these, if you have a visionary bent, you can imagine *amazing* new things
that are possible with current technologies; and then trying to implement the
most modest project is slow, grinding, often disillusioning work. Even when
people buy into a particular vision, the resources needed to execute it well are
often far beyond what can be marshaled — a situation I know well from projects
like [WorldWide Telescope][wwt] (WWT) and the efforts to use it in education
like [Cosmic Data Stories][cds]. And there are, frankly, often people in
positions of power who don’t seem to have a great deal of vision beyond the
status quo.

[wwt]: https://worldwidetelescope.org/home
[cds]: https://www.cosmicds.cfa.harvard.edu/

[Cope’s essay][usf] is a very nice textualization of a talk recently delivered
at [USF][usfuniv]. It’s on the longer side, but well worth the read. (Side note:
if you’re going to go to the effort of carefully preparing a talk, it seems well
worth it to go the extra mile to write it up in this form — you’ll already have
done the hard work of planning the argument and preparing visuals, and the
result can propagate so much farther. Even if the talk is recorded, I find “too
video; didn’t watch” to be a very real thing.) 

Not being a museum person, I don’t have anything substantial to say about
several of his points, except that they ring true to me. My colleague David
Weigel of the [INTUITIVE Planetarium][ip] at the [US Space & Rocket
Center][ussrc], like Cope, spends a lot of effort on trying out ways that his
experience can “follow you out of the building” — something that WWT is great
for! — but I’m continually shocked-but-not-surprised at how few institutions
seem to be tackling this challenge. I suspect that many individuals working at
these institutions would love to do more, but just don’t have the resources they
need to get anything off the ground. Sadly, this lack of resources seems to
often turn into a sort of learned helplessness, rather than spurring creativity.

[usfuniv]: https://myusf.usfca.edu/arts-sciences/art-architecture/art-history-museum-studies
[ip]: https://www.rocketcenter.com/INTUITIVEPlanetarium
[ussrc]: https://www.rocketcenter.com/

Cope is really into the idea of museum visitors building up a durable, personal
relationship with the items in collections. Over the years, he’s been involved
in a few attempts to use technology to encourage this — I gather that the
[Cooper Hewitt Pen][pen] is the best-known of these, although it’s not something
that I’d heard of before. His most recent iteration is an idea that I love:
creating a Fediverse/Mastodon account for every item in the SFO museum.

[pen]: https://www.aaronland.info/weblog/2015/04/10/things/#mw2015

One of the motivations is as follows: right now, if someone in a museum sees
something that they want to remember, the most likely thing they'll do is take a
picture of it on their phone. There’s nothing wrong with that, but I can imagine
that as a museum curator it might feel like a hugely missed opportunity. It’s a
one-time interaction, and structured information about the object in question is
basically lost — in the sense that, say, if the object has a unique identifying
number, there’s no reliable way for software to obtain it. Without that, even
the most basic next steps that you can think of — e.g., “list all of the things
that I liked at that museum” — aren’t possible.

So: what if instead, visitors could “like and subscribe” to objects?

Even if the object in question never posts anything, the “follow” action records
the connection in a way that’s durable, bidirectional, and machine-actionable in
the future. And you can immediately start thinking about things you can build
based on the resulting social graph. (Which, of course, implies that privacy has
to be considered carefully when building a system in practice.)

Cope correctly points out that there are some significant practical hurdles to
getting this foundational interaction to work. A small one is that there’s no
easy way to convert a QR code scan into a follow, as far as I know; the much,
much bigger one is that so few people are on the Fediverse. But I am extremely
sympathetic to the argument that this kind of interaction *should* be on the
Fediverse, and not a proprietary network, and there’s something about this
vision that seems to me to be *so* sensible, in a certain way, that it actually
makes me more optimistic about the Fediverse taking off in general. While I
don’t see an ecosystem like the Fediverse ever having the social, viral grip of
a commercial product like TikTok, I could see it gaining traction for what you
might call “anti-viral” communication patterns: municipal updates about garbage
collection, that kind of thing. And there’s at least the possibility that
commercial experiences will bridge to the Fediverse, à la Threads, although
such bridging has become a whole, contentious can of worms.

To help motivate the focus on the Fediverse, Cope mentions that he asked someone
at FourSquare if he could create 50,000 venues in bulk — one for each item in
MoMA — and was, unsurprisingly, turned down. Likewise for creating 200,000
Twitter accounts. Beyond a desire to avoid vendor lock-in, which is a strong
motivation in its own right, the ability to mass-create accounts is something
that seems well-suited for a decentralized network. We see this with email, too.
More generally, you often arrive at interesting places if you start with a
service that has “user accounts” that were intended to be operated by a single
human, and ask yourself: why might one person want to have 100 accounts? Why
might one account might want to be associated with 100 people? Or zero? You see
the same patterns pop up with things like the [Apply Private Relay email
service][apr] or [virtual credit card numbers][vccn].

[apr]: https://www.spamresource.com/2023/08/private-relay-vs-private-relay-vs-hide.html
[vccn]: https://www.paypal.com/us/money-hub/article/virtual-credit-card-numbers

Beyond the connection-building functionality — social following is the new
bookmarking, you heard it here first — you then have the fact that your objects
can toot. (Which, if you’re not familiar, is what we’re supposed to call the
equivalent of tweeting on Mastodon. I guess “tweet” sounded pretty silly at
first, too.) Depending on your default level of optimism, this is probably
either very exciting or very scary. Surely you can spark joy and build amazing
connections with the right kinds of interactions. On the other hand — we have
enough trouble answering support questions in person, and now you want us to
monitor 50,000 inboxes? Content moderation? It’s hard for me to see how opening
up all of these touch points doesn’t become a huge new source of work. You can
definitely make the argument that it’s good work: if you’re a museum, what
better way can you be spending your time than interacting with patrons in a
sustained way? But it’s work all the same.

I’m not at all sure whether this particular idea will take off, but I love the
audacity, and I like the range of new ideas that can build on it. What if every
single [Harvard plate][ps] had a Fediverse account? Why would people decide to
“follow” a given plate? What would they say to it? What could we have the plates
“say” spontaneously? If people start having conversations with plates, does that
collected history start becoming a sort of per-item knowledgebase? I’m
particular intrigued by this last possibility. If you can somehow get people
into a habit of messaging objects — @-ing is the new annotation? — it seems like
a whole new mechanism for achieving the goals of projects like [The Underlay].

[ps]: https://platestacks.cfa.harvard.edu/
[The Underlay]: https://underlay.mit.edu/

What might ensue if I created a Fediverse account for every post on this blog?
What if every Wikipedia page was on the Fediverse, and posted about its updates?
Every star in [SIMBAD]? Every [drain in Somerville][aads]? Maybe — probably — a
lot of these ideas just wouldn’t work, but it would be fun to find out.

[SIMBAD]: https://simbad.u-strasbg.fr/simbad/
[aads]: https://somerville.mysticdrains.org/