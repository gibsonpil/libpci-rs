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

// Basically none of these features are documented by Apple. Most of this info had to be
// obtained by looking at public source code and reading old forum posts. If you have
// experience with IOKit and have found anything incorrect in this file, please make a pull request!

#include <CoreFoundation/CoreFoundation.h>
#include <IOKit/IOKitLib.h>

#include <stdint.h>
#include <optional>
#include <vector>
#include <string>
#include <iostream>

#include "libpci-rs/src/backend/include/common.h"

#define ToCFDataRef(x) static_cast<CFDataRef>(x)

#if (MAC_OS_X_VERSION_MAX_ALLOWED < 120000)
  #define kIOMainPortDefault kIOMasterPortDefault
#endif

/*
 * TODO: Test out on amd64 macOS. Fetching BDF values seems to be impossible on M1 Macs without
 * some kernel-level wizardry, but the situation may be different under amd64.
 */

CFTypeRef get_property_type_ref(io_service_t service, const CFStringRef key) {
    return IORegistryEntrySearchCFProperty(service, kIOServicePlane, key, NULL, 0);
}

template <typename T>
const T get_property(io_service_t service, const CFStringRef key) {
    CFTypeRef type_ref = get_property_type_ref(service, key);

    // type_ref == NULL evaluates first, making this statement safe.
    if(type_ref == NULL || CFGetTypeID(type_ref) != CFDataGetTypeID())
        return 0; // None of these properties are normally 0, so returning 0 is fine.
    
    T data = *reinterpret_cast<const T*>((CFDataGetBytePtr(ToCFDataRef(type_ref))));
    CFRelease(type_ref);
    return data;
}

rust::Vec<CXXPciDeviceHardware> _get_pci_list() {
    rust::Vec<CXXPciDeviceHardware> pci_devices; 
    CFMutableDictionaryRef matching_dictionary;
    io_service_t service;
    io_iterator_t iter;
    kern_return_t ret;
    
    matching_dictionary = IOServiceMatching("IOPCIDevice");
    if(matching_dictionary == NULL) {
        return pci_devices;
    }
    
    ret = IOServiceGetMatchingServices(kIOMainPortDefault, matching_dictionary, &iter);
    if(ret != KERN_SUCCESS) {
        return pci_devices;
    }
    
    while((service = IOIteratorNext(iter))) {
        CXXPciDeviceHardware device = {};
        
        device.vendor_id = get_property<uint16_t>(service, CFSTR("vendor-id"));
        // device-id seems to be 16-bit on some devices and 32-bit on others. Regardles,
        // the values inside never exceed 16-bits.
        device.device_id = get_property<uint16_t>(service, CFSTR("device-id"));
        device.subsys_device_id = get_property<uint16_t>(service, CFSTR("subsystem-id"));
        device.subsys_vendor_id = get_property<uint16_t>(service, CFSTR("subsystem-vendor-id"));
        device.revision_id = get_property<uint8_t>(service, CFSTR("revision-id"));
        
        // Darwin class codes have the following structure
        // TODO: Figure out the purpose of the other values.
        // 00 00 00 00
        // |  |  |  |
        // |  |  |  |-> Programming interface (probably)
        // |  |  |-> Subclass
        // |  |-> Class
        // |-> Unknown
        uint32_t darwin_class_code = get_property<uint32_t>(service, CFSTR("class-code"));
        device.class_id = (darwin_class_code >> 16) & 0xFF;
        device.subclass = (darwin_class_code >> 8) & 0xFF;
        device.programming_interface = darwin_class_code & 0xFF;
        
        pci_devices.push_back(device);
    }
    
    IOObjectRelease(iter);
    return pci_devices;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware hardware = {};

    hardware.vendor_id = 1;
    hardware.device_id = 1;
    hardware.subsys_device_id = 1;
    hardware.subsys_vendor_id = 1;
    hardware.revision_id = 1;
    hardware.class_id = 1;
    hardware.subclass = 1;
    hardware.programming_interface = 1;

    return hardware;
}

