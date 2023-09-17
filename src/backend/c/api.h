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
