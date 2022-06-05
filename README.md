A freestanding Rust crate that helps kernel-mode Rust code access PCI configuration space, such as enumerating PCI devices and reading / writing their configuration registers.

This crate does not contain driver code for any specific PCI device.

## Example: device enumeration

```rs
let mut cfg = unsafe { pci_access::PciConfigSpace::new() };
cfg.enumerate_all(|header, cfg| {
    println!("Found PCI device at: {:?}", header);
    let vendor = header.read_vendor_id(cfg);
    let device = header.read_device_id(cfg);
    println!("  Vendor: {:04x}   Device: {:04x}", vendor, device);
});
```

## References

- [PCI - OSDev Wiki](https://wiki.osdev.org/PCI)
- [kos/pci.c at master · kfreezen/kos](https://github.com/kfreezen/kos/blob/master/src/pci.c)
