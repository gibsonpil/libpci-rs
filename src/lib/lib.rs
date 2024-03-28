// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

//! A library to enumerate, identify, and retrieve info about PCI devices.
//!
//! # About
//! `libpci-rs` is a cross-platform Rust rewrite of
//! [libpci](https://github.com/pciutils/pciutils), which is written in C. It
//! contains functionality to list the devices installed in a system, and to
//! obtain information about a device, its functionality, or its manufacturer.
//! Currently, it supports Linux, Windows, macOS/Darwin kernels, FreeBSD,
//! OpenBSD, NetBSD, and DragonflyBSD. It is also not dependent on processor
//! architecture, and should support any architecture that the Rust compiler
//! supports. However, ***please take note***: Not all functionality is
//! available at all times or on all platforms. See
//! [Field Availability](crate::pci::PciDeviceHardware#availability) for
//! details.
//!
//! # Enumeration
//! By default, it includes the functions and structures needed to list the
//! PCI devices installed on the host system. All of the information obtained
//! by this core segment of the library gets its information directly from the
//! API of the operating system.
//!
//! # The `pciids` feature
//! The `pciids` feature includes functions, methods, and structures useful
//! for obtaining more detailed, catalogued information about a PCI device,
//! whether it's installed or not. Because this feature requires compiling in
//! the PCIIDs database, it will increase the size of the library. It is
//! enabled by default.

use cfg_if::cfg_if;

/// The platform-dependent backend modules responsible for handling platform
/// specific syscalls, parsing, and error handling.
mod backend;
/// Structures and functions related to enumerating PCI devices.
pub mod pci;

cfg_if! {
    if #[cfg(feature = "pciids")] {
        /// Structures and functions related to the PCI IDs database. Depends
        /// on the [pciids](#pciids) feature.
        pub mod ids;
        /// Structures and functions related to PCI device class
        /// classifications. Depends on the [pciids](#pciids) feature.
        pub mod class;
    }
}
