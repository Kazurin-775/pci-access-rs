use crate::{config_space::ConfigSpace, consts::*, types::ConfigAddr};

impl ConfigAddr {
    pub fn read_vendor_id(&self, config_space: &mut impl ConfigSpace) -> u16 {
        config_space.read_u16(self.with_offset(OFF_VENDOR_ID))
    }

    pub fn read_device_id(&self, config_space: &mut impl ConfigSpace) -> u16 {
        config_space.read_u16(self.with_offset(OFF_DEVICE_ID))
    }

    pub fn read_base_class(&self, config_space: &mut impl ConfigSpace) -> u8 {
        config_space.read_u8(self.with_offset(OFF_BASE_CLASS))
    }

    pub fn read_subclass(&self, config_space: &mut impl ConfigSpace) -> u8 {
        config_space.read_u8(self.with_offset(OFF_SUBCLASS))
    }

    pub fn read_header_type(&self, config_space: &mut impl ConfigSpace) -> u8 {
        config_space.read_u8(self.with_offset(OFF_HEADER_TYPE))
    }

    pub fn read_bridge_2nd_bus_num(&self, config_space: &mut impl ConfigSpace) -> u8 {
        config_space.read_u8(self.with_offset(OFF_SECONDARY_BUS))
    }
}
