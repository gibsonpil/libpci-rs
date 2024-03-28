// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

#pragma once
#include "libpci-rs/src/lib/backend/bridge.rs.h"
#include <stdint.h>

// Handy PCI information error macro.
#define PIE(x) static_cast<int>(x)

enum class PciInformationError {
    Unavailable = 1,
    PermissionDenied = 2,
    Unknown = 3,
};

CXXPciEnumerationError _get_pci_list(rust::Vec<CXXPciDeviceHardware> &output);
CXXPciDeviceHardware _get_field_availability();
