// Copyright (c) 2023 NamedNeon. All rights reserved.
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

#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>
#include <fcntl.h>

#include <unistd.h>
#include <stdbool.h>

#include <sys/types.h>
#include <sys/pciio.h>
#include <sys/ioctl.h>

#ifdef __OpenBSD__
#include <errno.h>
#endif

#ifdef __FreeBSD__
#include <std/errno.h>
#endif

#include "../api.h"

typedef struct bsd_pci_handle {
	int file_desc;
} bsd_pci_handle_t;

bool check_access() {
	return access("/dev/pci", O_RDONLY) == 0;
}

void free_pci_handle(bsd_pci_handle_t* handle) {
	free(handle);
}

bsd_pci_handle_t* get_pci_handle() {
	bsd_pci_handle_t* handle = malloc(sizeof(bsd_pci_handle_t));
	
	handle->file_desc = open("/dev/pci", O_RDONLY);

	if(handle->file_desc == -1) {
		free_pci_handle(handle);
		return NULL;
	}

	return handle;
}

pci_device_stack_t get_pci_stack() {
	pci_device_stack_t result;
	bsd_pci_handle_t *handle = get_pci_handle();
	// TODO
	free_pci_handle(handle);
	return result;
}

pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device) {
	pci_device_t result;
	// TODO
	return result;
}
