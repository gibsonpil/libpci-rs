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

// __arm__ should be fairly standard on ARM, but
// it wasn't defined on my Macbook Air while __arm64__
// was, so I'm adding in this code just to be safe.
#ifdef __arm64__
#define __arm__
#endif

union IOPCIAddressSpace {
    UInt32              bits;
    struct {
#if __BIG_ENDIAN__
        unsigned int    reloc:1;
        unsigned int    prefetch:1;
        unsigned int    t:1;
        unsigned int    resv:3;
        unsigned int    space:2;
        unsigned int    busNum:8;
        unsigned int    deviceNum:5;
        unsigned int    functionNum:3;
        unsigned int    registerNum:8;
#elif __LITTLE_ENDIAN__
        unsigned int    registerNum:8;
        unsigned int    functionNum:3;
        unsigned int    deviceNum:5;
        unsigned int    busNum:8;
        unsigned int    space:2;
        unsigned int    resv:3;
        unsigned int    t:1;
        unsigned int    prefetch:1;
        unsigned int    reloc:1;
#endif
    } s;
    struct {
#if __BIG_ENDIAN__
        unsigned int    resv:4;
        unsigned int    registerNumExtended:4;
        unsigned int    busNum:8;
        unsigned int    deviceNum:5;
        unsigned int    functionNum:3;
        unsigned int    registerNum:8;
#elif __LITTLE_ENDIAN__
        unsigned int    registerNum:8;
        unsigned int    functionNum:3;
        unsigned int    deviceNum:5;
        unsigned int    busNum:8;
        unsigned int    registerNumExtended:4;
        unsigned int    resv:4;
#endif
    } es;
};

CFTypeRef get_property_type_ref(io_service_t service, const CFStringRef key) {
    return IORegistryEntrySearchCFProperty(service, kIOServicePlane, key, NULL, 0);
}

template <typename T>
const T* get_property_ptr(io_service_t service, const CFStringRef key) {
    CFTypeRef type_ref = get_property_type_ref(service, key);

    // type_ref == NULL evaluates first, making this statement safe.
    if(type_ref == NULL || CFGetTypeID(type_ref) != CFDataGetTypeID())
        return NULL; // None of these properties are normally 0, so returning 0 is fine.
    
    const T* data = reinterpret_cast<const T*>((CFDataGetBytePtr(ToCFDataRef(type_ref))));
    CFRelease(type_ref);
    return data;
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

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    rust::Vec<CXXPciDeviceHardware> pci_devices; 
    CFMutableDictionaryRef matching_dictionary;
    io_service_t service;
    io_iterator_t iter;
    kern_return_t ret;
    
    matching_dictionary = IOServiceMatching("IOPCIDevice");
    if(matching_dictionary == NULL) {
        return CXXPciEnumerationError::OsError;
    }
    
    ret = IOServiceGetMatchingServices(kIOMainPortDefault, matching_dictionary, &iter);
    if(ret != KERN_SUCCESS) {
        return CXXPciEnumerationError::OsError;
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

	    // Fetching BDF values only works on x86_64.
#ifndef __arm__	
	    const IOPCIAddressSpace* address = get_property_ptr<IOPCIAddressSpace>(service, CFSTR("reg"));
	    device.bus = address->s.busNum;
	    device.device = address->s.deviceNum;
	    device.function = address->s.functionNum;
#endif

        output.push_back(device);
    }
    
    IOObjectRelease(iter);
    return CXXPciEnumerationError::Success;
}

CXXPciDeviceHardware _get_field_availability() {
    CXXPciDeviceHardware hardware = {};
#ifdef __arm__
    hardware.domain = PIE(PciInformationError::Unavailable);
    hardware.bus = PIE(PciInformationError::Unavailable);
    hardware.device = PIE(PciInformationError::Unavailable);
    hardware.function = PIE(PciInformationError::Unavailable);
#endif
    return hardware;
}

