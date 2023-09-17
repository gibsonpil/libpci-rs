// Stub for API on unknown systems.

#include <stdio.h>

#include "api.h"

pci_device_stack_t get_pci_stack(void) {
    pci_device_stack_t result = create_pci_device_stack();
    printf("Got call to stub function get_pci_stack.");
    return result;
}

pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device) {
    pci_device_t result;
    printf("Got call to stub function get_pci_by_id.");
    return result;
}
