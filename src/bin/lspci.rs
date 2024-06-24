// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

//! # lspci
//! This `lspci` is a Rust reimplementation of the `lspci` from `libpci`,
//! using the `libpci-rs` backend. It offers a limited subset of the
//! functionality from the original `lspci`. See the [args](crate::Args)
//! section for usage information.

use std::collections::BTreeMap;

use clap::{command, Parser};
use libpci_rs::pci::*;

#[derive(Parser)]
#[command(version, about = "A reimplementation of lspci using libpci-rs.", long_about = None)]
struct Args {
    /// Verbosity (`-v`, `--verbose`): Increases the amount of supplementary
    /// info printed. Use multiple flags for more info.
    #[arg(short, long, help = "Verbosity (use more than once for more details)", action = clap::ArgAction::Count)]
    verbose: u8,
    /// Numeracy (`-n`, `--numeric`): Change the format of the output to use
    /// just numbers, or numbers and text. Use more flags for different
    /// formats.
    #[arg(short, long, help = "Numeracy (use more than once for different options)", action = clap::ArgAction::Count)]
    numeric: u8,
    /// Tree (`-t`, `--tree`): Display a tree view. Requires at least one
    /// device with an obtainable address.
    #[arg(short, long, help = "Display a tree view")]
    tree: bool,
}

// Tree
// \- Branch<Domain, Children>
//    \- Branch<Bus, Children>
//       \- Branch<Device, Children>
//          \- Leaf<Function, PciDeviceHardware>

// Key: Domain number, Value: Domain (contains all child buses)
type PciDeviceTree = BTreeMap<u32, TreeDomain>;
// Key: Bus number, Value: Bus (contains all child devices)
type TreeDomain = BTreeMap<u8, TreeBus>;
// Key: Device number, Value: Device (contains all child functions)
type TreeBus = BTreeMap<u8, TreeDevice>;
// Key: Function number, Value: PciDeviceHardware (struct representing the device)
type TreeDevice = BTreeMap<u8, PciDeviceHardware>;

fn tree_from_vec(devices: Vec<PciDeviceHardware>) -> PciDeviceTree {
    let mut tree: PciDeviceTree = BTreeMap::new();
    for device in devices {
        if let Some(address) = device.address {
            tree.entry(address.domain)
                .or_default()
                .entry(address.bus)
                .or_default()
                .entry(address.device)
                .or_default()
                .entry(address.function)
                .or_insert(device);
        }
    }
    tree
}

fn print_tree(tree: PciDeviceTree) {
    // WARNING: This code SUCKS
    // don't complain unless you're willing to fix it.
    // (please fix it.)
    // - shibedrill

    const OTHER_CHILD: &str = "│   "; // prefix: pipe
    const OTHER_ENTRY: &str = "├── "; // connector: tee
    const FINAL_CHILD: &str = "    "; // prefix: no more siblings
    const FINAL_ENTRY: &str = "└── "; // connector: elbow

    println!("PCI root:");

    for (domain_index, (domain_value, domain_entry)) in tree.clone().into_iter().enumerate() {
        let last_domain = domain_index == tree.len() - 1;
        print!("{}", {
            if last_domain {
                FINAL_ENTRY
            } else {
                OTHER_ENTRY
            }
        });
        println!("0x{:04x}:", domain_value);
        for (bus_index, (bus_value, bus_entry)) in domain_entry.clone().into_iter().enumerate() {
            let last_bus = bus_index == domain_entry.len() - 1;
            print!("{}", {
                if last_domain {
                    FINAL_CHILD
                } else {
                    OTHER_CHILD
                }
            });
            print!("{}", {
                if last_bus {
                    FINAL_ENTRY
                } else {
                    OTHER_ENTRY
                }
            });
            println!("0x{:02x}:", bus_value);
            for (device_index, (device_value, device_entry)) in
                bus_entry.clone().into_iter().enumerate()
            {
                let last_device = device_index == bus_entry.len() - 1;
                print!("{}", {
                    if last_domain {
                        FINAL_CHILD
                    } else {
                        OTHER_CHILD
                    }
                });
                print!("{}", {
                    if last_bus {
                        FINAL_CHILD
                    } else {
                        OTHER_CHILD
                    }
                });
                print!("{}", {
                    if last_device {
                        FINAL_ENTRY
                    } else {
                        OTHER_ENTRY
                    }
                });
                println!("0x{:02x}:", device_value);
                for (function_index, (function_value, function_entry)) in
                    device_entry.clone().into_iter().enumerate()
                {
                    let last_function = function_index == device_entry.len() - 1;
                    print!("{}", {
                        if last_domain {
                            FINAL_CHILD
                        } else {
                            OTHER_CHILD
                        }
                    });
                    print!("{}", {
                        if last_bus {
                            FINAL_CHILD
                        } else {
                            OTHER_CHILD
                        }
                    });
                    print!("{}", {
                        if last_device {
                            FINAL_CHILD
                        } else {
                            OTHER_CHILD
                        }
                    });
                    print!("{}", {
                        if last_function {
                            FINAL_ENTRY
                        } else {
                            OTHER_ENTRY
                        }
                    });
                    println!(
                        "0x{:01x}: [{:04x}:{:04x}]",
                        function_value, function_entry.vendor_id, function_entry.device_id,
                    );
                }
            }
        }
    }
}

