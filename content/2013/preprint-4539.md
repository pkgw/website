+++
date = 2013-08-20T10:59:45Z
title = "Prosser & Grankin, CfA Preprint 4539: A Bibliographic Story"
path = "2013/08/preprint-4539"

[extra]
wp_rel_permalink = "/2013/08/preprint-4539/"
wp_shortlink = "/?p=754"
+++

A little while ago I was collecting published data on the rotation of Sun-like
stars. As one often finds, there are helpful papers out there that compile
lots measurements into a few big tables. If you’re going to be thorough (and
of course you are!), a standard part of the job is chasing down the references
to make sure that you honestly say that you know where all of the data came
from. [Pizzolato _et al_. 2003](http://dx.doi.org/10.1051/0004-6361:20021560)
is one of these compilation papers, and a very nice one at that. It has very
good referencing, which was why I was surprised to see that some of the
measurements were attributed to

> Prosser, C. F., & Grankin, K. N. 1997, CfA preprint, 4539

Not a respectable paper in a respectable refereed journal, but some random
preprint. Now, cutting-edge work can genuinely rely on results so new that
they haven’t yet appeared in print, but that’s not quite the case here:
Pizzolato _et al._ is from 2003, while the preprint citation is to 1997.
Presumably the work hadn’t been “in preparation” for six years.

This kind of thing generally happens when authors are reading a paper, find
some results that they want to use, and just blindly copy the reference down
for use in their own work. Citing an old preprint, like in this case, is a bit
of a red flag: it’s an academic no-no to be citing things that you haven’t
actually read, and if you’d actually read the paper, you’d have updated the
citation to refer to its published form.

In the spectrum of academic sins, I consider this one to be relatively minor,
and I suspect it’s committed pretty often. It’d be nice if Pizzolato _et al._
had chased down the refereed paper that the preprint became, but sometimes
people get lazy. There’s still enough information to dig up the published
article, so it’s an inconvenience but not much worse. If this kind of
blindly-copied reference gets propagated through several generations of papers
(which I _have_ seen before), you run the risk of a game-of-telephone
situation where the meaning of the original work is warped; but in this case
the reference is just providing measurements, which ought to propagate from
one generation to the next pretty reliable.

Since I expected to use the relevant data, and I did feel like I should
double-check that, say, the named preprint _even exists_, I set out to hunt
down the final published form. In this day and age this is usually easy:
everything modern is on [ArXiv](http://arxiv.org/), where preprints almost
always get cross-linked to their corresponding published paper when the latter
appears. (I figure this must happen more-or-less automatically, since most
authors don’t bother to fill in that information after they’ve made their
posting.) Older preprints are more work, but generally not hard to track down
thanks to [ADS](http://adsabs.harvard.edu/). If the citation has a title,
you’re pretty much set; if not, it’s easy to search for something published by
the lead author around the time of the preprint. No big deal.

I started doing my usual ADS searches: no “Prosser & Grankin, 1997”. No
“Prosser & Grankin, 1998”. The “Prosser et al., 1998” papers clearly weren’t
relevant. Hmm. A little bit more searching, and in fact there are _no_ papers
in the ADS database with both Prosser and Grankin as coauthors. Authorship
lists can change between preprint-dom and publication, sure, but it’d be
pretty surprising if “Prosser & Grankin” somehow became “Prosser &
not-Grankin”. Some kind of dramatic falling-out?

I decided to take another tack. Googling had revealed that there wasn’t
anything like an online database of the CfA preprint series, but the CfA
library sits two floors under my office. If anyone knows how to look up items
in the CfA preprint series, it’d better be them. And compared to some academic
services (_cough_ IT), I’ve virtually always had great “customer service”
experiences with libraries — I imagine librarians are awfully strongly
motivated (for a variety of reasons) to show how much more effective they can
be than the Google search box.

Sure enough, just a day after I submitted a question in the CfA “Ask a
Librarian” online form, I got a phenomenally helpful response from Maria
McEachern. She had fetched the hardcopy preprint from the Harvard Depository,
sent me {{ulink(path="wp/wp-content/uploads/2013/08/cfa4539.pdf", text="a nice OCR’d scan of it")}},
attempted to chase down the published form (also coming up empty), and even
emailed the coauthor, Konstantin Grankin, to see if he could he could shed
some light on the situation.

With a digital copy of the paper in hand, I felt confident. If the paper had
been published somewhere, we’d definitely be able to track it down. If not, we
had enough information to create a record in the ADS and upload the paper
text, so that the text of the preprint would be easily accessible in the
future — which, after all, is the whole point of providing precise references.

I had doubted that we’d ever hear anything from Grankin, but he actually
replied in just a few days, filling in the last missing piece of the puzzle:

> Dear Maria,
>
> Unfortunately, Charles Franklin Prosser, the talented scientist, was lost in
> automobile accident in August 1998. […] Some papers have not been published
> because of this tragedy. That paper about which you ask, has not been
> published also. […]

Konstantin’s email included a link to
[Charles Prosser’s AAS obituary](https://baas.aas.org/pub/charles-franklin-prosser-jr-1963-1998/),
written by his frequent collaborator John Stauffer. It makes for compelling
reading in its way:

> […] Charles worked harder and put in longer hours than all but a few
> present-day professional astronomers. He could usually be found at the
> office seven days a week, among the first to arrive and the last to leave.
> Charles enjoyed harvesting astronomical data both from long observing runs
> on mountain tops and from rarely-read observatory publications. He was
> conservative in both his personal and his professional life; he very much
> preferred simply to state the results of his observations and to make as few
> extravagant interpretations of those observations as possible.
>
> […] Charles was survived by his parents, Charles Franklin Prosser, Sr. and
> Lucy Hogan Prosser, of Suwanee, Georgia, his sister Evelyn, and other
> relatives. His scientific papers will be offered by the family to NOAO.
> Charles was buried in Monterville, West Virginia on August 22, 1998. He will
> be remembered as a kind, dedicated, and highly moral colleague, whose
> greatest desire was to be allowed to work 12 hours a day in the field to
> which he devoted himself entirely.

Well, it’s hard to leave things half-finished after reading that. So I used
the obscure
[ADS Abstract Submission](http://adsabs.harvard.edu/adsfeedback/submit_abstract.html)
form to submit a record and a copy of the paper text, resulting in the
creation of ADS record
[1997cfa..rept…..P](http://adsabs.harvard.edu/abs/1997cfa..rept.....P). This
achieves the most important thing to me: making it so that the preprint text
is available online in a durable way. There’s also a subtle importance to the
fact that the preprint is now integrated into the ADS citation system. Part of
it is a certain imprimatur: _this is a real publication, that you can
legitimately cite_. But there’s also something about how ADS bibcodes (the
`1997cfa..rept.....P` identifiers) are essentially our community’s vocabulary
for talking about our literature. Without a bibcode and its backing record,
it’s hard to use, share, discuss, or really do anything with a publication,
even if you want to. (Side note: ADS is an _incredibly important service in
the field_, and should get tons of money!)

I did some searching (mostly in Google Scholar, I must admit) and found
[12 citing papers](http://adsabs.harvard.edu/cgi-bin/nph-ref_query?bibcode=1997cfa..rept.....P&refs=CITATIONS&db_key=AST):
pretty good for an unpublished preprint whose full text was virtually
inaccessible until now. Those links have been added to ADS so that anyone
reading one of the citing papers will easily be able to pull up the record and
text of the preprint.

The ADS copy of the fulltext unfortunately loses the OCR in Maria’s PDF,
making it so you can’t easily copy and paste the text and (more importantly)
data tables. (_**Update**: ADS has fixed this; I had assumed that if the OCR
disappeared in the first place, their system didn’t allow for this._) So I’ve
submitted the tables to the [CDS](http://cds.u-strasbg.fr/) in the hopes that
they’ll host a more easily-usable version of the data. CDS has a stated policy
of only accepting data from papers published in refereed journals, but these
circumstances are clearly exceptional; I’m waiting to hear a response about
this case. If they accept the data, that should integrate the preprint into
the [Simbad](http://simbad.u-strasbg.fr/simbad) system, further increasing its
visibility.

I’m not sure if there’s a moral to this story, besides the fact that I have
some OCD tendencies. Twelve citations is, frankly, not a huge number, and the
very fact that there was a Mystery of Preprint 4539 signals its narrow reach:
something more important would already have been integrated into ADS and the
full-fledged literature, one way or another. But the research enterprise is
built out of lots of tiny steps, not all coming in the form of new
experimental results; I’d like to think that this is one of them.

Here’s the
{{ulink(path="wp/wp-content/uploads/2013/08/cfa4539.pdf", text="full text of CfA Preprint 4539")}},
including the OCR information so that the text and tables can be easily
copied. (**Update**: this is now fully redundant with the ADS version.) Here’s
{{ulink(path="wp/wp-content/uploads/2013/08/cfa4539-data-preliminary.tar.gz", text="a preliminary version of the data tables")}}
that I submitted to the CDS — there are likely improvements to be made in the
metadata to match the CDS formats, but the data should be fully usable and
complete.
