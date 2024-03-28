// Copyright (c) 2024 Gibson Pilconis, shibedrill, and contributors
// SPDX-License-Identifier: BSD-3-Clause

use std::collections::BTreeMap;

use clap::{command, Parser};
use libpci_rs::pci::*;

#[derive(Parser)]
#[command(version, about = "A reimplementation of lspci using libpci-rs.", long_about = None)]
struct Args {
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,
    #[arg(short, long, action = clap::ArgAction::Count)]
    numeric: u8,
    #[arg(short, long)]
    tree: bool,
}

/// Tree
/// \- Branch<Domain, Children>
///    \- Branch<Bus, Children>
///       \- Branch<Device, Children>
///          \- Leaf<Function, PciDeviceHardware>

type PciDeviceTree = BTreeMap<u32, TreeDomain>;
type TreeDomain = BTreeMap<u8, TreeBus>;
type TreeBus = BTreeMap<u8, TreeDevice>;
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
    // TODO: Add logic to draw box characters.
    // This whole thing sucks so I don't think I want to even begin
    // doing this. If I were using any data structure worth its weight
    // this would be easy, but I can't use a normal tree because my life
    // sucks.

    //const OTHER_CHILD: &str = "│   ";   // prefix: pipe
    //const OTHER_ENTRY: &str = "├── ";   // connector: tee
    //const FINAL_CHILD: &str = "    ";   // prefix: no more siblings
    //const FINAL_ENTRY: &str = "└── ";   // connector: elbow

    println!("PCI root");

    #[allow(unused_assignments)]
    let mut level: usize = 0;

    // Remove this once we add the thingy.
    #[allow(unused_variables)]
    for (domain_index, (domain_value, domain_entry)) in tree.clone().into_iter().enumerate() {
        level = 0;
        print!("{}", "  ".repeat(level));
        println!("{:04x}", domain_value);
        for (bus_index, (bus_value, bus_entry)) in domain_entry.clone().into_iter().enumerate() {
            level = 1;
            print!("{}", "  ".repeat(level));
            println!("{:02x}", bus_value);
            for (device_index, (device_value, device_entry)) in
                bus_entry.clone().into_iter().enumerate()
            {
                level = 2;
                print!("{}", "  ".repeat(level));
                println!("{:02x}", device_value);
                for (function_index, (function_value, function_entry)) in
                    device_entry.clone().into_iter().enumerate()
                {
                    level = 3;
                    print!("{}", "  ".repeat(level));
                    println!("{:01x} {}", function_value, numeracy_1(function_entry));
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

        match args.tree {
            true => {
                devices.retain(|dev| dev.address.is_some());
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