// Numeric level zero.
// 0000:00:00.0 Subclassname [classsubclass]: Vendor Devicename (rev 01)
fn numeracy_0(device: PciDeviceHardware) -> String {
    format!(
        "{} {}: {} {} {}",
        if let Some(addr) = device.address {
            format!(
                "{:04x}:{:02x}:{:02x}.{:01x}",
                addr.domain, addr.bus, addr.device, addr.function
            )
        } else {
            "<address unavailable>".to_string()
        },
        device
            .subclass_name()
            .unwrap_or(device.class_name().unwrap_or(format!(
                "<unknown class {:02x}{:02x}>",
                device.class, device.subclass
            ))),
        device
            .vendor_name()
            .unwrap_or("<unknown vendor>".to_string()),
        device
            .subdevice_name()
            .unwrap_or(device.device_name().unwrap_or(format!(
                "<unknown device {:04x}:{:04x}>",
                device.vendor_id, device.device_id
            ))),
        if device.revision_id != 0 {
            format!("(rev {:02x})", device.revision_id)
        } else {
            "".to_string()
        }
    )
    .trim()
    .to_string()
}

// Numeric level one.
// 0000:00:00.0 classcode: vid:did (rev 01)
fn numeracy_1(device: PciDeviceHardware) -> String {
    format!(
        "{} {:02x}{:02x}: {:04x}:{:04x} {}",
        if let Some(addr) = device.address {
            format!(
                "{:04x}:{:02x}:{:02x}.{:01x}",
                addr.domain, addr.bus, addr.device, addr.function
            )
        } else {
            "<address unavailable>".to_string()
        },
        device.class,
        device.subclass,
        device.vendor_id,
        device.device_id,
        if device.revision_id != 0 {
            format!("(rev {:02x})", device.revision_id)
        } else {
            "".to_string()
        }
    )
    .trim()
    .to_string()
}
// Numeric level two.
// 0000:00:00.0 Subclassname [classsubclass]: Vendor Devicename (rev 01)
fn numeracy_2(device: PciDeviceHardware) -> String {
    format!(
        "{} {} [{:02x}{:02x}]: {} {} [{:04x}:{:04x}] {}",
        if let Some(addr) = device.address {
            format!(
                "{:04x}:{:02x}:{:02x}.{:01x}",
                addr.domain, addr.bus, addr.device, addr.function
            )
        } else {
            "<address unavailable>".to_string()
        },
        device
            .subclass_name()
            .unwrap_or(device.class_name().unwrap_or(format!(
                "<unknown class {:02x}{:02x}>",
                device.class, device.subclass
            ))),
        device.class,
        device.subclass,
        device
            .vendor_name()
            .unwrap_or("<unknown vendor>".to_string()),
        device
            .subdevice_name()
            .unwrap_or(device.device_name().unwrap_or(format!(
                "<unknown device {:04x}:{:04x}>",
                device.vendor_id, device.device_id
            ))),
        device.vendor_id,
        device.device_id,
        if device.revision_id != 0 {
            format!("(rev {:02x})", device.revision_id)
        } else {
            "".to_string()
        }
    )
    .trim()
    .to_string()
}

// Verbosity 0 does not exist, since it won't print anything in the second
// line. We just detect verbosity 0 and do nothing for the second line.
// Verbosity 1 includes basic software info.
//fn verbosity_1(device: PciDeviceSoftware) {
//
//}

fn main() {
    let args = Args::parse();

    let pci_list = get_pci_list();

    if let Ok(mut devices) = pci_list {
        devices.sort();

        // Depending on our arg, we choose a formatter
        let line1_formatter: fn(PciDeviceHardware) -> String = match args.numeric {
            0 => numeracy_0,
            1 => numeracy_1,
            2.. => numeracy_2,
        };

        //let line2_formatter: fn(PciDeviceHardware) -> String = match args.verbose {
        //    0 => verbosity_0,
        //    1 => verbosity_1,
        //    2.. => verbosity_3,
        //}

        match args.tree {
            true => {
                devices.retain(|dev| dev.address.is_some());
                assert!(
                    !devices.is_empty(),
                    "Error: No devices with accessible addresses."
                );
                let device_tree: PciDeviceTree = tree_from_vec(devices);
                print_tree(device_tree);
            }
            false => {
                for device in devices {
                    println!("{}", line1_formatter(device));
                }
            }
        }
    } else {
        println!(
            "Error getting PCI device information: {}",
            pci_list.unwrap_err()
        );
    }
}
