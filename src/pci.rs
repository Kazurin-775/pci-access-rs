use crate::{config_space::ConfigSpace, sys};

pub struct PciConfigSpace {
    _private: (),
}

impl PciConfigSpace {
    pub const unsafe fn new() -> PciConfigSpace {
        PciConfigSpace { _private: () }
    }
}

impl ConfigSpace for PciConfigSpace {
    unsafe fn read_unchecked(&mut self, addr: u32) -> u32 {
        sys::pci_config_write_addr(addr);
        sys::pci_config_read_data()
    }

    unsafe fn write_unchecked(&mut self, addr: u32, data: u32) {
        sys::pci_config_write_addr(addr);
        sys::pci_config_write_data(data);
    }
}
