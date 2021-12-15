+++
date = 2021-12-15T10:50:57+00:00
title = "Set Up Windows Terminal for Conda Bash"
weight = 0 # all howtos have zero weight => alphabetical ordering
template = "howto.html"
+++

When I'm using Windows I often find that I need to use different shells: PowerShell, bash with Conda
set up, sometimes even `cmd`. I've found that [Windows Terminal][wt] is convenient for this, but
found the setup to be a bit tricky to get right. One big factor is that I also want these shells to
have the Visual Studio environment correctly initialized, which has to happen in `cmd` because
Windows is ridiculous. Here are the magic commands:

[wt]: https://github.com/microsoft/terminal

## Visual Studio, Conda, and Powershell

```
cmd.exe /K "call vcvars.bat && C:\Users\peter\miniforge3\Scripts\activate.bat && powershell"
```

## Visual Studio, Conda, and Bash

```
cmd.exe /K "call vcvars.bat && C:\Users\peter\miniforge3\Scripts\activate.bat && bash"
```

Note that on Windows, the `git` Conda package provides `bash`! The MSys2 package of Git is old and
out-of-date. So you should install `git` but neither `m2-git` nor `m2-bash`.
