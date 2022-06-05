use crate::{
    classes::{BridgeSubclass, PciClass},
    config_space::ConfigSpace,
    consts::*,
    types::ConfigAddr,
};

pub struct Enumerable<'a, C>
where
    C: ConfigSpace,
{
    inner: &'a mut C,
}

impl<'a, C> Enumerable<'a, C>
where
    C: ConfigSpace,
{
    pub fn new(inner: &'a mut C) -> Enumerable<'a, C> {
        Enumerable { inner }
    }

    pub fn enumerate_all(&mut self, mut callback: impl FnMut(ConfigAddr, &mut C)) {
        let root_header = ConfigAddr::new().with_enable(true);
        self.enumerate_bus(root_header, &mut callback);

        let header_type = root_header.read_header_type(self.inner);
        if header_type & HEADER_MULTI_FUNC != 0 {
            // multiple PCI host controllers
            for func in 1..=MAX_FUNC {
                let func_header = root_header.with_func(func);
                let vendor = func_header.read_vendor_id(self.inner);
                if vendor == VEN_INVALID {
                    break;
                }

                self.enumerate_bus(root_header.with_bus(func), &mut callback);
            }
        }
    }

    fn enumerate_bus(&mut self, header: ConfigAddr, callback: &mut impl FnMut(ConfigAddr, &mut C)) {
        for dev in 0..=MAX_DEV {
            self.enumerate_dev(header.with_dev(dev), callback);
        }
    }

    fn enumerate_dev(&mut self, header: ConfigAddr, callback: &mut impl FnMut(ConfigAddr, &mut C)) {
        // We assume that `addr` is valid, and its `func` and `offset` is set to 0.
        let vendor = header.read_vendor_id(self.inner);
        if vendor == VEN_INVALID {
            return;
        }

        self.enumerate_func(header, callback);
        let header_type = header.read_header_type(self.inner);
        if header_type & HEADER_MULTI_FUNC != 0 {
            for func in 1..=MAX_FUNC {
                let func_header = header.with_func(func);
                if func_header.read_vendor_id(self.inner) != VEN_INVALID {
                    self.enumerate_func(func_header, callback);
                }
            }
        }
    }

    fn enumerate_func(
        &mut self,
        header: ConfigAddr,
        callback: &mut impl FnMut(ConfigAddr, &mut C),
    ) {
        callback(header, self.inner);
        let base_class = header.read_base_class(self.inner);
        let subclass = header.read_subclass(self.inner);
        if base_class == PciClass::BridgeDevice as u8
            && subclass == BridgeSubclass::PciToPciBridge as u8
        {
            // PCI-to-PCI bridge
            let secondary_bus = header.read_bridge_2nd_bus_num(self.inner);
            self.enumerate_bus(
                ConfigAddr::new().with_bus(secondary_bus).with_enable(true),
                callback,
            );
        }
    }
}
