#ifndef LIBPCI_RS_API_H
#define LIBPCI_RS_API_H

#include <stdint.h>

typedef struct pci_device {
    uint32_t domain;
    uint8_t bus, device, function;
    uint16_t vendor_id, device_id;
    uint16_t subsys_device_id, subsys_vendor_id;
    uint8_t dev_class, subclass, programming_interface;
    char *label;
} pci_device_t;

typedef struct pci_device_stack {
    uint32_t len;
    pci_device_t *buffer;
} pci_device_stack_t;

// System specific functions.
pci_device_stack_t get_pci_stack(void);
pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device);

// General functions.
pci_device_stack_t create_pci_device_stack();
int pci_device_stack_push(pci_device_stack_t* stack, pci_device_t device);
pci_device_t pci_device_stack_pop(pci_device_stack_t* stack);
void free_pci_device_stack(pci_device_stack_t* stack);

#endif //LIBPCI_RS_API_H
