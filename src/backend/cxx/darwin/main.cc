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

#define ToCFDataRef(x) static_cast<CFDataRef>(x)

#if (MAC_OS_X_VERSION_MAX_ALLOWED < 120000)
  #define kIOMainPortDefault kIOMasterPortDefault
#endif

// TODO: test on x64 macOS.

struct PCIDeviceHardware {
    uint32_t domain;
    uint8_t bus;
    uint8_t device;
    uint8_t function;
    uint16_t vendor_id;
    uint16_t device_id;
    uint16_t subsys_device_id;
    uint16_t subsys_vendor_id;
    uint8_t class_id;
    uint8_t subclass;
    uint8_t programming_interface;
    uint8_t revision_id;
};

const CFTypeRef get_property_type_ref(io_service_t service, const CFStringRef key) {
    return IORegistryEntrySearchCFProperty(service, kIOServicePlane, key, NULL, NULL);
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

std::vector<PCIDeviceHardware> _get_pci_devices() {
    std::vector<PCIDeviceHardware> pci_devices;
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
        PCIDeviceHardware device = {};
        
        device.vendor_id = get_property<uint16_t>(service, CFSTR("vendor-id"));
        // device-id seems to be 16-bit on some devices and 32-bit on others. Regardles,
        // the values inside never exceed 16-bits.
        device.device_id = get_property<uint16_t>(service, CFSTR("device-id"));
        device.subsys_device_id = get_property<uint16_t>(service, CFSTR("subsystem-id"));
        device.subsys_vendor_id = get_property<uint16_t>(service, CFSTR("subsystem-vendor-id"));
        device.revision_id = get_property<uint8_t>(service, CFSTR("revision-id"));
        
        // Darwin class codes have the following structure
        // TODO: Figure out the purpose of the other two values.
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

// Temporary testing code.
int main(int argc, const char * argv[]) {
    auto devices = _get_pci_devices();
    for(auto device : devices) {
        std::cout << "VID: " << device.vendor_id << " ";
        std::cout << "DID: " << device.device_id << " ";
        std::cout << "CID: " << device.class_id << " ";
        std::cout << "REV: " << device.revision_id << " ";
        std::cout << "SYV: " << device.subsys_vendor_id << " ";
        std::cout << std::endl;
    }
}
