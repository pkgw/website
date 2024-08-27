+++
date = 2024-08-27T00:00:00-04:00
title = "Layered Metadata: Output Diffs"
+++

Building on the idea I sketched out [last week][prev], today I’ve been waiting
for some jobs to run so I sat down and put together a proof-of-concept
demonstrating “output diffs” in a pull request.

[prev]: @/2024/layered-deployment-metadata.md

<!-- more -->

The basic problem we’re trying to solve is: how can we (ab)use continuous
integration frameworks like [Azure Pipelines][ap] or [GitHub Actions][gha] to
get more information about how a pull request changes our build outputs?

[ap]: https://azure.microsoft.com/en-us/products/devops/pipelines
[gha]: https://docs.github.com/en/actions

When you file a pull request, you can see the changes in the build *inputs*:
every GitHub-like system out there will show you the [diff] representing how
your source code has changed. By running a continuous integration pipeline, we
can build various outputs and report on them; usually the key result is whether
a test suite passed.

[diff]: https://en.wikipedia.org/wiki/Diff

This is all good and valuable, but it’s slightly limited. We can probe various
“absolute” characteristics of the build outputs, but we can’t easily track
“relative” characteristics: how various things have *changed* since the last
build. In other words, we can’t diff the outputs in the same way that we diff
the inputs. It’s as if whenever you filed a pull request you had to read over
the entire codebase to understand that just a single line was being edited.

Code-coverage tools demonstrate why tracking relative characteristics can be
helpful. Yes, it would be nice if I achieved 100% code coverage and stayed
there; but realistically, most projects have incomplete coverage and aim to
improve it over time. [Last time][prev] I gave some other examples of cases
where it would be nice to be able to review output diffs. If you’re compiling a
binary executable, you might want to track changes in its size and
shared-library dependencies. If you’re maintaining a library API, you’d like to
be able to track additions and breakages. The latter is an example where a
certain change isn’t necessarily “good” or “bad” — it’s not exactly a test that
either succeeds or fails. (Although for some projects, any API breakage would
indeed be a problem!) Even so, human reviewers would often find it extremely
valuable to be able to review such a diff. (“Oh, wait, we don’t want this method
to become part of the API — it should be marked as private.”)

There’s a fairly clear information-flow problem that makes it difficult to
produce these kinds of output diffs in current systems. In order to produce a
diff, one build needs to have access to the previous build’s results — but there
isn’t an obvious place to store such results during a build so that they can be
accessed later. Code-coverage tools can accomplish this because they’re
integrated with various third-party data APIs. If you think about it, the only
reason that code coverage analysis needs to involve external services (in
comparison with, say, other kinds of linters) is precisely to handle this
data-storage issue!

The [layered metadata][prev] concept that I discussed last time, which expands
the idea behind my [Cranko] release automation tool, basically asks what we can
accomplish if we store data using the Git repository that’s already storing our
source code. In particular, we can mechanically maintain extra Git branches that
layer various pieces of metadata on top of the “raw” source code.

[Cranko]: https://pkgw.github.io/cranko/

This approach has been a game-changer for me when it comes to making software
releases. Last week, I [added a small layer to my website deployment
process][pr7] that makes sure that new posts on my site get sensible timestamps.
It’s not the most earth-shattering application or anything, but I am very happy
with it as a demonstration of the benefits of thinking clearly about what are
“build-time” or “deploy-time” metadata versus the “raw” project source.

[pr7]: https://github.com/pkgw/website/pull/7

Just now I [added a demonstration of the “output diff” technique][pr10]. My
website build pipeline now computes the total on-disk size of the built website,
and if I submit a change as a pull request — [like the one for this very
post][pr11] — it will compare this number against the previous size and report
the change in [a GitHub comment on the pull request][rc]. Once again, this isn’t
incredibly important in this particular application, but I’m really happy to be
able to demonstrate the workflow:

