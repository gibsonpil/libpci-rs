// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

// This module is for BSD operating systems that share FreeBSD's PCIIO calls,
// such as DragonFlyBSD.

#if defined(__FreeBSD__) || defined(__DragonFly__)
#define MODULE_FOUND

#include <errno.h>
#include <iostream>
#include <unistd.h>

#include <sys/fcntl.h>
#include <sys/pciio.h>

#include "libpci-rs/src/lib/backend/include/common.h"

#define CONF_SIZE 512

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    struct pci_conf_io pc = {};
    int fd;

    // Official FreeBSD utilities (i.e. pciconf) deal with accomodating
    // variable PCI device counts by simply allocating a generously
    // large array, so we will simply do the same in lieu of a syscall
    // to get the needed buffer size ahead of time.
    struct pci_conf conf[CONF_SIZE], *device;

    fd = open("/dev/pci", O_RDONLY, 0);
    if(fd < 0) { // Catch errors.
        if(errno == EACCES) {
            return CXXPciEnumerationError::PermissionDenied;
        } else if(errno == ENOENT) {
            return CXXPciEnumerationError::NotFound;
        } else {
            return CXXPciEnumerationError::OsError;
        }
    }

    pc.match_buf_len = CONF_SIZE;
    pc.matches = conf;

    do {
        if(ioctl(fd, PCIOCGETCONF, &pc) == -1) {
            return CXXPciEnumerationError::OsError;
        }

        if(pc.status == PCI_GETCONF_LIST_CHANGED) {
            // Close the file descriptor and start over.
            close(fd);
            return _get_pci_list(output);
        } else if(pc.status == PCI_GETCONF_ERROR) {
            return CXXPciEnumerationError::OsError;
        }

        for(device = conf; device < &conf[pc.num_matches]; device++) {
            CXXPciDeviceHardware d;

            d.vendor_id = device->pc_vendor;
            d.device_id = device->pc_device;
            d.subsys_device_id = device->pc_subdevice;
            d.subsys_vendor_id = device->pc_subvendor;
            d.revision_id = device->pc_revid;
            d.class_id = device->pc_class;
            d.subclass = device->pc_subclass;
            d.programming_interface = device->pc_progif;
            d.domain = device->pc_sel.pc_domain;
            d.bus = device->pc_sel.pc_bus;
            d.device = device->pc_sel.pc_dev;
            d.function = device->pc_sel.pc_func;

            output.push_back(d);
        }
    } while(pc.status == PCI_GETCONF_MORE_DEVS);

    close(fd);

    return CXXPciEnumerationError::Success;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware hardware = {};
    return hardware;
}

#endif // __FreeBSD__ || __DragonFly__
