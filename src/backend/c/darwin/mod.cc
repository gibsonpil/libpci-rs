#include <IOKit/IOKitLib.h>
#include <IOKit/IOKitKeys.h>
#include <Availability.h>

#include <vector>

extern "C" {
    #include "../api.h"
}

using namespace std;

extern "C" pci_device_stack_t get_pci_list(void) {
    vector<pci_device_stack_t> pci_device_vector;

    io_iterator_t iterator = IO_OBJECT_NULL;

    kern_return_t registry = IORegistryCreateIterator(
#if __MAC_OS_X_VERSION_MAX_ALLOWED >= MAC_OS_VERSION_12_0
            kIOMainPortDefault, // kIOMasterPortDefault is deprecated after macOS 12.0
#elif
            kIOMasterPortDefault,
#endif
            kIODeviceTreePlane, 0, &iterator);

    // Convert vector to conform to C api.
    pci_device_stack_t result;
    result.len = pci_device_vector.size();
    auto* buffer = (pci_device_t*) malloc(result.len);
    memcpy(buffer, pci_device_vector.data(), result.len);

    IOObjectRelease(iterator);

    return result;
}

extern "C" pci_device_t get_pci_by_id(uint16_t vendor, uint16_t device) {
    pci_device_t result;



    return result;
}
