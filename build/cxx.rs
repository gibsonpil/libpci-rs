// Copyright (c) 2024 Gibson Pilconis. All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
// 1. Redistributions of source code must retain the above copyright notice,
// this list of conditions and the following disclaimer.
//
// 2. Redistributions in binary form must reproduce the above copyright notice,
// this list of conditions and the following disclaimer in the documentation
// and/or other materials provided with the distribution.
//
// 3. Neither the name of the copyright holder nor the names of its contributors
// may be used to endorse or promote products derived from this software without
// specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
// AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
// IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
// ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE
// LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
// DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR
// SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER
// CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY,
// OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#![allow(dead_code)]

pub fn build_cxx_module() {
    cfg_if::cfg_if! {
        if #[cfg(any(target_os = "macos", target_os = "ios"))] { // Darwin targets.
            build_cxx_darwin();
        } else if #[cfg(any(target_os = "freebsd", target_os = "dragonfly"))] {
            build_cxx_freebsd();
        } else if #[cfg(any(traget_os = "netbsd", target_os = "openbsd"))] {
            build_cxx_netbsd();
        } else {
            panic!("No suitable CXX modules found. Cannot build.");
        }
    }
    println!("cargo:rerun-if-changed=src/backend/include/common.h");
}

pub fn build_cxx_darwin() {
    cxx_build::bridge("src/backend/bridge.rs")
        .file("src/backend/darwin/darwin.cc")
        .std("c++17")
        .compile("libpci-rs-darwin");

    println!("cargo:rerun-if-changed=src/backend/darwin/darwin.cc");
    println!("cargo:rustc-flags=-l framework=CoreFoundation");
    println!("cargo:rustc-flags=-l framework=IOKit");
}

pub fn build_cxx_freebsd() {
    cxx_build::bridge("src/backend/bridge.rs")
        .file("src/backend/freebsd/freebsd.cc")
        .std("c++17")
        .compile("libpci-rs-freebsd");

    println!("cargo:rerun-if-changed=src/backend/freebsd/freebsd.cc");
}

pub fn build_cxx_netbsd() {
    cxx_build::bridge("src/backend/bridge.rs")
        .file("src/backend/netbsd/netbsd.cc")
        .std("c++17")
        .compile("libpci-rs-netbsd");

    println!("cargo:rerun-if-changed=src/backend/netbsd/netbsd.cc");
}

