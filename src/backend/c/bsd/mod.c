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
