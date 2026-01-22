+++
date = 2024-08-07T13:20:00-04:00
title = "Seven-Figure Scientific Software Projects"
+++

“Get this — I just got a six-million dollar grant to develop a new astronomical
image viewer!” That’s not something that’s ever been said, I’ll wager. But why
not?

<!-- more -->

I started thinking about this in the context of a slogan that I’ve been toying
with for a little while: “Every astronomy department should have a tenured
software specialist”. Bold (and self-serving), I know. But it’s almost
defensible, I believe, if we think optimistically based on the analogy to
hardware instrumentation development. In both cases we’re talking about people
who build tools. We have well-known problems giving these people enough credit,
but I’d like to think that astronomers generally appreciate that our field moves
forward on the basis of their work. Having a tool-builder in-house gives your
faculty a leg up on the competition. Developing new tools tends to be expensive,
and to require specialized skills … but by the same token, good tool-builders
should be able to bring in a lot of overhead!

And this is true of hardware development. Compared to your baseline generic [NSF
AAG] research grant of around $500k, hardware projects can access bigger pots of
money. To pick a few awardees from the older [NSF MSIP] program, you can get
$2.5 million to [build an exoplanet imaging spectrograph for Keck][2216481] (UC
Irvine), or around $7 million for [an integral field spectrograph for
Magellan][1836002] (MIT). You can get a lot more, of course, as you scale up
from individual instruments to whole facilities.

[NSF AAG]: https://new.nsf.gov/funding/opportunities/astronomy-astrophysics-research-grants-aag
[NSF MSIP]: https://new.nsf.gov/funding/opportunities/mid-scale-innovations-program-astronomical
[2216481]: https://www.nsf.gov/awardsearch/showAward?AWD_ID=2216481
[1836002]: https://www.nsf.gov/awardsearch/showAward?AWD_ID=1836002

If I’m applying for tenure-track positions as a person who builds software (I’m
not — that ship has sailed), I want to be able to tell a story about how I’ll
secure grants in that seven-figure-and-beyond range. Even if we ignore the very
real fact that people do care how much overhead you bring in, this is simply the
scale of funding that you need if you want to start something important that has
the chance to make a lasting difference in the field. (Something like [emcee]
might be an exception, but I also bet that emcee would have a lot more impact if
a few million dollars were spent on it!) For reference, typical seed funding
rounds for Silicon Valley startups [look like they’re $3 million][seed], and
Series A rounds are larger by a factor of a few.

[emcee]: https://emcee.readthedocs.io/
[seed]: https://carta.com/learn/startups/fundraising/seed-funding/

Compared to the hardware domain, though, it’s a *lot* harder to tell that story.
Your intuition probably screams out that you’d have zero chance of getting the
NSF to hand you seven-figure sums on the basis of “I have an idea for the next
[Astropy]” or even much more specific, but still ambitious projects like “I want
to build a new VLBI data reduction package”. [NSF ATI] (grants going up to ~$2M,
total pool this year of ~$8M) nominally supports software development but the
framing of the program (“enable observations for ground-based astronomy that are
difficult or impossible to obtain with existing means”) makes that a virtual
non-starter, and I don't see any pure-software projects in [the recently awarded
ATI projects][atiaw].

[Astropy]: https://astropy.org/
[NSF ATI]: https://new.nsf.gov/funding/opportunities/advanced-technologies-instrumentation-astronomical
[atiaw]: https://www.nsf.gov/awardsearch/advancedSearchResult?ProgEleCode=121800&BooleanElement=Any&BooleanRef=Any&ActiveAwards=true#results

Now, there is [CSSI] out of the NSF [CISE] directorate: “Cyberinfrastructure for
Sustained Scientific Innovation”. This is probably the closest in spirit to the
kind of funding that I’d like to see, and the program scale is in the right
ballpark. CSSI “Framework Implementation” awards come in around $2 million. But
there are planned to be around ten of these given out in the 2024 round, across
pretty much the whole NSF; framework implementations are “aimed at solving
common research problems faced by NSF researchers in one or more areas of
science and engineering”. This is all well and good, but think of the hardware
analogy: would that Keck imaging spectrograph fit that definition? [WorldWide
Telescope][wwt] got [a smaller CSSI Elements grant][wwtg], and I would love to
go for a Framework Implementation, but it would be a difficult sell.

[CSSI]: https://new.nsf.gov/funding/opportunities/cyberinfrastructure-sustained-scientific
[CISE]: https://new.nsf.gov/cise
[wwt]: https://worldwidetelescope.org/
[wwtg]: https://www.nsf.gov/awardsearch/showAward?AWD_ID=2004840

