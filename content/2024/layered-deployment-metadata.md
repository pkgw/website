+++
date = 2024-08-22T20:14:33-04:00 # deploytool
title = "Layered Deployment Metadata"
+++

I’ve updated the way that I deploy updates to [this very website][siterepo]. I’m
now using an [Azure Pipelines][ap] continuous integration process to build the
site and sync the files to my server. This is totally standard for 2024, but my
workflow includes a pilot-test version of a family of new techniques that I’m
interested in exploring further, both for this website and for other projects.
These techniques center on what I’m tentatively calling “layered metadata”. In
the case of my website update, they’re “layered deployment metadata”.

[siterepo]: https://github.com/pkgw/website
[ap]: https://azure.microsoft.com/en-us/products/devops/pipelines

<!-- more -->

The motivation boils down to something pretty simple. I create this website
using [Zola], a static site generator that I like a lot — it’s written in Rust
so the software is delivered as a single, robust executable, as opposed to some
giant mess of dependencies expressed in a scripting language like Python or
Ruby. The basis of my website is a bunch of [Markdown] files that I can manage
in [a Git repository][siterepo], which is exactly how I want to handle things.

[Zola]: https://getzola.org/
[Markdown]: https://commonmark.org/

In Zola, as in many other tools of the same ilk, each page is defined by a
Markdown file that has metadata expressed in a “frontmatter” section that might
look like this:

```
+++
date = 2024-06-26T09:30:00-04:00
title = "Reprocessing DASCH’s Astrometry"
+++

Yesterday I completed a large effort to reprocess all of [DASCH’s][dasch]
astrometry. […]
```

The frontmatter is in [TOML] format, as used by lots of Rust tools, and is
delimited by lines of three plus signs. Different tools might use different
delimiters or formats for the frontmatter, but this is all very standard.

[TOML]: https://toml.io/en/

What has bothered me is the `date` field above, which specifies the publication
date of the blog post. The date *should* indicate when the post actually went
online. But I have to insert the date as soon as I start drafting it: Zola will
(quite reasonably) error out if I don’t specify something. If I write quickly,
the two times don’t differ much, but maybe it takes me a week to work on my
post. Or maybe I write it quickly, but then wait a week to actually publish it.

This is the same sort of issue that
[motivated](@/2020/implementing-software-versioning.md) me to to develop the
[“just-in-time versioning” scheme][jitv] used by [Cranko]. If I have some kind
of “build process” that transforms inputs to outputs, sometimes the process
needs me to specify something as an input (the software version or post date)
that really should only be locked in if and when the build process actually
succeeds. If I try to publish a post with some kind of syntax error that causes
Zola to fail, any publication date that I've entered manually is going to need
updating.

[jitv]: https://pkgw.github.io/cranko/book/latest/jit-versioning/
[Cranko]: https://pkgw.github.io/cranko/

In principle, at least. As with the versioning issue that inspired me to work on
Cranko, this might hardly seem like a real problem, or at least not one worth
writing a bunch of code to address. “Just fix up the publication dates by hand
if you care about them so much!” But something about these workflow rough edges
seems desperately *wrong* to me, for reasons I can’t fully articulate.
Fortunately for my peace of mind, with the work leading up to Cranko I was able
to figure out how to fix that feeling.

In the examples above, one aspect of the problem is relatively narrow: the tools
that I’m using want me to commit things to my source code repository that really
are, or ought to be, metadata produced during the build process itself. In
general, I can address that by preprocessing my source code at the beginning of
my CI pipeline.

But a tricky circularity arises if I further want to preserve the metadata
generated during CI/CD such that they can be incorporated into *future* builds.
For instance, in the case of my website, you can imagine configuring the CI
pipeline to detect new posts and assign a publication timestamp to them at build
time. (Spoiler alert: this is [exactly what I’m now doing][1].) But the next
time I update my website, something needs to “remember” when the post was
published and make sure that it receives the same timestamp as it was assigned
before. If your website is a dynamic app with content stored in a database à la
Wordpress, that’s not a challenge. But if I’m storing my website in Git and
using standard CI/CD techniques, it’s a lot less clear how to propagate those
kinds of metadata out of the build process. How do you get that information back
into the Git repository without triggering some kind of infinite loop of
updates?

[1]: https://github.com/pkgw/website/blob/main/src/main.rs#L78

The solution I devised for Cranko is the same as what I’m now using to deploy
this website. Fundamentally, important metadata generated during build time are
stored by comitting them to *another branch* of the Git repository, using merge
commits that join together the latest update of the “base” content plus whatever
metadata were accumulated in the previous deployments. We can push this branch
during deployment to preserve the metadata without having to worry about
cyclicity.

To be specific: my website repository now contains code for a “deployment tool”
strongly derived from [Cranko]. When I push an update to the `main` branch, the
deployment tool runs in CI, searches for any new posts, and if any are found
[assigns them a fresh timestamp][step1] by rewriting the checked-out source
files. The website is built and the modifications are preserved by [saving them
as a new merge commit on a special `deploy` branch][step2]. If website
deployment is successful, [the update to the branch is pushed back to
GitHub][step3].

