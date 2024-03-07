+++
date = 2024-03-07T10:53:26-05:00
title = "Sneak Peek: daschlab"
+++

[Recently](@/2024/fun-python-filtering-pattern.md) I mentioned that I’ve been
working on a Python package called [daschlab], which will be the recommended
analysis toolkit for [DASCH] data. It’s designed for interactive data
exploration, so I thought that I’d make a video giving a sense of what it’s
like.

[daschlab]: https://github.com/pkgw/daschlab
[DASCH]: https://dasch.cfa.harvard.edu/

<!-- more -->

Here it is!

{{ youtube(id="GofXy8BZxjY", aspect="4by3") }}

I haven’t yet written up a *lot* of the needed documentation, but if you’re
feeling adventurous you can play with it today. Besides local installation,
[this MyBinder link][mybinder] will load up a JupyterLab environment with
daschlab installed. Any data that you download won’t persist, but you should be
able to try a few things out.

[mybinder]: https://mybinder.org/v2/gh/pkgw/daschlab/HEAD

There‘s also some [API reference documentation][apidocs], but these docs are
intentionally low-level; there no installation instructions, tutorials, etc. That
kind of stuff will end up on [the main DASCH website][DASCH].

[apidocs]: https://daschlab.readthedocs.io/

In the course of making this video I went through my quasi-annual revisitation
of what it takes to do efficient desktop video capture on Linux; a lot of that
stuff is extremely hardware- and distribution- specific, but if you want to see
what a glutton for punishment I am, the gory details are documented on my
[Capture Video Efficiently on
Linux](@/howto/capture-video-efficiently-on-linux.md) and [Get a Standardized
Browser Window](@/howto/get-a-standard-browser.md) HOWTOs.
