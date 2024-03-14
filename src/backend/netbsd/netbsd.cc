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

// This module is for BSD operating systems that share NetBSD's PCIIO calls,
// such as OpenBSD.

// Though NetBSD has libpci, this module more or less uses the same code
// the functions within libpci would use, and since OpenBSD doesn't have
// libpci, it makes more sense to not use it and keep this all in one module.

#include <iostream>
#include <vector>
#include <optional>

#ifdef __OpenBSD__
#include <sys/pciio.h>
#else
#include <dev/pci/pciio.h>
#endif

#include <sys/ioctl.h>
#include <sys/types.h>

#include <dev/pci/pcireg.h>
#include <dev/pci/pcidevs.h>
#include <dev/pci/pcidevs_data.h>

#include <unistd.h>
#include <errno.h>
#include <fcntl.h>

#include "libpci-rs/src/backend/include/common.h"

#ifdef __OpenBSD__
#define PCI_SUBSYS_VENDOR(x) PCI_VENDOR(x)
#define PCI_SUBSYS_ID(x) PCI_PRODUCT(x)
#define PCI_IOC_BDF_CFGREAD PCIOCREAD
#define PCI_DEV "/dev/pci"
#else
#define PCI_DEV "/dev/pci0"
#endif

int pci_fd;

int pci_read(int bus, int dev, int func, uint32_t reg, uint32_t *out) {
	int status;
#ifdef __OpenBSD__
	struct pci_io io = {};	
	io.pi_sel.pc_bus = bus;
	io.pi_sel.pc_dev = dev;
	io.pi_sel.pc_func = func;
	io.pi_reg = reg;
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
	uint32_t id_reg, class_reg, subsys_reg;
	
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
	for(int bus = 0; bus < 256; bus++) {
		for(int dev = 0; dev < 32; dev++) {
			int nfuncs;
			uint32_t hdr;

			// Find out how many functions the device has.
			if(pci_read(bus, dev, 0, PCI_BHLC_REG, &hdr) != 0)
				continue; // TODO: maybe handle better?

			nfuncs = PCI_HDRTYPE_MULTIFN(hdr) ? 8 : 1;

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

