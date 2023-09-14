pub mod backend;

#[cfg(test)]
mod tests {
    #[test]
    fn test_pci_listing() {
        let device_list = crate::backend::get_pci_list();
        for device in device_list {
            println!("{:#?}", device);
        }
    }
}