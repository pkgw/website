+++
date = 2021-12-13T09:43:01-05:00
title = "Capture Video Efficiently on Linux"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

Sometimes you want to record a video of your desktop screen on Linux, perhaps to
create a prerecorded presentation. With modern screen resolutions, it can be
super compute-intensive to record such a video, to the extent that it's worth
taking special steps to make sure the recording process works well.

On my machine (2020 Dell XPS 15 9500), the most important thing is to use
hardware-accelerated video encoding using the discrete NVidia graphics card.
This (currently?) requires use of the proprietary NVidia kernel module and
X.org. After setting everything up the workflow is:

1. Connect up fancy microphone, if possible
1. Reboot machine
1. Select one of the kernel variations with the NVidia kernel module
1. Before actually booting, check the kernel command line and make sure that
   magic arguments are there: `rd.driver.blacklist=nouveau modprobe.blacklist=nouveau nvidia-drm.modeset=1`
1. Once booted, make sure to use X.Org rather than Wayland for the session
1. Open up [OBS] and make a test recording

[OBS]: https://obsproject.com/


# Dual-boot kernel magic

For video capture, I need the proprietary NVidia kernel driver. For day-to-day
usage, I prefer [Nouveau] because it has Wayland support, which I need for
my dual-screen setup where the screens have different preferred HiDPI settings.
So I need a way to sometimes use one, sometimes use the other.

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
