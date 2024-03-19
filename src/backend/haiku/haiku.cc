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

#include <fcntl.h>

#include "libpci-rs/src/backend/include/common.h"
#include "libpci-rs/src/backend/include/haiku/poke.h"

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    CXXPciEnumerationError result = CXXPciEnumerationError::Success;
    int fd;

    // Try to open the poke device.
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

    pci_info_args args = {};
    args.signature = POKE_SIGNATURE;

    for(int i = 0 ;; i++) {
        CXXPciDeviceHardware device;
        pci_info info = {};

        args.index = i;
        args.info = &info;

        ioctl(poke_driver_fd, POKE_GET_NTH_PCI_INFO, &args, sizeof(args));

        if(args.status == B_OK)
            break; // Assume we are done.

        device.bus = info.bus;
        device.device = info.device;
        device.function = info.function;

        device.vendor_id = info.vendor_id;
        device.device_id = info.device_id;
        device.class_id = info.class_base;
        device.subclass = info.class_sub;
        device.programming_interface = info.class_api;
        device.revision_id = info.revision;

        switch(info.header) {
            case 0:
                device.subsys_vendor_id = info.u.h0.subsystem_vendor_id;
                device.subsys_device_id = info.u.h0.subsystem_id;
                break;
            case 1:
                device.subsys_vendor_id = info.u.h1.subsystem_vendor_id;
                device.subsys_device_id = info.u.h1.subsystem_id;
                break;
            case 2:
                device.subsys_vendor_id = info.u.h2.subsystem_vendor_id;
                device.subsys_device_id = info.u.h2.subsystem_id;
                break;
        }

        output.push_back(device);
    }

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
