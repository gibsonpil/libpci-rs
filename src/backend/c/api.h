#ifndef LIBPCI_RS_API_H
#define LIBPCI_RS_API_H

#include <stdint.h>

typedef struct pci_device {
    uint32_t domain;
    uint8_t bus, device, function;
    uint16_t vendor_id, device_id;
    uint16_t subsys_device_id, subsys_vendor_id;
    uint32_t device_class;
    uint8_t revision_id;
    char *label;
} pci_device_t;

typedef struct pci_device_list {
    size_t len;
    pci_device_t *buffer;
} pci_device_list_t;

pci_device_list_t get_pci_list(void);

pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device);

#endif //LIBPCI_RS_API_H
