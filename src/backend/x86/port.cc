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

#include "libpci-rs/src/backend/include/x86/port.h"

// This file facilitates direct reading and writing to PCI devices for operating systems
// that don't provide their own means to applications in the userspace (i.e. Haiku).
// The Linux kernel was my primary resource for the information needed for this file.
// https://github.com/torvalds/linux/blob/master/arch/x86/pci/direct.c

// NOTE: This file exclusively contains code for accessing PCI configuration space
// via method #1 as opposed to method #2, which has been deprecated for over 30 years.

#define PCI_ENABLE_BIT       0x80000000
#define PCI_CONFIG_PORT      0xCF8
#define PCI_CONFIG_DATA_PORT 0xCFC
#define PCI_CONFIG_ADDRESS(bus, fn, reg) \
	(PCI_ENABLE_BIT | ((reg & 0xF00) << 16) | (bus << 16) \
	| (fn << 8) | (reg & 0xFC))

namespace port {
    __attribute__((weak, always_inline))
    void outb(unsigned char __val, unsigned short __port) {
        __asm__ volatile ("outb %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((weak, always_inline))
    void outw(unsigned short __val, unsigned short __port) {
        __asm__ volatile ("outw %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((weak, always_inline))
    void outl(unsigned int __val, unsigned short __port) {
        __asm__ volatile ("outl %0,%1" : : "a" (__val), "dN" (__port));
    }

    __attribute__((weak, always_inline))
    unsigned char inb(unsigned short __port) {
        unsigned char __val;
        __asm__ volatile ("inb %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }

    __attribute__((weak, always_inline))
    unsigned short inw(unsigned short __port) {
        unsigned short __val;
        __asm__ volatile ("inw %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }

    __attribute__((weak, always_inline))
    unsigned int inl(unsigned short __port) {
        unsigned int __val;
        __asm__ volatile ("inl %1,%0" : "=a" (__val) : "dN" (__port));
        return __val;
    }

    bool pci_read_config(unsigned int domain, unsigned int bus, unsigned int fn, int reg, int len, uint32_t *output) {
        // Sanity check.
        if (domain || (bus > 255) || (dfn > 255) || (reg > 4095))
            return false;

        io_lock();

        // Select PCI device.
        outl(PCI_CONFIG_ADDRESS(bus, fn, reg), PCI_CONFIG_PORT);

        // Get the resultant data from the PCI config data port.
        switch(len) {
            case 1:
                *value = inb(PCI_CONFIG_DATA_PORT + (reg & 3));
                break;
            case 2:
                *value = inw(PCI_CONFIG_DATA_PORT + (reg & 2));
                break;
            case 4:
                *value = inl(PCI_CONFIG_DATA_PORT);
                break;
        }

        io_unlock();

        return true;
    }

    bool pci_write_config(unsigned int domain, unsigned int bus, unsigned int fn, int reg, int len, uint32_t value) {
        // Sanity check.
        if (domain || (bus > 255) || (dfn > 255) || (reg > 4095))
            return false;

        io_lock();

        // Select PCI device.
        outl(PCI_CONFIG_ADDRESS(bus, fn, reg), PCI_CONFIG_PORT);

        // Set the configuration data.
        switch(len) {
            case 1:
                outb((uint8_t)value, PCI_CONFIG_DATA_PORT + (reg & 3));
                break;
            case 2:
                outw((uint16_t)value, PCI_CONFIG_DATA_PORT + (reg & 2));
                break;
            case 4:
                outl((uint32_t)value, PCI_CONFIG_DATA_PORT);
                break;
        }

        io_unlock();

        return true;
    }

    bool pci_access_check() {
        unsigned int tmp;
        bool result = false;

        io_lock();

        outb(0x01, 0xCFB); // Apparently some old systems require this.

        // In order to see whether or not we can write to the PCI port, we are going to
        // store the original value stored there, write a new one and then get the value
        // the again to see if the value was actually changed. After that we will restore
        // the original value.
        tmp = inl(PCI_PORT); // Get PCI_PORT value.
        outl(PCI_ENABLE_BIT, PCI_PORT); // Write test value.
        if(inl(PCI_PORT) == PCI_ENABLE_BIT) // See if our test value was set.
            result = true;
        outl(tmp, PCI_PORT) // Restore the original value.

        io_unlock();

        return result;
    }

    // This function typically isn't going to be invoked directly as most operating systems
    // require extra operations to raise the IOPL level.
    CXXPciEnumerationError get_pci_list(rust::Vec<CXXPciDeviceHardware> &result) {
        if(!pci_access_check())
            return CXXPciEnumerationError::OsError;


    }
}
