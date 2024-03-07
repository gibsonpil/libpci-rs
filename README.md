# libpci-rs 0.2.7-testing
### ⚠ UNDER CONSTRUCTION ⚠

![Crates.io](https://img.shields.io/crates/v/libpci-rs)
![GitHub](https://img.shields.io/github/license/NamedNeon/libpci-rs)
![GitHub Workflow Status (with event)](https://img.shields.io/github/actions/workflow/status/NamedNeon/libpci-rs/tests.yml)

Unstable work-in-progress cross-platform library to detect and list PCI devices and their information. 

This library does not bind to or require libpci. As much of the library as possible is written in Rust as a general rule.
Unfortunately, not all operating systems have stable or maintained Rust APIs, so code for those will be split off into a
C++17 backend.

This project uses [Semantic Versioning](https://semver.org/).

### Platform Support
- Windows
- Linux

### Planned Platform Support
- macOS
- FreeBSD
- OpenBSD