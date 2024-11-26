+++
date = 2024-11-25T22:33:08-05:00 # deploytool
title = "All In on Pixi"
+++

I’ve been playing around with the [Pixi] software environment management tool
for a little while now. After incorporating it into a several different
projects, I think that I’m ready to declare: this is the way of the future. You
should think about adopting it yourself.

[Pixi]: https://pixi.sh/

<!-- more -->

Some context might be helpful. [Pixi] is a tool that creates and manages
software environments, most similar to [conda] or its variants ([mamba],
[micromamba]); you can think of it as yet another drop-in replacement for any of
these. In particular, [Pixi] uses exactly the same file formats and web APIs as
conda and friends, and so in a certain sense they’re all interchangeable. My
claim — always hard to prove — is that despite this, there are some key
differences about Pixi’s design that make it a dramatic improvement on the
alternatives.

[conda]: https://docs.conda.io/
[mamba]: https://mamba.readthedocs.io/
[micromamba]: https://mamba.readthedocs.io/en/latest/user_guide/micromamba.html

When I use the term “software environments,” I mean a freestanding set of
software packages installed somewhere on a computer; different environments are
independent and can be activated at the user’s discretion. In the context of the
past half-century of computing, it’s worth emphasizing how novel, perhaps even
revolutionary, this concept is. People have always installed custom software on
computers (that’s kind of the whole reason they exist), but for much of the
history of computing, *managing* those installations has been an awfully ad-hoc
affair. Traditionally, if I’m on a Unix machine, I might compile a library that
I need and install it into `$HOME/lib`; then futz with some compiler flags to
compile a binary that depends on that library and install it into `$HOME/bin`;
and so on, on a case-by-case basis. If I want to update or uninstall anything, I
need to manually recompile code, set up environment variables, rerun build
scripts, and all other sorts of finicky things.

Given where I’m going to take things below, it’s worth pointing out explicitly
that this difficulty, in turn, made it quite impractical to maintain *multiple*
environments on a single machine. In general, you had what was installed
globally, your user-specific customizations, and that was it. If you needed to
install some special tool to support a specific workflow, you’d drop it into one
of these big buckets. Anything finer-grained would take way too much effort.

The quality of the modern tools has completely changed things. It used to be the
problem was getting people to be able to install Python at all; nowadays the
problem is that they’ve lost track of all of their different Python
installations. This *is* a real problem, but more and more I’m convinced that
it’s a gamechanger to escape the “scarcity mindset” regarding environments. We
can truly have as many of them as we want! (This can be hugely inefficient, it’s
true, but disk space is cheap, man.)

The core breakthrough, as I see it, was the idea of creating
standardized software packages that a generic [package manager] can then
manipulate. But I’d argue that it wasn’t until Conda that we got the first
end-to-end system with a really killer combination of features:

[package manager]: https://en.wikipedia.org/wiki/Package_manager

- *Arbitrary roots:* You can create any number of software environments, rooted
  in any directory on your computer.
- *Language-agnostic:* You can reliably install software written in any
  language.
- *User-level:* No system administrator privileges needed.
- *Decently cross-platform:* Support for the Big Three of Linux, Mac, and
  Windows.

(For what it’s worth, there’s a key technical innovation that enabled Conda to
deliver all this: install-time rewriting of the install prefix and [RPATH]
records in text and binary files, respectively, which makes the “arbitrary
roots” feature possible without admin privileges.)

[RPATH]: https://en.wikipedia.org/wiki/Rpath

The conda ecosystem certainly has its issues — it has sometimes been frustrating
to see these tools rediscover problems that were solved by [RPM] and [dpkg]
twenty years ago — but I have found this particular combination of capabilities
to be very powerful. Above all, the ability to create and throw away
environments on-demand opens up all sorts of new work patterns. It’s exactly the
same sort of space of opportunities that were created by [Docker] — which has
been utterly transformative — just at a somewhat more user-facing level.

[RPM]: https://rpm.org/
[dpkg]: https://en.wikipedia.org/wiki/Dpkg
[Docker]: https://www.docker.com/

But if the conda ecosystem is so great, why am I saying that [Pixi] in
particular is such an improvement? Because Pixi finally nails two things that
the original conda tools didn’t do so well: it’s *declarative* and
*reproducible*.

In contrast to the declarative-ness of Pixi, conda environments have to be
managed *imperatively*. To create, change, or destroy an environment, you have
explicitly signal your intent by running a command — `conda create` to create an
environment, `conda install` to add a new piece of software into it, and so on.

This is fine as far as it goes, but it’s actually a fairly cumbersome, low-level
approach. Pixi’s approach is higher-level: you declare that you want an
environment with certain characteristics by putting a [`pixi.toml`] file in a
directory; and then Pixi will do whatever is needed to ensure that the requested
environment is available there.

[`pixi.toml`]: https://pixi.sh/latest/reference/project_configuration/

This might not sound like a major difference, but I believe that it’s hugely
important. Imagine that my team has a set of scripts that need to run in a
certain software environment (say, ones supporting a research paper in
progress), and the environment is evolving along with the scripts as we work. In
the imperative model, every user needs to drive that evolution themselves: run
the command to create the environment when they're first getting set up; install
new packages as needed; delete and recreate the environment if anything goes
wrong. It’s really hard to ensure that everyone is actually using the same
software.

