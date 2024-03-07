+++
date = 2021-12-13T09:43:01-05:00
title = "Capture Video Efficiently on Linux"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

Sometimes you want to record a video of your desktop screen on Linux, perhaps to
create a prerecorded presentation. With modern screen resolutions, it can be
super compute-intensive to encode such a video, to the extent that it's worth
taking special steps to make sure the recording process works well.

On my machine (2020 Dell XPS 15 9500), the most important thing is to use
hardware-accelerated video encoding using the discrete NVidia graphics card.
This (currently?) requires use of the proprietary NVidia kernel module and
X.org. After setting everything up the workflow is:

1. Consider hardware setup: fancy microphone; external keyboard and mouse;
   physical isolation between the mike and other items.
1. Reboot machine.
1. Select one of the kernel variations with the NVidia kernel module.
1. Before actually booting, check the kernel command line and make sure that
   magic arguments are there: `rd.driver.blacklist=nouveau modprobe.blacklist=nouveau nvidia-drm.modeset=1`
1. Once booted, make sure to use X.Org rather than Wayland for the session.
   (This may be the only possibility if the NVidia drivers are properly
   activated; check `$XDG_SESSION_TYPE` after logging in.)
1. Open up [OBS] and make a test recording.

[OBS]: https://obsproject.com/


# Dual-boot kernel magic

For video capture, I need the proprietary NVidia kernel driver. For day-to-day
usage, I prefer [Nouveau] because it has Wayland support. So I need a way to
sometimes use one, sometimes use the other.

[Nouveau]: https://nouveau.freedesktop.org/

I found some recipes for setting this up, but didn't love any of them. It seemed
to me like the best approach was to choose the video option at boot time. To use
the proprietary driver, I use the [rpmfusion/akmod approach][fusion], which sets
up default blacklists to disable nouveau. In order to make the driver option
selectable at boot, I needed to futz with my Grub setup to add extra bootloader
entries with different kernel command-line settings.

[fusion]: https://rpmfusion.org/Howto/NVIDIA

I was confused for a while because lots of the information on the web talks
about regenerating the config file with `grub-mkconfig`, but on the
latest-and-greatest setup that's not actually how menu options are created
anymore. Instead, files have to be dropped into `/boot/loader/entries/`. And it
seems that the way to make that happen on Fedora is to hook into the
`/usr/bin/kernel-install` framework, which isn't very well documented.

I ended up creating the file
`/etc/kernel/install.d/99-pkgw-nouveau-variants.install` as follows:

```sh
#!/usr/bin/bash
#
# Create files in /boot/loader/entries that replace the nouveau
# blacklists with nvidia blacklists, so that we can choose on boot
# whether we want nvidia (hardware-accelerated video capture) or
# nouveau (Wayland and per-monitor HiDPI support).
#
# This script derived from
# `/usr/lib/kernel/install.d/90-loaderentry.install`. Note however that
# $ENTRY_DIR_ABS is not `/boot/loader/entries` on my system, so that
# script isn't actually what sets up the loader entries -- it exits
# early becuse $ENTRY_DIR_ABS doesn't even exist. The BLS entries come
# from `20-grub.install`.

COMMAND="$1"
KERNEL_VERSION="$2"
KERNEL_IMAGE="$4"

if ! [[ $KERNEL_INSTALL_MACHINE_ID ]]; then
    exit 0
fi

MACHINE_ID=$KERNEL_INSTALL_MACHINE_ID
BLS_DIR="/boot/loader/entries"

if [[ $COMMAND == remove ]]; then
    rm -f "$BLS_DIR/$MACHINE_ID-nouveau-$KERNEL_VERSION.conf"
    rm -f "$BLS_DIR/$MACHINE_ID-nouveau-$KERNEL_VERSION+"*".conf"
    exit 0
fi

if ! [[ $COMMAND == add ]]; then
    exit 1
fi

if ! [[ $KERNEL_IMAGE ]]; then
    exit 1
fi

# Create new Nouveau variants based on existing non-Nouveau files.

for template in "$BLS_DIR/$MACHINE_ID-$KERNEL_VERSION"*.conf ; do
    new="$(echo $template |sed -e s/$MACHINE_ID/$MACHINE_ID-nouveau/)"
    sed \
        -e 's/Fedora/Fedora +Nouveau/' \
        -e 's/rd.driver.blacklist=nouveau/rd.driver.blacklist=nvidia/' \
        -e 's/modprobe.blacklist=nouveau/modprobe.blacklist=nvidia/' \
        -e 's/nvidia-drm.modeset=1//' \
        <"$template" >"$new" || exit 1
done

# Set saved_entry to make Nouveau the default.
#
# TODO: we're just using whatever was last in the for-loop above.
#
# Template files check $GRUB_UPDATE_DEFAULT_KERNEL in the analogous code,
# but it doesn't seem to be set when this script is run, so just update
# unconditionally.

newdefault="$(basename $new .conf)"
grub2-editenv - set "saved_entry=${newdefault}"

exit 0
```

