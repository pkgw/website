+++
date = 2025-08-29T14:02:11-04:00 # deploytool
title = "One Good Tutorial: The Plan"
+++

The past few posts have been about prep work for [my BSSw
project](@/2024/bssw-fellowship.md): [interviews](@/2025/state-of-the-docs.md)
and [a survey of tools](@/2025/state-of-the-doc-tools.md). After all this
throat-clearing, I’m ready to sketch out the resource that I’m actually planning
to create!

<!-- more -->

I plan to call it **One Good Tutorial**. The target audience will be, of course,
developers of small-to-medium scientific software projects. The centerpiece of
the project will be a checklist: complete these actions, and you can sleep easy
knowing that you’ve documented your project adequately.

One thing that I absolutely want to beat people over the head with is the point
that *there is more to documentation than API docs*. This is the big idea behind
[Diátaxis][dtx], of course, and I’m completely sold on it; and I also believe
that it’s an idea that many scientific software developers need to be exposed
to. I think this is such a big deal that, well, I let it drive the whole project
branding. In particular, I believe that the most important thing that *most*
projects lack is introductory, “getting started”-type material. So: *if your
docs have One Good Tutorial, you’ve done your job.* If the only thing that
people retain from being exposed to my project is those three words, I’ll be
happy.

[dtx]: https://diataxis.fr/

I also believe that many scientific software developers don’t feel confident
about how they should approach documentation in general. This is, well, totally
reasonable: technical writing and information architecture are whole fields of
human endeavor, and we’re generally approaching them with no training or
support. Realistically, that’s not going to change: the goal is *not* to train
people to become expert technical writers. But I think it will make a real
difference if we can help scientific software developers feel like they’re not
quite so at sea. Hence the checklist format. I’m hopeful that a checklist will
work well to provide both tangible instructions and a rewarding sense of
clarity: “OK, I did everything they said I should — gold star for me!”

I also think that such a checklist will fill an unoccupied niche in this space.
The [Good Docs Project][gdp] provides templates for authoring specific
documents, but doesn’t quite provide the holistic work plan that I think a
checklist will offer. The [Write the Docs Guide][wtdg] has a lot of resources
and guidance but, once again, doesn’t quite meet the needs of someone saying,
“Just tell me what to do!”

[gdp]: https://www.thegooddocsproject.dev/
[wtdg]: https://www.writethedocs.org/guide/

Another nice aspect of the checklist format, I think, is that it leads to a
natural structuring of the resource materials. The core artifact is, of course,
the checklist itself, which I’d expect to deliver as both HTML and a PDF
one-pager. Then, for each item on the checklist, there will be an associated
webpage with deeper explanation, references, and examples. In certain cases this
page might be quite short, but in other cases, it could get fairly extensive.
Contrast this with “cookbook” or “recipe” format, which tend to be structured
more like prose text — which means that the length keeps on increasing as you
think of little details or clarifications to throw in. The recipe format also
implies that the steps should be followed in strict order, whereas checklists
allow for some level of skipping around. I think that’s a good thing in this
case.

More specifically, I’m currently envisioning what you might call a “checklist
matrix”. The checklist items will mostly correspond to important pieces of
documentation that must exist: **Tutorial** (of course!), **Citation
Information**, **Installation Instructions**, and so on. These are the rows of
the checklist matrix.

But I’m also envisioning four columns corresponding to four phases that I will
encourage people to work through: **Plan**, **Draft**, **Assess**, and
**Revise**. The basic guidance would be to go through these phases in order:

1. Take some time to think about your plan for all of the different components
   of your documentation. Consider creating a Google Doc, or something similar,
   to hold notes about your plans.
1. Actually draft the materials, and do the initial setup of whatever tools
   you’re going to need to get your docs published.
1. Assess the complete first draft. Did the process of drafting reveal any
   problems that need to be fixed?
1. Revise. Self-explanatory.

I pointedly do *not* include a “publish” phase, because I think that encourages
people to think of the docs as a one-time project: “I wrote them and published
them, and now they’re done.” I think it’s important to approach the both the
code and the docs as things that are never quite *done*, which to me means
having a mindset oriented around “making releases”, rather than “publishing”.

Here’s my first draft of the rows for the checklist matrix:

- **Synopsis.** A few sentences summarizing the software. Good to do first
  because it helps you keep the big picture in mind, and it’s likely to be
  copied around in READMEs, website landing pages, package descriptions, etc.
