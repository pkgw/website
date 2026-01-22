+++
date = 2015-11-19T09:23:54Z
title = "“The First Millimeter Detection of a Non-Accreting Ultracool Dwarf”"
path = "2015/11/the-first-millimeter-detection-of-a-non-accreting-ultracool-dwarf"

[extra]
wp_rel_permalink = "/2015/11/the-first-millimeter-detection-of-a-non-accreting-ultracool-dwarf/"
wp_shortlink = "/?p=967"
+++

Our latest paper is finally out! It’s called
“[The First Millimeter Detection of a Non-Accreting Ultracool Dwarf](http://arxiv.org/abs/1511.05559)”
and is accepted to
[The Astrophysical Journal](http://iopscience.iop.org/0004-637X/). The Center
for Astrophysics and the NRAO are putting out
[press releases](https://www.cfa.harvard.edu/news/2015-26)
[about it](https://public.nrao.edu/news/pressreleases/alma-dwarf-star-2015)
including [a cool animation](https://vimeo.com/145885859).

{% figure(path="wp/wp-content/uploads/2015/11/base-800-604x270.jpg") %}
Image credit: NRAO/AUI/NSF; Dana Berry / SkyWorks
{% end %}

This is a project that Edo and I did in collaboration with a group of brown
dwarf experts in the UK:
[Sarah Casewell](http://www2.le.ac.uk/departments/physics/people/academic-staff/slc25),
[Craig Stark](http://www-star.st-and.ac.uk/~crs21/),
[Stu Littlefair](http://slittlefair.staff.shef.ac.uk/), and
[Christiane Helling](http://leap2010.wp.st-andrews.ac.uk/christiane-helling/).
Craig is an expert in exotic plasma processes and came up with an idea about
how some of them might operate in the upper atmospheres of very cool stars and
brown dwarfs — in line with an overall physical model that Christiane has been
developing for quite a while. If so, these processes would cause these cool
stars to become quite bright at millimeter wavelengths (or, equivalently,
frequencies around 100 GHz; these are the sorts of wavelengths used in new
full-body airport scanners). The
[ALMA telescope](http://www.almaobservatory.org/) would be the perfect tool to
try to detect this emission, and just about two years ago Sarah got in touch
with me about collaborating on this project since I have experience working
with the long-wavelength interferometric data that ALMA produces.

ALMA has annual proposal deadlines and generally does things at a … stately …
pace. Sarah led up the proposal, which was eventually accepted, and ALMA
finally delivered our data in late April of this year.

Craig’s plasma processes are, well, quite exotic, so frankly we weren’t sure
if we were going to see anything. So I was quite excited when I took a first
look at the data and saw a strong detection of our target, a famous low-mass
object called
[TVLM 513–46546](http://simbad.u-strasbg.fr/simbad/sim-id?protocol=html&Ident=TVLM%20513-46546)!
(Well, it’s famous among people who care about exotic phenomena in ultracool
dwarfs.) Stuart promptly went out and obtained a night’s worth of
visible-light monitoring of our target, since it’s known to vary periodically
at visible wavelengths — if there were any variability in the ALMA data, it
would be interesting to see how it compared to the optical, and you want
contemporaneous data in order to be able to compare effectively.

Once we got to sit down and look at the data carefully, it didn’t seem as if
the emission matched Craig’s exotic plasma processes. Instead, it comes from a
different surprising source! It seems to come from the same general kind of
[synchrotron](https://en.wikipedia.org/wiki/Synchrotron_radiation#Synchrotron_radiation_in_astronomy)
process that is likely responsible for TVLM 513’s
[emission at centimeter radio wavelengths](https://scixplorer.org/abs/2002ApJ...572..503B/)
(at least its non-bursting emission, but that’s a story for another time),
just operating at much high energies. The physics of the synchrotron process
tell us that emission at ALMA wavelengths must be coming from very energetic
electrons — ones moving close to the speed of light — gyrating in fairly
strong magnetic fields. This is really awesome! It’s kind of incredible to me
that this big, relatively cold ball of gas can both self-generate strong
magnetic fields (in some places thousands of times stronger than the Earth’s)
and somehow channel its energy into accelerating electrons to at least 99% the
speed of light.

Even though you might think that TVLM 513 would be a fairly
dinky little object, our data help paint a picture in which it’s surrounded by
an intense radiation environment — much stronger than the Sun’s, in fact. This
is important since astronomers these days are very interested in finding
planets around low-mass red dwarfs. In order to be warm enough to have liquid
water, such planets would need to orbit fairly close to their host stars, but
this means that they’d also be that much closer to the kind of radiation
sources we observed with ALMA. It’s an active area of research to try to
understand how harmful this radiation, and related effects, would be for any
life on such planets.

Our finding is also very interesting physically since the electrons that
produce the ALMA emission probably lose all their energy in a matter of a few
hours. So, if these electrons are accelerated to such high energies only in
big bursty events, we got very lucky during our observation. On the other
hand, if the emission we observe turns out to be very steady, that implies
that the acceleration process is probably operating continuously at a low
level. The mechanism of this acceleration process is precisely one of the big
mysteries in this field, so I’m excited about the possibility of using the
time-dependence of the ALMA emission to learn more. In fact, there’s a hint in
our data that the ALMA emission had a bit of a burst in synch with the time
when the optical modulation would have been at its maximum. (We’re glad that
Stu got his data!) My best estimate is that there’s about a 7% chance that the
observed data are just a noise blip, which isn’t too high, but is higher than
you generally want when trying to make a scientific claim. To be honest, I’m
not quite sure what the explanation would be if these pieces of emission turn
out to be synchronized, but you can bet that I want to find out if they are.

Unfortunately (or is it
[ironically](https://en.wikipedia.org/wiki/Ironic_%28song%29)?) we got these
data the day before the 2015 ALMA proposal deadline, so we weren’t able to
submit a plan for follow-up observations. We’ll certainly submit one in 2016,
which means that hopefully we’ll get more data in 2017. I’d like to get data
sooner, but ALMA is the only telescope in the world that’s powerful enough to
get the data we need, so it’s hard to complain too much! This result opens the
door to a lot of new investigations. While there are lots of details that we’d
like to fill in about TVLM 513, I’d _really_ like to observe a larger sample
of similar objects and see how many can sustain this high energy acceleration
process. Properties of the emission such as its time-dependence and
polarization will fill in the physics and help us understand what’s driving
these processes, as well as painting a clearer picture of how this radiation
might affect habitability.

To be honest, there’s probably a small silver lining in the fact that we’ll
have to wait a bit to try to get more ALMA data. The way the ALMA schedules
observations right now is very rigid, in the sense that it’s hard to make
special requests: right now you can’t even request a single observation that
lasts longer than two hours. (I had a proposal for a four-hour observation
rejected as being “technically impossible”; not that I’m bitter.) The kinds of
observations that we’d really like to do — variability monitoring;
simultaneous observations with the
[Very Large Array](https://public.nrao.edu/telescopes/vla) — do not fit well
into that scheme at all. There are not-crazy reasons for the current way of
doing things, but I think it does preclude a lot of exciting science. We will
advocate for more astronomer-friendly rules and with luck we’ll have more
flexibility the next time we apply.
