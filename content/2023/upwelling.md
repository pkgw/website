+++
date = 2023-11-10T10:01:20-05:00
title = "Upwelling"
+++

Earlier this year I came across [Upwelling], a research project created by a
team at [Ink & Switch], an independent technology research lab. Upwelling
sketches out a fascinating vision for collaborative digital writing — like
Google Docs — that, in my view, has enormous potential for researchers and other
scholars.

[Upwelling]: https://www.inkandswitch.com/upwelling/
[Ink & Switch]: https://www.inkandswitch.com/

<!-- more -->

I was thrilled to discover Upwelling because, as far as I can tell, the team
behind it feels the same way that I do: I hate, hate, *hate* trying to do real
writing in Google Docs. There are a few reasons for this. One of them is a sort
of imprecision: while Google Docs automatically tracks changes, it can’t do so
with nearly the sophistication of developer-facing version control systems
(i.e., [Git]). Empirically, many people don’t seem to mind this, but it really
bothers me. And as the Upwelling team notes [in their article][Upwelling], if
you have a substantial group of people working on a complex document — say, a
large proposal — that sophistication really buys you a lot. In particular, in
bigger efforts, *reviewing* changes is very important, and that becomes
immensely difficult if the authors of those changes aren’t able to organize them
into things like commits and pull requests, to use the Git terminology.

[Git]: https://git-scm.com/

It’s also true that Google Docs, in particular, is WYSIWYG-only. This is another
thing that bothers me, but it’s a bit to the side of the core issue. For
instance, while [Overleaf] lets you compose in LaTeX, I still don’t really enjoy
writing in it.

[Overleaf]: https://www.overleaf.com/

There's also what the Upwelling team calls “the fishbowl effect”: the way in
which these real-time collaborative editing systems mean that you’re always
conscious that while you’re writing, someone might be watching every typo and
false start you make. I also feel this, although my understanding is that in
these sorts of systems the fraction of the time when two people are genuinely
editing a document simultaneously is vanishingly small. In my view this feeling
is largely a knock-on of the lack of precise change-tracking. These systems
don’t need to, and in fact *shouldn’t*, preserve every keystroke. That level of
detail makes it impossible to meaningfully review changes: the big picture is
drowned in a sea of irrelevancies.

The Upwelling team also seems to share my belief that these kinds of limitations
*matter*. If nothing else, people in fields like mine collaboratively write
complex proposals on which millions of dollars are at stake. But there are
plenty of documents with no conceivable economic value that still deserve to be
created with the very best tools that society can offer.

There’s a solution to the problems of document authoring that’s great for me:
just use Git! Since I like the power that comes with composing technical
documents as LaTeX source, I don’t miss the WYSIWYG editing experience at all.
And I can integrate my text with related source code in the same repository.
Perfect!

Sadly, this solution isn’t great for the whole “collaborative” part of things.
My collaborators are happy to write LaTeX in Overleaf, but I’m confident that
literally *no one* that I regularly work with would want to use Git for
collaborative writing. It would be nice for me if things were different, but
they’re not. The end.

On the other hand, it’s worth emphasizing that *everyone* that I work with uses
Overleaf. I believe that every single paper I’ve been involved with over the
past few years has been hosted on Overleaf — it’s probably the most rapid and
complete adoption of a new tool that I’ve seen in my career. That’s incredible!
There is an *immense* thirst for better ways for a group of people to work
together to produce professional documents, although many groups don’t seem to
have consciously understood that a lot of their difficulties can be traced back
to very concrete characteristics of the tools they’re using.

The Upwelling approach tries to land somewhere in the middle. Their underlying
technology is the same as that behind Google-Docs-style editors: [conflict-free
replicated data types][crdt] or CRDTs. CRDTs have properties that allow multiple
people to make changes to the same document, combine them asynchronously with
only weak ordering constraints, and get the same final result. Git’s data
formalism, on the other hand, requires more explicit centralization and
combination.

[crdt]: https://en.wikipedia.org/wiki/Conflict-free_replicated_data_type

Unlike Google Docs, though, Upwelling borrows some Git-like workflows: writing
is done in branch-like “layers” that are combined to build up the canonical
version of the document, and the prototype places a lot of emphasis on the
ability to meaningfully review proposed changes. These workflows are based on
fairly substantial interviewing and usability testing — one thing I really like
about [the Upwelling report][Upwelling] is that the team’s prototyping was not
just about writing code, but also doing a lot of on-the-ground investigation of
what approaches actually work well for real people.

As a matter of fact, I tend to think that the UX design of the workflows may be
more important than the actual implementation of this particular tool. When I
think of the problems that Upwelling is trying to solve, they seem to be a lot
more about human factors than technical magic. That being said, the report
describes a lot of underlying technical work and unsolved problems, and an
Upwelling creator told me that they’re convinced that CRDTs are essential to the
software’s feasibility — they do not believe that you could successfully build
it on top of Git.

Here I have to confess that I haven’t actually used the Upwelling prototype.
[The report][Upwelling] does such a good job of describing the project’s key
insights that I honestly don’t feel that typing into its UI would actually teach
me much more. And since it’s a prototype, and since it’s WYSIWYG and not
LaTeX-based, I don’t see much of an opportunity to use it in a real
collaborative project just yet. I would absolutely love for those things to
change, though!

More than the specific prototype, I love the vision behind this project. Trying
to write collaboratively is one of those things that makes me feel like either
I’m missing something, or the rest of the world is — “Isn’t everyone appalled by
how much easier this could be?!” In a certain sense, the answer to that question
is a resounding yes — the wild success of Overleaf (in my admittedly small
corner of the world) shows that. But we can do so much better. Even if it’s just
a prototype, Upwelling feels like an important step in the right direction. My
only complaint is that the proverbial Someone should be spending absolute
truckloads of money to take us even farther.
