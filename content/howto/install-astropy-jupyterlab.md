+++
date = 2022-04-18T11:27:58-04:00
title = "Install Astropy and JupyterLab"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

*I occasionally run introductory Python workshops for folks at the [CfA][cfa]
and beyond. Here are my go-to instructions for how to install Python and related
software. Last updated: April 2022.*

[cfa]: https://cfa.harvard.edu/

The intent is that these instructions should be useful for people with nearly no
experience with Python or even computing in general. If the instructions below
seem very elementary to you, great! You still might want to skim them to get an
idea of what the approach will be during the workshop.


# The Goal

The goal of these instructions is to help you get [Python], [Astropy], and
[JupyterLab] installed on your laptop. You are ready to go when:

[Python]: https://www.python.org/
[Astropy]: https://www.astropy.org/
[JupyterLab]: https://jupyter.org/

1. You can start up the JupyterLab web application on your laptop, possibly
   by running the shell command `jupyter lab` at a terminal.
2. Inside a JupyterLab “Python console”, the Python command `import astropy`
   succeeds: when you run it, no error messages come back.

If these conditions hold for you, you are all set! If not, let’s proceed.


# The Overall Plan

Your computer probably already comes with Python installed, but it turns out to
be very helpful to install a personal copy that you can use for the workshop.
People generally refer to this as a Python **environment**.

