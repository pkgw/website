+++
date = 2024-01-02T12:43:11-05:00
title = "A New Design for Interactive Data Visualization in JupyterLab"
+++

Over the past few years, I’ve put a lot of work into [WorldWide
Telescope][wwt]’s support for [Jupyter]. The motivation is pretty simple: a lot
of astronomers are already using Jupyter; I expect that that number will only
increase over time; and WWT and Jupyter fit really well together from the
standpoints of both broad design and the technical details of their
implementations. So, it really makes sense to lean into this “form factor” for
WWT if you want to get it into the hands of researchers — and I do!

[wwt]: https://worldwidetelescope.org/home/
[Jupyter]: https://jupyter.org/

WWT has had a Jupyter integration since 2017, when [Tom Robitaille][tom] started
working on a WWT-based Jupyter widget in the [pywwt] package. Going all the way
back to the first “IPython notebook” release in 2011 — before “Jupyter” even
existed — [widgets] have been the way that you incorporated graphical and/or
interactive elements into the notebook environment. But over the past few years,
I’ve become less and less satisfied with the user experience (UX) that we can
achieve by cramming a sophisticated app like WWT into the widget framework. The
work I’ve been doing recently has focused on whether we can do better, and I
believe that the answer is a resounding yes. The system that we’ve built for WWT
is described in the article *[A Novel JupyterLab User Experience for Interactive
Data Visualization][wcnw22]*, coauthored with Jon Carifio, Henrik Norman, and
David Weigel; it’s currently [available on ArXiv.org][wcnw22] and will appear in
the proceedings of [ADASS32].

[tom]: https://www.trobitaille.dev/
[pywwt]: https://pywwt.readthedocs.io/
[widgets]: https://ipywidgets.readthedocs.io/
[wcnw22]: https://arxiv.org/abs/2212.03907
[ADASS32]: https://www.adass2022.ca/

<!-- more -->

The key to our new approach is hidden in the title of the article: not just
Jupyter, but Jupyter*Lab*. The original IPython/Jupyter system was really
tightly focused on the “notebook” concept: a single electronic document
interleaving code, text, and graphics in a vertical stack. (As far as I know,
Mathematica was the first software to introduce this format, but I haven't dug
into its origins.) IPython’s contribution was to implement this design for
Python, *and* to do so using web technologies, which had gotten good enough that
it made sense to build the UI/UX using a browser-based framework rather than a
traditional desktop app system like [Qt]. The leap from IPython to Jupyter (the
latter being first released in
2014) was about expanding this notebook concept from being specific to the
Python language, to embracing a whole range of programming languages.

[Qt]: https://www.qt.io/

In my view, the jump from “plain Jupyter” to JupyterLab (first released in 2018)
is a much bigger change — although, from what I’ve seen the Jupyter community
doesn’t seem eager to promote it as such. Jupyter*Lab* isn’t just a tool for
working notebooks — it’s an entire environment for interactive scientific
computation on the web. It happens to include thorough support for notebooks,
but that support lives inside an application framework that is *much* richer and
more sophisticated than the one provided in plain Jupyter. You can get a sense
of this from the fact that the [@jupyterlab NPM namespace][jlnpm], which is
roughtly a collection of the modules that build up the main JupyterLab frontend,
currently consists of [160 different packages][jlnpm]. You can write [JupyterLab
extensions][jlext] that code against the APIs provided by all of those packages
to add new kernels, document types, menu items, and on and on and on. The
JupyterLab browser app integrates with its associated web server, which
integrates with the various computational kernels that it manages, and this
whole framework is extensible and pluggable into the usual menagerie of cloud
infrastructure tools.

[jlnpm]: https://www.npmjs.com/search?q=%40jupyterlab
[jlext]: https://jupyterlab.readthedocs.io/en/stable/user/extensions.html

What does this have to do with WWT? The key thing is that in JupyterLab, *you’re
not limited to providing interactive functionality only through notebook
widgets.* You can write JupyterLab extensions that provide interactive features
in a whole range of ways, if you want. And — you can probably see where this is
going — I don’t think that the notebook widget form factor is a great one for
interactive analysis apps like WWT. Specifically:

- At the most basic level, as you work in a notebook, widgets scroll off the
  screen. This is fine for certain kinds of plots, but for WWT’s interactive
  image display, it’s much more likely that you're going to want the app to
  remain in a view for a long time as you execute a whole series of cells.
- Widgets are generally small and inconvenient to resize. Once again, this isn’t
  *always* the case, but you’ll often want to make your WWT view really big to
  take in as much imagery as you can at once.
- The lifetime of a widget is intimately connected with the lifetime of the
  kernel backing its notebook. For an app as complicated as WWT, there’s
  basically no realistic way to re-establish its state when the backing kernel
  restarts, so that you’ll lose your work in frustratingly hard-to-reproduce
  ways if that happens.
- Jupyter’s built-in infrastructure for “connecting” computational kernels to
  widgets is a bit limited, especially in the area of bidirectional
  communications, which is something that WWT would like to use a lot.

These problems are individually solveable-ish in the notebook widget paradigm,
as demonstrated by hacks like [jupyterlab-sidecar] which lets you “pop out” a
widget so that it doesn’t scroll out of view. But as I thought about the kind of
experience that I wanted WWT-in-Jupyter to feel like, it felt like the widget
framework was causing problems everywhere I looked.

[jupyterlab-sidecar]: https://github.com/jupyter-widgets/jupyterlab-sidecar

Is it realistic to give up the widget paradigm, though? It turns out that in
JupyterLab, and with an app like WWT, it’s not a major undertaking … *if* you
are patient, familiar with a wide range of web technologies, and willing to do
some digging.

The vision first clicked for me when I read the [JupyterLab Extension
Tutorial][jltut], which (coincidentally) sets up a demo extension that pulls
images from [APOD]. The tutorial extension adds a new kind of tabbed document to
the JupyterLab install and fills it with web content, which is just the kind of
foundation that makes sense for integrating a WWT app! A first-draft “native”
WWT extension could “just” build on this tutorial and change out the APOD HTML
with a WWT iframe.

[jltut]: https://jupyterlab.readthedocs.io/en/latest/extension/extension_tutorial.html
[APOD]: https://apod.nasa.gov/apod/astropix.html

The bad news is that even this “easy” extension is still a fairly complex piece
of software to set up, spanning a range of languages and tools — feel free to do
the tutorial yourself and see if you agree! And while I am *deeply* sympathetic
to how hard this is, it felt like the relevant documentation was always missing,
incomplete, or even actively misleading. The good news is that once all of the
tooling is set up, an extension like this is conceptually pretty simple. You
call the right JupyterLab hooks and boom, you can add a tab to the environment
and insert your own web content.

On top of this foundation we built what we call the WWT “app” for JupyterLab.