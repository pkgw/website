+++
date = 2024-02-23T14:36:31-05:00
title = "A Fun Python API Pattern for Filtering"
+++

The past few weeks, I’ve been working away on a new Python package called
[daschlab], which will be the recommended analysis toolkit for [DASCH] data. As
part of this, I’ve come up with what feels like a nice solution to a problem
that’s annoyed me several times in the past: how do we provide a nice
user-friendly API for filtering and subsetting data collections?

[daschlab]: https://github.com/pkgw/daschlab
[DASCH]: https://dasch.cfa.harvard.edu/

<!-- more -->

To be more specific: one of the core data types in [daschlab] is a `Lightcurve`,
which is just a wide [Astropy Table][at] with a bunch of photometry data about a
source. In order to do their analysis, users are going to want to extract
subsets of the lightcurve based on any one of a variety of dimensions: the date
of the measurement, the telescope that was used, the plate emulsion, quality
flags, and on and on.

[at]: https://docs.astropy.org/en/stable/table/

Users are *also* going to want to do a lot of different things with those
subsets. They might want to remove selected rows from the table. They might want
to remove *un*-selected rows from the table. They might want to plot those rows.
They might want to apply some kind of tag to those rows.

The same consideration comes up with other kinds of data collections as well. In
general, you want to support both a lot of different ways of subsetting, and a
lot of different actions. But you also don't want your API to become too
“indirect”: it’s a lot nicer for users to be able to type
`lightcurve.do_something_concrete()` without needing to create a bunch of
intermediate variables along the way, or call multiple functions:

```python
lc = Lightcurve(...)
lc.remove_nondetections()      # nice
lc.remove(lc.nondetections())  # gross, redundant
lc.nondetections().remove()    # less redundant, still not as nice
```

But if we start providing functions like `lc.remove_nondetections()`, then we
quickly reach a multiplicative explosion: the natural progression is to add
`lc.keep_nondetections()`, `lc.count_nondetections()`,
`lc.count_between_dates()`, and so on. That’s clearly not sustainable.

The approach that I’ve devised looks like this:

```python
lc = Lightcurve(...)
lc.drop.nondetections()
lc.count.nondetections()
lc.count.beween_dates(d1, d2)
```

This has a nice subject-verb-object structure and is super amenable to chaining:

```python
lc.keep_only.between_dates(d1, d2).count.detections()
```

This pattern is really easy to implement, too. The approach that I’ve taken is
as follows. An action like `drop` is a method that returns a lightweight
`Selector` object:

```python
class Lightcurve(Table):
    # [...]

    @property
    def drop(self) -> Selector:
        return Selector(self, self._apply_drop)

    def _apply_drop(self, selection: np.array) -> Lightcurve:
        # reality is a bit more complicated, but basically:
        return self[~selection]
```

The selector worries about different subsetting operations, but hands things
back to the original object to actually take action:

```python
class Selector:
    def __init__(self, parent: Lightcurve, action: Function):
        self._parent = parent
        self._action = action

    def detections(self, **kwargs):
        selection = np.isfinite(self._parent["flux"])
        return self._action(selection, **kwargs)
```

The `selection` is always expressed as a boolean array. This is going to be
inefficient sometimes, but establishing that invariant makes a lot of the
surrounding logic *so* much simpler. By always accepting and forwarding
`**kwargs` in the subsetting functions, we have a generic way to provide
information to the action/application function. For instance:

```python
class Lightcurve(Table):
    @property
    def tag(self) -> Selector:
        return Selector(self, self._apply_tag)

    def _apply_tag(self, selection, name: str = None):
        if not name:
            raise ValueError(name)

        self[name] |= selection

# [...]

lc
  .keep_only.between_dates(1920, 1930)
  .tag.detections(name="roaring_twenties")
```

I really like this pattern because it feels really extensible while maintaining
a pleasing directness. Adding new actions and filters can be incredibly easy:

```python
class Lightcurve(Table):
    @property
    def count(self) -> Selector:
        return Selector(self, lambda s: s.sum())

class Selector:
    def brighter_than(self, limit, **kwargs):
        selection = self._parent["flux"] > limit
        return self._action(selection, **kwargs)
```

I dunno — this all feels kind of obvious, but I haven’t seen this pattern before
(although I’m sure it’s not at all novel), and I had to sit down and think for
a while before I thought of this design.

You might note that we don’t have a great way to do boolean logic on subsets. A
few extensions can make it possible, although not quite beautiful:


```python
class Lightcurve(Table):
    @property
    def match(self) -> Selector:
        return Selector(self, lambda s: s)

class Selector:
    def where(self, selection, **kwargs):
        return self._action(selection, **kwargs)

# [...]

lc.drop.where(
    lc.match.brighter_than(100) & ~lc.match.between_dates(1900, 1905)
)
```

I think the key to this pattern is something that I alluded to above — it covers
the N×M space of subset-action combinations with what looks like a single
function call, rather than two. We pull this off with two tricks. First, we
express the desired actions as objects (initialized `Selector` instances), not
functions, even though the latter choice is the “obvious” one. Second, by saying
that selector methods will forward their keyword arguments back to the
applicator function, one function call can straightforwardly provide any needed
parameters to both “halves” of the operation: positional arguments to the
subsetting piece, keyword arguments to the action piece.
