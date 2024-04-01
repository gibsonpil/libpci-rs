// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

// Basically none of these features are documented by Apple. Most of this info
// had to be obtained by looking at public source code and reading old forum
// posts. If you have experience with IOKit and have found anything incorrect in
// this file, please make a pull request!

#ifdef __APPLE__

#include <CoreFoundation/CoreFoundation.h>
#include <IOKit/IOKitLib.h>

#include <iostream>
#include <optional>
#include <stdint.h>
#include <string>
#include <vector>

#include "libpci-rs/src/lib/backend/include/common.h"

template <typename T> constexpr CFDataRef TO_CFDATAREF(T ref) {
    return static_cast<CFDataRef>(ref);
}

static constexpr int DARWIN_CLASS = 16;
static constexpr int DARWIN_SUBCLASS = 8;
static constexpr int EIGHT_BIT_MASK = 0xFF;

// Thanks for the pointless deprecation Apple.
#if(MAC_OS_X_VERSION_MAX_ALLOWED < 120000)
#define kIOMainPortDefault kIOMasterPortDefault
#endif

// __arm__ should be fairly standard on ARM, but
// it wasn't defined on my Macbook Air while __arm64__
// was, so I'm adding in this code just to be safe.
#if defined(__arm64__) && !defined(__arm__)
#define __arm__ // NOLINT: Defines __arm__ only on platforms where it should be defined
#endif

union IOPCIAddressSpace {
    UInt32 bits;
    struct {
#if __BIG_ENDIAN__
        unsigned int reloc       : 1;
        unsigned int prefetch    : 1;
        unsigned int t           : 1;
        unsigned int resv        : 3;
        unsigned int space       : 2;
        unsigned int busNum      : 8;
        unsigned int deviceNum   : 5;
        unsigned int functionNum : 3;
        unsigned int registerNum : 8;
#elif __LITTLE_ENDIAN__
        unsigned int registerNum : 8;
        unsigned int functionNum : 3;
        unsigned int deviceNum   : 5;
        unsigned int busNum      : 8;
        unsigned int space       : 2;
        unsigned int resv        : 3;
        unsigned int t           : 1;
        unsigned int prefetch    : 1;
        unsigned int reloc       : 1;
#endif
    } s;
    struct {
#if __BIG_ENDIAN__
        unsigned int resv                : 4;
        unsigned int registerNumExtended : 4;
        unsigned int busNum              : 8;
        unsigned int deviceNum           : 5;
        unsigned int functionNum         : 3;
        unsigned int registerNum         : 8;
#elif __LITTLE_ENDIAN__
        unsigned int registerNum         : 8;
        unsigned int functionNum         : 3;
        unsigned int deviceNum           : 5;
        unsigned int busNum              : 8;
        unsigned int registerNumExtended : 4;
        unsigned int resv                : 4;
#endif
    } es;
};

CFTypeRef get_property_type_ref(io_service_t service, const CFStringRef key) {
    return IORegistryEntrySearchCFProperty(service, kIOServicePlane, key, NULL, 0);
}

template <typename T> const T *get_property_ptr(io_service_t service, const CFStringRef key) {
    CFTypeRef type_ref = get_property_type_ref(service, key);

    if(type_ref == NULL) {
        return 0;
    } else if(CFGetTypeID(type_ref) != CFDataGetTypeID()) {
        CFRelease(type_ref);
        return 0;
    }

    // NOLINTNEXTLINE: reinterpret_cast is needed to handle opaque data.
    const T *data = reinterpret_cast<const T *>((CFDataGetBytePtr(TO_CFDATAREF(type_ref))));
    CFRelease(type_ref);
    return data;
}

template <typename T> const T get_property(io_service_t service, const CFStringRef key) {
    CFTypeRef type_ref = get_property_type_ref(service, key);

    if(type_ref == NULL) {
        return 0;
    } else if(CFGetTypeID(type_ref) != CFDataGetTypeID()) {
        CFRelease(type_ref);
        return 0;
    }

    // NOLINTNEXTLINE: reinterpret_cast is needed to handle opaque data.
    T data = *reinterpret_cast<const T *>((CFDataGetBytePtr(TO_CFDATAREF(type_ref))));
    CFRelease(type_ref);
    return data;
}

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output) {
    rust::Vec<CXXPciDeviceHardware> pci_devices;
    CFMutableDictionaryRef matching_dictionary = nullptr;
    io_service_t service = 0;
    io_iterator_t iter = 0;
    kern_return_t ret = 0;

    matching_dictionary = IOServiceMatching("IOPCIDevice");
    if(matching_dictionary == NULL) {
        return CXXPciEnumerationError::OsError;
    }

    ret = IOServiceGetMatchingServices(kIOMainPortDefault, matching_dictionary, &iter);
    if(ret != KERN_SUCCESS) {
        return CXXPciEnumerationError::OsError;
    }

    while((service = IOIteratorNext(iter)) != 0U) {
        CXXPciDeviceHardware device = {};

        device.vendor_id = get_property<uint16_t>(service, CFSTR("vendor-id"));
        // device-id seems to be 16-bit on some devices and 32-bit on others.
        // Regardles, the values inside never exceed 16-bits.
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
        device.class_id = (darwin_class_code >> DARWIN_CLASS) & EIGHT_BIT_MASK;
        device.subclass = (darwin_class_code >> DARWIN_SUBCLASS) & EIGHT_BIT_MASK;
        device.programming_interface = darwin_class_code & EIGHT_BIT_MASK;

        // Fetching BDF values only works on x86_64.
#ifndef __arm__
        const IOPCIAddressSpace *address =
            get_property_ptr<IOPCIAddressSpace>(service, CFSTR("reg"));
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

#endif // __APPLE__
