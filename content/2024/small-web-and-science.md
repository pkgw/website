+++
date = 2024-05-14T11:06:30-04:00
title = "The Small Web and Science"
+++

Over the past few years I’ve been noticing more and more of a discourse about
the “small web” in my online communities. It’s cool to see! I mentioned the
concept in [an earlier post](@/2023/newsletter.md) and thought I might write a
few more words about it today.

<!-- more -->

As far as I can tell, [this 2020 essay by Parimal Satyal][1] seems to be the
origin of “the small web” as a particular term. Nowadays, I see a lot of other
highly similar ideas percolating around that use slightly different language:
[the humane web][2]; the [indie web][3]; the [rewilding of the internet][4]. To
be honest, you probably don’t even need to click these links to guess the broad
arguments these essays are making. The modern web has become corporatized,
user-hostile, siloed, sterile; it used to be better, and it would be nice if it
became better again. Small-web advocates generally encourage people to make
their own websites and to do so in a way that consciously rejects the
pressures of the [attention economy][ae].

[1]: https://neustadt.fr/essays/the-small-web/
[2]: https://humanewebmanifesto.com/
[3]: https://indieweb.org/
[4]: https://www.noemamag.com/we-need-to-rewild-the-internet/
[ae]: https://en.wikipedia.org/wiki/Attention_economy

Now, if you have an ounce of self-awareness and you find yourself fixating on
how things used to be better, you start getting worried. Am I just
nostalgia-tripping? I’m not going to try to construct a detailed argument but I
think that we can be pretty confident that there’s more going on here than some
people being having fondness for how things were when they were younger. Well
within my lifetime, the web has gone from being a new invention to becoming an
inescapable element of everyday life for billions of people. The modern web is a
genuinely different thing than it was in the “good old days”, however you choose
to define them (and however “good” you feel the old days truly were).

In particular, I see two main themes in what makes the modern-day web so
disappointing compared to the early years: greed and bloat.

Greed is the less-technical side of what’s been going wrong. SEO, engagement
optimization, personal-information mining, scams — they’re pretty terrible, and
they’re everywhere! In my view, these phenomena are fundamentally expressions of
the fact that the web is now a part of society; they’re just the web-specific
manifestations of unchecked capitalism, [the rot economy][5], at work. While the
idea of [enshittification][6] was originally articulated by Cory Doctorow in
reference to the web, it’s clearly resonating in other realms as well: I see it
pop up in conversations about topics ranging from [science][8] to
[aerospace][9], [urbanism][10], and [Manchester United][12].

[5]: https://www.wheresyoured.at/the-rot-economy/
[6]: https://pluralistic.net/2023/01/21/potemkin-ai/#hey-guys
[8]: https://garymarcus.substack.com/p/the-exponential-enshittification
[9]: https://globalcomment.com/the-enshittification-of-boeing/
[10]: https://www.residenturbanist.com/p/enshittification-small-urban-spaces
[12]: https://www.graceonfootball.com/p/the-enshittification-of-manchester

Bloat is the more-technical side of things. Most writers in the small-web field
have the feeling that websites have become slow, flaky, and overcomplicated.
[This is empirically true][13] in pretty much any metric you care to analyze.
While looking for a particular quantification of this I ran into [the following
delightful sentence][14]:

[13]: https://almanac.httparchive.org/en/2022/page-weight
[14]: https://liaogg.medium.com/its-time-to-stop-using-create-react-app-a99917dbfc

> [Vite], the build tool I’m going to introduce later, only has 34.2 MBs of
> dependencies […]

*Only!*

[Vite]: https://vitejs.dev/

It’s worth pointing out that in principle, greed and bloat could be separate
issues. We can imagine a web that’s flooded with sites that are full of
AI-generated clickbait, but at least are clean and lightweight too. In practice,
however, the two do seem to go together, which is interesting in its own right.
The technology sector seems unusual in that greed and laziness generally to lead
to *bigger and more complicated systems, rather than smaller and simpler ones*.
This recalls the Kroger website case study [that I wrote about
earlier](@/2024/really-fast-websites.md): it took a lot of effort to design an
incredibly minimal, fast website! Such a site would surely be cheaper to operate
than whatever Kroger was running instead, but I have to conclude that the
savings from hiring [cheaper engineers][15] dominate. (I’m sure that insightful
people have pursued this thought much more deeply, somewhere out there.)

[15]: https://www.baldurbjarnason.com/2024/react-electron-llms-labour-arbitrage/

The good news that everyone in this area [likes to emphasize][16] is that the
web is still fundamentally open. There’s nothing stopping me from launching a
surveillance-free, labor-of-love website that anyone with an unrestricted
internet connection can enjoy, and the core technologies of the web are still
pretty dang efficient: the vast majority of web bloat lives in frameworks that
are stacked on top of them. (Witness the Kroger example.) It’s absolutely true
that an individual with the right skills and resources can create a website
that’s simply lightyears beyond anything that was possible in 1998. This is not
to imply that there aren’t deep inequities at play, but it’s fair to say that
“write access” to the web is as equitable as it’s ever been.

[16]: https://www.citationneeded.news/we-can-have-a-different-web/

The more difficult question is whether a bunch of people making indie websites
is actually going to have any effect upon the median person’s experience of the
web. To be honest, I’m not too optimistic. I tend to think that the analogy with
“indie scenes” in other fields is likely a good one. Whether that’s encouraging
or not depends on your ambitions. If your goal is to overthrow the Googles of
the world, not so much. But the platform of the web does seem well-suited to
supporting an indie scene where some people can actually make a living doing
what they enjoy without a boss (be it a person or The Algorithm) breathing down
their necks: witness the success of outfits like [Pinboard], [Buttondown], or
[Defector]. That’s not nothing!

[Pinboard]: https://pinboard.in/
[Buttondown]: https://buttondown.email/
[Defector]: https://defector.com/

We can’t, however, take for granted that the architecture of the web will always
be quite so friendly to independent operators — protocols and expectations are
always evolving. The proverbial “someone” needs to apply pressure to keep the
infrastructure of the web friendly to small operators. (As well as to create
supporting systems: for instance, [Let’s Encrypt] has surely been an incredible
boon for the small-web ecosystem.) My worry here is that the small-web ethos is
definitely susceptible to the tendency that you can get in environmentalism and
other underdog movements: hoping if enough people just display enough personal
virtue, the large-scale problem will solve itself. I doubt that many people
would seriously argue that there’s *no* role for public policy or other forceful
efforts in trying to achieve these goals, but I worry the DIY approach can
easily become a trap. Small-scale effort is *much* easier and yields rewards on
*much* shorter timelines than large-scale action. From what I’ve seen it easily
soaks up *all* of people’s time and energy, leaving nothing left for the big
stuff.

[Let’s Encrypt]: https://letsencrypt.org/

I did title this post “The Small Web and Science” but I haven’t mentioned
science at all yet. Really, I just think that it’s important for scientists to
be aware of this family of concepts and some of the discourse surrounding them.
Scientists are often making indie-type websites — we invented the web, after
all! — and I hope that the concept of the “small web” (or “indie web”, or
whichever) gives them a way to think and talk about the choices that they make
when they do so. Most of the tools and materials for building websites out there
are implicitly or explicitly aimed at people that want to be like the next
Facebook; it is valuable, both pragmatically and as an act of solidarity, to
understand that our goals are different.
