+++
date = 2024-02-08T12:40:48-05:00
title = "A Scheme for Organizing HPC Jobs"
+++

For my work on DASCH, I’ve been trying out a new scheme for organizing the
computational jobs that I’m launching on [Cannon], Harvard’s primary research
HPC cluster. It’s been working really well! You could call it a “location-based”
system, and below I’ll sketch out the approach.

[Cannon]: https://www.rc.fas.harvard.edu/services/cluster-computing/

<!-- more -->

First, some context. When I’m using a cluster like Cannon, I’m generally doing
bulk data processing. In the case of DASCH, I have a pipeline with a lot of
steps in it. I’ve got chunks of data moving through different stages of the
pipeline, and each data+step combination will run in one HPC job on our
scheduler, which happens to be [Slurm]. On top of pipeline-style processing, I’m
pretty much constantly running experiments, testing new code, or executing
one-off projects that require single-use scripts. So I’ve often got many
different kinds of jobs running at once, corresponding to many different mental
“threads”: one group of jobs is dealing with today’s incoming data; another is
doing some weekly maintenance; a third is testing out a new algorithm.

[Slurm]: https://slurm.schedmd.com/

My problem is that Slurm (and every other scheduler that I’ve seen) really only
offers very primitive support for organizing and keeping track of everything
that I’ve got going on. The well-known [`squeue`] command will tell you what
jobs of yours are currently running, but only in the form of one big list. I’m
not aware of any built-in mechanisms to “tag” jobs or otherwise group them. It’s
also surprisingly annoying to get information out of Slurm about completed jobs.
The upshot is that if I have a question like, “what’s the average amount of time
taken by the `astrometry` jobs that I ran as part of my `new-matching` test?”, I
have to write a bunch of code to get an answer. It’s frustrating because a
scheduler like Slurm holds a lot of really useful information about your jobs,
and you if you want to analyze that information you really want to be working
within its framework as much as possible, not duplicating it. But Slurm just
gives you very little to work with.

[`squeue`]: https://slurm.schedmd.com/squeue.html

When I started working on new tooling for the DASCH data processing, I knew that
this situation would bother me a lot, so I spent some time thinking about what I
could do. You can imagine a lot of overengineered solutions (create and maintain
your own separate job database!) but I felt that it was super important to keep
things as lightweight as possible.

The “location-based” approach that I came up with manages that, I think. The key
idea is to *use the filesystem to organize jobs*. Specifically: whenever you
launch a job, the directory that you launched it from “remembers” the job. (Side
note: the job *launch* directory need not be the same as the job’s *working*
directory.)

This one little concept unlocks a lot of powerful workflows. The two commands
that I find myself using the most often are “job [top]”, which prints out the
pending and running jobs *launched from the current directory*, and “job
[tail]”, which prints out information about recently-completed jobs *launched
from the current directory*. The other one that instantly comes to mind would be
a sort of “job [ls]”, but I actually haven’t had much of a need for it.

[top]: https://www.unixtutorial.org/commands/top
[tail]: https://www.unixtutorial.org/commands/tail
[ls]: https://www.unixtutorial.org/commands/ls

The great thing about this approach is that you instantly get a flexible system
for organizing your jobs in whatever kind of hierarchy you prefer, and it’s a
system based on the “database” that’s more popular, better-supported, and
better-understood than any other: the Unix filesystem. It feels very “clean” to
me to extend the idea of “a directory can contain files” to “a directory can
contain files *and jobs*”. You can immediately think of a bunch of tools flowing
from that mental model, and they feel like they work at the right — low — level.

This approach is also pretty easy to implement. The most naive model I can think
of would be set up an [`sbatch`] wrapper to log the IDs of newly-launched jobs
to a file named something like `.jobids`, and I’m pretty sure that would work
just fine. The main wrinkle is that when a job finishes, you generally need to
ensure that some kind of “postmortem” analysis records information about the job
outcome. There are two reasons for this: one, Slurm throws that information away
relatively promptly; and two, the Slurm commands for retrieving job data are
pretty slow, so you don’t want to rely on them for analytics. Grouping jobs on
the filesystem provides a great framework for keeping this information without
needing to have a single giant file logging data about every single job you’ve
ever run.

[`sbatch`]: https://slurm.schedmd.com/sbatch.html

The system that I’ve built for DASCH is a bit more sophisticated than the above,
tying into some other data-management tooling that I’ve put together. The most
important idea to mention is that every job is identified using a human-friendly
name, rather than Slurm’s numeric IDs (`astrometry_a17401` instead of
`18070012`). Slurm lets you give jobs names, but its tools all encourage you to
using the numeric IDs instead. As long as I’m writing code to implement the
“location-based” model, I can fix that issue.

The naming feature is a quality-of-life issue, but also gets to some relatively
deep engineering considerations. I suspect that Slurm doesn’t encourage you to
refer to jobs by name because those names aren’t reliably unique at any level —
whereas job IDs are guaranteed to be unique on the whole system. (Well,
eventually they get recycled …) The location-based approach allows for a middle
ground. I ensure that job names are locally unique: no two jobs within the same
launch directory have the same name, in exactly the same way that no two files
within a directory are allowed to have the same name. In fact, under the hood I
ensure job name local-uniqueness through file name local-uniqueness — my jobs
have associated “state directories” created using [`mkdtemp()`], so that my job
names actually have the form `astrometry_a17401.a_q9xl`.

[`mkdtemp()`]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/mkdtemp.html

Contrast this to Slurm itself, which assigns jobs globally unique IDs by giving
them sequential numbers. The weakness of the Slurm approach is that it’s
inherently centralized — every single request to create a job has go through
some single process that keeps track of the next job ID. It’s the same
architectural problem as Subversion, with its sequential revision numbers, as
compared to Git. Which isn’t something that should be *that* big of a deal, but
more and more I notice just how slow Slurm feels to me. Harvard’s instance is
really busy, but I still feel like it shouldn’t take 10 seconds for the daemon
to accept a job-launch request. If you’re writing some kind of distributed
software and you find yourself assigning sequential identifiers to something, be
aware that you’re probably creating an unfixable bottleneck!