[pr10]: https://github.com/pkgw/website/pull/10
[pr11]: https://github.com/pkgw/website/pull/11
[rc]: https://github.com/pkgw/website/pull/11#issuecomment-2313186048

{% relfig(path="sample-comment.jpg") %}
The numbers in the comment are now outdated because I added this screenshot to
this post.
{% end %}

The process works by comparing the built size with the contents of a file named
`_output_treesize.txt` stored within [the `deploy` branch][db] of [the repo for
my website][wr]. When the `main` branch is successfully updated, the deployment
process includes a job to update the `deploy` branch with information about both
page timestamps and the new tree size metric. The next build will then be able
to compare its value with the updated number.

[db]: https://github.com/pkgw/website/tree/deploy
[wr]: https://github.com/pkgw/website/

You could probably all of this in shell script, but I wrote [some Rust code][sr]
to do it since I already had the framework in place. Last week I wrote about
implementing the feedback as a [GitHub “check run”][gcr], but doing that would
require setting up a full [“GitHub App”][gha], which looks like a pretty heavy
process. For this demonstration, I’m keeping things as simple as possible, and
so I’m avoiding the app. Using comments for the reporting is a bit kludgey: for
instance, as I was developing the functionality, the pull request [acquired a
series of redundant comments][repsz] because I had to keep on retrying the
pipeline. But it’s basically one easy API call to make the comment, and it
surfaces the reporting results in a nice prominent way. I considered using the
[GitHub “commit status” API][gcsa], which likewise doesn’t need a full-fledged
App, but it’s designed for binary pass/fail tests. As I wrote above, I think
it’s important to recognize that output diffs are, generally, analysis tools to
help human reviewers; their results shoulnd’t always be reduced to binary
pass/fail outcomes.

[sr]: https://github.com/pkgw/website/blob/main/src/size_report.rs
[gcr]: https://docs.github.com/en/rest/checks/runs?apiVersion=2022-11-28
[gha]: https://docs.github.com/en/apps/overview
[repsz]: https://github.com/pkgw/website/pull/10#issuecomment-2313024854
[gcsa]: https://docs.github.com/en/rest/commits/statuses?apiVersion=2022-11-28

I did realize, however, that in a non-demonstration context, you’ll probably
need to go with a full-strength app, or at least a comparable heavyweight
solution. The problem is that in order to post the comment reporting the results
of website-size analysis, the build task needs access to a GitHub API token.
People now recognize that this exposes [a security risk][mpr]: a malicious pull
request could modify the CI scripts to expose that token and then use it for
their own purposes. As such, pretty much every CI system now has a way to mark
certain values as “secure” and to prevent most pull requests from accessing
their values at all (see, e.g., [Travis CI’s discussion][tci]). This is the
configuration for my website’s pipeline, which is fine since I’m the only person
that’s going to be filing pull requests on the repo. But for other projects,
you’re going to need a more sophisticated security model to be able to somehow
report the output-diff results into the PR safely.

[mpr]: https://nathandavison.com/blog/shaking-secrets-out-of-circleci-builds
[tci]: https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions

I have to acknowledge that all of this is, well, complicated. It gets a bit
better as you get more familiar with the guts of your CI tools, but it’s true
that there are a lot of moving parts — not only did I have to write [a custom
Rust tool][dt] to do some Git shenanigans, but I had to put together a
[multi-stage CI pipeline][pl] to execute the necessary operations. ([Here are
the logs][apl] for the first public draft of this post, although they’ll get
deleted off of Azure eventually.) For some use cases, I think it’s definitely
worth setting everything up, full stop. But what I’ve found with Cranko is that
once you have a demo like this one put together, it becomes way easier to adopt
the workflows in smaller projects too — copy-paste can be a wonderful thing!

[dt]: https://github.com/pkgw/website/tree/main/src
[pl]: https://github.com/pkgw/website/tree/main/ci
[apl]: https://dev.azure.com/pkgw/Misc/_build/results?buildId=1545&view=results