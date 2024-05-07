# libpci-rs 0.2.10-testing

## ⚠ UNDER CONSTRUCTION ⚠

![Crates.io](https://img.shields.io/crates/v/libpci-rs)
![GitHub](https://img.shields.io/github/license/gibsonpil/libpci-rs)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/gibsonpil/libpci-rs/tests.yml)

Unstable work-in-progress cross-platform library to detect and list PCI devices and their information.  

This library does not bind to or require libpci. As much of the library as possible is written in Rust as a general rule.
Unfortunately, not all operating systems have stable or maintained Rust APIs, so code for those will be split off into a
C++17 backend.

This project uses [Semantic Versioning](https://semver.org/).

### Building

Building libpci-rs easy. Below is an example of how you might go about it.

- `git clone --recurse-submodules https://github.com/gibsonpil/libpci-rs.git`
- `cargo build`

### Build Dependencies

Aside from the dependencies pulled in by Cargo, libpci-rs requires a few packages be installed on your system,
some are required and some are only needed if you are trying to do certain things. Notes will be added in for those that aren't always needed.

- `rustc`
- `cargo`
- `clang` (needed for platforms that have a C++ backend)
- `python3` (needed for helper.py helper script, mostly only needed by devs)
- `clang-format` (needed for formatting C++ code, applies styling requirement)
- `cppcheck` (needed for C++ code linting)

### Documentation

Documentation is included in the code itself, in the form of Rustdoc comments. It includes the following useful pieces of information:

- Total coverage of the public API
- Code examples for some common use cases
- Detailed information on per-platform field availability

To view the docs, clone the repo and enter the directory, before running `cargo doc --open`.

### Platform Support

| Platform      | OS API         | Backend Language |
|---------------|----------------|------------------|
| Windows       | setupapi       | Rust backend     |
| Linux/Android | sysfs          | Rust backend     |
| macOS/Darwin  | IOKit          | C++ backend      |
| FreeBSD       | /dev/pci       | C++ backend      |
| DragonflyBSD  | /dev/pci       | C++ backend      |
| OpenBSD       | /dev/pci       | C++ backend      |
| NetBSD        | /dev/pci       | C++ backend      |
| Haiku         | /dev/misc/poke | C++ backend      |