[step1]: https://github.com/pkgw/website/blob/main/ci/azure-build.yml#L43-L44
[step2]: https://github.com/pkgw/website/blob/main/ci/azure-build.yml#L57-L62
[step3]: https://github.com/pkgw/website/blob/main/ci/azure-deploy.yml#L83-L86

The key to breaking the circularity is that during the build process, timestamps
from the existing `deploy` branch are — and in fact *must be* — applied to all
non-new posts. I haven’t, er, actually implemented that part yet for my website
tool, but that’s how it’ll work. Cranko demonstrates an analogous behavior
operation when it rewrites project metafiles to include proper version numbers.

Now, all of this might strike you as overkill to try to make sure that my blog
posts have accurate timestamps. And in isolation, you’d be absolutely right. But
I’m interested in exploring these sorts of techniques because I see a whole
family of of applications besides the ones I’ve already mentioned.

For instance, you could use a similar approach to express metadata that aren’t
necessarily generated inside the CI/CD pipeline. Say that we have a blog or
documentation website that’s managed in Git and edited by lots of authors.
Ideally we would show authorship information for each page, and that information
would be derived from the Git repository’s logged history: as soon as I make a
commit that edits a given page, my name is added to the list of its authors. But
you don’t want your website-generation code to have to know how to analyze the
history of your Git repository, and it really shouldn’t even require your input
files to have to be managed by Git anyway. We can achieve the desired effect by
having a tool that runs at the start of the CI/CD process and rewrites the input
files to insert authorship information derived from the Git history. The rest of
the build can then proceed without having to know anything about Git. The
authorship information doesn’t strictly *need* to be preserved separately, since
it’s already implicit in the Git history, but if you commit it to a
robotically-managed branch, you can do things like examine diffs on the
resulting metadata.

Or, you can use similar techniques to provide useful diagnostics during pull
request processing similar to what you get from code-coverage services. For
instance, if my CI process generates a binary executable, I could record the
size of the executable in a metadata file that's added to a `deploy`-type
branch. During pull request processing, I can compare the binary size to what it
was after the most recent merge to `main`: if the binary size increased a lot, I
should probably figure out what added the bloat. This would be especially
useful, I think, to monitor API breakage in libraries: if you have a tool that
can extract the API and compare two extracts, you can see if a pull request
introduces any breakages. If you’re using Cranko, you could compare APIs against
not only the most recent merge to `main`, but also the most recent release,
which would be useful for writing release notes. I think there’s a ton of
potential in this area: code-coverage tools demonstrate that it can be super
valuable during code review to be able to examine diffs not only of the CI
*inputs* (i.e., the source code), but also the *outputs*: API structures,
characterisics of built binaries, rendered documentation, and so on. Using the
metadata-branch approach and a little elbow grease, you can give yourself access
to diffs of any build output that you care about monitoring.

Combining the above scenarios, you could imagine a tool that prevents linkrot in
a static website by building up a history of all URLs that have ever been
published, and prohibiting any of those URLs from ever disappearing. If you want
to remove one, you need to at least set up a redirect to a new destination.

Or in [conda-forge], a robotically-updated branch in each feedstock could
maintain a list of the files included in every package, and in pull request
review you could generate a diff of which files were added or removed.

[conda-forge]: https://conda-forge.org/

I called this *layered* metadata because I don’t see any a-priori constraint on
how many such processes I might want to invoke during a CI/CD pipeline. Maybe my
base Git source is augmented with authorship information generated from the Git
history, and then after the build is complete I want to record metadata about
built binaries. In most cases I think that you’ll basically be adding
information at the very beginning and very end of the CI/CD processes, but it
seems possible that there might be cases where it makes sense to cascade several
augmentations across a multi-stage pipeline.

It’s worth saying explicitly that these techniques don’t specifically need to
use Git branches to propagate information. Anything that lets you publish
information that’s generated during CI/CD, and retrieve that information during
subsequent runs, will work. But a Git-based approach lets you associate
generated metadata with the input code with perfect precision, and does so using
infrastructure that’s guaranteed to be in place.

The [actual code for my website’s deployment tool][toolcode] is kind of overkill
since it’s largely derived from Cranko, which aims to be robust and featureful
in a way that I don’t expect this tool to need to be. The immediate next step
for the tool is to fix it to apply the correct timestamps for non-new blog
posts; after that, I’m interested in seeing if I can put together a
proof-of-concept of the “output diff” functionality mentioned above. I’m not
entirely sure if this will be possible, but I think I might be able to use the
[GitHub “check run” APIs][crapi] to cause these diffs to appear in pull request
CI summaries, which would be really neat to be able to show.

[toolcode]: https://github.com/pkgw/website/tree/main/src
[crapi]: https://docs.github.com/en/rest/checks/runs
