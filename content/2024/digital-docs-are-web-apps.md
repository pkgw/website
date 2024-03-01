+++
date = 2024-03-01T09:41:38-05:00
title = "Digital Documents are Web Applications"
+++

I spend a lot of time thinking about digital technical documents. A computer’s
screen is a more capable medium than the printed page: can we use that
capability to actually communicate more effectively? To date, the answer appears
to be flatly “no”. To an incredible extent, scientists’ preferred digital format
for technical material is the PDF file, a completely print-oriented format. I’m
not saying that scientists are wrong to use PDFs so much. Indeed, the fact that
they do (unlike almost everyone else!) tells us something profoundly important.
But it seems like we *should* be able to do better.

<!-- more -->

And, like it or not, the alternative is clear: when I read a news story, an
essay, or a recipe, I’m reading it in a web browser. [The latest irreverent
sports blog from Defector][blog] may not always feel like a “document”, but
that’s a totally valid way to look at it. Through that lens, we can see it as a
document targeting [the web platform][twp] as its “output format” rather than
something like PDF. The web platform is a complete mess, but (because) it
benefits from billions of dollars of annual investment. You can use it to play
videos, 3D games, analyze data, and, yes, read. Maybe things will change one
day, but for the foreseeable future, it’s an inescapable conclusion that if we
want to create digital technical documents that anyone actually uses, we need to
target the web.

[blog]: https://defector.com/how-many-angry-fellas-does-it-take-to-dislodge-cam-newtons-hat
[twp]: https://en.wikipedia.org/wiki/Web_platform

More recently, I’ve come round to thinking that we should go a bit farther and
try to start thinking of digital documents as *web applications*. This idea runs
into a bit of an early roadblock because I’m not sure how exactly I would draw
the line between a web application and a web … not-application. But it feels
like a real and meaningful distinction. On the not-application side, we can have
things like traditional [latex2html] output: static HTML, CSS, asset files, and
little or no JavaScript. On the other end of the spectrum, we have Jupyter,
Google Maps, and all the rest. My intuition says that somewhere in the middle
there’s a phase transition. Practically, it’s probably the point at which you
start building your content using the web dev stack (NPM, bundlers, etc.) rather
than hand-coding your HTML/CSS/JS as a bunch of static files.

[latex2html]: https://www.latex2html.org/

In this schema, it’s intuitive to want to put the things that we think of as
“documents” on the not-app (let’s say “static content”) end of the spectrum, and
not without reason. My problem with this is that you cut yourself off from a
whole world of possibilities if you think of your document as an HTML file with
some CSS and JS sprinkled on top. Instead of asking yourself “I can do
*anything* that the web platform permits — what *should* I do?”, you end up
asking yourself, “What *can* I do that won’t be too hard?”

Consider search. One of the key benefits of having a digital document is that
you can do full-text searches instead of having to consult an index. It’s great!
And the longer your document is, the more valuable a search feature will be. But
if your document isn’t [one giant webpage][moby], the browser’s find-in-page
feature isn’t going to suffice. So it’s reasonable, perhaps even incumbent, to
implement a search UI. How are we going to show results? Preview snippets would
be nice. So would complex queries. And history. Do we want to hand-code this all
ourselves in low-level HTML/CSS/JS? The [mdBook][mdb] system used by a lot of
Rust documentation does, and when I look over that code, I say to myself … yeah,
this is why people adopt web development frameworks.

[moby]: https://clickhole.com/the-time-i-spent-on-a-commercial-whaling-ship-totally-c-1825124286/
[mdb]: https://github.com/rust-lang/mdBook/blob/master/src/theme/searcher/searcher.js

The same considerations apply in other areas, like interactive figures and
tables, navigation, embedded computation, or theming. I can imagine a lot of
really cool features in these areas and I do *not* want to implement them all
using the lowest level of tooling.

Getting a bit more ambitious, we can also start envisioning how the structure of
a document could become more dynamic. I was once at a conference session about
Python documentation tools where people got into a big discussion about whether
it was better to have each function documented on its own page, or to group
chunks of related methods on one big page. *Why do we have to pick one?* If we
think of our documentation as being a bunch of static files, anything else is a
pain. But if we think of ourselves as building a web app, it’s not like there’s
*less* work to do, but now we have a problem-solving vocabulary to bring to
bear. OK, if we want to provide both options, what will the UX design be like?
How to we need to structure the backend to implement that design?

All this being said, digital documents are also *unusual* web applications. It
is essential that documents are durable and can stand alone: if I deliver a
document as a web app that becomes unusable if some third-party service goes
down, I’ve done something wrong. They’re a perfect fit for the [local-first] web
design mentality.

Likewise, when I describe a document as a “web app”, I don’t mean that it ought
to have a million buttons, animations, and widgets everywhere. It’s reasonable
to think of a document-as-app as having a “user interface”, but it’s one that
should consist of almost all text, almost all of the time. The app-ness is about
the sophistication of the tooling on the backend, not the apparent complexity of
the frontend.

[local-first]: https://www.inkandswitch.com/local-first/

The biggest challenge to this vision is undoubtedly archiving. I can save a PDF
article as a file on disk with the confidence that my grandchildren will be able
to read it if they want. Even if we ignore things like dependencies on
third-party services, how can I possibly save a document-app with anything like
the same level of confidence? At the most basic level, there seems to be no
agreed-upon standard for archiving a website to disk — [WARC] seems to be the
leader, but to me it’s telling that none of the browsers have anything like a
built-in “open WARC file” functionality. We can still work on *creating*
state-of-the-art digital technical documents without solving this problem, but
until that happens, the PDF isn’t going anywhere.

[WARC]: https://en.wikipedia.org/wiki/WARC_(file_format)