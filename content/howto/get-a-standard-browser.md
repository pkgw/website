+++
date = 2024-03-06T12:08:06-05:00
title = "Get a Standardized Browser Window"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

To demonstrate an interactive webapp, it can be very helpful to record a
screencast of its usage. If you just pop open your day-to-day web browser and
record something, it *might* be fine, but there are two concerns. First is
privacy/professionalism: if you type something in the URL bar, say, it will
likely autocomplete items from your personal history. Second is repeatability:
if you’re making a series of related videos, or updating an existing one,
ideally the browser environment would change as little as possible; the window
size in particular should stay the same.

Here are some steps to create a repeatable browser environment using Firefox.
Nothing here is magic; there are surely other approaches that would also work,
but this is what I do.

The key is that Firefox takes a command-line argument `--profile <path>` that
lets you specify a custom input profile directory. The basic idea is to set up
an extremely generic browser profile in a custom directory, then archive it.
Every time you want to record a video, unpack a copy of the archive, use it, and
then throw it away.

This approach can of course be extended: you can create any number of
standardized profiles, and they can be tweaked to save logins to relevant
webservices, use project branding, etc.


# Creating a Standardized Browser Profile

1. In some kind of work directory, `mkdir standard`
1. Run `firefox --profile standard`
1. Skip all personalization steps.
1. Ctrl-Shift-B to hide the bookmark bar.
1. Remove superfluous icons/features from the address bar.
1. Right-click to “Customize Toolbar” and remove more stuff, and flexible spaces
   around the address bar.
1. Open up the Settings screen.
1. Set homepage and new tab page to Blank.
1. Disable all “Home” content.
1. In Search, set the default engine to Wikipedia.
1. Disable search suggestions and address bar suggestions.
1. Remove as many default search shortcuts as possible.
1. In Privacy & Security, turn off “Ask to save passwords”.
1. Turn off Autofill options.
1. Turn *off* popup blocking (seems like the better default for this use case?).
1. Follow the steps for standardizing the window size below.
1. Quit Firefox.
1. In a terminal, run `sqlite3 standard/places.sqlite`. We’ll clear some tables.
   The list here comes from looking at the schema and checking which tables are
   non-empty by default.
    1. `DELETE FROM moz_origins;`
    1. `DELETE FROM moz_places;`
    1. `DELETE FROM moz_historyvisits;`
    1. `DELETE FROM moz_bookmarks;`
    1. `DELETE FROM moz_keywords;`
    1. `DELETE FROM moz_anno_attributes;`
    1. `DELETE FROM moz_annos;`
    1. `DELETE FROM moz_places_metadata;`
1. `rm -rf standard/cache2`
1. Tar up `standard` and save the resulting archive somewhere.


# Using the Standardized Profile

1. Unpack the archived `standard` tree
1. Run `firefox --profile standard`
1. If in doubt, follow the steps for standardizing the window size below.


# Standardizing the Window Size

1. Ctrl-Shift-I to open devtools
1. Pop the devtools out to their own window
1. F1 to enable devtool options
1. Activate “Toggle rulers for the page”
1. Click the new right-angle ruler icon in the devtools header
1. Resize the window so that the content area is 1280×875 px (see below)
1. Close devtools

The size here is chosen so that the final window with toolbars and decorations
has dimensions of 1280×960, which is a 4:3 aspect ratio. 

You can figure out the padding by setting the browser content area to a
reasonable size, then taking a screenshot of the browser window and looking at
the size of the resulting image.

**However**, when using X.org, which is necessary to efficiently capture video,
the final window size includes a fair amount of transparent blank space
surrounding the actual window — I believe this is used for drop shadows and
such. I can configure OBS to crop away this border, and this is desirable to
avoid black borders around the video edge. **Also**, if the window is on a HiDPI
display, various pixel sizes are doubled — we'd be aiming for 2560×1920.

Currently, on my laptop’s display, which is HiDPI, the X.org window border
consists of 46 px at top (in HiDPI pixels), 52 px left and right, and 58 px at
bottom. This means that the target window size we're actually going for, in
terms of what a screenshot will yield, is 2664×2024. On an attached regular-DPI
monitor, all of these measurements are halved.

Compared to the devtools content-area size readout, the *cropped* window size
has the same width, but is 85 non-HiDPI px taller, at the moment.


# Bonus: Setting up OBS

The primary use case for all this is making [OBS] video captures. Here are some
notes about the setup of that.

[OBS]: https://obsproject.com/

1. See [the video capture howto][vidcap] for notes about proper video encoding
   settings. In theory that is all separate from the issues considered here.
1. Set up a “window capture” device to capture the browser window.
1. Set it up to crop as above: 46 / 52 / 58 / 52 px (in CSS ordering) if HiDPI;
   half those values if not. There is a "source screenshot" functionality that
   you can use to check the results. (Crop one pixel smaller on each axis and
   open in Gimp to verify that there is just 1px of border on all edges.)
1. Use the “Edit Transform” (Ctrl-E) box to check the size of the stream, post-crop.
1. Use "Resize output (source size)" to match the output to the window.
1. If you're going to overlay a webcap feed, how about putting it in the
   top-right, 24 px from the edges? And giving it a size of 412×232 px? (These
   measurements assuming HiDPI).
1. Finally, use `ffprobe` to check the pixel dimensions of a test recording.

[vidcap]: @/howto/capture-video-efficiently-on-linux.md


# See Also

- [How to capture video efficiently on Linux][vidcap]