- **Personas.** I want to encourage people to take a few minutes to imagine user
  personas for their documentation: who’s going to be reading the docs? I hope
  this will be genuinely helpful for people as they’re thinking about docs … and
  I don’t hate the idea of sneaking in an idea that they might be able to apply
  much more broadly, too. A row like this one would have checkboxes for the Plan
  and Assess phases, but not Draft or Revise, since it doesn’t explicitly appear
  in the documentation product.
- **Tutorial.** This had better come early! I hope that suggesting that people
  sit down and plan their tutorial before actually putting (virtual) pen to
  (virtual) paper will help them think through bigger-picture issues: “oh, the
  user is going to need to have access to some 10-gigabyte data file, so I had
  better come up with a way to distribute it”. If people just plunge straight
  into writing the tutorial (perhaps after burning out developing the code),
  that’s the kind of problem that gets left unresolved.
- **Installation Instructions.** Just one of those things that you need to have.
- **Citation Instructions.** This is one of those sections that is unlikely to
  take up many words in published documentation, but I really want to make sure
  that people sit down and think about how they want to approach it. It would be
  beyond the scope of One Good Tutorial to tell people what approach to citation
  to adopt, but the supporting materials can point them to resources on the
  topic.
- **License.** This is similar to citation instructions: a lot of people just
  don’t even think about this issue.
- **How to Contribute.** Another item that I find is often overlooked. This is
  the sort of thing where I can offer people boilerplate language to use. This
  is where I would suggest that larger projects think about adopting a Code of
  Conduct too.
- **API Reference.** I want to make a point of putting this really far down in
  the list … but yeah, this is important for software.
- **Other Reference Materials.** I’m not quite sure how to name or describe this
  element, but in scientific software, there’s often some kind of theory
  underlying the software, and it’s really important to document it precisely.
  In many cases, the documentation here might basically consist of a reference
  to a formally-published journal article.
- **Acknowledgments.** Don’t forget to thank your funders! I will also ask
  people to acknowledge One Good Template if they have found it useful.
- **Authoring Tools.** Once someone has gone down the list and thought about all
  of the above materials, *now* it’s the time to think about: what tools are we
  going to need to create this documentation? For some projects, you could
  absolutely cover every item above in a single `README.md` on GitHub; for
  others, you want to think about whether Sphinx (etc.) will suffice, or whether
  you might need to adopt a combination tools. The Draft phase is where you
  would actually start wiring these tools into your workflows such as CI.
- **Release Processes.** Once you’ve thought about what your docs are going to
  look like and what tools you’ll use to write them … how specifically are they
  going to make it out into the world? As mentioned above, here I want to
  encourage people to think of doc publication as an ongoing process, possibly
  one that’s integrated with the software release process.

Each of these items will have a corresponding article on the One Good Tutorial
website, providing advice on how to approach the item in each of the four
phases. In at least some cases, these will branch out into more specific how-to
pages. For the Authoring Tools and Release Processes items, this is where I will
provide specific tool recommendations and step-by-step tutorials on how to
handle common scenarios (e.g., using Sphinx and ReadTheDocs to document a
pure-Python package; depositing your software to Zenodo). There should be ample
opportunity to refer people to existing resources like the [Good Docs
Project][gdp] templates. Time permitting, I could also see myself adding
supporting “explainers” giving information about, say, the topic of software
citation in general.

It will probably also make sense to have a section that I would describe as
“Extra Credit”. This would mostly be aimed at slightly larger projects,
addressing topics like codes of conduct, organizing multiple tutorials, how-tos,
social sites like StackExchange, and so on. There’s no shortage of material that
could be written here, but I expect that these topics will be out of scope for
most of the developers that I would want to visit One Good Tutorial. And in the
end, I’m aiming to reach *people* rather than projects, so I’d rather be
relevant to lots of people working on smaller efforts, even if the bulk of the
documentation-reading and -writing that happens might be concentrated on a small
number of high-profile pieces of software.

I’m feeling pretty good about this plan, so I’ve gone ahead and registered
[onegoodtutorial.org](https://onegoodtutorial.org/), set up a GitHub repo
([pkgw/onegoodtutorial](https://github.com/pkgw/onegoodtutorial)), and wired up
a static site built with [Zola](https://getzola.org/) and hosted via [GitHub
Pages](https://pages.github.com/), with deployment automated using GitHub
Actions. I am pretty sure that a basic static site generator will suffice for
setting up the OGT website; if I run into limitations, it will be easy to
rebuild it to use different infrastructure instead.

*The work described in this post was supported by a [Better Scientific Software
Fellowship](https://bssw.io/pages/bssw-fellowship-program)*.
