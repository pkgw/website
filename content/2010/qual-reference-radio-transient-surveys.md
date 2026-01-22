+++
date = 2010-07-27T19:29:32Z
title = "Qual Reference: Radio Transient Surveys"
path = "2010/07/qual-reference-radio-transient-surveys"

[extra]
wp_rel_permalink = "/2010/07/qual-reference-radio-transient-surveys/"
wp_shortlink = "/?p=301"
+++

I’ve already tabulated a bunch of surveys in the
[scientific merit post](@/2010/qual-scientific-merit.md), but here I’ll add
some extra information on a few of the key ones. I’ve attempted to compute
quantities as uniformly as possible, but this can be pretty difficult. Carets
denote quantities I’ve estimated indirectly myself, rather than gotten fairly
directly from one of the references.

<!-- TODO: turn into a real table? But I don't want to fight with Markdown
tables right now. -->

```
Quantity                  AGCTS  Hyman+/GMRT  Becker+/VLA  Bower+/VLA
Total time (h)            ~200^  66           ~200^        315^
FWHM (deg)                1.13   2            0.17^        0.17, 0.1^
Freq (GHz)                3.09   0.235        4.86         5, 8.4
Snapshot RMS (mJy/bm)     ~35^   3-10         0.3^         0.04-0.05
Snapshot duration (m)     ~5^    138          ~1.5^        ~20
2-epoch eff. area (deg²)  854    69           23.2         30.97
```

Still not quite sure that I’m compute the 2-epoch effective area correctly,
and I’m definitely sure that I’m being sloppy since that metric is a
technically function of the detection limit. I’m basically taking
`N_snapshot * A_snapshot` for that assessment. (Hand-written table in not-ATA
notebook #1, p. 73.)

(Survey references are:
[Hyman+ GMRT](https://scixplorer.org/abs/2009ApJ...696..280H),
[Becker+ VLA](https://scixplorer.org/abs/2010AJ....140..157B),
[Bower+ VLA](https://scixplorer.org/abs/2007ApJ...666..346B).)