In the current environment, if you want access to substantial resources for
software development, you can tune your CSSI pitch, and you can try to piggyback
on tangible facilities: maybe you can secure a big subaward to develop something
like a pipeline for a major observatory. That’s simply where the significant
pots of money for astronomical software development can actually be found —
attached to very large projects like [Rubin] and space missions.

[Rubin]: https://www.lsst.org/

These projects are only going to support certain kinds of software development,
though. Not to undersell the importance of pipelines and other facility-type
software, but when I think of software efforts that ambitious “software
instrumentalists” would want to be able to point to as significant professional
accomplishments, I think of things like [Astropy], [Jupyter], [MESA], or [ds9],
the project behind of this year’s [ADASS Software Prize]. These are also the
kind of project that we need much more of, I think. Historically, people have
found ways to support work on these sorts of foundational systems through
facilities funding (ds9 probably being the best example), but as funding gets
tighter, software gets more expensive, and people appreciate more and more just
how difficult software projects are, this approach seems less and less viable to
me.

[Jupyter]: https://jupyter.org/
[MESA]: https://docs.mesastar.org/
[ds9]: https://sites.google.com/cfa.harvard.edu/saoimageds9
[ADASS Software Prize]: https://www.adass.org/softwareprize.html

There’s a much bigger problem here than simply the lack of an
appropriately-targeted funding program, though. As almost everyone has come to
recognize by now, most software projects are **fundamentally** different
undertakings than hardware projects, in ways that have significant implications
for how they need to be supported and managed. This is despite the fact that in
other ways, software and hardware projects indeed have much in common.

Consider some of the MSIP examples above. *Some* of the key aspects of the
deliverables are extremely concrete: I will build a spectrometer with
such-and-such resolving power, operating in such-and-such waveband, attaching to
the back of such-and-such telescope. It’s possible to specify software
deliverables in the same way: Astropy will allow users to load FITS files;
[CASA] will allow users to calibrate VLA data. You *can* build software this
way, and sometimes you have to; but even the most straitlaced engineering
organizations now understand that software-by-specification is at best a deeply
limited approach. Say what you will about [agile], [scrum], and the rest, but
these methods were invented because traditional ones were utterly failing in the
software context.

[CASA]: https://casa.nrao.edu/
[agile]: https://agilemanifesto.org/
[scrum]: https://www.scrum.org/

Many thousands of words have probably been written about “why software is
different.” To a certain extent, the specific reasons probably aren’t even that
important. But as someone who cares a lot about the quality of software, in the
gestalt [Zen-and-the-Art-of-Motorcycle-Maintenance sense][q], I can say that I
find the things that make certain pieces of software the *most* exciting and
inspiring are the things that are farthest from what would be captured in a
typical specification. [Git] versus [Subversion], [Ninja] versus [make],
[Beancount] versus [hledger], [Rust] versus [C++]: each pair of tools would
likely satisfy the same written spec, but you’ll never convince me that they’re
of equal quality.

[q]: https://en.wikipedia.org/wiki/Pirsig%27s_Metaphysics_of_Quality
[Git]: https://git-scm.com/
[Subversion]: https://subversion.apache.org/
[Ninja]: https://ninja-build.org/
[make]: https://en.wikipedia.org/wiki/Make_(software)
[Beancount]: https://beancount.github.io/
[hledger]: https://hledger.org/
[Rust]: https://rust-lang.org/
[C++]: https://en.wikipedia.org/wiki/C%2B%2B

Anyway, all that is to say that in my view, the reason that the NSF doesn’t have
a great way to give you $5 million to build the next Astropy is that everyone
involved recognizes that doing so would rarely yield good results within the
current framework. You could have very little confidence up-front about what was
going to come out of the whole effort, and it would be really easy to spend all
that money and get with something that no one actually wanted to use. The
early-2000’s [US NVO] experience isn’t exactly inconsistent with all this. I’ve
been harping on the NSF here for specificity, but any traditional grantmaker is
going to face the same issues.

[US NVO]: https://scixplorer.org/abs/2001ASPC..238....3S/abstract

