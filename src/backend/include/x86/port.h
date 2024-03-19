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

#pragma once

#include <vector>
#include <stdint.h>

#include "libpci-rs/src/backend/include/common.h"
#include "libpci-rs/src/backend/bridge.rs.h"

// OS-specific stuff.
namespace os {
    void io_lock();
    void io_unlock();
}

namespace port {
    // x86 IN/OUT assembly instructions.
    void outb(unsigned char __val, unsigned short __port);
    void outw(unsigned short __val, unsigned short __port);
    void outl(unsigned int __val, unsigned short __port);
    unsigned char inb(unsigned short __port);
    unsigned short inw(unsigned short __port);
    unsigned int inl(unsigned short __port);

    bool pci_read_config(unsigned int domain, unsigned int bus, unsigned int fn,
                         int reg, int len, uint32_t *output);

    bool pci_write_config(unsigned int domain, unsigned int bus, unsigned int fn,
                          int reg, int len, uint32_t value);

    bool pci_access_check();

    CXXPciEnumerationError get_pci_list(rust::Vec<CXXPciDeviceHardware> &result);
}