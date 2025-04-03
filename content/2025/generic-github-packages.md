+++
date = 2025-04-03T11:38:56-04:00 # deploytool
title = "“Generic” Artifacts in GitHub Packages"
+++

Service blogging today! For a while I’ve been pondering if it would be possible
to use the [GitHub Packages][gp] service to host “generic” files: namely,
arbitrary binary artifacts that aren’t necessarily NPM packages, Docker images,
etc. Motivated by some of the my current [MPC] projects, I sat down this week to
look into the topic more deeply than I have before. Lo and behold, you can do
this! And it isn’t even (that big of) a hack.

[gp]: https://docs.github.com/en/packages
[MPC]: https://minorplanetcenter.net/

<!-- more -->

**Warning:** *I realized that I wrote this blog like some ridiculous internet
casserole recipe. Skip down to the code blocks at the end if you just want to
see what to do.*

Back when I was a lad, installing software was an adventure: for every program
you needed, you dug up its website, found the Downloads page, pulled whatever
file(s) the authors provided, and figured out how to actually get the damn thing
installed. (OK, well, actually, I remember the days of installing software from
stacks of floppy disks, but we're not going back *that* far.) From the very
earliest days of the internet, though, people saw the value of pulling files
into shared repositories: [CPAN] and [CTAN] were among the first; then we had
Linux distributions that packaged up and hosted amazingly wide-ranging
collections of software. But I feel like it took a while for people to
appreciate just how valuable these systems could be; I remember being struck by
the remarkably tight integration between the `npm` tool and [npmjs.com] when
they launched, which was 2010.

[CPAN]: https://www.cpan.org/
[CTAN]: https://ctan.org/
[npmjs.com]: https://www.npmjs.com/

Nowadays, you would be foolish to launch a new language or framework *without*
some kind of central package registry. But we're actually seeing a trend towards
*de*-agglomeration as ecosystems get so large and complex that you start running
into problems if you're limited to a single, global package namespace. For
instance, while we started with a single original [Docker Hub] for hosting
Docker container images, we now live in a world where you can spin up your own
organizational registry using infrastructure provided by [Amazon][ecr],
[Azure][acr], or [Google][gar], not to mention many other options. You see the
same pattern for NPM, Cargo, Conda, and other major packaging ecosystems as
well.

[Docker Hub]: https://hub.docker.com/
[ecr]: https://aws.amazon.com/ecr/
[acr]: https://azure.microsoft.com/en-us/products/container-registry
[gar]: https://cloud.google.com/artifact-registry/docs

(As a side note, this emergent flexibility is a testament to the brilliant
simplicity of the Internet’s architecture! None of this would be possible
without the URL. Good job, team.)

In 2019, GitHub joined the fray with its own package hosting infrastructure:
[GitHub Packages][gp] (GHP). While a lot of people might only be familiar with
GHP through the [GitHub Container Registry][ghcr], the subset of the service
that deals specifically with Docker containers, it also supports NPM, RubyGems,
Maven, Gradle, and NuGet. You can see how all of these systems might have a lot
in common under the hood: they're all basically dealing with versioned sets of
binary artifacts, and you can imagine building a common infrastructure for
naming, hosting, access control, and more.

[ghcr]: https://github.blog/news-insights/product-news/github-packages-container-registry-generally-available/

That’s cool. But. What if I’d like to leverage the GHP infrastructure to manage
a binary artifact that isn’t a Docker image, an NPM package, or any of those
other things? A “generic” package, if you will — some kind of file whose
contents could be anything?

Obviously, if all else failed, you could embed your file in one of the schemes
that GHP *does* support. You could write a Dockerfile that constructed a Docker
image containing your file, and then you could fetch the image and extract the
file. It’s not pretty, but it works — it’s an approach that I’ve used myself
more than once. You could also do the same with NPM’s tooling, or probably *any*
of the other packaging systems supported by GHP.

Can we do better? Thankfully, we can.

The short story is that nowadays you can use the GitHub Container Registry to
manage generic packages in a pretty clean way. I’m not familiar with the
detailed history, but as best I can gather, the [Open Container Initiative][oci]
has driven the development of standards and tools to allow container registries
to handle arbitrary file formats, and a side benefit is that we can (ab)use that
support to leverage these registries even if our binaries don’t correspond to
what we would normally think of as “container images”.

[oci]: https://opencontainers.org/

In particular, there’s a tool called [`oras`] that can talk to GHCR in a
“generic“ way rather than a “Docker-specific” way. (It seems that ORAS stands
for OCI Registry As Storage, based on the title of its webpage.) With this tool,
it’s quite straightforward to deal with generic packages.

[`oras`]: https://oras.land/

Specifically, if you’re like me and you’d like to publish a generic package to
GHCR in a [GitHub Actions][gha] workflow, all you need is the following:

[gha]: https://github.com/features/actions

```yaml
- uses: oras-project/setup-oras@v1
  with:
    version: 1.2.2

# ... create `myfile.zip` somehow
# $SLUG is your package slug, e.g. `pkgw/my-generic-package`
# $TAG is the version tag, e.g. `latest`

- name: Push package
  run: |
    echo ${{ secrets.GITHUB_TOKEN }} |oras login --username ${{ github.repository_owner }} --password-stdin ghcr.io
    oras push ghcr.io/$SLUG:$TAG --artifact-type application/vnd.pkgw myfile.zip
```

It’s basically the same thing as pushing with the `docker` CLI, except the
artifact data come from a file on disk, and you need to specify an associated
"media type". If you need your artifact to be consumable by third-party systems
(say, Docker), you’re going to need to set up a variety of other metadata too.
But if all you care about is pushing and pulling bytes, you can skip that, make
up a meaningless media type, and call it a day.

To retrieve your package later, it's exactly what you would hope:

```sh
# this will create `myfile.zip`:
oras pull ghcr.io/$SLUG:$TAG
```

Boom, done!

Readers experienced with GitHub will note that all of this might seem a bit
redundant with [GitHub Releases][ghr], which you can also use to distribute
versioned binary artifacts associated with your repository.

[ghr]: https://docs.github.com/en/repositories/releasing-projects-on-github/about-releases

That’s not at all off-base. As someone with plenty of [experience automating the
creation of GitHub releases][cranko], though, I have to say that the GHCR
approach feels a lot more lightweight. You don’t have to make up release notes,
and you can just push a file instead of having to make API calls to declare a
release and then attach artifacts to it. I also suspect that GHCR offers more
fine-grained access control settings. For my MPC needs, I was willing to use the
Releases system if it felt necessary, but I’m much happier to be able to use
GHCR and [`oras`] instead.

[cranko]: https://pkgw.github.io/cranko/book/latest/
