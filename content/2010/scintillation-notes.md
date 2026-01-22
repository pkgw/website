+++
date = 2010-05-21T15:44:51Z
title = "Scintillation Notes"
path = "2010/05/scintillation-notes"

[extra]
wp_rel_permalink = "/2010/05/scintillation-notes/"
wp_shortlink = "/?p=205"
+++

I gave myself a crash course on interstellar scintillation (ISS) today.
(Sounds fancier than “twinkling”.) For posterity, here are some quick notes of
the key results I found.

First of all, scintillation _is_ twinkling. For my work, it’s in the radio and
caused by plasma in the ISM. The Narayan paper (below) was a very helpful
reference for booting up.

The basic model is an infinitely distant point source with incoming plane
waves. There’s a zero- thickness phase screen at distance D defined by its
phase change as a function of position phi(x,y). We’re observing at wavelength
lambda. The formal expression for the effect of the scattering screen is the
Fresnel-Kirchoff integral (cf Narayan eq 2.1):

![\psi(X,Y) = \frac{e^{-i\pi/2}}{2\pi r_F^2}\int\int exp\left[i \phi(x,y) + i \frac{(x-X)^2 + (y - Y)^2}{2r_F^2}\right]dx dy](https://s0.wp.com/latex.php?latex=%5Cpsi%28X%2CY%29+%3D+%5Cfrac%7Be%5E%7B-i%5Cpi%2F2%7D%7D%7B2%5Cpi+r_F%5E2%7D%5Cint%5Cint+exp%5Cleft%5Bi+%5Cphi%28x%2Cy%29+%2B+i+%5Cfrac%7B%28x-X%29%5E2+%2B+%28y+-+Y%29%5E2%7D%7B2+r_F%5E2%7D%5Cright%5Ddx+dy&bg=ffffff&fg=000000&s=0 "\psi(X,Y) = \frac{e^{-i\pi/2}}{2\pi r_F^2}\int\int exp\left[i \phi(x,y) + i \frac{(x-X)^2 + (y - Y)^2}{2 r_F^2}\right]dx dy")

Here r_F is the Fresnel length, which can easily be interpreted as an angle at
the distance of the screen:

![r_F = \sqrt{\lambda D / 2 \pi}](https://s0.wp.com/latex.php?latex=r_F+%3D+%5Csqrt%7B%5Clambda+D+%2F+2+%5Cpi%7D&bg=ffffff&fg=000000&s=0 "r_F = \sqrt{\lambda D / 2\pi}")

![\theta_F = \sqrt{\lambda / 2 \pi D} \ll 1 \textrm{ for this to work}](https://s0.wp.com/latex.php?latex=%5Ctheta_F+%3D+%5Csqrt%7B%5Clambda+%2F+2+%5Cpi+D%7D+%5Cll+1+%5Ctextrm%7B+for+this+to+work%7D&bg=ffffff&fg=000000&s=0 "\theta_F = \sqrt{\lambda / 2 \pi D} \ll 1 \textrm{ for this to work}")

The Fresnel length is the transverse displacement on the scattering screen
that causes a change in the arrival phase of the incoming wave due to path
length differences. There’s a right triangle with adjacent of length D and
opposite of length r_F, and you’re thinking about the phase change of the
hypotenuse versus the adjacent. Assuming a gentle (or absent) phase screen,
the dominant contribution to the integral comes from within r_F, where the
phase isn’t changing much. As you get farther out, phase variations become
rapid and signals tend to interfere and cancel.

For ISS and other natural cases, the phase screen has some length scale r_d on
which it affects phases by ~1 radian. The usual assumption is that the phase
structure function is Kolmogorov-distributed

![D_\phi(x,y) = \langle[\phi(x'+x,y'+y) - \phi(x,y)]^2\rangle_{x',y'} = (r / r_d)^{5/3}](https://s0.wp.com/latex.php?latex=D_%5Cphi%28x%2Cy%29+%3D+%5Clangle%5B%5Cphi%28x%27%2Bx%2Cy%27%2By%29+-+%5Cphi%28x%2Cy%29%5D%5E2%5Crangle_%7Bx%27%2Cy%27%7D+%3D+%28r+%2F+r_d%29%5E%7B5%2F3%7D&bg=ffffff&fg=000000&s=0 "D_\phi(x,y) = \langle[\phi(x'+x,y'+y) - \phi(x,y)]^2\rangle_{x',y'} = (r / r_d)^{5/3}")

but the precise distribution doesn’t seem to matter so long as D_phi increases
with r and there’s some characteristic scale r_d.

There are two main scintillation cases: weak and
strong.

If r_d > r_F, we have weak scintillation. As mentioned above, most of the
contribution to the received flux comes from within r_F, and the phase screen
is nearly constant over that scale. The fluctuations will mostly be on this
spatial scale, and so if the scattering screen is moving transversely at
velocity v, the characteristic timescale will be r_F / v. A point source will
broaden to the Fresnel angular scale,

![\theta_s \approx \theta_F \approx \sqrt{\frac{\lambda}{D}}](https://s0.wp.com/latex.php?latex=%5Ctheta_s+%5Capprox+%5Ctheta_F+%5Capprox+%5Csqrt%7B%5Cfrac%7B%5Clambda%7D%7BD%7D%7D&bg=ffffff&fg=000000&s=0 "\theta_s \approx \theta_F \approx \sqrt{\frac{\lambda}{D}}")

If r_d < r_F, we have strong scintillation. The incoming phase is stirred up
even within the Fresnel patch, so r_F basically is irrelevant. The “size” of a
scatterer is the phase coherency length, so the typical angular scale is

![\theta_s = \frac{\lambda}{r_d}](https://s0.wp.com/latex.php?latex=%5Ctheta_s+%3D+%5Cfrac%7B%5Clambda%7D%7Br_d%7D&bg=ffffff&fg=000000&s=0 "\theta_s = \frac{\lambda}{r_d}")

This is what point sources get scatter-broadened into, and it’s bigger than
theta_F. (Smaller scatterer -> more diffraction.)

Strong scintillation has contributions from two components. First of all,
there’s the variation on those r_d scales — this is “diffractive strong
scintillation”. A source of angular size

![\theta_d > \frac{r_d}{D}](https://s0.wp.com/latex.php?latex=%5Ctheta_d+%3E+%5Cfrac%7Br_d%7D%7BD%7D&bg=ffffff&fg=000000&s=0 "\theta_d > \frac{r_d}{D}")

has larger angular extent than the phase coherency scale and so crazy things
start happening (cf Narayan). Note that this is not the same as theta_s: it is
much smaller. The fluctuation timescale for a pointlike source here with
transverse motion is r_d / v. Narayan says this effect is narrowband.

There’s also “refractive strong
scintillation”. This is because a point source has that angular size theta_s,
which backprojects to

![r_r = \theta_s D = r_F^2 / r_d \gg r_d ](https://s0.wp.com/latex.php?latex=r_r+%3D+%5Ctheta_s+D+%3D+r_F%5E2+%2F+r_d+%5Cgg+r_d+&bg=ffffff&fg=000000&s=0 "r_r = \theta_s D = r_F^2 / r_d \gg r_d ")

and so is sensitive to variations on that lengthscale in the scattering
screen. There’s a consequent timescale of r_r / v. Narayan says this effect is
broadband.

# Takeaways

- Scintillation tends to the weak case as the frequency gets higher.
- Scintillation damps as source gets larger. If weak scintillation, nu ~ few
  GHz, D ~ 100 pc, the angular size upper limit is ~10s of microarcsec. (This
  limit can be used to calculate brightness temperature lower limits.)
- You see annual variation in the amount of scintillation for some sources if
  the transverse v of the screen is not >> the Earth orbital velocity.

# References

- Great conceptual paper: Narayan 92:
  [ADS](https://scixplorer.org/abs/1992RSPTA.341..151N),
  [journal](http://rsta.royalsocietypublishing.org/content/341/1660/151.abstract)
- [Wikipedia Fresnel Diffraction](http://en.wikipedia.org/wiki/Fresnel_diffraction)
- Linking ISS and IDV: Dennett-Thorpe & de Bruyn 02:
  [ADS](https://scixplorer.org/abs/2002Natur.415...57D),
  [Journal](http://www.nature.com/nature/journal/v415/n6867/full/415057a.html)
