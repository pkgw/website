+++
date = 2024-10-10T11:28:29-04:00 # deploytool
title = "Elastic HPC Mini-Services"
+++

The end is in sight! My long-running project to completely reprocess the [DASCH]
photometry should wrap up this week, although this morning progress is once
again being held up by a flaky [Lustre] filesystem. As I’ve been working on
porting the DASCH scientific data services [to run in the
cloud](@/2024/dasch-cloud-apis/index.md), I’ve been thinking about some features
that cloud providers make available that I wish HPC systems like [Cannon]
offered.

[DASCH]: https://dasch.cfa.harvard.edu/
[Lustre]: https://www.lustre.org/
[Cannon]: https://www.rc.fas.harvard.edu/services/cluster-computing/#Cannon

<!-- more -->

The specific motivation is pretty simple: I’d love to be able to use [SQLite] on
Cannon. For the kinds of data analysis work that I do, there are tons of times
where I'd like to be able to set up a miniature, temporary database to
coordinate work between different tasks. At the hand-waving level, an SQLite
file on one of our shared Lustre drives would be perfect for this. In practice,
it doesn’t work. On Cannon’s Lustre filesystems in particular, I run into a
locking issue that means that I’d basically have to operate any such database in
read-only mode, which eliminates the most important use-cases. In general,
[SQLite over network filesystems is strongly discouraged][sqlitenet] for fairly
foundational reasons. (I like that document a lot — “You want to run SQLite over
NFS? Well first let’s consider the fundamental properties of the new
communication channel that you’re introducing into the dataflow.” That’s very
much my style.) So the specific technology choice of SQLite has issues.

[SQLite]: https://sqlite.org/
[sqlitenet]: https://sqlite.org/useovernet.html

What’s interesting is that on Cannon, unlike most users I actually have access
to a SQL server that I fully control, as part of DASCH. And yet I still find
myself wanting to use SQLite. Why is that?

The reason, I think, is something I discussed when describing the [HPC job
organization scheme][hpcorg] that I’ve been using. The DASCH SQL server is a
durable, unique, global resource. But I’ve found that there’s a ton of value in
being able to organize my cluster work into independent projects, using subsets
of resources that are intentionally isolated from each other. That way, I can
try an experiment and be confident that it won’t affect the production system,
and I can confidently throw it all away when I’m done with it. When I want to
use SQLite, it’s for these reasons: I want databases that are temporary and
isolated, and I want to be able to create and destroy them at will.

[hpcorg]: @/2024/hpc-job-organization.md

From that perspective, SQLite is a tempting option because it turns “running an
SQL service” into “managing a file on the filesystem”, and [as I wrote in the
HPC organization post][hpcorg], HPC filesystems have exactly the characteristics
I mentioned above. I can create and destroy files at will; I can organize them
hierarchically through directory structure; the resources are globally visible
and always available.

From the cloud perspective, what I want is an *elastic* service. I want to be
able to create and destroy resources — specifically, SQL databases — on-demand.
An important aspect of this is that I need to be able to do it in a self-service
mode. You could imagine a world where I submit a stream of helpdesk tickets
asking the [FASRC] staff to start and stop different PostgreSQL VMs for me as I
work on various projects; that would be pretty awful for everyone involved.

[FASRC]: https://www.rc.fas.harvard.edu/

In the past, I’ve tried to achieve this elasticity by spinning up my own servers
as [Slurm] jobs. My takeaway from the experience is that with a lot of work, you
can sometimes get something that works most of the time, but that the overall
experience is not reliable enough to be worthwhile. If you’re going to commit to
designing your workflows around something like an elastic SQL service, it has to
be ultra-reliable, at the same level as the filesystem. (Sometimes it feels like
HPC filesystems are breaking all of the time, but think of how many operations
they successfully execute every day. In these systems, a [five-nines] success
rate in completing transactions would be an utter disaster.) If you try to wrap
a comparable service in the HPC job framework, there are just too many issues:
how do you make sure to spin up exactly one server? What do you do when the
server job inevitably must terminate? How about when it terminates early? How do
the other jobs discover it? How do you transfer state between different
incarnations of the server? The issues never stopped.

[Slurm]: https://slurm.schedmd.com/
[five-nines]: https://en.wikipedia.org/wiki/Five_nines

Another relevant tool is [MPI], which we can think of as providing additional
primitives that help to coordinate distributed work. Another way of thinking
about a “service” is as a mechanism for exactly this kind of coordination: the
whole point of having an SQL server is that a bunch of different workers can
talk to it independently and obtain some kind of coherent shared view of their
collective state. We can imagine building on MPI’s primitives to achieve similar
effects. The problem here is that MPI can only coordinate within a group of
coextensive jobs. In all of my work, the thing that I’m really missing are
coordination tools whose existence spans the lifetimes of many jobs, which may
start and stop at basically arbitrary times. In general, I’ve found MPI to be
far too rigid for anything I want to do, although I’m certainly no MPI guru.

