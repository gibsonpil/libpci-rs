#ifndef PCI_INFO_RS_API_H
#define PCI_INFO_RS_API_H

#include <stdint.h>

typedef struct pci_device {
    char *slot;
    char *label;
    uint16_t vendor_id, device_id;
    uint16_t sub_id, sub_vendor;
    uint16_t device_class;
    uint8_t revision_id;
} pci_device_t;

typedef struct pci_device_list {
    size_t len;
    pci_device_t *buffer;
} pci_device_list_t;

pci_device_list_t get_pci_list(void);

pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device);

#endif //PCI_INFO_RS_API_H
