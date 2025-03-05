+++
date = 2025-03-05T13:35:03-05:00 # deploytool
title = "Fun with Databases"
+++

Yikes, it’s been hard to keep up a weekly posting schedule lately. I’m hoping to
ramp that back up, though — I’ve just been occupied the past couple of weeks
with some nitty-gritty database performance work.

<!-- more -->

A couple of weeks ago we migrated the MPC’s primary production database from
physical hardware to a virtual machine running our “Borg” cluster, which runs
the [XCP-ng][xcpng]/[Xen Orchestra][xo] virtualization stack. This database runs
on [PostgreSQL][pg], is about 2 terabytes in size, and really constitutes the
beating heart of the MPC: it stores all of our core data assets, such as orbits,
designations, and about 500 million astrometric observations. All of MPC’s
operational systems are querying and modifying the database non-stop, 24/7.

[xcpng]: https://xcp-ng.org/
[xo]: https://xen-orchestra.com/
[pg]: https://www.postgresql.org/

As one might imagine, you want to do such a migration carefully. Indeed, there
were several months of planning and rehearsals leading up to the switchover. It
went completely smoothly, thanks in no small part to all of the preparatory
work. We’re now running the latest version PostgreSQL on a VM backed by a
[Powervault] storage array with automated snapshots, hot migrations between
physical hosts, and all sorts of other nice resilience features.

[Powervault]: https://www.dell.com/en-us/shop/storage-servers-and-networking-for-business/sf/powervault

We did discover, however, that some aspects of the database performance weren’t
living up to what we expected. In particular, there was a lot of variability:
the same query would sometimes run quickly, and other times run orders of
magnitude (plural!) slower. It wasn’t a crippling issue, but definitely
something to address.

And that’s what’s been keeping me busy since then. Pretty early on, we
discovered that the variable performance had something to do with the particular
way in which the VM was connecting to its Powervault storage. You can register
Powervault volumes with the Xen system and then use them as “storage
repositories” for virtual disks, which lets you manage and migrate them all
through the Xen interface. But something about the Xen layer seemed to be
responsible for the uneven performance — because you can also configure a VM to
connect directly to the Powervault using [iSCSI], and doing so gave much more
consistent results. *Part* of the reason might be that Xen disks currently are
limited to a maximum size of 2 TB, so that we had to use [LVM] in the database
VM to create a sufficiently large virtual disk. On the other hand, while I’m
sure this didn’t help performance, it’s hard for me to see how it would induce
the variability that we were seeing. (For what it’s worth, that 2 TB limit [is
going away][twotb].)

[iSCSI]: https://en.wikipedia.org/wiki/ISCSI
[LVM]: https://en.wikipedia.org/wiki/Logical_Volume_Manager_(Linux)
[twotb]: https://xcp-ng.org/forum/topic/10308/dedicated-thread-removing-the-2tib-limit-with-qcow2-volumes

Migrating from the Xen + LVM approach to the direct mount would require us to
dump and reload the whole database, so we didn’t want to make that change unless
we were convinced that it would help things. So, we spent a while running tests
and trying to understand the performance characteristics of the stack, and then
planning out the changeover once we were convinced that it was worth trying.
Yesterday we made the change, and things have been a lot smoother since.

The performance that we’re getting is still noticeably less than bare metal, but
the benefits of using the virtualized system are extremely real. In particular
I’m really excited that we have the option to “hot migrate” the database from
one physical machine to another, which means that we can do hardware maintenance
with zero downtime of the actual database service. But, if we really need to, we
can sacrifice that and get better performance by adopting a technology called
[SR-IOV], which basically allows the VM to access the physical host’s network
cards in a lower-level, but still virtualized, way. In our low-level tests
SR-IOV gave a ~50% performance boost for some workloads, which apparently stems
from the fact iSCSI often uses a lot of little packets that require a lot of
interrupts to service. For the time being, we’d rather keep the ability to hot
migrate, but the SR-IOV option is there in our back pocket if we need it.

[SR-IOV]: https://docs.kernel.org/PCI/pci-iov-howto.html