[MPI]: https://en.wikipedia.org/wiki/Message_Passing_Interface

Thinking back over my recent work, these considerations actually completely
drive how I architect software that’s meant to run on HPC systems. On most HPC
systems the filesystem is the *only* globally-available, reliable, elastic
service, and so *everything* that involves any kind of flexible coordination of
distributed work becomes a filesystem problem. Managing uniqueness? [O_EXCL].
Atomically committing completed work? [rename()]. Merging multiple data streams?
[O_APPEND].

[O_EXCL]: https://www.gnu.org/software/libc/manual/html_node/Open_002dtime-Flags.html
[rename()]: https://en.wikipedia.org/wiki/Rename_(computing)#Atomic_rename
[O_APPEND]: https://lustre-discuss.lustre.narkive.com/jUCK1WoV/does-lustre-support-atomic-o-append-and-atomic-rename

You can build a lot of complexity on top of these primitives, but the
limitations are significant. The example that I run into most frequently in my
work is what you might call “microtransactions”: I might have a data processing
job that can be expressed as 100,000 very small tasks to run, with certain
interdependencies between them. [It’s way too slow to submit them as individual
Slurm jobs](@/2024/dasch-astrometry.md), so ideally you’d partition that work
among, say, 100 worker jobs. You *can* use stamp files on the filesystem to
coordinate all this, but it gets sloooow, and it’s not how the filesystem is
meant to be used. A transient SQL-like database would be better in every way.

I’ve been talking about SQL this entire time, but the same needs arise with
other database-y paradigms. Really, the term “database” implies exactly this
sort of service. On Cannon, I most often wish that I could use [key-value
stores][kvs] (think [DynamoDB] or [Redis]) or [NoSQL] systems like [MongoDB].

[kvs]: https://en.wikipedia.org/wiki/Key%E2%80%93value_database
[DynamoDB]: https://aws.amazon.com/dynamodb/
[Redis]: https://redis.io/
[NoSQL]: https://en.wikipedia.org/wiki/NoSQL
[MongoDB]: https://www.mongodb.com/

Once again, it’s worth emphasizing that while I can ask for such services to be
set up manually, the element that I’m really missing is the elasticity: I want
to create and destroy them on-demand, preferably with organizational tools that
allow me to manage hundreds of instances conveniently. Ignoring all of the
implementation details, if I had the ability to run a command to spin up a new
[PostgreSQL] server on the cluster, have it run continuously for as long as I
want, and then tear it down at-will, that would be a game-changer for a lot of
my workflows. Even better if [Redis] and [MongoDB] were options as well.

[PostgreSQL]: https://www.postgresql.org/

As I was thinking about all this, it finally occurred to me: you know, I do have
an AWS account. I *can* create all of the services on demand! They just won't
live on the HPC cluster, and I’ll have to pay for them. In certain situations,
this is probably a non-starter: ingress/egress costs and the latency of a
non-local server could be dealbreakers. But in other cases, such as my
“microtransactions” example, I could see a short-lived [DynamoDB] instance in
`us-east-1` as being a great solution.

For all of those other situations, I’d love to see HPC systems start offering
this kind of elasticity themselves. (I’m not aware of any that do.) Of course,
we can only try to imagine the extra headaches this would add for our
already-overworked HPC operators. Managing quotas, dealing with out-of-control
processes, security, software updates, backups … there’s a lot to worry about.
You basically have to become a local, on-prem “cloud” provider. I’m pretty sure
that there are frameworks that help you do exactly this; I think this might be
what [OpenShift] is all about? Maybe there’s a way to provide this kind of
service that isn’t *too* much work if you are extremely limited and rigid about
what you provide; but much past that and you’re probably looking at more than
doubling your administrative workload.

[OpenShift]: https://www.redhat.com/en/technologies/cloud-computing/openshift

Because of that reality, while I think that it would be super valuable to have
these kinds of services available in clusters like Cannon, I’m not going to hold
my breath. In particular I have trouble seeing how I’d convince someone that it
was worth a bunch of extra money to pay to make them available. This seems like
the classic kind of intangible thing that *I* would describe as utterly
foundational and transformative, but if someone’s not already on your wavelength
about that, I have no idea how to change their mind. (I have the same feeling
when I talk to people about zoning rules.) In the meantime, I might start
exploring what you might call the DIY approach, spinning up these kinds of
services elastically using standard cloud providers. Most of the DASCH
infrastructure is too locked-in to fiddle with at this point, but I see some
opportunities to do this with the final pieces of the upcoming DASCH data
release.
