use x86_64::instructions::port::{Port, PortWriteOnly};

const PORT_CONFIG_ADDR: u16 = 0xCF8;
const PORT_CONFIG_DATA: u16 = 0xCFC;

#[inline]
pub unsafe fn pci_config_write_addr(addr: u32) {
    PortWriteOnly::new(PORT_CONFIG_ADDR).write(addr);
}

#[inline]
pub unsafe fn pci_config_read_data() -> u32 {
    Port::new(PORT_CONFIG_DATA).read()
}

#[inline]
pub unsafe fn pci_config_write_data(data: u32) {
    Port::new(PORT_CONFIG_DATA).write(data);
}
