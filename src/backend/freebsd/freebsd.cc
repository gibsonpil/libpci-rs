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

// This module is for BSD operating systems that share FreeBSD's PCIIO calls,
// such as DragonFlyBSD.

#include <unistd.h>
#include <iostream>

#include <sys/fcntl.h>
#include <sys/pciio.h>

#include "libpci-rs/src/backend/include/common.h"

#define CONF_SIZE 512

rust::Vec<CXXPciDeviceHardware> _get_pci_list() {
	rust::Vec<CXXPciDeviceHardware> devices;
	struct pci_conf_io pc = {};
	int fd;

	// Official FreeBSD utilities (i.e. pciconf) deal with accomodating
	// variable PCI device counts by simply allocating a generously
	// large array, so we will simply do the same in lieu of a syscall
	// to get the needed buffer size ahead of time.
	struct pci_conf conf[CONF_SIZE], *device;

	fd = open("/dev/pci", O_RDONLY, 0);
	if(fd < 0) { // Catch errors.
		return {};
	}

	pc.match_buf_len = CONF_SIZE;
	pc.matches = conf;

	do {
		if(ioctl(fd, PCIOCGETCONF, &pc) == -1) {
			return {};
		}

		if(pc.status == PCI_GETCONF_LIST_CHANGED) {
			// Close the file descriptor and start over.
			close(fd);
			return _get_pci_list(); 
		} else if(pc.status == PCI_GETCONF_ERROR) {
			return {};
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

			devices.push_back(d);
		}
	} while(pc.status == PCI_GETCONF_MORE_DEVS);

	close(fd);

	return devices;
}

CXXPciDeviceHardware _get_field_availability() {
	CXXPciDeviceHardware hardware = {};
	return hardware;
}

