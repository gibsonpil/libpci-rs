// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#ifdef __HAIKU__

#include <fcntl.h>
#include <iostream>

#include "libpci-rs/src/lib/backend/haiku/poke.h"
#include "libpci-rs/src/lib/backend/include/common.h"

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    int fd;

    // Try to open the poke device.
    fd = open(POKE_DEVICE_FULLNAME, O_RDWR);
    if(fd < 0) { // Error
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

    for(int i = 0;; i++) {
        CXXPciDeviceHardware device = {};
        pci_info info = {};

        args.index = i;
        args.info = &info;

        ioctl(fd, POKE_GET_NTH_PCI_INFO, &args, sizeof(args));

        if(args.status != B_OK)
            break; // Assume we hit the end.

        device.bus = info.bus;
        device.device = info.device;
        device.function = info.function;

        device.vendor_id = info.vendor_id;
        device.device_id = info.device_id;
        device.class_id = info.class_base;
        device.subclass = info.class_sub;
        device.programming_interface = info.class_api;
        device.revision_id = info.revision;

        switch(info.header_type) {
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

    close(fd);

    return CXXPciEnumerationError::Success;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware res = {};
    return res;
}

#endif // __HAIKU__
