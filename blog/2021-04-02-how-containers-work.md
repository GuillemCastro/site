---
title: What is actually a container?
date: 2021-04-02
---

For me, one of the most interesting technologies used by developers are containers. They're so popular that all programmers have used it, or at least heard about them. Yet, most people don't really know what they are, or how they work. We all have probably heard a definition similar to this one, — this one was given to me by one of my professors at university, but it's also [the definition Google gives to us](https://cloud.google.com/containers)—

> They are kind of like a more light-weight virtual machine

While not completely false, this is a very deceiving definition. Not only there use different technologies, but both (VMs and containers) also solve different problems —different solutions for different problems—, and behave completely differently. It's not a coincidence that containers seem hard to use at first, "Why the container is killed when I press Ctrl+C? That doesn't happen with my VMs!".

So, what is a container? A container is simply **a process** that we execute in an isolated environment. For creating an isolated environment we need two things,

* A root filesystem (rootfs), often called "images". Think of it as an operating system minus the kernel, anything else you'd find on a regular Linux machine is included (bash, libc, etc.). As for desktop/server installations they also come in distributions, so you can find rootfs for Ubuntu, CentOS, Alpine, etc.

* A way to trick the process into thinking that it is alone in the machine. The process won't be able to interact with other processes, and won't be able to access your files.

A virtual machine is also an isolated environment that executes one process, there are some differences though,

* The isolation is achieved by means of virtualization. Everything from the CPU to the memory or the external devices is virtualized.

* In a VM the process that gets executed is almost always an operating system or a special program that can be booted by a bootloader.

