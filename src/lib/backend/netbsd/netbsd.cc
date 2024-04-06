// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

// This module is for BSD operating systems that share NetBSD's PCIIO calls,
// such as OpenBSD.

// Though NetBSD has libpci, this module more or less uses the same code
// the functions within libpci would use, and since OpenBSD doesn't have
// libpci, it makes more sense to not use it and keep this all in one module.

#if defined(__NetBSD__) || defined(__OpenBSD__)

#include <iostream>
#include <optional>
#include <vector>

#ifdef __OpenBSD__
#include <sys/pciio.h>
#else
#include <dev/pci/pciio.h>
#endif

#include <sys/ioctl.h>
#include <sys/types.h>

// OpenBSD freaks out if pcireg.h isn't included first.
// clang-format off
#include <dev/pci/pcireg.h>
#include <dev/pci/pcidevs.h>
#include <dev/pci/pcidevs_data.h>
// clang-format on

#include <errno.h>
#include <fcntl.h>
#include <unistd.h>

#include "libpci-rs/src/lib/backend/include/common.h"

// TODO: handle multiple domains

#ifdef __OpenBSD__
constexpr uint32_t PCI_SUBSYS_VENDOR(uint32_t id) { return PCI_VENDOR(id); }
constexpr uint32_t PCI_SUBSYS_ID(uint32_t id) { return PCI_PRODUCT(id); }
constexpr auto PCI_IOC_BDF_CFGREAD = PCIOCREAD;
constexpr auto PCI_DEV = "/dev/pci";
#else
constexpr auto PCI_DEV = "/dev/pci0";
#endif

constexpr auto PCI_BUS_LENGTH = 256;
constexpr auto PCI_DEVICE_LENGTH = 32;

static int pci_fd; // NOLINT

int pci_read(int bus, int dev, int func, uint32_t reg, uint32_t *out) {
    int status = 0;
#ifdef __OpenBSD__
    struct pci_io io = {};
    io.pi_sel.pc_bus = bus;
    io.pi_sel.pc_dev = dev;
    io.pi_sel.pc_func = func;
    io.pi_reg = static_cast<int>(reg);
    io.pi_width = 4;
#else
    struct pciio_bdf_cfgreg io = {};
    io.bus = bus;
    io.device = dev;
    io.function = func;
    io.cfgreg.reg = reg;
#endif
    status = ioctl(pci_fd, PCI_IOC_BDF_CFGREAD, &io);
    if(status != 0)
        return status;
#ifdef __OpenBSD__
    *out = io.pi_data;
#else
    *out = io.cfgreg.val;
#endif

    return 0;
}

std::optional<CXXPciDeviceHardware> pci_read_info(int bus, int dev, int func) {
    CXXPciDeviceHardware device = {};
    uint32_t id_reg = 0;
    uint32_t class_reg = 0;
    uint32_t subsys_reg = 0;

    if(pci_read(bus, dev, func, PCI_ID_REG, &id_reg) != 0)
        return {}; // TODO: treat as error.

    if(PCI_VENDOR(id_reg) == PCI_VENDOR_INVALID || PCI_VENDOR(id_reg) == 0)
        return {}; // The device doesn't exist.

    if(pci_read(bus, dev, func, PCI_CLASS_REG, &class_reg) != 0)
        return {}; // TODO: treat as error.

    if(pci_read(bus, dev, func, PCI_SUBSYS_ID_REG, &subsys_reg) != 0)
        return {}; // TODO: treat as error.

    device.vendor_id = PCI_VENDOR(id_reg);
    device.device_id = PCI_PRODUCT(id_reg);
    device.subsys_vendor_id = PCI_SUBSYS_VENDOR(subsys_reg);
    device.subsys_device_id = PCI_SUBSYS_ID(subsys_reg);
    device.class_id = PCI_CLASS(class_reg);
    device.subclass = PCI_SUBCLASS(class_reg);
    device.programming_interface = PCI_INTERFACE(class_reg);
    device.revision_id = PCI_REVISION(class_reg);

    device.bus = bus;
    device.device = dev;
    device.function = func;

    return device;
}

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    pci_fd = open(PCI_DEV, O_RDONLY);

    if(pci_fd < 0) {
        if(errno == EACCES) {
            return CXXPciEnumerationError::PermissionDenied;
        } else if(errno == ENOENT) {
            return CXXPciEnumerationError::NotFound;
        } else {
            return CXXPciEnumerationError::OsError;
        }
    }

    // Though this method of discovering PCI devices may seem kind of dumb,
    // it is what the NetBSD developers used in pcictl, so it is kosher.
    for(int bus = 0; bus < PCI_BUS_LENGTH; bus++) {
        for(int dev = 0; dev < PCI_DEVICE_LENGTH; dev++) {
            int nfuncs = 0;
            uint32_t hdr = 0;

            // Find out how many functions the device has.
            if(pci_read(bus, dev, 0, PCI_BHLC_REG, &hdr) != 0)
                continue; // TODO: maybe handle better?

            // TODO: figure out the purpose of the magic number 8 and
            // create a named constant for it.
            nfuncs = PCI_HDRTYPE_MULTIFN(hdr) ? 8 : 1; // NOLINT

            for(int func = 0; func < nfuncs; func++) {
                auto info = pci_read_info(bus, dev, func);
                if(info)
                    output.push_back(info.value());
            }
        }
    }

    close(pci_fd);

    return CXXPciEnumerationError::Success;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware hardware = {};
    return hardware;
}

#endif // defined(__NetBSD__) || defined(__OpenBSD__)
