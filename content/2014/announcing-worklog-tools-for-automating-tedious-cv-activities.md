+++
date = 2014-02-05T09:35:15Z
title = "Announcing: worklog-tools, for automating tedious CV activities"
path = "2014/02/announcing-worklog-tools-for-automating-tedious-cv-activities"

[extra]
wp_rel_permalink = "/2014/02/announcing-worklog-tools-for-automating-tedious-cv-activities/"
wp_shortlink = "/?p=824"
+++

There are a lot of annoyances surrounding academic CV’s. Making a document
that looks nice, for one. Maintaining different versions of the same
information — short and full CV’s, PDF and HTML formats. Remembering how
you’ve categorized your talks and publications so that you know where to file
the latest one.

For me, one of the great frustrations has been that a CV is full of useful
information, but that information is locked up in a format that’s impossible
to do anything useful with — besides generate a CV. I’d like to collect
citation statistics for my publications, and my CV contains the needed list of
references, but I can’t pull them out for automatic processing. Likewise for
things like previously-awarded NSF grants (hypothetically …) and lists of
collaborators in the past _N_ years. Some of these things are just interesting
to know, and others are needed by agencies and employers.

Well, problem solved. Enter
[worklog-tools](https://github.com/pkgw/worklog-tools/).

Based on the issues I’ve mentioned above, I feel like it’s pretty clear what
you want to do: log CV-type activities — your academic output — in some kind
of simple data format, and populate some kind of LaTeX template with
information from the log. While we’re at it, there’s no need to restrict
ourselves to LaTeX — we can also fill in an HTML template for
[slick, web- native versions of the same information](@/pubs.md).

I’ve actually gone and done this. There are a lot of ways you could implement
things, but here’s what I do:

- I log activities in a series of records in simple
  [“INI format”](http://en.wikipedia.org/wiki/INI_format) files named
  `2011.txt`,
  [`2012.txt`](https://github.com/pkgw/worklog-tools/blob/master/example/2012.txt),
  _etc._

- Special hooks for publication records tie in to
  [ADS](http://adsabs.harvard.edu/) to fetch citation information and compute
  things like [_h_-indices](http://en.wikipedia.org/wiki/H-index).

- A simple command- line tool fills in templates using information from these
  files, in the form of either arbitrary data from the raw records, or more
  specialized derived data like citation statistics.

Two components of this system are data — the log files and the templates. One
component is software — the glue that derives things and fills in the
templates. The [worklog- tools](https://github.com/pkgw/worklog-tools/) are
that software. They come with
[example data](https://github.com/pkgw/worklog-tools/tree/master/example) so
you can see how they work in practice.

As is often the case, most of the work in this project involved making the
system _less_ complicated. I also spent a lot of time
[documenting the final design](https://github.com/pkgw/worklog-tools/#readme).
Finally, I also worked to put together some LaTeX templates that I think are
quite nice — you can {{ulink(path="files/pubs.pdf", text="judge the results")}} for
yourself.

Is any of this relevant to you? Yes! I sincerely think this system is
straightforward enough that normal people would want to use it. A tiny bit of
Python hacking is needed for certain kinds of changes, but
[the code is very simple](https://github.com/pkgw/worklog-tools/blob/master/worklog.py).
I think my templates are pretty nice — and I’m happy for people to use them.
(If nothing else, you might learn
[some nice LaTeX tricks](https://github.com/pkgw/worklog-tools/blob/master/example/cv.tmpl.tex#L23).)
Finally, I think the value-add of being able to do things like collect
citation statistics is pretty nice — and of course, you can build on this
system to do whatever “career analytics” you want. For instance, I log all of
my submitted proposals, so I can compute success rates, total time allocated,
and so on.

The [README on GitHub](https://github.com/pkgw/worklog-tools/#readme) has many
more details, including instructions about how to get started if you want to
give it a try. I hope you enjoy!

**By the way:** [INI format](http://en.wikipedia.org/wiki/INI_format) is
_highly_ underrated as a simple data format. It’s almost as underrated as XML
is overrated.

**By the way #2:** Of course, nothing in this system makes it specific to CV’s
— with different templates and log files, you can insert structured data into
any kind of document.

**By the way #3:** Patches and pull requests are welcome! There are a zillion
features that could be added.

**By the way #4:** A lot of this work was probably motivated by the fact that
my name isn’t very ADS-able — a search for
[P Williams](https://scixplorer.org/search?p=1&q=author%3A%22P+Williams%22&sort=score+desc&sort=date+desc&d=astrophysics)
pulls in way too many hits, and though I can’t get a search for exactly
[PKG Williams](https://scixplorer.org/search?d=astrophysics&p=1&q=author%3A%22P+K+G+Williams%22&sort=score+desc&sort=date+desc)
to work, I have a fair number of publications without the middle initials.
