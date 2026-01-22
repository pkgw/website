+++
date = 2010-08-02T22:06:19Z
title = "Qual Reference: Transient Rate Measurements"
path = "2010/08/qual-reference-transient-rate-measurements"

[extra]
wp_rel_permalink = "/2010/08/qual-reference-transient-rate-measurements/"
wp_shortlink = "/?p=331"
+++

There are many measurements of radio transient rates out there in the
literature. Unfortunately, there are many different ways to quantify transient
rates and they tend to be very hard to intercompare. In lieu of trying to
standardize everything, I’ll just report some of the values that I’ve seen and
the way in which they’re measured:

- **Becker+ 2010**. The headline number is 1.6 Galactic sources per square
  degree which vary by more than 50% on a timescale of <~ years. They ignored
  scintillation. This is based on guessing how many XG sources were in the
  background. (Found ~4:1 G:XG.) They had to work around the fact that their
  two comparison images were made in different VLA configurations and so
  fluxes are not as easily comparable as you’d like. Found a strongly
  increasing variable fraction as they got closer to the GC, suggesting that
  the areal variable density there would be quite high; likely ISS.
- **Bower+ 07**. By far the most thorough job of thinking about / reporting
  transient rates. For short timescale, S > 370 uJy, 5 & 8.4 GHz, rate is
  between 0.07 and 40 per sqdeg per yr. (Timescale is between 20 min and 7 d)
  Two-epoch survey should get 1.5 ± 0.4 transients per sqdeg for S > 370 uJy.
  They have tables of effective area searched as a function of flux limit for
  various timescales. Based on two transients with galaxy hosts, compute
  volumetric rates of 3e5 / Gpc^3 / yr (t\_char = 7d) and 4e3 / Gpc^3 / yr
  (t\_char = 60d), or joint rate of (~2 ± 2)e5 / Gpc^3 / yr. Reported SN Ib/c
  rate is 4.8e4. Figure 9 summarizes their multi-depth results and those of
  other surveys:

  {% figure(path="wp/wp-content/uploads/2010/08/bowerfig9.png") %}
  From 2007 ApJ 666 346
  {% end %}
- **Hyman+ 2008 (GMRT)**. No effort to compute rates here. For the GMRT work,
  1 transient found in 66 hours of observing (22 epochs) with a typical
  limiting flux of ~20 mJy at 0.235 GHz. Snapshot FWHM of 2 deg gives you an
  equivalent 2-epoch area of 66 sqdeg. This is of course looking at the GC.
- **Gregory & Taylor 1986**. Nothing explicitly given on rates here, either.
  500 sqdeg surveyed at 4.76 GHz. 32 variables and 27 “possible variables”
  detected, about 11 of which are false positives, with 64% showing
  “short-term” variability. (Most sources are faint so variability implies
  changes in S of order 50%.) 1274 sources total -> ~3% variable fraction.
  Estimate >90% are XG. Some comparisons to early 80’s work that I’m going to
  ignore.
- **[Carilli+ 03](https://scixplorer.org/abs/2003ApJ...590..192C).** 19d
  and 17m timescales probed at 1.4 GHz. Find rate < 18 / sqdeg for S > 100
  uJy.
- [**Frail+ 03**](https://scixplorer.org/abs/2003astro.ph..9557F). Find
  four variables, no transients in GRB afterglow searches. Find R\_variable <
  6 / sqdeg.
- **Niinuma+ 07, Kuniyoshi+ 07, Matsumura+ 07** (see Bower+ 08 for refs).
  Bright transients in drift scans; three >1 Jy lasting < 1d, one > 3Jy
  lasting ~3d; not widely believed.
- **[Levinson+ 02](https://scixplorer.org/abs/2002ApJ...576..923L),
  [Gal-Yam+ 06](https://scixplorer.org/abs/2006ApJ...639..331G)**.
  FIRST/NVSS comparisons. 2400 sqdeg at S > 6 mJy and 1.4 GHz. Limit that
  there are < 65 RTs in entire sky above 6 mJy at once (95% CL). Limit
  volumetric rate < 1000 / Gpc^3 / yr. (Typo in abstract: Gpc^1 not Gpc^-3.)
  Predict ~1000 in similar 1/17-sky survey if S > 1 mJy.