In the declarative, Pixi world, it’s much simpler: if you have the same
[`pixi.toml`], you’ll be running the same code. In essence, you’re telling the
computer what you want and letting it figure out how to get there, instead of
explicitly taking it there yourself. This is the difference between compiling
software by manually running `gcc` and `ld` commands, and having a `Makefile`
(or, my preference, a [`build.ninja`]) that reliably delivers your final product
no matter what your exact starting point is.

[`build.ninja`]: https://ninja-build.org/

You might be about to bring up [`environment.yml`] files, which can play a role
similar to that of `pixi.toml`. I will claim, without delving into every detail,
that they simply can’t support the declarative paradigm. They can help with the
initial creation of an environment, but not with maintenance after it’s created.
They also don’t work if you want to declare an environment that can be created
on multiple OSes. Finally, they miss the mark on the other virtue of Pixi:
maintainable reproducibility.

[`environment.yml`]: https://docs.conda.io/projects/conda/en/latest/user-guide/tasks/manage-environments.html#creating-an-environment-from-an-environment-yml-file

Pixi makes environments more reproducible using a technique that is now
bog-standard: [lockfiles]. When I write a [`pixi.toml`] file, I specify
requirements about what software needs to be in my environment, which can often
be quite loose: “I need Python 3.10 or newer, but beyond that I don’t care about
the precise version.” Whenever Pixi sets up an environment, it resolves my
requirements into a specific set of packages to install (say,
`python-3.12.7-hc5c86c4_0_cpython`) and records that result into a companion
file, [`pixi.lock`]. Future operations will stick with what’s in the lockfile,
unless I explicitly specify that I want to reconsider some of those decisions.
If I distribute my lockfile, other users can get an environment that isn’t just
consistent with my requirements, but that matches mine exactly. The hallmark of
the lockfile approach is while a human controls the file that defines the space
of allowed solutions (`pixi.toml`, here), the software tool controls the file
capturing the specific solution that was arrived at (`pixi.lock`), and the tool
ensures that the lockfile always reflects the ground truth of the actual
installed environment.

[lockfiles]: https://docs.npmjs.com/cli/v9/configuring-npm/package-lock-json
[`pixi.lock`]: https://pixi.sh/latest/features/lockfile/

Pixi’s developers describe it as being “project-oriented.” To me, this
terminology gets at the fact that the state of a Pixi environment is completely
compartmentalized on the filesystem — not just the installed environment but the
`pixi.toml` and `pixi.lock` files that define it all necessarily live within the
same directory; and nothing outside of that directory affects them. (Ideally, at
least.) I feel that there’s something about this compartmentalization that
enables a mental inversion: instead of setting up a software environment, inside
of which I run code for various projects, instead I set up various projects,
some of which might happen to define software environments that can run their
code.

There are other aspects of Pixi that I like but are perhaps less interesting.
One worth mentioning is that it’s written in Rust and distributed as a single
executable, so installation is easy and reliable. (One of the decade-old
mistakes repeated by conda: don’t write a package manager in a scripting
language!) This also makes it pretty fast.

That being said, there will be times when Pixi isn’t the right tool to use. In
particular, it’s not the kind of thing that would replace [pip] (or [poetry] or
[uv] or …) for managing a Python software project, or [npm] (or [pnpm] or [yarn]
or …) for JavaScript. Those are, broadly speaking, software-development tools
that can manage software environments; Pixi is exclusively focused on the
latter.

[pip]: https://pip.pypa.io/
[poetry]: https://python-poetry.org/
[uv]: https://docs.astral.sh/uv/
[npm]: https://npmjs.com/
[pnpm]: https://pnpm.io/
[yarn]: https://yarnpkg.com/

But I’m pretty sure that I can use Pixi everywhere that I used to use conda and
mamba. I still maintain a what you might call a personal-but-global software
environment, activated by default in all of my terminals — now it’s just more
reproducible and better defined, since I have a `pixi.toml` defining what goes
into that environment. Whereas in the past, I always dreaded upgrading my Python
version, because it could easily break conda and wedge everything, now I’m kind
of looking forward to the next upgrade — I’ll just update the `python =
"3.12.*"` spec in my `pixi.toml` file and ask Pixi to re-solve everything. If
anything goes wrong, I’ll `git restore pixi.toml pixi.lock` and I’ll be back
where I started. Doing this kind of reversion is super hard in an imperative
system, but trivial in a declarative one.

The other change is that now my personal-but-global software environment is
getting slimmer, since I can break out rarely-used packages into more
purpose-oriented environments. For instance, I use the phenomenal [Beancount]
software to do accounting. (I like Beancount so much that I now think that
accounting is *fun*.) The Beancount ecosystem is almost entirely written in
Python, and I used to have all of those tools installed in my global
environment. But now with Pixi, it makes way more sense to put a `pixi.toml` in
the same Git repository where I maintain my Beancount data — I’m only going to
use the tools in connection with those data files. The Beancount tools aren’t
always compatible with the latest-and-greatest versions of things, so it’s
genuinely helpful to be able to isolate that environment from my general-purpose
one.

[Beancount]: https://beancount.github.io/

I got some good use out of conda environments before Pixi, but I was always a
little dissatisfied with the tools that conda provided for working with them —
or the lack thereof. Pixi solves essentially all of the problems that were
bothering me: the reliable, declarative behavior takes conda’s support for
environments from a sketch to a fully-realized, new tool in my toolbox. I can
imagine how it will be especially helpful for collaborative projects, but even
for purely individual efforts, I have a feeling that it will unlock powerful new
work patterns. Kudos to the Pixi team!