You can manually cause this script to be run with a command such as:

```sh
$ kernel-install -v add 5.14.10-200.fc34.x86_64 /lib/modules/5.14.10-200.fc34.x86_64/vmlinuz
```

This started working, but then I found that the NVidia kernels were booting into
Nouveau mode. It looks like this was because
`/usr/lib/kernel/install.d/20-grub.install`, which creates the mainline
`/boot/loader/entries/` files for me, sets the Grub kernel options from
`/proc/cmdline`, propagating the Nouveau settings if I install a kernel in the
Nouveau mode. I worked around this by putting the desired settings into
`/etc/kernel/cmdline`, which that script prefers if it exists.


# Other setup

1. Set up `/etc/X11/xorg.conf.d/nvidia.conf` as a clone of
   `/usr/share/X11/xorg.conf.d/nvidia.conf` with `Option "PrimaryGPU" "yes"`
   added. **Seemingly needed to get GLX using NVidia; maybe not needed for OBS?**
1. OBS should be configured to use hardware encoding! Settings → Output →
   Recording → Video Encoder should be "Hardware (NVENC, HEVC)".
1. For more OBS settings, set “Output Mode” to “Advanced” and then the Type of
   the recording pane to Standard. This gave me the knobs I needed to resolve
   the following.
1. As of 2024 March, I needed to set “Max B-frames” to 0 for NVENC HEVC output.
   Who knows.

RPMs needed (I think?):

```
libva-nvidia-driver
nvidia-gpu-firmware
nvidia-modprobe
nvidia-persistenced
nvidia-settings
xorg-x11-drv-nvidia
xorg-x11-drv-nvidia-cuda
xorg-x11-drv-nvidia-cuda-libs
xorg-x11-drv-nvidia-kmodsrc
xorg-x11-drv-nvidia-libs
xorg-x11-drv-nvidia-power
```


# Dead Ends / Non-Factors

For *some* GPU use cases, it’s important to use the laptop’s display panel, and
not an external monitor, because the GPU simply isn't physically connected to
the external ports. But here, we’re just using the GPU for the encoding, so that
doesn’t matter. I have successfully captured on my external monitor and
everything is OK.

Likewise, it’s probably not necessary to get GLX using the GPU here, although I
haven’t explicitly done the experiment to confirm that.

Unlike some other systems, here we should not, and indeed cannot, prevent the
Intel `i915` drivers from being loaded — the GPU needs the Intel hardware to
work at all. So no need to worry about that stuff.



# Diagnostic Steps

Run `lsmod |grep nvidia`. Various modules should be present. Run `lsmod |grep
nouveau`. Nothing should be present.

Run `glxinfo`. The OpenGL renderer should be NVidia. This may not actually be
needed for OBS recording?

Run `nvidia-smi`. It should be happy and if OBS is running, it should be a
client.

Run `vdpauinfo`. It should report various NVidia things.

Run `vainfo`. It should be happy and use NVidia.

Run `ffmpeg -h encoder=h264_nvenc`. It should report various NVidia things.

Try a test FFmpeg with a sample file [like this one][sample]:

```
ffmpeg \
  -i preview_657cdb983affb5769f425e10.mp4 \
  -c:v hevc_nvenc -cq 28 -b:v 0k -b_ref_mode 0 \
  output.mp4
```

[sample]: https://cx.wwtassets.org/previews/preview_657cdb983affb5769f425e10.mp4

One time, a lot of experimenting with this revealed to me that the `h264_nvenc`
mode, which used to work for me, seems to no longer work; I could get HEVC
going, though.

Run `obs` from the command line and examine output. One time, this revealed to
me that I needed to install the `libva-nvidia-driver` RPM in order to provide a
needed VA shared library.


# See Also

- [How to get a standardized browser window](@/howto/get-a-standard-browser.md)
  which has notes about vidcap setup for one of my common use cases: capturing a
  web browser window.