# Rust bindings for the illumos "fake kernel" library

The [illumos](https://illumos.org) operating system comes with
a library called, `libfakekernel`.  This implements or otherwise
stubs out a subset of kernel interfaces, so that some kernel
code can be compiled in userspace; this mainly useful for
testing.

This crate exists simply as a vehicle for linking against
`libfakekernel` on illumos, or stubbing out interfaces on other
systems, so that we can write unit tests in illumos kernel
modules and drivers that are written in Rust.