It’s true that projects that have *already* achieved a high level of
significance can attract big grants: Jupyter landed [$6 million in 2015][jm];
Astropy broke through with [$900k from Moore in 2019][apm]. But unfortunately,
it’s really, really hard to build up a compelling software project on a series
of small grants. My understanding is that [STScI] made a long-term investment on
the order of tens of millions to get Astropy going, and ds9 has benefited from
long-term, steady funding via [Chandra] — funding that’s now in extreme danger
thanks to [Chandra’s budget being blown up][cxb]. [PlasmaPy] did [get $1.4
million][ppf] relatively early in the project history, but they likely benefited
from having an extremely legible pitch: “let’s make an Astropy for plasma
physics”.

[jm]: https://blog.jupyter.org/new-funding-for-jupyter-12009a836867
[apm]: https://www.moore.org/grant-detail?grantId=GBMF8435
[STScI]: https://www.stsci.edu/
[Chandra]: https://chandra.harvard.edu/
[cxb]: https://cxc.cfa.harvard.edu/cdo/announcement.html
[PlasmaPy]: https://www.plasmapy.org/
[ppf]: https://www.nsf.gov/awardsearch/showAward?AWD_ID=1931388

I’m sure that hardware development has comparable bootstrapping problems, but it
seems to me that the challenges for software are going to be worse. If you’re
starting out a new hardware development program, you might convince the NSF or
your institution to invest in lab space, a vector network analyzer, a mass
spectrometer, or whatever. If it all goes belly-up, you still have your capital
investment. Software projects, on the other hand, are all [opex] to a good
approximation — people. If a project fails, you’ll have essentially *nothing* to
show for it. What’s worse, talented people care about things like “whether they
will have a job in a year“ or “what their long-term career prospects look like,”
whereas vector network analyzers emphatically don’t. I believe strongly that if
you want to recruit and retain good software developers, you’ll have to be able
to offer them a level of stability and career growth potential that is
*extremely* foreign to university standards. And you’re not going to get good
software without good developers.

[opex]: https://www.investopedia.com/ask/answers/112814/whats-difference-between-capital-expenditures-capex-and-operational-expenditures-opex.asp

So, how do we make it possible for someone to establish themselves as a
“software instrumentalist”? It goes without saying that more funding wouldn’t
hurt, but the key point is that if we want to enable significant, innovative,
PI-driven scientific software projects — and I think we do — we need different
*kinds* of funding. The software projects that I think, frankly, are the most
interesting and valuable entail a kind of uncertainty that does not match well
to traditional grant-proposal models, and the challenge is made only more
difficult because building a sustainable software-production practice requires
stable, substantial investment.

Undoubtedly people have ideas about ways that grantmakers could do things
differently to better support innovative software development. The obvious
source of inspiration would be Silicon Valley: call it the “startup” model. I
think the key through-line would be that the funder would have to think of
itself as investing in a team of people, rather than a particular product.
Startups pivot all the time, after all. Maybe your initial product idea wasn’t
any good, but if you can show that you’ve become skilled at figuring out how to
build something interesting that people will actually use, that’s a success. I’m
not finding anything to link to at the moment, but I’m sure this sort of idea is
well-trodden ground.

What’s interesting is that I see elements of this approach in the design of the
[NASA Science Activation][sciact] program, NASA’s umbrella funding vehicle for
science education projects. Grants are relatively large and long-lasting;
oversight is relatively hands-on, with regular meetings and each project having
to retain an external evaluator; and there’s a big emphasis on inter-project
collaboration and the development of an overall education-focused community of
practice. If I had a really big pot of money to support innovative PI-driven
software projects, those would all be things that I’d want to have as well.

[sciact]: https://science.nasa.gov/learn/about-science-activation/

You could also say that the upshot of all of this is that if you want to produce
innovative scientific software, stay out of the universities. Go get a job at
[STScI] and convince a higher-up to peel off some money to support your vision.
It’s not terrible advice, but I’d really like to think that we can do better. I
think there are a ton of PI-driven software projects that could be executed for
an amount of money that’s totally in line with hardware development efforts, and
would deliver comparable if not much more impact for the expense — think Astropy
and [ADS]. The benefits might be extremely diffuse, but that’s exactly the kind
of thing that grantmakers are supposed to figure out how to support.

[ADS]: https://ui.adsabs.harvard.edu/

I don’t have a way to make money magically appear, but if it does, the key is to
be able to spend with confidence. That means having better tools to estimate
cost and schedule for specific software projects; a clear idea of how we’re
going to do oversight; realistic models for developer retention, software
adoption, and other social processes; and ultra-clear definitions of success. If
we understand and even embrace the distinctive characteristics of software
development, and think carefully about how those characteristics interact with
our existing institutions, we can tap into an immense amount of potential.

*See also [an addendum](@/2024/seven-figure-addendum.md).*
