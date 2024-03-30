// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![allow(dead_code)]

macro_rules! build_platform {
    ($name:literal) => {
        cxx_build::bridge("src/lib/backend/bridge.rs")
            .file(concat!("src/lib/backend/", $name, "/", $name, ".cc"))
            .std("c++17")
            .warnings(true)
            .extra_warnings(true)
            .cargo_warnings(true)
            .compile(concat!("libpci-rs-", $name));

        println!(concat!("cargo:rerun-if-changed=", $name));
    };
}

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
    build_platform!("darwin");

    println!("cargo:rustc-flags=-l framework=CoreFoundation");
    println!("cargo:rustc-flags=-l framework=IOKit");
}

pub fn build_cxx_freebsd() {
    build_platform!("freebsd");
}

pub fn build_cxx_netbsd() {
    build_platform!("netbsd");
}

pub fn build_cxx_haiku() {
    build_platform!("haiku");
}
