+++
date = 2024-09-11T15:38:41-04:00 # deploytool
title = "Plug in a USB-A Device on the First Try"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

It’s a running joke that it takes three tries to plug in a USB-A device:

{% relfig(path="usb-meme.jpg") %}
From [HowToGeek][1] since it was the first version of this meme to come
up in my image search.

[1]: https://www.howtogeek.com/713525/the-usb-paradox-why-do-usb-connections-need-three-tries/
{% end %}

However, we don’t have to live this way. The key is that USB-A plugs and sockets
both have tops and bottoms. Align top-to-top, and you’re in business.

In many cases, there’s a natural up/down orientation to the object with the USB
socket, and it’s more likely that the socket mechanicals will respect this
orientation. So you rarely have to consciously think about the orientation of
the socket half of the pair. But if you need to, the interior sticky-outy part
(technical term) of a USB-A socket is at the top, as seen in this monitor stand:

{% relfig(path="socket-labeled.jpg") %}
Standard USB socket orientation.
{% end %}

I remember this as the socket being like a mouth and you’re going to stick a
thermometer under its tongue. Yum!

The plug side is more often the problem. Sometimes there’s some kind of labeling
or context clue that obviously indicates which side is the top, but not always.
A more reliable indicator of the plug’s orientation is that **the bottom of the
plug has the seam in the metal grounding sheath.** 

{% relfig(path="plug-labeled.jpg") %}
USB plug orientation guide.
{% end %}

There are, of course, exceptions. But once you know what to look for, you can
often get things right on the first try, and you can start noticing which
devices get things wrong. If you have a USB-to-wall-socket adaptor, make a point
of plugging it in using the standard orientation.

Just remember: **keep the seam down and you’ll never frown**.
