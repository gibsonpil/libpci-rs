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

// This backend should work on versions of the Haiku operating system compiled
// after June of 2021 thanks to a change being made that automatically raises
// the process's IOPL when the poke driver is opened. In the interest of
// not adding in unnecessary code, the old way of calling the driver using ioctl()
// is being omitted, and conventional x86 IN/OUT instructions are being used
// instead. https://review.haiku-os.org/c/haiku/+/1077

#include <fcntl.h>

#include "libpci-rs/src/backend/include/common.h"
#include "libpci-rs/src/backend/include/x86/port.h"

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    CXXPciEnumerationError result = CXXPciEnumerationError::Success;
    rust::Vec<CXXPciDeviceHardware> &tmp;
    int fd;

    // Try to elevate our IOPL by opening the poke device.
    fd = open("/dev/misc/poke", O_RDWR);
    if(fd < 0) { // Error
        // Here we should simply return instead of going to ret since
        // closing a non-existent file descriptor isn't a good idea.
        if(errno == EACCES) {
            return CXXPciEnumerationError::PermissionDenied;
        } else if(errno == ENOENT) {
            return CXXPciEnumerationError::NotFound;
        } else {
            return CXXPciEnumerationError::OsError;
        }
    }

    // See if we can access the PCI configuration space.
    if(!port::pci_access_check()) {
        result = CXXPciEnumerationError::OsError;
        goto ret;
    }

    // Try to read the PCI list.
    if(port::get_pci_list(&tmp) != CXXPciEnumerationError::Success) {
        result = CXXPciEnumerationError::OsError;
        goto ret;
    }

    // Copy temporary vector to output buffer.
    std::copy(tmp.begin(), tmp.end(), std::back_inserter(output));

ret:
    // Cleanup.
    close(fd);
    return result;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware res = {}
    return res;
}

// TODO: pciutils doesn't seem to do any kind of locking while reading IO on Haiku.
// Frankly, I'm curious to know how exactly that doesn't lead to a race condition,
// and this code should NOT be shipped as "stable" until that is figured out.
namespace os {
    void io_lock() {}
    void io_unlock() {}
}
