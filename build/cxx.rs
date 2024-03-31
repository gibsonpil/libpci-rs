// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#![allow(dead_code)]

const SOURCE_FILES: &[&str] = &[
    "src/lib/backend/darwin/darwin.cc",
    "src/lib/backend/freebsd/freebsd.cc",
    "src/lib/backend/netbsd/netbsd.cc",
    "src/lib/backend/haiku/haiku.cc",
];

#[allow(unreachable_code)]
pub fn build_cxx_module() {
    cxx_build::bridge("src/lib/backend/bridge.rs")
        .files(SOURCE_FILES)
        .std("c++17")
        .warnings(true)
        .extra_warnings(true)
        .cargo_warnings(true)
        .compile("libpci-rs-backend");

    for file in SOURCE_FILES {
        println!("cargo:rerun-if-changed={}", file);
    }

    #[cfg(any(target_os = "macos", target_os = "ios"))]
    println!("cargo:rustc-flags=-l framework=CoreFoundation -l framework=IOKit");

    println!("cargo:rerun-if-changed=src/lib/backend/include/common.h");
}
