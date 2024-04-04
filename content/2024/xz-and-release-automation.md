+++
date = 2024-04-04T10:37:08-04:00
title = "The xz Backdoor and Release Automation"
+++

Some of you may have spent a lot of the past week following the drama ensuing
from the discovery of [a backdoor inserted into the open-source `xz`
library][1]. Others of you probably have no idea what I’m talking about. There’s
been more than enough chin-stroking on this topic over the past week, but I want
to at least point out a connection to a topic near and dear to my
software-engineering heart: release automation.

[1]: https://www.schneier.com/blog/archives/2024/04/xz-utils-backdoor.html

<!-- more -->

The short version of the `xz` story is that, more or less by luck, an engineer
discovered that someone had subtly inserted a malicious [backdoor] into a
low-level compression library called `xz`. The backdoor was only introduced a
few months ago, so it didn’t really have time to spread much around the internet
— but it was designed in a way such that if it had, it could plausibly have
given the perpetrator a nearly-invisible way to hack into huge numbers of
computers all across the internet. Yikes!

[backdoor]: https://en.wikipedia.org/wiki/Backdoor_(computing)

The scary thing is that the “someone” who inserted the backdoor was the nominal
maintainer of the package — the person in charge of it. This “person”, operating
under the name Jia Tan, almost certainly doesn't actually exist. And they
weren’t the *original* maintainer of the package.

With hindsight, it appears that `xz` was subject to a multi-year,
state-sponsored social engineering attack. The original, non-nefarious developer
posted in 2021 about being too burned out to maintain the library; almost
immediately, “Jia Tan” appeared and started helping out. Eventually “Jia Tan”
was given maintainership rights, and finally, after a few years, attempted to
sneak in their malicious code. I’m no security expert, but I agree with most of
the armchair folks out there that between the long-time horizon of the attack
and the complexity of its implementation, it seems overwhelmingly likely that it
was supported by an organization of national scale.

As a side note, you might recall that I [posted about seeking new Tectonic
co-maintainers](@/2023/seeking-tectonic-co-maintainers.md) and eventually [added
two people](https://github.com/tectonic-typesetting/tectonic/discussions/1142).
I remain very happy that I did! They were both previous contributors and during
that process I got some information about their offline identities; I can’t say
that there’s *zero* chance that I’m not the subject of a sophisticated
intelligence op, but I’m not losing sleep over it and I don’t think that you
should either. (Also, hindsight further suggests that first step of the
social-engineering campaign against the original `xz` maintainer was targeted
bullying about his failure to be sufficiently responsive to feature requests;
although it would be hard to distinguish this from everyday life as an
open-source maintainer! Either way, that’s thankfully not something that I’ve
experienced with Tectonic.)

Anyway. The discovery of this attack has prompted a lot of musing about the
open-source ecosystem; [this toot][2] from Mikey Dickerson (or is it “Mikey
Dickerson”?) put it well:

[2]: https://mastodon.world/@mikey@soylent.green/112186694087462654

> hey does anybody out there have any thoughts about the xz compromise or
> perhaps have you thought of a way to relate it to some axe you have been
> grinding for 20 years

I sure do, and I sure have! But here I’m going to keep it to one specific axe
that’s only a few years old.

For something like the `xz` attack to succeed, the malicous code has to be
well-hidden. People use libraries without reading their source code all of the
time, but if your repository contains a big function called
`trojan_horse_implementation`, someone is *eventually* going to look into it.
You also only want to enable the code in narrow circumstances, so that routine
security tests don’t even see that it’s there. A lot of the sophistication in
the `xz` attack is how the malicious payload was obfuscated and disabled
altogether when it wasn’t needed. One component of this was that the code that
developers download and install from GitHub is unproblematic; a single,
innocuous-looking line inserted into one of the build scripts in the packaged
release files starts up the whole machinery that activates the hack.

This has gotten some people worried, and rightly so. Generally, developers use
the GitHub version of a piece of software, but most deployed versions (i.e., the
stuff that ends up installed on thousands or millions of computers) are based on
release packages created by the maintainer. The release package is supposed to
capture what’s on GitHub, but usually the maintainer creates it by hand, so
really it could contain … *anything*. If I offer a compiled version of a
program, for instance, I could compile in whatever extra code that I want, and
the nature of the packaging and compilation process makes it effectively
impossible to verify that the “release artifacts” truly correspond to the public
version of the source code found on GitHub. This system requires you to
completely trust the maintainer who creates the releases — which is now feeling
like a scarier proposition than it used to!

How can we mitigate this? Well, I claim if we want to be able to verify that
release artifacts truly correspond to their alleged source code, we need to
automate the processes by which they are produced. And would you know that I’ve
been excited about release automation [for several years now][3]? Once you ask
the question “why are maintainers creating these things by hand, anyway?”, I
feel that the proper solution becomes self-evident.

[3]: @/2020/implementing-software-versioning.md

That being said … why *are* maintainers creating these things by hand? In my
view, a big reason is simply inertia. The counter-question — “what’s the
alternative to making a release by hand?” — only has an answer thanks to the
existence of publicly-available, cloud-based [CI/CD] (“continuous integration
and deployment”) services, and I think that a lot of projects still really
haven’t internalized the kinds of workflows that these services unlock. It’s
been interesting to watch the evolution in this space. When I started using
Travis CI, it was basically a way to trigger a VM to run test suites for various
programming languages. But at some point we collectively realized that if you
can do that, you can really run *any* kind of code — these services are really
free, easy-to-configure platforms for cloud compute on demand, that just happen
to have their workflows tied to Git repositories. (I have *no* idea how these
services prevent people from abusing them to mine crypto; I’d guess that they
have whole teams dedicated to stopping just that.) [Conda-forge] was quite ahead
of its time in realizing that you could use CI/CD to build and publish software
packages, but that sort of insight hasn’t sunk in everywhere.

[CI/CD]: https://www.redhat.com/en/topics/devops/what-is-ci-cd
[Conda-forge]: https://conda-forge.org/

The other piece is that there are some genuine workflow problems that need to be
solved in order for maintainers to create high-quality release artifacts on
these public CI/CD systems. Admittedly, lot of maintainers have been perfectly
happy with the status quo, but I feel that there are some subtle issues at the
root of this activity that need to be addressed carefully. This problem bothered
me for *years* before [I first sketched out a solution that felt adequate][3],
and then I had to create a whole [new tool][Cranko] and [workflow][4] to
implement it. While the ideas behind [Cranko] are, I think, quite general, it’s
also true that its approach also benefits greatly from the sophistication of
modern CI/CD systems; its release automation would be a lot more annoying
without [Azure Pipelines’][5] tools for accumulating artifacts and managing
multi-stage builds across the Linux, Windows, and Mac platforms. That is to say,
there are a *lot* of pieces of infrastructure that need to come together to make
high-quality release automation feasible.

[Cranko]: https://pkgw.github.io/cranko/
[4]: https://pkgw.github.io/cranko/book/latest/jit-versioning/
[5]: https://azure.microsoft.com/en-us/products/devops/pipelines

Fortunately, those pieces currently exist. Release automation on public CI/CD is
far from an airtight solution, of course — a malicious maintainer can insert
obfuscated build steps, use hidden environmental settings, or simply replace
automatically-generated release artifacts with tampered ones after the fact. But
it at least *enhances* our ability to audit release artifacts and understand how
they’re produced. I didn’t have the supply-chain security angle in mind when I
developed [Cranko], but I wouldn‘t be surprised if people start adding release
automation to the list of security-enhancing practices that they want to see
open-source projects adopt.
