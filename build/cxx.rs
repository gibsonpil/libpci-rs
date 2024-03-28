// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![allow(dead_code)]

#[allow(unreachable_code)]
pub fn build_cxx_module() {
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "macos", target_os = "ios"))] { // Darwin targets.
            build_cxx_darwin();
        } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] {
            build_cxx_freebsd();
        } else if #[cfg(any(target_os = "netbsd", target_os = "openbsd"))] {
            build_cxx_netbsd();
        } else if #[cfg(target_os = "haiku")] {
            build_cxx_haiku();
        } else {
            panic!("No suitable CXX modules found. Cannot build.");
        }
    }
    println!("cargo:rerun-if-changed=src/lib/backend/include/common.h");
}

pub fn build_cxx_darwin() {
    cxx_build::bridge("src/lib/backend/bridge.rs")
        .file("src/lib/backend/darwin/darwin.cc")
        .std("c++17")
        .compile("libpci-rs-darwin");

    println!("cargo:rerun-if-changed=src/lib/backend/darwin/darwin.cc");
    println!("cargo:rustc-flags=-l framework=CoreFoundation");
    println!("cargo:rustc-flags=-l framework=IOKit");
}

pub fn build_cxx_freebsd() {
    cxx_build::bridge("src/lib/backend/bridge.rs")
        .file("src/lib/backend/freebsd/freebsd.cc")
        .std("c++17")
        .compile("libpci-rs-freebsd");

    println!("cargo:rerun-if-changed=src/lib/backend/freebsd/freebsd.cc");
}

pub fn build_cxx_netbsd() {
    cxx_build::bridge("src/lib/backend/bridge.rs")
        .file("src/lib/backend/netbsd/netbsd.cc")
        .std("c++17")
        .compile("libpci-rs-netbsd");

    println!("cargo:rerun-if-changed=src/lib/backend/netbsd/netbsd.cc");
}

pub fn build_cxx_haiku() {
    cxx_build::bridge("src/lib/backend/bridge.rs")
        .file("src/lib/backend/haiku/haiku.cc")
        .std("c++17")
        .compile("libpci-rs-haiku");

    println!("cargo:rerun-if-changed=src/lib/backend/haiku/haiku.cc");
}