If you haven’t installed any environment(s) on your computer before, it’s
straightforward to set one up these days. Proceed to [the next
section](#installing-a-new-environment).

On the other hand, you might have already created one or more personal Python
environments on your computer, likely using the [conda] software. Since our
workshop won’t be relying on any exotic software setups, it will almost surely
be fine if you install JupyterLab and Astropy into an existing environment. If
you’re happy with your existing environment but not quite sure how to install
any needed software, Google for instructions specific to the system you’re
using.

[conda]: https://docs.conda.io/

If you *do* have one or more environments installed, *but* you feel that they
are screwed up somehow, it’s best to ask for help from a Python expert before
trying anything. While it will probably work to create a new installation using
the instructions above, it’s good to straighten these things out, and running a
new installer might overwrite some settings that could be helpful for
understanding your situation.


# Installing a New Environment

If a new Python environment is called for, we’ll it using the aforementioned
[conda] tool in conjunction with [conda-forge], a project that packages up a
*huge* range of software for installation using conda. To set them up we’ll use
an installer named [Miniforge].

[conda-forge]: https://conda-forge.org/
[Miniforge]: https://github.com/conda-forge/miniforge

By default, Miniforge sets up a new environment in a personal directory named
`miniforge3`, which shouldn’t conflict with other installations or environments
unless you’ve used this specific installer before. (However, the “activation”
systems for different environments can conflict even if they live in different
directories, hence the advice above.)

Specific instructions are grouped by operating system:

- [MacOS](#macos-installation-instructions)
- [Linux](#linux-installation-instructions) (any kind)
- [Windows](#windows-installation-instructions)


# MacOS Installation Instructions

1. [Click this link][mf-osx-x64] to download the Miniforge installer for MacOS
   on Intel CPUs. If you are on a recent “Apple Silicon” (ARM CPU) laptop, you
   can also try [clicking this link][mf-osx-amd64] to download Miniforge for
   MacOS on Silicon, but it’s still a bit experimental.
2. Open up your terminal application. If unsure about how to do that, [see these
   instructions][osx-terminal-howto].
3. Launch the installer program with:
   ```sh
   bash ~/Downloads/Miniforge*.sh
   ```
   (That is, run a `bash` command from inside your shell, which might also be
   `bash`.)
4. Answer the installer’s prompts, accepting the default settings up until it
   asks you if you want to “initialize by running conda init”. Say yes to this.
5. The installer should instruct you to close your terminal program and reopen
   it. Do this.
6. After restarting your terminal, run the command:
   ```sh
   which conda
   ```
   It should print out the name of a file inside the Miniforge installation
   location. If it says that conda is not found or gives an unexpected location,
   seek assistance.
7. Otherwise, run the command
   ```sh
   conda install -y astropy jupyterlab
   ```
   Conda should go ahead and download and install your software. It will take a
   little while.
8. If that went well, you should now be able to run the command
   ```sh
   jupyter lab
   ```
   to start up the JupyterLab web application. This command will start a private
   web server and open a new browser window containing JupyterLab.

[mf-osx-x64]: https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-MacOSX-x86_64.sh
[mf-osx-amd64]: https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-MacOSX-arm64.sh
[osx-terminal-howto]: https://support.apple.com/guide/terminal/open-or-quit-terminal-apd5265185d-f365-44cb-8b09-71a064a42125/mac

You should now be all set! You can proceed to the [JupyterLab Quick
Test](#jupyterlab-quick-test) if you’d like to double-check that everything is
working properly.


# Linux Installation Instructions

1. [Click this link][mf-linux-x64] to download the Miniforge installer for Linux
   on Intel CPUs. If you’re using a different CPU architecture, we expect that
   you don’t need these instructions.
2. Open up your terminal application.
3. Navigate your shell to your downloads folder with a command such as:
   ```sh
   cd ~/Downloads
   ```
   (The precise folder name may vary depending on your setup.)
4. Launch the installer program with:
   ```sh
   bash Miniforge*.sh
   ```
   (That is, run a `bash` command from inside your shell, which might also be
   `bash`.)
5. Answer the installer’s prompts, accepting the default settings up until it
   asks you if you want to “initialize by running conda init”. Say yes to this.
6. The installer should instruct you to close your terminal program and reopen
   it. Do this.
7. After restarting your terminal, run the command:
   ```sh
   which conda
   ```
   It should print out the name of a file inside the Miniforge installation
   location. If it says that conda is not found or gives an unexpected location,
   seek assistance.
8. Otherwise, run the command
   ```sh
   conda install -y astropy jupyterlab
   ```
   Conda should go ahead and download and install your software. It will take a
   little while.
9. If that went well, you should now be able to run the command
   ```sh
   jupyter lab
   ```
   to start up the JupyterLab web application. This command will start a private
   web server and open a new browser window containing JupyterLab.

[mf-linux-x64]: https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Linux-x86_64.sh

You should now be all set! You can proceed to the [JupyterLab Quick
Test](#jupyterlab-quick-test) if you’d like to double-check that everything is
working properly.


# Windows Installation Instructions

1. [Click this link][mf-windows-x64] to download the Miniforge installer for Windows
   on Intel CPUs. If you’re using a different CPU architecture, we expect that
   you don’t need these instructions.
2. Run the downloaded installer program.
3. Answer the installer’s prompts, accepting the recommended settings.
4. Once the installer completes, your Start Menu should provide new commands
   `Anaconda Prompt (miniconda3)` and `Anaconda Powershell Prompt (miniconda3)`.
   You can run either of these (depending on whether you prefer the classic
   Windows command prompt or Powershell) to open a terminal that is configured to
   access your new Python environment.
5. Launch one of the Anaconda Prompt programs (either one).
6. Run the command
   ```sh
   conda install -y astropy jupyterlab
   ```
   Conda should go ahead and download and install your software. It will take a
   little while.
9. If that went well, you should now be able to run the command
   ```sh
   jupyter lab
   ```
   to start up the JupyterLab web application. This command will start a private
   web server and open a new browser window containing JupyterLab.

[mf-windows-x64]: https://github.com/conda-forge/miniforge/releases/latest/download/Miniforge3-Windows-x86_64.exe

You should now be all set! You can proceed to the [JupyterLab Quick
Test](#jupyterlab-quick-test) if you’d like to double-check that everything is
working properly.


# JupyterLab Quick Test

Regardless of which operating system you’re using, you should have pretty much
the same experience once you’ve gotten JupyterLab started up.

In general, feel free to explore in the JupyterLab interface — it’s pretty hard
to do anything that will seriously mess up your environment.

As a first thing to try, you can click the big “Python 3” button in the
“Console” section of the launcher to open up an interactive Python prompt. To
actually execute Python commands, type them in and hit **shift-enter** — a plain
“Enter” is for entering multi-line commands. The command `import astropy` should
succeed, although in this case “success” means that nothing will visibly happen.
(But, importantly, no error message should appear!) After running the `import`
command, the command `help(astropy)` should cause some documentation to be
printed.
